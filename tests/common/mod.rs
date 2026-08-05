use derive_rs::{types::Environment, ws_client::WsClient};

pub async fn get_test_ws_client() -> WsClient {
    let private_key = "0x8ae4d41c2f6a49eebb4d5a3efbca809ae537d4316fb86bf9dd36287f5aa64988";
    let derive_wallet = "0x3177a268d59aDAACfc5ea1440Fd9f5c98eA5E53b";
    let subaccount_id = 75734;
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

#[allow(dead_code)]
pub async fn get_test_ws_client_2() -> WsClient {
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
