use anyhow::Result;
use solver_common::diagnostics::logs::configure_logs;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    configure_logs();
    info!("Starting Spoke Chain Caller");
    Ok(())
}
