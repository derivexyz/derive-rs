use ethers::{
    signers::{LocalWallet, Signer},
    utils::hex,
};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

fn utc_now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_millis()
}

pub async fn sign_ws_login(smart_contract_wallet: &str, wallet: &LocalWallet) -> Value {
    // Parse private key into a LocalWallet
    let timestamp = utc_now_ms().to_string();
    let signature = wallet
        .sign_message(timestamp.clone())
        .await
        .expect("Failed to sign message");
    let signature_hex = hex::encode(signature.to_vec());

    let result = serde_json::json!({
        "wallet": smart_contract_wallet.to_string(),
        // "wallet": format!("{:#x}", smart_contract_wallet),
        "timestamp": timestamp,
        "signature": signature_hex,
    });
    result
}
