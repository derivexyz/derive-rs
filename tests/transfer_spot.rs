use std::str::FromStr;

use bigdecimal::BigDecimal;
use derive_rs::{
    actions::SpotTransferArgs,
    models::{MarginType, PrivateTransferSpotResponse},
    types::ClientError,
};

mod common;

#[tokio::test]
#[ignore]
async fn test_transfer_spot() {



    let ws_client = common::get_test_ws_client().await;
    ws_client.login().await.expect("Failed to login");
    let subaccount_id = ws_client.subaccount_id.unwrap();

    let sub_manager = ws_client
        .risk_universe_cache
        .get("PRIME")
        .expect("Expected to find prime risk universe!");

    let sub_managers = sub_manager
        .managers
        .clone()
        .into_iter()
        .filter(|manager| manager.margin_type == MarginType::Sm)
        .collect::<Vec<_>>();
    assert!(
        sub_managers.len() == 1,
        "Expected to find exactly one subaccount manager for PRIME with margin type PM2, found {}",
        sub_managers.len()
    );

    let submanager_id = sub_managers
        .first()
        .expect("Expected to find a subaccount manager for PRIME with margin type PM2")
        .manager_id;

    let args = SpotTransferArgs::builder()
        .asset("USDC".to_string())
        .max_fee_usd(BigDecimal::from_str("1.50").unwrap())
        .amount(BigDecimal::from_str("25.00").unwrap())
        .new_subaccount_manager(submanager_id)
        .subaccount_id(subaccount_id)
        .to_subaccount_id(75748)
        .build();

    let transfer_result: Result<PrivateTransferSpotResponse, ClientError> =
        ws_client.fund_movements().transfer_spot(args).await;
    assert!(
        transfer_result.is_ok(),
        "Spot transfer failed: {:?}",
        transfer_result.err()
    );
}
