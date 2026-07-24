use alloy::{
    hex, signers::{Signer, local::PrivateKeySigner}
};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

fn utc_now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_millis()
}

pub async fn sign_ws_login(smart_contract_wallet: &str, wallet: &PrivateKeySigner) -> Value {
    // Parse private key into a LocalWallet
    let timestamp = utc_now_ms().to_string();
    let signature = wallet
        .sign_message(timestamp.as_bytes())
        .await
        .expect("Failed to sign message");
    let signature_hex = hex::encode(signature.as_bytes());

    let result = serde_json::json!({
        "wallet": smart_contract_wallet.to_string(),
        // "wallet": format!("{:#x}", smart_contract_wallet),
        "timestamp": timestamp,
        "signature": signature_hex,
    });
    result
}
