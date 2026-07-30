use derive_rs::models::openapi::GetSubaccountsRequest;

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

    for sub in subaccounts.subaccount_ids {
        println!("{}", sub);
    }
}
