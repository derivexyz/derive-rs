use derive_rs::{types::Environment, ws_client::WsClient};

pub async fn get_test_ws_client() -> WsClient {
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
