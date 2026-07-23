use derive_rs::models::openapi::{MmpScopeRequest, SetMmpConfigRequest};

mod common;

#[tokio::test]
async fn test_ws_configure_mmp() -> Result<(), Box<dyn std::error::Error>> {
    let ws_client = common::get_test_ws_client().await;
    ws_client.login().await.expect("Failed to login");

    let params = SetMmpConfigRequest::builder()
        .subaccount_id(ws_client.subaccount_id.expect("Must have a subaccount_id"))
        .currency("ETH")
        .mmp_interval(5000)
        .mmp_frozen_time(60000)
        .mmp_amount_limit(50)
        .mmp_delta_limit(10)
        .try_into()?;
    let result = ws_client
        .rpc()
        .market_maker_protection()
        .set_mmp_config(params)
        .await;
    assert!(result.is_ok(), "Set MMP config failed: {:?}", result.err());
    Ok(())
}

#[tokio::test]
async fn test_ws_get_mmp() -> Result<(), Box<dyn std::error::Error>> {
    let ws_client = common::get_test_ws_client().await;
    ws_client.login().await.expect("Failed to login");

    let params = MmpScopeRequest::builder()
        .subaccount_id(ws_client.subaccount_id.expect("Must have a subaccount_id"))
        .try_into()?;
    let result = ws_client
        .rpc()
        .market_maker_protection()
        .get_mmp_config(params)
        .await;
    assert!(result.is_ok(), "Get MMP config failed: {:?}", result.err());
    Ok(())
}

#[tokio::test]
async fn test_ws_reset() -> Result<(), Box<dyn std::error::Error>> {
    let ws_client = common::get_test_ws_client().await;
    ws_client.login().await.expect("Failed to login");

    let params = MmpScopeRequest::builder()
        .subaccount_id(ws_client.subaccount_id.expect("Must have a subaccount_id"))
        .try_into()?;
    let result = ws_client
        .rpc()
        .market_maker_protection()
        .reset_mmp(params)
        .await;
    assert!(result.is_ok(), "Get MMP config failed: {:?}", result.err());
    Ok(())
}
