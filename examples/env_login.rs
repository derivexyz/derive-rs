use derive_rs::WsClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Reads DERIVE_PRIVATE_KEY, DERIVE_WALLET, DERIVE_SUBACCOUNT_ID, DERIVE_ENVIRONMENT from the environment
    let client = WsClient::from_env().await?;
    client.login().await?;
    Ok(())
}
