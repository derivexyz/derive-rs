use std::{collections::HashMap, str::FromStr};

use alloy::primitives::Address;
use bigdecimal::BigDecimal;
use derive_rs::actions::{DepositArgs, DepositTypes, DirectDepositType, SupportDepositAssets};

mod common;

#[tokio::test]
async fn test_deposit_to_new_subaccount() {
    let ws_client = common::get_test_ws_client().await;
    ws_client.login().await.expect("Failed to login");

    let address: Address = ws_client
        .derive_wallet
        .clone()
        .unwrap()
        .parse()
        .expect("Couldnt parse wallet address");

    let result = ws_client
        .rpc()
        .market_data()
        .get_risk_universes()
        .await
        .expect("Failed to get risk universes.");

    let manager = result
        .iter()
        .flat_map(|u| &u.managers)
        .find(|m| {
            m.instruments.contains(&"ETH-OPTION".to_string())
                && m.collaterals.iter().any(|c| c.name == "USDC")
        })
        .expect("Manager not found");

    let args = DepositArgs::builder()
        .asset(SupportDepositAssets::USDC)
        .amount(BigDecimal::from_str("10.00").unwrap())
        .recipient(address)
        .manager_id(manager.manager_id)
        .deposit_type(DepositTypes::Direct(
            DirectDepositType::DepositToNewSubaccount,
        ))
        .build();

    let deposit_result = ws_client.fund_movements().deposit(args).await;
    assert!(
        deposit_result.is_ok(),
        "Deposit failed: {:?}",
        deposit_result.err()
    );

    let hashes = deposit_result.unwrap();

    for hash in hashes {
        println!("Deposit transaction hash: {:?}", hash);
    }
}

#[tokio::test]
async fn test_deposit_to_existing_subaccount() {
    let ws_client = common::get_test_ws_client().await;
    ws_client.login().await.expect("Failed to login");

    let address: Address = ws_client
        .derive_wallet
        .clone()
        .unwrap()
        .parse()
        .expect("Couldnt parse wallet address");

    let args = DepositArgs::builder()
        .asset(SupportDepositAssets::USDC)
        .amount(BigDecimal::from_str("10.00").unwrap())
        .recipient(address)
        .subaccount_id(75741)
        .deposit_type(DepositTypes::Direct(DirectDepositType::Deposit))
        .build();

    let deposit_result = ws_client.fund_movements().deposit(args).await;
    assert!(
        deposit_result.is_ok(),
        "Deposit failed: {:?}",
        deposit_result.err()
    );

    let hashes = deposit_result.unwrap();

    for hash in hashes {
        println!("Deposit transaction hash: {:?}", hash);
    }
}

#[tokio::test]
async fn are_all_assets_supported_for_deposit() {
    let ws_client = common::get_test_ws_client().await;
    let currencies = ws_client
        .rpc()
        .market_data()
        .get_all_currencies()
        .await
        .expect("Failed to get all currencies");

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

    // create an array of tuples of (currency, address) for all currencies with an address
    let currencies_with_address = asset_name_to_erc20_details
        .iter()
        .filter_map(|(currency, curreny)| {
            curreny
                .erc20
                .underlying_erc20
                .as_ref()
                .map(|address| (currency.clone(), address.clone()))
        })
        .collect::<Vec<_>>();

    for (currency, address) in currencies_with_address {
        // we check that the SupportDepositAssets enum has a variant for this currency
        let variant_name = format!("SupportDepositAssets::{}", currency);
        let variant_exists = SupportDepositAssets::from_str(&currency).is_ok();
        assert!(
            variant_exists,
            "Variant {} does not exist for currency {}",
            variant_name, currency
        );
        println!(
            "Currency: {}, Address: {}, Variant: {}",
            currency, address, variant_name
        );
    }
}
