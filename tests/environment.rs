use derive_rs::{Environment, Network, actions::get_domain_separator, constants::OnChainConstants};

#[test]
fn custom_endpoints_preserve_urls_and_selected_network_settings() {
    for (network, preset) in [
        (Network::Mainnet, Environment::Mainnet),
        (Network::Testnet, Environment::Testnet),
    ] {
        for rpc_url in [
            None,
            Some("http://localhost:8545/CaseSensitive".to_string()),
        ] {
            let url = "ws://localhost:8080/v3/ws?token=CaseSensitive";
            let custom = Environment::Custom {
                network,
                ws_url: url.to_string(),
                rpc_url: rpc_url.clone(),
            };
            assert_eq!(custom.network(), network);
            assert_eq!(custom.get_url(), url);
            assert_eq!(
                custom.get_default_rpc(),
                rpc_url.unwrap_or_else(|| preset.get_default_rpc())
            );
            assert_eq!(get_domain_separator(&custom), get_domain_separator(&preset));
            assert_contracts_match(custom, preset.clone());
        }
    }
}

fn assert_contracts_match(custom: Environment, preset: Environment) {
    let custom = OnChainConstants::new(custom);
    let preset = OnChainConstants::new(preset);
    assert_eq!(
        custom.action_manager_address(),
        preset.action_manager_address()
    );
    assert_eq!(custom.vapp_address(), preset.vapp_address());
    assert_eq!(
        custom.withdrawal_outbox_address(),
        preset.withdrawal_outbox_address()
    );
    assert_eq!(custom.spot_vault_address(), preset.spot_vault_address());
    assert_eq!(
        custom.withdrawal_factory_address(),
        preset.withdrawal_factory_address()
    );
}
