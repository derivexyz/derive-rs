use bigdecimal::BigDecimal;

use crate::{models::Instrument, types::ClientError};

pub fn round_to_ticks(price: &BigDecimal, tick_size: &BigDecimal) -> BigDecimal {
    let ratio = price / tick_size;
    let rounded = ratio.with_scale_round(0, bigdecimal::RoundingMode::HalfUp);
    rounded * tick_size
}

/// Floors rather than rounds: rounding a size up can take an order past the limit the caller
/// sized it against.
pub fn floor_to_step(amount: &BigDecimal, step: &BigDecimal) -> BigDecimal {
    let ratio = amount / step;
    let floored = ratio.with_scale_round(0, bigdecimal::RoundingMode::Floor);
    floored * step
}

/// Snaps a price and an amount onto the grid the venue will accept them on.
///
/// `minimum_amount` is a floor to refuse at, not a step to round onto. Rounding an amount onto
/// it inflates sizes (0.25 becomes 0.30 against a 0.1 minimum) and turns anything under half
/// the minimum into a zero-size order.
pub fn snap_to_instrument_grid(
    instrument: &Instrument,
    price: &BigDecimal,
    amount: &BigDecimal,
) -> Result<(BigDecimal, BigDecimal), ClientError> {
    let amount_step = instrument.amount_step.parse::<BigDecimal>().map_err(|e| {
        ClientError::InvalidOrder(format!(
            "{} has an unparseable amount_step '{}': {e}",
            instrument.instrument_name, instrument.amount_step
        ))
    })?;

    let price = if instrument.tick_size > 0i32 {
        round_to_ticks(price, &instrument.tick_size)
    } else {
        price.clone()
    };
    let amount = if amount_step > 0i32 {
        floor_to_step(amount, &amount_step)
    } else {
        amount.clone()
    };

    if amount < instrument.minimum_amount {
        return Err(ClientError::InvalidOrder(format!(
            "{}: amount {amount} is below the minimum {} (amount_step {amount_step})",
            instrument.instrument_name, instrument.minimum_amount
        )));
    }
    if amount > instrument.maximum_amount {
        return Err(ClientError::InvalidOrder(format!(
            "{}: amount {amount} is above the maximum {}",
            instrument.instrument_name, instrument.maximum_amount
        )));
    }
    Ok((price, amount))
}

/// The one spot entry of each currency, keyed by currency name. A currency with no spot entry
/// is skipped; one with more than one is an error naming it. The venue controls this data, so a
/// second spot market for a currency must not be a panic inside every client's constructor.
pub fn single_spot_per_currency<T>(
    currencies: impl IntoIterator<Item = (String, Vec<T>)>,
) -> Result<Vec<(String, T)>, ClientError> {
    let mut out = Vec::new();
    for (name, spots) in currencies {
        let mut spots = spots.into_iter();
        let Some(spot) = spots.next() else {
            continue;
        };
        if spots.next().is_some() {
            return Err(ClientError::InvalidOrder(format!(
                "currency {name:?} has more than one spot entry; the client cannot choose one"
            )));
        }
        out.push((name, spot));
    }
    Ok(out)
}
