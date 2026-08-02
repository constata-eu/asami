use std::str::FromStr;

use anyhow::Context;
use ethers::{
    providers::{Middleware, ProviderError, RpcError},
    signers::{LocalWallet, Signer},
    types::{Address, Bytes, H256, U256, U512},
    utils::keccak256,
};
use sqlx::{PgPool, Row};

use crate::{
    chain::{AsamiContract, Erc20Contract},
    config::ZERO_ADDRESS,
    db, notifications, App,
};

#[derive(Debug)]
struct Incoming {
    id: i64,
    asset: String,
    sender: String,
    amount: U256,
}

#[derive(Debug)]
struct Member {
    id: String,
    account: String,
    name: String,
    score: U256,
}

pub async fn convert_transfers(app: &App) -> anyhow::Result<usize> {
    let rows = sqlx::query(
        "SELECT id, asset_address, sender, amount::text
         FROM incoming_transfers
         WHERE disbursement_id IS NULL
           AND (
             processing_status = 'unprocessed'
             OR id IN (
               SELECT id FROM incoming_transfers
               WHERE disbursement_id IS NULL
                 AND processing_status <> 'unprocessed'
               ORDER BY id DESC
               LIMIT $1
             )
           )
         ORDER BY id",
    )
    .bind(app.settings.transfer_reconsideration_limit)
    .fetch_all(&app.db)
    .await?;
    let mut converted = 0;
    for row in rows {
        let incoming = Incoming {
            id: row.get(0),
            asset: row.get(1),
            sender: row.get(2),
            amount: U256::from_dec_str(row.get::<String, _>(3).as_str())?,
        };
        if evaluate_transfer(app, incoming).await? {
            converted += 1;
        }
    }
    Ok(converted)
}

