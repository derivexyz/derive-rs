mod common;

#[tokio::test]
async fn test_ws_client_login() {
    let ws_client = common::get_test_ws_client().await;
    let login_result = ws_client.login().await;
    assert!(
        login_result.is_ok(),
        "Login failed: {:?}",
        login_result.err()
    );
}
