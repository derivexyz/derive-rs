use std::str::FromStr;

use bigdecimal::BigDecimal;
use derive_rs::models::{AssetType, Instrument};
use derive_rs::types::ClientError;
use derive_rs::utils::snap_to_instrument_grid;

fn dec(s: &str) -> BigDecimal {
    BigDecimal::from_str(s).unwrap()
}

fn perp(tick_size: &str, amount_step: &str, minimum_amount: &str) -> Instrument {
    Instrument {
        amount_step: amount_step.to_string(),
        base_asset_address: "0x0".to_string(),
        base_asset_sub_id: "0".to_string(),
        base_currency: "ETH".to_string(),
        base_fee: "0".to_string(),
        erc20_details: None,
        fifo_min_allocation: "0".to_string(),
        instrument_name: "ETH-PERP".to_string(),
        instrument_type: AssetType::Perp,
        is_active: true,
        maker_fee_rate: "0".to_string(),
        mark_price_fee_rate_cap: None,
        maximum_amount: dec("10000"),
        minimum_amount: dec(minimum_amount),
        option_details: None,
        perp_details: None,
        pro_rata_amount_step: "0".to_string(),
        pro_rata_fraction: "0".to_string(),
        quote_currency: "USD".to_string(),
        risk_universe_id: 0,
        scheduled_activation: 0,
        scheduled_deactivation: 0,
        taker_fee_rate: "0".to_string(),
        tick_size: dec(tick_size),
    }
}

#[test]
fn price_snaps_to_the_tick_grid() {
    let (price, _) =
        snap_to_instrument_grid(&perp("0.1", "0.01", "0.01"), &dec("100.04"), &dec("1")).unwrap();
    assert_eq!(price, dec("100.0"));
}

#[test]
fn amount_snaps_to_the_amount_step_not_the_minimum() {
    let (_, amount) =
        snap_to_instrument_grid(&perp("0.1", "0.01", "0.1"), &dec("100"), &dec("0.25")).unwrap();
    assert_eq!(amount, dec("0.25"));
}

#[test]
fn an_amount_below_the_minimum_is_refused_not_rounded_to_zero() {
    let err = snap_to_instrument_grid(&perp("0.1", "0.01", "0.1"), &dec("100"), &dec("0.04"))
        .unwrap_err();
    assert!(matches!(err, ClientError::InvalidOrder(_)), "got {err:?}");
}

#[test]
fn flooring_never_produces_a_zero_size_order() {
    let (_, amount) =
        snap_to_instrument_grid(&perp("0.1", "0.1", "0.1"), &dec("100"), &dec("0.19")).unwrap();
    assert_eq!(amount, dec("0.1"));
    assert!(
        snap_to_instrument_grid(&perp("0.1", "0.1", "0.1"), &dec("100"), &dec("0.09")).is_err()
    );
}

#[test]
fn an_amount_above_the_maximum_is_refused() {
    let err = snap_to_instrument_grid(&perp("0.1", "0.01", "0.1"), &dec("100"), &dec("20000"))
        .unwrap_err();
    assert!(matches!(err, ClientError::InvalidOrder(_)), "got {err:?}");
}

/// A `ClientError` that cannot cross a task boundary makes every reconnect loop that would
/// handle one unspawnable.
#[test]
fn client_error_is_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<ClientError>();
}