async fn evaluate_transfer(app: &App, incoming: Incoming) -> anyhow::Result<bool> {
    let config = sqlx::query(
        "SELECT enabled, minimum_amount::text
         FROM current_asset_configs
         WHERE chain_id = $1 AND asset_address = $2",
    )
    .bind(app.settings.chain_id as i64)
    .bind(&incoming.asset)
    .fetch_optional(&app.db)
    .await?;

    let (enabled, minimum) = match config {
        Some(row) => (
            row.get::<bool, _>(0),
            U256::from_dec_str(row.get::<String, _>(1).as_str())?,
        ),
        None => {
            mark_ineligible(&app.db, incoming.id, "not_whitelisted").await?;
            return Ok(false);
        }
    };
    if !enabled {
        mark_ineligible(&app.db, incoming.id, "not_whitelisted").await?;
        return Ok(false);
    }
    if incoming.amount < minimum {
        mark_ineligible(&app.db, incoming.id, "below_minimum").await?;
        return Ok(false);
    }

    let rows = sqlx::query(
        "SELECT member_id::text, account, name, score::text
         FROM current_members
         WHERE chain_id = $1
           AND account <> $2
           AND score > 0
         ORDER BY member_id",
    )
    .bind(app.settings.chain_id as i64)
    .bind(ZERO_ADDRESS)
    .fetch_all(&app.db)
    .await?;
    if rows.is_empty() {
        mark_ineligible(&app.db, incoming.id, "no_active_members").await?;
        return Ok(false);
    }
    let members: Vec<Member> = rows
        .into_iter()
        .map(|row| {
            Ok(Member {
                id: row.get(0),
                account: row.get(1),
                name: row.get(2),
                score: U256::from_dec_str(row.get::<String, _>(3).as_str())?,
            })
        })
        .collect::<anyhow::Result<_>>()?;
    let total_score = members.iter().fold(U256::zero(), |total, member| total + member.score);
    let distributable = mul_div(incoming.amount, U256::from(9), U256::from(10))?;
    let retained = incoming.amount - distributable;

    let balance = asset_balance(app, &incoming.asset).await?;
    let snapshot_block: i64 = sqlx::query(
        "SELECT COALESCE(MAX(block_number), 0)
         FROM member_update_events WHERE chain_id = $1",
    )
    .bind(app.settings.chain_id as i64)
    .fetch_one(&app.db)
    .await?
    .get(0);

    let mut tx = app.db.begin().await?;
    sqlx::query("SELECT pg_advisory_xact_lock(hashtext($1))")
        .bind(format!("{}:{}", app.settings.chain_id, incoming.asset))
        .execute(&mut *tx)
        .await?;
    let reserved_value: String = sqlx::query(
        "SELECT COALESCE(SUM(amount), 0)::text FROM payouts
         WHERE asset_address = $1
           AND status IN ('pending', 'retryable', 'submitting', 'submitted', 'blocked_low_gas')",
    )
    .bind(&incoming.asset)
    .fetch_one(&mut *tx)
    .await?
    .get(0);
    let reserved = U256::from_dec_str(&reserved_value)?;
    let payable = balance >= reserved.saturating_add(distributable);
    let status = if payable { "ready" } else { "failed" };
    let failure = (!payable).then_some("funds_not_available");
    let disbursement_id: i64 = sqlx::query(
        "INSERT INTO disbursements
         (incoming_transfer_id, chain_id, asset_address, received_amount,
          distributable_amount, retained_amount, member_snapshot_block,
          total_active_score, sender, status, failure_reason)
         VALUES ($1, $2, $3, $4::numeric, $5::numeric, $6::numeric, $7,
                 $8::numeric, $9, $10, $11)
         ON CONFLICT(incoming_transfer_id) DO NOTHING
         RETURNING id",
    )
    .bind(incoming.id)
    .bind(app.settings.chain_id as i64)
    .bind(&incoming.asset)
    .bind(incoming.amount.to_string())
    .bind(distributable.to_string())
    .bind(retained.to_string())
    .bind(snapshot_block)
    .bind(total_score.to_string())
    .bind(&incoming.sender)
    .bind(status)
    .bind(failure)
    .fetch_optional(&mut *tx)
    .await?
    .map(|row| row.get(0))
    .context("incoming transfer was converted concurrently")?;

    for member in members {
        let amount = mul_div(distributable, member.score, total_score)?;
        let payout_status = if amount.is_zero() {
            "skipped_zero"
        } else if payable {
            "pending"
        } else {
            "cancelled_underfunded"
        };
        sqlx::query(
            "INSERT INTO payouts
             (disbursement_id, member_id, recipient, member_name, member_score,
              asset_address, amount, status, completed_at)
             VALUES ($1, $2::numeric, $3, $4, $5::numeric, $6, $7::numeric, $8,
                     CASE WHEN $8 IN ('skipped_zero', 'cancelled_underfunded') THEN now() END)",
        )
        .bind(disbursement_id)
        .bind(member.id)
        .bind(member.account)
        .bind(member.name)
        .bind(member.score.to_string())
        .bind(&incoming.asset)
        .bind(amount.to_string())
        .bind(payout_status)
        .execute(&mut *tx)
        .await?;
    }
    sqlx::query(
        "UPDATE incoming_transfers
         SET disbursement_id = $2, processing_status = 'converted',
             ineligibility_reason = NULL,
             first_evaluated_at = COALESCE(first_evaluated_at, now()),
             last_evaluated_at = now()
         WHERE id = $1",
    )
    .bind(incoming.id)
    .bind(disbursement_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;

    let (severity, subkind, message) = if payable {
        ("info", "created", "Disbursement and member payout intents created.")
    } else {
        (
            "warn",
            "funds_not_available",
            "Disbursement recorded as failed because contract funds are unavailable.",
        )
    };
    db::log(
        &app.db,
        severity,
        "disbursement",
        subkind,
        message,
        Some(app.settings.chain_id),
        Some(("disbursement", disbursement_id.to_string())),
    )
    .await;
    Ok(true)
}

async fn mark_ineligible(pool: &PgPool, id: i64, reason: &str) -> anyhow::Result<()> {
    sqlx::query(
        "UPDATE incoming_transfers
         SET processing_status = $2, ineligibility_reason = $2,
             first_evaluated_at = COALESCE(first_evaluated_at, now()),
             last_evaluated_at = now()
         WHERE id = $1 AND disbursement_id IS NULL",
    )
    .bind(id)
    .bind(reason)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn reconcile_submitted(app: &App) -> anyhow::Result<usize> {
    let rows = sqlx::query(
        "SELECT attempts.id, attempts.payout_id, attempts.transaction_hash,
                attempts.raw_transaction
         FROM payout_attempts attempts
         JOIN payouts ON payouts.id = attempts.payout_id
         WHERE payouts.status IN ('submitting', 'submitted')
         ORDER BY attempts.id",
    )
    .fetch_all(&app.db)
    .await?;
    let mut resolved = 0;
    for row in rows {
        let attempt_id: i64 = row.get(0);
        let payout_id: i64 = row.get(1);
        let hash = H256::from_str(row.get::<String, _>(2).as_str())?;
        let raw_transaction: Option<Vec<u8>> = row.get(3);
        if let Some(receipt) = app.provider.get_transaction_receipt(hash).await? {
            store_receipt(app, attempt_id, payout_id, receipt).await?;
            resolved += 1;
            continue;
        }
        if app.provider.get_transaction(hash).await?.is_some() {
            continue;
        }

        // Attempts created by older binaries have no raw transaction and
        // require explicit operator recovery. Never guess and advance a nonce.
        let Some(raw_transaction) = raw_transaction else {
            continue;
        };
        match app.provider.send_raw_transaction(Bytes::from(raw_transaction)).await {
            Ok(_) => {
                sqlx::query(
                    "UPDATE payout_attempts SET submitted_at = now(), error_type = NULL,
                     error_message = NULL WHERE id = $1",
                )
                .bind(attempt_id)
                .execute(&app.db)
                .await?;
            }
            Err(error) if is_ambiguous_submission_error(&error) => {}
            Err(error) if error.as_error_response().is_some() => {
                record_submission_rejection(app, attempt_id, payout_id, &error).await?;
                resolved += 1;
            }
            Err(error) => {
                record_submission_unknown(app, attempt_id, payout_id, &error).await?;
            }
        }
    }
    Ok(resolved)
}

pub async fn recover_unsent_attempts(app: &App, hashes: &[String]) -> anyhow::Result<usize> {
    if hashes.is_empty() {
        anyhow::bail!("at least one --transaction-hash is required");
    }

    let mut verified = Vec::with_capacity(hashes.len());
    for value in hashes {
        let hash = H256::from_str(value).context("invalid transaction hash")?;
        if app.provider.get_transaction_receipt(hash).await?.is_some() {
            anyhow::bail!("{hash:#x} has a receipt; let the worker reconcile it");
        }
        if app.provider.get_transaction(hash).await?.is_some() {
            anyhow::bail!("{hash:#x} exists in the node and cannot be marked unsent");
        }

        let row = sqlx::query(
            "SELECT attempts.id, attempts.payout_id, attempts.from_address,
                    attempts.nonce::text
             FROM payout_attempts attempts
             JOIN payouts ON payouts.id = attempts.payout_id
             WHERE attempts.transaction_hash = $1
               AND attempts.receipt_status IS NULL
               AND payouts.status IN ('submitting', 'submitted')",
        )
        .bind(format!("{hash:#x}"))
        .fetch_optional(&app.db)
        .await?
        .with_context(|| format!("no unresolved payout attempt found for {hash:#x}"))?;
        let attempt_id: i64 = row.get(0);
        let payout_id: i64 = row.get(1);
        let from: Address = row.get::<String, _>(2).parse()?;
        let nonce = U256::from_dec_str(row.get::<String, _>(3).as_str())?;
        let pending_nonce = app
            .provider
            .get_transaction_count(
                from,
                Some(ethers::types::BlockId::Number(ethers::types::BlockNumber::Pending)),
            )
            .await?;
        if pending_nonce > nonce {
            anyhow::bail!(
                "{hash:#x} used nonce {nonce}, but the pending account nonce is {pending_nonce}; manual reconciliation is required"
            );
        }
        verified.push((attempt_id, payout_id, format!("{hash:#x}")));
    }

    let mut tx = app.db.begin().await?;
    for (attempt_id, payout_id, _) in &verified {
        sqlx::query(
            "UPDATE payout_attempts SET resolved_at = now(),
             error_type = 'operator_confirmed_unsent',
             error_message = 'Operator verified transaction absent from RPC'
             WHERE id = $1",
        )
        .bind(attempt_id)
        .execute(&mut *tx)
        .await?;
        sqlx::query(
            "UPDATE payouts SET status = 'retryable',
             last_error = 'operator_confirmed_unsent', updated_at = now()
             WHERE id = $1 AND status IN ('submitting', 'submitted')",
        )
        .bind(payout_id)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;

    for (_, payout_id, hash) in &verified {
        db::log(
            &app.db,
            "warn",
            "payout",
            "operator_confirmed_unsent",
            &format!("Operator verified that payout transaction {hash} was not submitted."),
            Some(app.settings.chain_id),
            Some(("payout", payout_id.to_string())),
        )
        .await;
    }
    Ok(verified.len())
}

pub async fn process_one_payout(app: &App) -> anyhow::Result<bool> {
    let private_key = match &app.settings.automator_private_key {
        Some(value) => value,
        None => return Ok(false),
    };
    // Never allocate the next nonce while an earlier submission is unresolved.
    // Reconciliation must resolve or safely rebroadcast it first.
    let unresolved: i64 = sqlx::query("SELECT COUNT(*) FROM payouts WHERE status IN ('submitting', 'submitted')")
        .fetch_one(&app.db)
        .await?
        .get(0);
    if unresolved != 0 {
        return Ok(false);
    }

    let row = match sqlx::query(
        "SELECT payouts.id, payouts.disbursement_id, payouts.recipient,
                payouts.asset_address, payouts.amount::text, payouts.attempt_count
         FROM payouts
         JOIN disbursements ON disbursements.id = payouts.disbursement_id
         WHERE payouts.status IN ('pending', 'retryable', 'blocked_low_gas')
           AND disbursements.status IN ('ready', 'processing')
         ORDER BY payouts.disbursement_id, payouts.member_id
         LIMIT 1",
    )
    .fetch_optional(&app.db)
    .await?
    {
        Some(row) => row,
        None => return Ok(false),
    };
    let payout_id: i64 = row.get(0);
    let disbursement_id: i64 = row.get(1);
    let recipient: Address = row.get::<String, _>(2).parse()?;
    let asset: String = row.get(3);
    let amount = U256::from_dec_str(row.get::<String, _>(4).as_str())?;
    let attempt_number: i32 = row.get::<i32, _>(5) + 1;

    let remaining = remaining_for_disbursement(&app.db, disbursement_id).await?;
    let contract_balance = asset_balance(app, &asset).await?;
    if contract_balance < remaining {
        fail_underfunded(app, disbursement_id).await?;
        return Ok(true);
    }

    let wallet: LocalWallet = private_key
        .parse::<LocalWallet>()
        .context("AUTOMATOR_PRIVATE_KEY")?
        .with_chain_id(app.settings.chain_id);
    let automator = wallet.address();
    let contract = AsamiContract::new(app.settings.contract_address, app.provider.clone());
    let mut transaction = if asset == ZERO_ADDRESS {
        contract.disburse_native(recipient, amount).legacy().tx
    } else {
        let token: Address = asset.parse()?;
        contract.disburse_token(token, recipient, amount).legacy().tx
    };
    transaction.set_from(automator);
    transaction.set_chain_id(app.settings.chain_id);
    let nonce = app
        .provider
        .get_transaction_count(
            automator,
            Some(ethers::types::BlockId::Number(ethers::types::BlockNumber::Pending)),
        )
        .await?;
    transaction.set_nonce(nonce);
    let gas = app.provider.estimate_gas(&transaction, None).await?;
    let gas_price = app.provider.get_gas_price().await?;
    transaction.set_gas(gas);
    transaction.set_gas_price(gas_price);

    let estimated = gas.saturating_mul(gas_price).saturating_mul(U256::from(app.settings.gas_safety_basis_points))
        / U256::from(10_000_u64);
    let required = estimated.saturating_add(app.settings.min_automator_gas_reserve);
    let gas_balance = app.provider.get_balance(automator, None).await?;
    if gas_balance < required {
        sqlx::query(
            "UPDATE payouts SET status = 'blocked_low_gas', updated_at = now(),
             last_error = 'insufficient_automator_gas' WHERE id = $1",
        )
        .bind(payout_id)
        .execute(&app.db)
        .await?;
        db::log(
            &app.db,
            "warn",
            "payout",
            "blocked_low_gas",
            "Payout processing stopped because the automator gas reserve is insufficient.",
            Some(app.settings.chain_id),
            Some(("payout", payout_id.to_string())),
        )
        .await;
        notifications::queue_low_gas(
            app,
            &format!("{automator:#x}"),
            &gas_balance.to_string(),
            &required.to_string(),
        )
        .await?;
        return Ok(true);
    }

    let signature = wallet.sign_transaction(&transaction).await?;
    let encoded = transaction.rlp_signed(&signature);
    let transaction_hash = H256::from(keccak256(&encoded));
    let mut db_tx = app.db.begin().await?;
    let attempt_id: i64 = sqlx::query(
        "INSERT INTO payout_attempts
         (payout_id, attempt_number, from_address, nonce, transaction_hash,
          gas_limit, effective_gas_price, raw_transaction)
         VALUES ($1, $2, $3, $4::numeric, $5, $6::numeric, $7::numeric, $8)
         RETURNING id",
    )
    .bind(payout_id)
    .bind(attempt_number)
    .bind(format!("{automator:#x}"))
    .bind(nonce.to_string())
    .bind(format!("{transaction_hash:#x}"))
    .bind(gas.to_string())
    .bind(gas_price.to_string())
    .bind(encoded.to_vec())
    .fetch_one(&mut *db_tx)
    .await?
    .get(0);
    sqlx::query(
        "UPDATE payouts SET status = 'submitting', attempt_count = $2,
         updated_at = now() WHERE id = $1",
    )
    .bind(payout_id)
    .bind(attempt_number)
    .execute(&mut *db_tx)
    .await?;
    sqlx::query(
        "UPDATE disbursements SET status = 'processing'
         WHERE id = $1 AND status = 'ready'",
    )
    .bind(disbursement_id)
    .execute(&mut *db_tx)
    .await?;
    db_tx.commit().await?;

    match app.provider.send_raw_transaction(Bytes::from(encoded.to_vec())).await {
        Ok(pending) => {
            let mut submitted_tx = app.db.begin().await?;
            sqlx::query("UPDATE payout_attempts SET submitted_at = now() WHERE id = $1")
                .bind(attempt_id)
                .execute(&mut *submitted_tx)
                .await?;
            sqlx::query("UPDATE payouts SET status = 'submitted', updated_at = now() WHERE id = $1")
                .bind(payout_id)
                .execute(&mut *submitted_tx)
                .await?;
            submitted_tx.commit().await?;
            db::log(
                &app.db,
                "info",
                "payout",
                "submitted",
                "Individual member payout transaction submitted.",
                Some(app.settings.chain_id),
                Some(("payout", payout_id.to_string())),
            )
            .await;
            if let Some(receipt) = pending.await? {
                store_receipt(app, attempt_id, payout_id, receipt).await?;
            }
        }
        Err(error) if is_ambiguous_submission_error(&error) => {
            record_submission_unknown(app, attempt_id, payout_id, &error).await?;
        }
        Err(error) if error.as_error_response().is_some() => {
            record_submission_rejection(app, attempt_id, payout_id, &error).await?;
        }
        Err(error) => record_submission_unknown(app, attempt_id, payout_id, &error).await?,
    }
    Ok(true)
}

fn is_ambiguous_submission_error(error: &ProviderError) -> bool {
    error
        .as_error_response()
        .map(|error| is_ambiguous_submission_message(&error.message))
        .unwrap_or(false)
}

fn is_ambiguous_submission_message(message: &str) -> bool {
    let message = message.to_ascii_lowercase();
    [
        "already known",
        "known transaction",
        "nonce too low",
        "replacement transaction underpriced",
    ]
    .iter()
    .any(|needle| message.contains(needle))
}

async fn record_submission_rejection(
    app: &App,
    attempt_id: i64,
    payout_id: i64,
    error: &ProviderError,
) -> anyhow::Result<()> {
    let message = db::sanitize_log_message(&error.to_string());
    let mut tx = app.db.begin().await?;
    sqlx::query(
        "UPDATE payout_attempts
         SET submitted_at = now(), resolved_at = now(),
             error_type = 'submission_rejected', error_message = $2
         WHERE id = $1",
    )
    .bind(attempt_id)
    .bind(&message)
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        "UPDATE payouts SET status = 'retryable', last_error = $2,
         updated_at = now() WHERE id = $1",
    )
    .bind(payout_id)
    .bind(&message)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    db::log(
        &app.db,
        "fail",
        "payout",
        "submission_rejected",
        &message,
        Some(app.settings.chain_id),
        Some(("payout", payout_id.to_string())),
    )
    .await;
    Ok(())
}

async fn record_submission_unknown(
    app: &App,
    attempt_id: i64,
    payout_id: i64,
    error: &ProviderError,
) -> anyhow::Result<()> {
    let message = db::sanitize_log_message(&error.to_string());
    let mut tx = app.db.begin().await?;
    sqlx::query(
        "UPDATE payout_attempts
         SET submitted_at = now(), error_type = 'submission_result_unknown',
             error_message = $2 WHERE id = $1",
    )
    .bind(attempt_id)
    .bind(&message)
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        "UPDATE payouts SET status = 'submitted', last_error = $2,
         updated_at = now() WHERE id = $1",
    )
    .bind(payout_id)
    .bind(&message)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    db::log(
        &app.db,
        "warn",
        "payout",
        "submission_unknown",
        "RPC did not confirm submission; the exact signed transaction will be reconciled and safely rebroadcast.",
        Some(app.settings.chain_id),
        Some(("payout", payout_id.to_string())),
    )
    .await;
    Ok(())
}

async fn store_receipt(
    app: &App,
    attempt_id: i64,
    payout_id: i64,
    receipt: ethers::types::TransactionReceipt,
) -> anyhow::Result<()> {
    let succeeded = receipt.status == Some(1_u64.into());
    let block = receipt.block_number.map(|value| value.as_u64() as i64);
    let gas_used = receipt.gas_used.unwrap_or_default().to_string();
    let gas_price = receipt.effective_gas_price.unwrap_or_default().to_string();
    let mut tx = app.db.begin().await?;
    sqlx::query(
        "UPDATE payout_attempts
         SET receipt_status = $2, receipt_block_number = $3, gas_used = $4::numeric,
             effective_gas_price = $5::numeric, resolved_at = now(),
             error_type = CASE WHEN $2 = 1 THEN NULL ELSE 'transaction_reverted' END,
             error_message = CASE WHEN $2 = 1 THEN NULL ELSE 'Transaction reverted' END
         WHERE id = $1",
    )
    .bind(attempt_id)
    .bind(if succeeded { 1_i64 } else { 0_i64 })
    .bind(block)
    .bind(gas_used)
    .bind(gas_price)
    .execute(&mut *tx)
    .await?;
    if succeeded {
        sqlx::query(
            "UPDATE payouts
             SET status = 'confirmed', successful_transaction_hash = $2,
                 successful_block_number = $3, successful_receipt_recorded_at = now(),
                 completed_at = now(), updated_at = now(), last_error = NULL
             WHERE id = $1",
        )
        .bind(payout_id)
        .bind(format!("{:#x}", receipt.transaction_hash))
        .bind(block)
        .execute(&mut *tx)
        .await?;
    } else {
        sqlx::query(
            "UPDATE payouts SET status = 'retryable',
             last_error = 'transaction_reverted', updated_at = now()
             WHERE id = $1",
        )
        .bind(payout_id)
        .execute(&mut *tx)
        .await?;
    }
    tx.commit().await?;
    db::log(
        &app.db,
        if succeeded { "info" } else { "warn" },
        "payout",
        if succeeded { "confirmed" } else { "reverted" },
        if succeeded {
            "Successful individual payout receipt stored."
        } else {
            "Individual payout transaction reverted and is retryable."
        },
        Some(app.settings.chain_id),
        Some(("payout", payout_id.to_string())),
    )
    .await;
    if succeeded {
        maybe_complete(app, payout_id).await?;
    }
    Ok(())
}

async fn maybe_complete(app: &App, payout_id: i64) -> anyhow::Result<()> {
    let disbursement_id: i64 = sqlx::query("SELECT disbursement_id FROM payouts WHERE id = $1")
        .bind(payout_id)
        .fetch_one(&app.db)
        .await?
        .get(0);
    let remaining: i64 = sqlx::query(
        "SELECT COUNT(*) FROM payouts
         WHERE disbursement_id = $1
           AND status NOT IN ('confirmed', 'skipped_zero')",
    )
    .bind(disbursement_id)
    .fetch_one(&app.db)
    .await?
    .get(0);
    if remaining != 0 {
        return Ok(());
    }
    let row = sqlx::query(
        "SELECT sender FROM disbursements
         WHERE id = $1 AND status IN ('ready', 'processing')
         FOR UPDATE",
    )
    .bind(disbursement_id)
    .fetch_optional(&app.db)
    .await?;
    let Some(row) = row else {
        return Ok(());
    };
    let sender: String = row.get(0);
    let sender_name = sqlx::query("SELECT name FROM current_sender_names WHERE chain_id = $1 AND account = $2")
        .bind(app.settings.chain_id as i64)
        .bind(&sender)
        .fetch_optional(&app.db)
        .await?
        .map(|row| row.get::<String, _>(0))
        .filter(|name| !name.is_empty());
    let mut tx = app.db.begin().await?;
    sqlx::query(
        "UPDATE disbursements SET status = 'completed', completed_at = now()
         WHERE id = $1 AND status IN ('ready', 'processing')",
    )
    .bind(disbursement_id)
    .execute(&mut *tx)
    .await?;
    notifications::queue_completion(&mut tx, disbursement_id, &sender, sender_name).await?;
    tx.commit().await?;
    db::log(
        &app.db,
        "info",
        "disbursement",
        "completed",
        "All individual member payouts are confirmed; English thanks queued.",
        Some(app.settings.chain_id),
        Some(("disbursement", disbursement_id.to_string())),
    )
    .await;
    Ok(())
}

async fn fail_underfunded(app: &App, disbursement_id: i64) -> anyhow::Result<()> {
    let mut tx = app.db.begin().await?;
    sqlx::query(
        "UPDATE disbursements
         SET status = 'failed', failure_reason = 'funds_not_available'
         WHERE id = $1 AND status IN ('ready', 'processing')",
    )
    .bind(disbursement_id)
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        "UPDATE payouts
         SET status = 'cancelled_underfunded', completed_at = now(), updated_at = now(),
             last_error = 'funds_not_available'
         WHERE disbursement_id = $1
           AND status NOT IN ('confirmed', 'skipped_zero')",
    )
    .bind(disbursement_id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    db::log(
        &app.db,
        "warn",
        "disbursement",
        "funds_not_available",
        "Remaining payout intents were cancelled because contract funds are unavailable.",
        Some(app.settings.chain_id),
        Some(("disbursement", disbursement_id.to_string())),
    )
    .await;
    Ok(())
}

async fn asset_balance(app: &App, asset: &str) -> anyhow::Result<U256> {
    if asset == ZERO_ADDRESS {
        Ok(app.provider.get_balance(app.settings.contract_address, None).await?)
    } else {
        let token = Erc20Contract::new(Address::from_str(asset)?, app.provider.clone());
        Ok(token.balance_of(app.settings.contract_address).call().await?)
    }
}

async fn remaining_for_disbursement(pool: &PgPool, id: i64) -> anyhow::Result<U256> {
    let value: String = sqlx::query(
        "SELECT COALESCE(SUM(amount), 0)::text FROM payouts
         WHERE disbursement_id = $1
           AND status IN ('pending', 'retryable', 'submitting', 'submitted', 'blocked_low_gas')",
    )
    .bind(id)
    .fetch_one(pool)
    .await?
    .get(0);
    Ok(U256::from_dec_str(&value)?)
}

pub fn mul_div(value: U256, multiplier: U256, divisor: U256) -> anyhow::Result<U256> {
    if divisor.is_zero() {
        anyhow::bail!("division by zero");
    }
    let result = U512::from(value) * U512::from(multiplier) / U512::from(divisor);
    U256::from_dec_str(&result.to_string()).context("mul_div result exceeds uint256")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_without_intermediate_overflow() {
        let max = U256::MAX;
        assert_eq!(
            mul_div(max, U256::from(9), U256::from(10)).unwrap(),
            max / 10 * 9 + max % 10 * 9 / 10
        );
    }

    #[test]
    fn preserves_rounding_dust() {
        assert_eq!(
            mul_div(U256::from(10), U256::from(1), U256::from(3)).unwrap(),
            U256::from(3)
        );
    }

    #[test]
    fn identifies_rpc_responses_that_do_not_prove_rejection() {
        assert!(is_ambiguous_submission_message("already known"));
        assert!(is_ambiguous_submission_message("nonce too low"));
        assert!(is_ambiguous_submission_message("replacement transaction underpriced"));
        assert!(!is_ambiguous_submission_message("transaction type not supported"));
    }
}
