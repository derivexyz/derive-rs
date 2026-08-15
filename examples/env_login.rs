use derive_rs::WsClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WsClient::from_env().await?;

    client.login().await?;

    Ok(())
}
