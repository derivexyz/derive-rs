//! A currency with more than one spot entry used to panic inside the client's constructor; it
//! is an error now, and currencies without a spot entry are skipped as before.

use derive_rs::types::ClientError;
use derive_rs::utils::single_spot_per_currency;

#[test]
fn one_spot_per_currency_is_kept_and_none_is_skipped() {
    let currencies = vec![
        ("ETH".to_string(), vec!["eth-spot"]),
        ("NOSPOT".to_string(), vec![]),
        ("BTC".to_string(), vec!["btc-spot"]),
    ];
    let kept = single_spot_per_currency(currencies).unwrap();
    assert_eq!(
        kept,
        vec![
            ("ETH".to_string(), "eth-spot"),
            ("BTC".to_string(), "btc-spot")
        ]
    );
}

#[test]
fn a_second_spot_entry_is_an_error_naming_the_currency_not_a_panic() {
    let currencies = vec![
        ("ETH".to_string(), vec!["eth-spot"]),
        ("USDC".to_string(), vec!["usdc-spot-1", "usdc-spot-2"]),
    ];
    let err = single_spot_per_currency(currencies).unwrap_err();
    match err {
        ClientError::InvalidOrder(message) => {
            assert!(message.contains("USDC"), "{message}");
            assert!(message.contains("more than one spot"), "{message}");
        }
        other => panic!("expected an invalid-order error, got {other:?}"),
    }
}
