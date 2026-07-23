use derive_rs::models::openapi::{AssetType, GetAllInstrumentsRequest};

mod common;

#[tokio::test]
async fn test_ws_get_instruments() {
    let ws_client = common::get_test_ws_client().await;

    let params = GetAllInstrumentsRequest::builder()
        .instrument_type(AssetType::Option)
        .currency("ETH".to_string())
        .expired(false)
        .try_into()
        .expect("Failed to build GetInstrumentRequest");

    let result = ws_client
        .rpc()
        .market_data()
        .get_all_instruments(params)
        .await;
    assert!(result.is_ok(), "Get instruments failed: {:?}", result.err());
}
