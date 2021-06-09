use nm::*;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let context = glib::MainContext::default();

    context.block_on(run())
}

async fn run() -> Result<()> {
    let client = Client::new_async_future()
        .await
        .context("Failed to create NM Client")?;

    let connectivity = client
        .check_connectivity_async_future()
        .await
        .context("Failed to check connectivity")?;
    let check_enabled = client.connectivity_check_get_enabled();

    println!("Connectivity: {:?}", connectivity);
    println!("Connectivity check enabled: {:?}", check_enabled);

    Ok(())
}
