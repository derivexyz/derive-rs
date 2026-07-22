// Simple test of public endpoints.
// Run with `cargo run --example ws_stream_tickers`

mod common;
use derive_rs::{models::asyncapi_subs::TickerSlimNotification, types::ExternalEvent};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

use crate::common::get_test_ws_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws_client = get_test_ws_client().await;

    let mut eth_ticker_stream: BroadcastStream<TickerSlimNotification> = ws_client
        .subscriptions()
        .market_data()
        .ticker_slim("ETH-USD", "100")
        .await?;

    let mut btc_ticker_stream: BroadcastStream<TickerSlimNotification> = ws_client
        .subscriptions()
        .market_data()
        .ticker_slim("BTC-USD", "100")
        .await?;

    loop {
        tokio::select! {
            Some(eth_ticker) = eth_ticker_stream.next() => {
                match eth_ticker {
                    Ok(ticker) => println!("ETH Ticker: {:?}", ticker),
                    Err(e) => {
                        eprintln!("Error receiving ETH ticker: {:?}", e);
                        break; // Exit the loop on error
                    }
                }
            }
            Some(btc_ticker) = btc_ticker_stream.next() => {
                match btc_ticker {
                    Ok(ticker) => println!("BTC Ticker: {:?}", ticker),
                    Err(e) => {
                        eprintln!("Error receiving ETH ticker: {:?}", e);
                        break; // Exit the loop on error
                    }
                }
            }
            event = ws_client.run_till_event() => {
                match event {
                    ExternalEvent::Connected => {
                        // let _ = ws_client.login().await;
                        // let _ = ws_client.resubscribe_all().await;
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
        }
    }

    Ok(())
}
