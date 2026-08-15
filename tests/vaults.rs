use std::str::FromStr;

use bigdecimal::BigDecimal;
use derive_rs::{
    actions::{
        BurnVaultSharesArgs, CancelAllVaultRequestsArgs, CreateVaultArgs, DepositVaultArgs,
        MintVaultSharesArgs, WithdrawVaultArgs,
    },
    models::{
        GetCuratedVaultsRequest, GetLiveBurnRequestsRequest, GetLiveMintRequestsRequest,
        GetSubaccountRequest, GetVaultRequest, GetVaultSharesRequest,
    },
};

mod common;

#[tokio::test]
#[ignore]
async fn test_vault_create() {
    let ws_client = common::get_test_ws_client_2().await;
    ws_client.login().await.expect("Failed to login");
    let subaccount_id = ws_client.subaccount_id.unwrap();
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
    let curator_ws_client = common::get_test_ws_client_3().await;
    curator_ws_client.login().await.expect("Failed to login");

    let ws_client = common::get_test_ws_client().await;
    ws_client.login().await.expect("Failed to login");

    let vaults = curator_ws_client
        .rpc()
        .vault_curators()
        .get_curated_vaults(GetCuratedVaultsRequest {
            wallet: curator_ws_client.derive_wallet.clone().unwrap(),
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

    assert!(
        vault_info.simulated_share_price_usd.is_some(),
        "Expected vault to have a simulated share price, but found none."
    );

    let deposit_args = DepositVaultArgs::builder()
        .subaccount_id(ws_client.subaccount_id.unwrap())
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

    // we now try to mint the shares for the vault.
    let mint_requests = curator_ws_client
        .rpc()
        .vault_curators()
        .get_live_mint_requests(GetLiveMintRequestsRequest {
            subaccount_id: *first_vault_id,
            limit: 10,
        })
        .await
        .expect("Failed to get live mint requests");

    // let subaccount_id = curator_ws_client.subaccount_id.unwrap();
    // we get the vault_price
    for mint_request in mint_requests.requests {
        println!("Mint Request: {:?}", mint_request);
        // we process them all
        let mint_shares_args = MintVaultSharesArgs::builder()
            .vault_id(*first_vault_id)
            .share_price(
                vault_info
                    .simulated_share_price_usd
                    .clone()
                    .expect("Should have a share price"),
            )
            .user_action_hash(mint_request.user_action_hash.clone())
            .request_id(mint_request.id.clone())
            .build();

        let mint_result = curator_ws_client
            .vaults()
            .mint_shares(mint_shares_args)
            .await;
        assert!(
            mint_result.is_ok(),
            "Mint Vault Shares failed: {:?}",
            mint_result.err()
        );
    }
}

#[tokio::test]
async fn test_vault_withdraw() {
    let curator_ws_client = common::get_test_ws_client_3().await;
    curator_ws_client.login().await.expect("Failed to login");

    let ws_client = common::get_test_ws_client().await;
    ws_client.login().await.expect("Failed to login");

    let vault_id = 75766;
    // we ensure there is at least one vault to deposit into

    let vault_info = ws_client
        .rpc()
        .vault_shareholders()
        .get_vault(GetVaultRequest {
            subaccount_id: vault_id,
        })
        .await
        .expect("Failed to get vault info");

    assert!(
        vault_info.simulated_share_price_usd.is_some(),
        "Expected vault to have a simulated share price, but found none."
    );

    // we get the holdings for the vault to determine how many shares we can burn
    let vault_shares = ws_client
        .rpc()
        .vault_shareholders()
        .get_vault_shares(GetVaultSharesRequest {
            wallet: ws_client.derive_wallet.clone().unwrap(),
        })
        .await
        .expect("Failed to get vault shares");

    // we now filter out the shares for the specific vault we are testing
    let vault_shares_for_vault = vault_shares
        .vaults
        .into_iter()
        .find(|vault| vault.vault.protocol.subaccount_id == vault_id)
        .expect("Should have shares in this vault.");

    let withdraw_args = WithdrawVaultArgs::builder()
        .subaccount_id(ws_client.subaccount_id.unwrap())
        .vault_id(vault_id) // Assuming the vault ID is 1 for testing
        .shares_to_burn(vault_shares_for_vault.shares / BigDecimal::from(2)) // burn half of the shares
        .build();

    ws_client
        .vaults()
        .withdraw(withdraw_args)
        .await
        .expect("Vault withdraw failed");

    // we now try to mint the shares for the vault.
    let burn_requests = curator_ws_client
        .rpc()
        .vault_curators()
        .get_live_burn_requests(GetLiveBurnRequestsRequest {
            subaccount_id: vault_id,
            limit: 10,
        })
        .await
        .expect("Failed to get live mint requests");

    // // we get the vault_price
    for burn_request in burn_requests.requests {
        println!("Burn Request: {:?}", burn_request);
        // we process them all
        let burn_shares_args = BurnVaultSharesArgs::builder()
            .vault_id(vault_id)
            .share_price(
                vault_info
                    .simulated_share_price_usd
                    .clone()
                    .expect("Should have a share price"),
            )
            .user_action_hash(burn_request.user_action_hash.clone())
            .request_id(burn_request.id.clone())
            .build();

        let burn_result = curator_ws_client
            .vaults()
            .burn_shares(burn_shares_args)
            .await;

        if burn_result.is_err() {
            let err = burn_result.err().unwrap();
            println!("Burn Vault Shares failed: {:?}", err);
            let msg = "Vault withdrawal cooldown active";
            if err.to_string().contains(msg) {
                println!("Cooldown active, skipping burn.");
                continue;
            } else {
                panic!("Burn Vault Shares failed: {:?}", err);
            }
        }
        assert!(
            burn_result.is_ok(),
            "Burn Vault Shares failed: {:?}",
            burn_result.err()
        );
    }
}

#[tokio::test]
async fn test_vault_cancel_all() {
    let ws_client = common::get_test_ws_client().await;
    ws_client.login().await.expect("Failed to login");

    let vault_id = 75763;

    let args = CancelAllVaultRequestsArgs::builder()
        .subaccount_id(ws_client.subaccount_id.unwrap())
        .vault_id(vault_id)
        .build();

    let cancel_result = ws_client.vaults().cancel_all_vault_requests(args).await;

    assert!(
        cancel_result.is_ok(),
        "Cancel all vault requests failed: {:?}",
        cancel_result.err()
    );
}
