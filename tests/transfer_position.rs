use bigdecimal::BigDecimal;
use derive_rs::{
    actions::TransferPositionsArgs,
    models::openapi::{Direction, GetSubaccountsRequest, PricedLegParamsAndResponse},
};
use std::str::FromStr;

mod common;

#[tokio::test]
async fn test_transfer_position() {
    let ws_client = common::get_test_ws_client().await;
    ws_client.login().await.expect("Failed to login");
    let _subaccount_a = ws_client.subaccount_id.unwrap();
    let wallet = ws_client.smart_contract_wallet_address.clone().unwrap();

    let get_sub_params = GetSubaccountsRequest::builder()
        .wallet(wallet)
        .try_into()
        .expect("Failed to build GetSubaccountsRequest");

    let subaccounts = ws_client
        .rpc()
        .subaccounts()
        .get_subaccounts(get_sub_params)
        .await
        .expect("Failed to get subaccounts");

    let position = PricedLegParamsAndResponse::builder()
        .amount(BigDecimal::from_str("0.1").expect("Failed to parse amount"))
        .direction(Direction::Buy)
        .price(BigDecimal::from_str("1920.0").expect("Failed to parse price"))
        .instrument_name("ETH-PERP")
        .try_into()
        .expect("Failed to build PricedLegParamsAndResponse");

    let pos_transfer = TransferPositionsArgs::builder()
        .legs(vec![position])
        .from_subaccount_id(subaccounts.subaccount_ids[1])
        .to_subaccount_id(subaccounts.subaccount_ids[2])
        .maker_direction(Direction::Sell)
        .max_fee(BigDecimal::from_str("0").expect("Failed to parse max fee"))
        .build();

    println!("Transfer positions args: {:?}", pos_transfer);

    let transfer_result = ws_client
        .fund_movements()
        .transfer_positions(pos_transfer)
        .await;

    assert!(
        transfer_result.is_ok(),
        "Position transfer failed: {:?}",
        transfer_result.err()
    );
}
