use anyhow::Result;
use reloop::server;

#[tokio::main]
async fn main() -> Result<()> {
    server::init().await;
    Ok(())
}
