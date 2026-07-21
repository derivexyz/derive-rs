// Simple test of public endpoints.
// Run with `cargo run --example fetch_instruments`

use derive_rs::types::{Environment, ExternalEvent};
use derive_rs::ws_client::WsClient;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws_client = get_test_ws_client().await;
    ws_client.login().await.expect("Failed to login");


    loop {
        match ws_client.run_till_event().await {
            ExternalEvent::Connected => {
                let _ = ws_client.login().await;
                let _ = ws_client
                    .resubscribe_all()
                    .await;
                println!("WebSocket connected");
            }
            ExternalEvent::Disconnected => {
                println!("WebSocket disconnected");
            }
            ExternalEvent::Exited => {
                println!("WebSocket exited");
                break;
            }
        }
    }
    Ok(())
}
