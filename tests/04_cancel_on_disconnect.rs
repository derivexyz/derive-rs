mod common;

#[tokio::test]
async fn test_ws_set_cancel_on_disconnect() {
    let ws_client = common::get_test_ws_client().await;
    let login_result = ws_client.login().await;
    assert!(
        login_result.is_ok(),
        "Login failed: {:?}",
        login_result.err()
    );
    let set_cancel_result = ws_client.set_cancel_on_disconnect(true).await;
    assert!(
        set_cancel_result.is_ok(),
        "Set cancel on disconnect failed: {:?}",
        set_cancel_result.err()
    );

    println!(
        "Set cancel on disconnect response: {:?}",
        set_cancel_result.unwrap()
    );
}
