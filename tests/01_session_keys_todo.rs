use alloy::signers::{k256::elliptic_curve::bigint::Random, local::PrivateKeySigner};
use derive_rs::actions::session_key::{CreateSessionKeyArgs, OffChainScope, ProtocolScope};

mod common;

#[tokio::test]
async fn test_ws_create_session_key() -> Result<(), Box<dyn std::error::Error>> {
    let ws_client = common::get_test_ws_client().await;

    let session_wallet = PrivateKeySigner::random();

    let session_key_args = CreateSessionKeyArgs::builder()
        .public_session_key(session_wallet.address().to_string())
        .expiry_second(1793491200)
        .protocol_scopes(vec![ProtocolScope::Admin])
        .off_chain_scopes(vec![OffChainScope::AccountInfo])
        .label("test_session_key".to_string())
        .subaccount_ids(vec![ws_client.subaccount_id.expect("Subaccount id must be set")])
        .build();

    let result = ws_client.session_keys().create(session_key_args).await;
    println!("Create session key result: {:?}", result);
    assert!(result.is_ok(), "Failed to create session key: {:?}", result);
    Ok(())
}


