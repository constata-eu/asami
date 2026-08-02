use std::cmp::min;

use anyhow::{bail, Context};
use ethers::{
    abi::{decode, ParamType, Token},
    contract::abigen,
    providers::Middleware,
    types::{Address, BlockNumber, Filter, Log, H256, U256},
    utils::keccak256,
};
use sqlx::{Postgres, Row, Transaction};

use crate::{config::ZERO_ADDRESS, db, App};

abigen!(
    AsamiContract,
    r#"[
      function automator() view returns (address)
      function disburseNative(address recipient, uint256 amount)
      function disburseToken(address token, address recipient, uint256 amount)
      event MemberUpdated(uint256 indexed memberId, address indexed account, uint96 score, string name)
      event AssetConfigUpdated(address indexed asset, bool enabled, uint256 minimumAmount)
      event SenderNameUpdated(address indexed account, string name)
      event NativeFundsReceived(address indexed sender, uint256 amount)
      event PaymentSent(address indexed asset, address indexed recipient, uint256 amount)
    ]"#
);

abigen!(
    Erc20Contract,
    r#"[
      function balanceOf(address account) view returns (uint256)
      event Transfer(address indexed from, address indexed to, uint256 value)
    ]"#
);

pub async fn index_once(app: &App) -> anyhow::Result<usize> {
    let stream = "asami_events";
    let row = sqlx::query(
        "INSERT INTO asami_indexer_states(chain_id, stream, next_block)
         VALUES ($1, $2, $3)
         ON CONFLICT(chain_id, stream) DO UPDATE SET stream = EXCLUDED.stream
         RETURNING next_block, last_block_hash",
    )
    .bind(app.settings.chain_id as i64)
    .bind(stream)
    .bind(app.settings.contract_deploy_block as i64)
    .fetch_one(&app.db)
    .await?;
    let next_block: i64 = row.get(0);
    let last_hash: Option<String> = row.get(1);

    if next_block as u64 > app.settings.contract_deploy_block {
        if let Some(expected) = last_hash {
            let previous =
                app.provider.get_block((next_block as u64) - 1).await?.context("last indexed block disappeared")?;
            let actual = previous.hash.context("last indexed block has no hash")?;
            if format!("{actual:#x}") != expected {
                db::log(
                    &app.db,
                    "fail",
                    "indexer",
                    "deep_reorganization",
                    "Finalized block hash changed; indexing halted for operator review.",
                    Some(app.settings.chain_id),
                    None,
                )
                .await;
                bail!("finalized block hash mismatch");
            }
        }
    }

    let head = app.provider.get_block_number().await?.as_u64();
    let finalized = head.saturating_sub(app.settings.finality_depth);
    if next_block as u64 > finalized {
        return Ok(0);
    }
    let from = next_block as u64;
    let to = min(
        finalized,
        from.saturating_add(app.settings.block_chunk_size.saturating_sub(1)),
    );

    let contract_filter = Filter::new()
        .address(app.settings.contract_address)
        .from_block(BlockNumber::Number(from.into()))
        .to_block(BlockNumber::Number(to.into()));
    let transfer_filter = Filter::new()
        .from_block(BlockNumber::Number(from.into()))
        .to_block(BlockNumber::Number(to.into()))
        .topic0(H256::from(keccak256("Transfer(address,address,uint256)")))
        .topic2(address_topic(app.settings.contract_address));

    let mut contract_logs = app.provider.get_logs(&contract_filter).await?;
    let transfer_logs = app.provider.get_logs(&transfer_filter).await?;
    contract_logs.extend(transfer_logs);
    contract_logs.sort_by_key(|log| {
        (
            log.block_number.unwrap_or_default(),
            log.transaction_index.unwrap_or_default(),
            log.log_index.unwrap_or_default(),
        )
    });

    let mut tx = app.db.begin().await?;
    for log in &contract_logs {
        insert_log(&mut tx, app, log).await?;
    }
    let block = app.provider.get_block(to).await?.context("final indexed block unavailable")?;
    let hash = block.hash.context("final indexed block has no hash")?;
    sqlx::query(
        "UPDATE asami_indexer_states
         SET next_block = $3, last_block_hash = $4, updated_at = now()
         WHERE chain_id = $1 AND stream = $2",
    )
    .bind(app.settings.chain_id as i64)
    .bind(stream)
    .bind((to + 1) as i64)
    .bind(format!("{hash:#x}"))
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(contract_logs.len())
}

