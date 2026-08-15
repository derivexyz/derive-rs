use std::str::FromStr;

use bigdecimal::BigDecimal;
use derive_rs::{
    actions::{CreateVaultArgs, DepositVaultArgs},
    models::{GetCuratedVaultsRequest, GetSubaccountRequest, GetVaultRequest},
};

mod common;

#[tokio::test]
#[ignore]
async fn test_vault_create() {
    let ws_client = common::get_test_ws_client_2().await;
    ws_client.login().await.expect("Failed to login");
    let subaccount_id = ws_client.subaccount_id.unwrap();
    // we use the tracing subscriber to capture logs for debugging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let subaccount = ws_client
        .rpc()
        .subaccounts()
        .get_subaccount(GetSubaccountRequest { subaccount_id })
        .await
        .expect("Failed to get subaccount");

    // we now fetch the assets and filter for this subaccount manager

    let risk_universes = ws_client
        .rpc()
        .market_data()
        .get_risk_universes()
        .await
        .expect("Failed to get risk universes");

    let risk_universe = risk_universes
        .into_iter()
        .find(|ru| {
            ru.managers
                .iter()
                .any(|m| m.manager_id == subaccount.manager_id)
        })
        .expect("Expected to find PRIME risk universe!");

    println!("Risk Universe: {:?}", risk_universe);

    let manager = risk_universe
        .managers
        .iter()
        .find(|m| m.manager_id == subaccount.manager_id)
        .expect("Expected to find subaccount manager in risk universe!");

    println!("Manager: {:?}", manager);

    // we now get the specific asset we want to use as collateral for the vault
    let collateral = manager
        .collaterals
        .iter()
        .find(|c| c.name == "USDC")
        .expect("Expected to find USDC collateral for the subaccount manager!");

    let create_args = CreateVaultArgs::builder()
        .subaccount_id(subaccount_id)
        .manager_id(subaccount.manager_id as u64)
        .deposit_spot_asset(collateral.clone().address)
        .initial_deposit(BigDecimal::from_str("10200").unwrap())
        .initial_share_price_usd(BigDecimal::from_str("1").unwrap())
        .management_fee_bps(100) // 1% management fee
        .performance_fee_bps(2000) // 10% performance fee
        .max_slippage_bps(100)
        .cooldown_sec(3600) // 10 seconds for testing purposes
        .max_fee_usd(BigDecimal::from_str("1000").unwrap())
        // .benchmark_asset(value) // Optional: denominate the HWM in a spot asset
        .build();

    let create_result = ws_client.vaults().create(create_args).await;
    assert!(
        create_result.is_ok(),
        "Vault creation failed: {:?}",
        create_result.err()
    );
}

#[tokio::test]
async fn test_vault_deposit() {
    let ws_client = common::get_test_ws_client_2().await;
    ws_client.login().await.expect("Failed to login");
    let subaccount_id = ws_client.subaccount_id.unwrap();
    // we use the tracing subscriber to capture logs for debugging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let vaults = ws_client
        .rpc()
        .vault_curators()
        .get_curated_vaults(GetCuratedVaultsRequest {
            wallet: ws_client.derive_wallet.clone().unwrap(),
        })
        .await
        .expect("Failed to list vaults");

    // we ensure there is at least one vault to deposit into
    assert!(
        !vaults.subaccount_ids.is_empty(),
        "Expected at least one vault to deposit into, but found none."
    );
    let first_vault_id = &vaults.subaccount_ids[0];

    let vault_info = ws_client
        .rpc()
        .vault_shareholders()
        .get_vault(GetVaultRequest {
            subaccount_id: *first_vault_id,
        })
        .await
        .expect("Failed to get vault info");

    let deposit_args = DepositVaultArgs::builder()
        .subaccount_id(subaccount_id)
        .vault_id(*first_vault_id) // Assuming the vault ID is 1 for testing
        .deposit_amount(BigDecimal::from_str("10").unwrap())
        .deposit_spot_asset(vault_info.protocol.config.deposit_spot_asset.to_string())
        .build();

    let deposit_result = ws_client.vaults().deposit(deposit_args).await;
    assert!(
        deposit_result.is_ok(),
        "Vault deposit failed: {:?}",
        deposit_result.err()
    );
}
