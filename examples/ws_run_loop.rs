// Simple test of public endpoints.
// Run with `cargo run --example fetch_instruments`

mod common;
use derive_rs::types::ExternalEvent;

use crate::common::get_test_ws_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws_client = get_test_ws_client().await;
    ws_client.login().await.expect("Failed to login");

    loop {
        match ws_client.run_till_event().await {
            ExternalEvent::Connected => {
                let _ = ws_client.login().await;
                let _ = ws_client.resubscribe_all().await;
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