async fn insert_log(tx: &mut Transaction<'_, Postgres>, app: &App, log: &Log) -> anyhow::Result<()> {
    let topic = match log.topics.first() {
        Some(topic) => *topic,
        None => return Ok(()),
    };
    let block_number = log.block_number.context("log block number")?.as_u64() as i64;
    let block_hash = format!("{:#x}", log.block_hash.context("log block hash")?);
    let transaction_hash = format!("{:#x}", log.transaction_hash.context("log transaction hash")?);
    let transaction_index = log.transaction_index.context("log transaction index")?.as_u64() as i64;
    let log_index = log.log_index.context("log index")?.as_u64() as i64;
    let chain_id = app.settings.chain_id as i64;

    if topic == signature("MemberUpdated(uint256,address,uint96,string)") {
        let member_id = topic_u256(log, 1)?;
        let account = topic_address(log, 2)?;
        let values = decode(&[ParamType::Uint(96), ParamType::String], &log.data)?;
        let score = token_uint(&values[0])?;
        let name = token_string(&values[1])?;
        sqlx::query(
            "INSERT INTO member_update_events
             (chain_id, member_id, account, score, name, block_number, block_hash,
              transaction_hash, transaction_index, log_index)
             VALUES ($1, $2::numeric, $3, $4::numeric, $5, $6, $7, $8, $9, $10)
             ON CONFLICT DO NOTHING",
        )
        .bind(chain_id)
        .bind(member_id.to_string())
        .bind(format!("{account:#x}"))
        .bind(score.to_string())
        .bind(name)
        .bind(block_number)
        .bind(&block_hash)
        .bind(&transaction_hash)
        .bind(transaction_index)
        .bind(log_index)
        .execute(&mut **tx)
        .await?;
    } else if topic == signature("AssetConfigUpdated(address,bool,uint256)") {
        let asset = topic_address(log, 1)?;
        let values = decode(&[ParamType::Bool, ParamType::Uint(256)], &log.data)?;
        let enabled = values[0].clone().into_bool().context("enabled")?;
        let minimum = token_uint(&values[1])?;
        sqlx::query(
            "INSERT INTO asset_config_update_events
             (chain_id, asset_address, enabled, minimum_amount, block_number, block_hash,
              transaction_hash, transaction_index, log_index)
             VALUES ($1, $2, $3, $4::numeric, $5, $6, $7, $8, $9)
             ON CONFLICT DO NOTHING",
        )
        .bind(chain_id)
        .bind(format!("{asset:#x}"))
        .bind(enabled)
        .bind(minimum.to_string())
        .bind(block_number)
        .bind(&block_hash)
        .bind(&transaction_hash)
        .bind(transaction_index)
        .bind(log_index)
        .execute(&mut **tx)
        .await?;
    } else if topic == signature("SenderNameUpdated(address,string)") {
        let account = topic_address(log, 1)?;
        let values = decode(&[ParamType::String], &log.data)?;
        sqlx::query(
            "INSERT INTO sender_name_update_events
             (chain_id, account, name, block_number, block_hash, transaction_hash,
              transaction_index, log_index)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             ON CONFLICT DO NOTHING",
        )
        .bind(chain_id)
        .bind(format!("{account:#x}"))
        .bind(token_string(&values[0])?)
        .bind(block_number)
        .bind(&block_hash)
        .bind(&transaction_hash)
        .bind(transaction_index)
        .bind(log_index)
        .execute(&mut **tx)
        .await?;
    } else if topic == signature("NativeFundsReceived(address,uint256)") {
        let sender = topic_address(log, 1)?;
        let values = decode(&[ParamType::Uint(256)], &log.data)?;
        let amount = token_uint(&values[0])?;
        sqlx::query(
            "INSERT INTO native_receipt_events
             (chain_id, sender, amount, block_number, block_hash, transaction_hash,
              transaction_index, log_index)
             VALUES ($1, $2, $3::numeric, $4, $5, $6, $7, $8)
             ON CONFLICT DO NOTHING",
        )
        .bind(chain_id)
        .bind(format!("{sender:#x}"))
        .bind(amount.to_string())
        .bind(block_number)
        .bind(&block_hash)
        .bind(&transaction_hash)
        .bind(transaction_index)
        .bind(log_index)
        .execute(&mut **tx)
        .await?;
        insert_incoming(
            tx,
            chain_id,
            ZERO_ADDRESS,
            &format!("{sender:#x}"),
            &format!("{:#x}", app.settings.contract_address),
            amount,
            &transaction_hash,
            log_index,
            block_number,
            &block_hash,
            transaction_index,
        )
        .await?;
    } else if topic == signature("PaymentSent(address,address,uint256)") {
        let asset = topic_address(log, 1)?;
        let recipient = topic_address(log, 2)?;
        let values = decode(&[ParamType::Uint(256)], &log.data)?;
        let amount = token_uint(&values[0])?;
        sqlx::query(
            "INSERT INTO payment_sent_events
             (chain_id, asset_address, recipient, amount, block_number, block_hash,
              transaction_hash, transaction_index, log_index)
             VALUES ($1, $2, $3, $4::numeric, $5, $6, $7, $8, $9)
             ON CONFLICT DO NOTHING",
        )
        .bind(chain_id)
        .bind(format!("{asset:#x}"))
        .bind(format!("{recipient:#x}"))
        .bind(amount.to_string())
        .bind(block_number)
        .bind(&block_hash)
        .bind(&transaction_hash)
        .bind(transaction_index)
        .bind(log_index)
        .execute(&mut **tx)
        .await?;
    } else if topic == signature("Transfer(address,address,uint256)") && log.address != app.settings.contract_address {
        let sender = topic_address(log, 1)?;
        let recipient = topic_address(log, 2)?;
        if recipient != app.settings.contract_address {
            return Ok(());
        }
        let values = decode(&[ParamType::Uint(256)], &log.data)?;
        insert_incoming(
            tx,
            chain_id,
            &format!("{:#x}", log.address),
            &format!("{sender:#x}"),
            &format!("{recipient:#x}"),
            token_uint(&values[0])?,
            &transaction_hash,
            log_index,
            block_number,
            &block_hash,
            transaction_index,
        )
        .await?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn insert_incoming(
    tx: &mut Transaction<'_, Postgres>,
    chain_id: i64,
    asset: &str,
    sender: &str,
    recipient: &str,
    amount: U256,
    transaction_hash: &str,
    log_index: i64,
    block_number: i64,
    block_hash: &str,
    transaction_index: i64,
) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO incoming_transfers
         (chain_id, asset_address, sender, recipient, amount, transaction_hash,
          log_index, block_number, block_hash, transaction_index)
         VALUES ($1, $2, $3, $4, $5::numeric, $6, $7, $8, $9, $10)
         ON CONFLICT DO NOTHING",
    )
    .bind(chain_id)
    .bind(asset)
    .bind(sender)
    .bind(recipient)
    .bind(amount.to_string())
    .bind(transaction_hash)
    .bind(log_index)
    .bind(block_number)
    .bind(block_hash)
    .bind(transaction_index)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

fn signature(value: &str) -> H256 {
    H256::from(keccak256(value))
}

fn address_topic(address: Address) -> H256 {
    let mut value = [0_u8; 32];
    value[12..].copy_from_slice(address.as_bytes());
    H256::from(value)
}

fn topic_address(log: &Log, index: usize) -> anyhow::Result<Address> {
    let topic = log.topics.get(index).context("missing address topic")?;
    Ok(Address::from_slice(&topic.as_bytes()[12..]))
}

fn topic_u256(log: &Log, index: usize) -> anyhow::Result<U256> {
    let topic = log.topics.get(index).context("missing uint topic")?;
    Ok(U256::from_big_endian(topic.as_bytes()))
}

fn token_uint(token: &Token) -> anyhow::Result<U256> {
    token.clone().into_uint().context("expected uint")
}

fn token_string(token: &Token) -> anyhow::Result<String> {
    token.clone().into_string().context("expected string")
}
