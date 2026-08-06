use derive_rs::{
    Environment, WsClient,
    models::{AssetType, GetAllInstrumentsRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WsClient::new_public(Environment::Testnet).await?;

    let params = GetAllInstrumentsRequest::builder()
        .expired(false)
        .instrument_type(AssetType::Option)
        .try_into()?;

    let instruments = client
        .rpc()
        .market_data()
        .get_all_instruments(params)
        .await?;

    println!("Available instruments: {:#?}", instruments);
    Ok(())
}
