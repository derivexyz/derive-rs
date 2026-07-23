mod common;
use derive_rs::models::openapi::{AssetType, GetAllInstrumentsRequest};

#[tokio::test]
async fn test_ws_get_instruments() -> Result<(), Box<dyn std::error::Error>> {
    let ws_client = common::get_test_ws_client().await;
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
