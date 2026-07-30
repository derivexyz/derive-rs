mod common;

use crate::common::get_test_ws_client;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws_client = get_test_ws_client().await;
    ws_client.login().await.expect("Failed to login");
    let wallet = ws_client.smart_contract_wallet_address.clone().unwrap();

    let mut new_rfs = ws_client
        .subscriptions()
        .rfqs()
        .wallet_rfqs(&wallet)
        .await?;

    loop {
        match new_rfs.next().await {
            Some(notification) => {
                println!("Received notification: {:?}", notification);
            }
            None => {
                println!("No more notifications");
                break;
            }
        }
    }
    Ok(())
}
