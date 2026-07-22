use derive_rs::{
    models::openapi::{AssetType, GetAllInstrumentsRequest},
    types::Environment,
    ws_client::WsClient,
};

pub async fn get_test_ws_client() -> WsClient {
    let private_key = "0xf20701f7e29ce946e79a70cb53067f837950841f77edb3e685ce370db7ed7bdd";
    let derive_wallet = "0x5cb67F7829d01d9C75385A920De5E51060663374";
    let subaccount_id = 75723;
    let env = Environment::Testnet;

    WsClient::new(
        env,
        Some(private_key.to_string()),
        Some(derive_wallet.to_string()),
        Some(subaccount_id),
    )
    .await
    .expect("Failed to create WS client")
}

#[tokio::test]
async fn test_ws_get_instruments() -> Result<(), Box<dyn std::error::Error>> {
    let ws_client = get_test_ws_client().await;
    let params = GetAllInstrumentsRequest::builder()
        .instrument_type(AssetType::Perp)
        .expired(false)
        .try_into()?;
    let result = ws_client
        .rpc()
        .market_data()
        .get_all_instruments(params)
        .await;
    assert!(result.is_ok(), "Get instruments failed: {:?}", result.err());
    Ok(())
}
