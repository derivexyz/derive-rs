use bigdecimal::BigDecimal;

pub fn round_to_ticks(price: &BigDecimal, tick_size: &BigDecimal) -> BigDecimal {
    let ratio = price / tick_size;
    let rounded = ratio.with_scale_round(0, bigdecimal::RoundingMode::HalfUp);
    rounded * tick_size
}
