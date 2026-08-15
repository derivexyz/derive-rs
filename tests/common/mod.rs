use derive_rs::{types::Environment, ws_client::WsClient};

#[allow(dead_code)]
pub async fn get_test_ws_client() -> WsClient {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();
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
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();
    let private_key = "0xf20701f7e29ce946e79a70cb53067f837950841f77edb3e685ce370db7ed7bdd";
    let derive_wallet = "0x0e94ecA48AC699d2237F3732210f8216A497ab16";
    let subaccount_id = 75736;
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
pub async fn get_test_ws_client_3() -> WsClient {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .try_init();
    let private_key = "19d7655dfe7de62e83f6c9424650986a4734029458add7eb2f74a7e249caa81c";
    let derive_wallet = "0x0e94ecA48AC699d2237F3732210f8216A497ab16";
    let subaccount_id = 75766;
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
