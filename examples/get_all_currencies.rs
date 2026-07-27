// Simple test of public endpoints.
// Run with `cargo run --example fetch_instruments`

mod common;

use std::collections::HashMap;

use crate::common::get_test_ws_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws_client = get_test_ws_client().await;
    ws_client.login().await.expect("Failed to login");

    let currencies = ws_client.rpc().market_data().get_all_currencies().await?;

    let asset_name_to_erc20_details = currencies
        .into_iter()
        .filter_map(|currency| {
            let mut spots = currency.spot.into_iter();
            let spot = spots.next()?;
            if spots.next().is_some() {
                panic!(
                    "Currency {:?} has more than one spot entry",
                    currency.currency,
                );
            }
            Some((currency.currency, spot))
        })
        .collect::<HashMap<_, _>>();

    let currency_to_find = "CC";
    if let Some(erc20_details) = asset_name_to_erc20_details.get(currency_to_find) {
        println!(
            "Found ERC20 details for {}: {:?}",
            currency_to_find, erc20_details
        );
    } else {
        println!("ERC20 details for {} not found", currency_to_find);
    }

    Ok(())
}
