use std::println;

use derive_rs::{types::Environment, ws_client::WsClient};

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
async fn test_ws_client_login() {
    let ws_client = get_test_ws_client().await;
    let login_result = ws_client.login().await;
    assert!(
        login_result.is_ok(),
        "Login failed: {:?}",
        login_result.err()
    );
}

#[tokio::test]
async fn test_ws_set_cancel_on_disconnect() {
    let ws_client = get_test_ws_client().await;
    let login_result = ws_client.login().await;
    assert!(
        login_result.is_ok(),
        "Login failed: {:?}",
        login_result.err()
    );
    let set_cancel_result = ws_client.set_cancel_on_disconnect(true).await;
    assert!(
        set_cancel_result.is_ok(),
        "Set cancel on disconnect failed: {:?}",
        set_cancel_result.err()
    );

    println!(
        "Set cancel on disconnect response: {:?}",
        set_cancel_result.unwrap()
    );
}
