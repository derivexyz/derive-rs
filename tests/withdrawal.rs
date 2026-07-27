use std::str::FromStr;

use bigdecimal::BigDecimal;
use derive_rs::actions::WithdrawArgs;

mod common;

#[tokio::test]
async fn test_withdrawal() {
    let ws_client = common::get_test_ws_client().await;
    ws_client.login().await.expect("Failed to login");

    let args = WithdrawArgs::builder()
        .asset("USDC".to_string())
        .max_fee_usd(BigDecimal::from_str("1.00").unwrap())
        .amount(BigDecimal::from_str("10.00").unwrap())
        .recepient_address(
            ws_client
                .smart_contract_wallet_address
                .clone()
                .unwrap()
                .parse()
                .expect("Couldnt parse wallet address"),
        )
        .force_batch(false)
        .build();

    let withdrawl = ws_client.fund_movements().withdraw(args).await;
    assert!(
        withdrawl.is_ok(),
        "Withdrawal failed: {:?}",
        withdrawl.err()
    );
}
