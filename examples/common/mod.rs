use derive_rs::{types::Environment, ws_client::WsClient};


pub async fn get_test_ws_client() -> WsClient {
    let private_key = "0x2ae8be44db8a590d20bffbe3b6872df9b569147d3bf6801a35a28281a4816bbd";
    let derive_wallet = "0xA419f70C696a4b449a4A24F92e955D91482d44e9"; // Replace with your wallet address
    let subaccount_id = 137627; // Replace with your subaccount ID

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
