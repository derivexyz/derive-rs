use std::str::FromStr;

use alloy::primitives::Address;
use bigdecimal::BigDecimal;
use derive_rs::actions::{DepositArgs, SupportDepositAssets};

mod common;

#[tokio::test]
async fn test_deposit() {
    let ws_client = common::get_test_ws_client().await;
    ws_client.login().await.expect("Failed to login");

    let address: Address = ws_client
        .smart_contract_wallet_address
        .clone()
        .unwrap()
        .parse()
        .expect("Couldnt parse wallet address");

    let args = DepositArgs::builder()
        .asset(SupportDepositAssets::USDC)
        // .max_fee_usd(BigDecimal::from_str("1.00").unwrap())
        .amount(BigDecimal::from_str("10.00").unwrap())
        .recepient_address(address)
        // .force_batch(false)
        .build();

    let deposit_result = ws_client.fund_movements().deposit(args).await;
    assert!(
        deposit_result.is_ok(),
        "Deposit failed: {:?}",
        deposit_result.err()
    );
}
