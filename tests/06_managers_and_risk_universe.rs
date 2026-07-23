mod common;

#[tokio::test]
async fn test_ws_get_risk_universe() -> Result<(), Box<dyn std::error::Error>> {
    let ws_client = common::get_test_ws_client().await;
    let result = ws_client.rpc().market_data().get_risk_universes().await?;

    let manager = result
        .iter()
        .flat_map(|u| &u.managers)
        .find(|m| {
            m.instruments.contains(&"ETH-OPTION".to_string())
                && m.collaterals.iter().any(|c| c.name == "USDC")
        })
        .expect("Manager not found");
    let _usdc = manager
        .collaterals
        .iter()
        .find(|c| c.name == "USDC")
        .expect("USDC collateral not found");
    Ok(())
}
