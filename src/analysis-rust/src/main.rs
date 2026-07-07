#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    analysis_rust::server::run_server().await?;
    Ok(())
}