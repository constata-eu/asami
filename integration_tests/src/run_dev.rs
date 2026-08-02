use anyhow::Result;
use ethers::types::Address;
use integration_tests::{TestEnvironment, MEMBER_ONE, MEMBER_THREE, MEMBER_TWO};

#[tokio::main]
async fn main() -> Result<()> {
    let mut club = TestEnvironment::start().await?;
    club.add_member(MEMBER_ONE, 1, "@rootstock").await?;
    club.add_member(MEMBER_TWO, 2, "Satoshi Friend").await?;
    club.add_member(MEMBER_THREE, 3, "Rootstock Builder").await?;
    club.configure_asset(Address::zero(), true, ethers::utils::parse_ether(1)?).await?;
    club.set_sender_name("Money on Chain").await?;
    club.send_native(ethers::utils::parse_ether(10)?).await?;
    club.run_cycles(4).await?;
    club.start_web(false).await?;

    println!("Seeded Asami is running at {}", club.web_url);
    println!("Press Ctrl-C to stop it and remove the disposable environment.");
    tokio::signal::ctrl_c().await?;
    club.stop().await;
    Ok(())
}
