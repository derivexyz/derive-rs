// use ethers::prelude::{I256, U256};
use std::str::FromStr;

use alloy::primitives::{I256, U256};
use anyhow::Result;
use bigdecimal::BigDecimal;

fn pow10_bigdecimal(prec: u32) -> BigDecimal {
    let mut v = BigDecimal::from(1u64);
    for _ in 0..prec {
        v *= BigDecimal::from(10u64);
    }
    v
}

pub fn decimal_to_u256(decimal: &BigDecimal) -> Result<U256> {
    decimal_to_u256_with_prec(decimal, 18)
}

pub fn decimal_to_i256(decimal: &BigDecimal) -> Result<I256> {
    decimal_to_i256_with_prec(decimal, 18)
}

pub fn decimal_to_u256_with_prec(decimal: &BigDecimal, prec: u32) -> Result<U256> {
    let factor = pow10_bigdecimal(prec);
    let scaled = (decimal * &factor).with_scale_round(0, bigdecimal::RoundingMode::HalfUp);
    let s = scaled.to_string();
    Ok(U256::from_str(&s)?)
}

pub fn decimal_to_i256_with_prec(decimal: &BigDecimal, prec: u32) -> Result<I256> {
    let factor = pow10_bigdecimal(prec);
    let scaled = (decimal * &factor).with_scale_round(0, bigdecimal::RoundingMode::HalfUp);
    let s = scaled.to_string();
    Ok(I256::from_dec_str(&s)?)
}

pub fn u256_to_decimal_with_prec(u256: U256, prec: u32) -> Result<BigDecimal> {
    let numer = BigDecimal::from_str(&u256.to_string())?;
    let denom = pow10_bigdecimal(prec);
    Ok(numer / denom)
}

pub fn i256_to_decimal_with_prec(i256: I256, prec: u32) -> Result<BigDecimal> {
    let numer = BigDecimal::from_str(&i256.to_string())?;
    let denom = pow10_bigdecimal(prec);
    Ok(numer / denom)
}

pub fn u256_to_decimal(u256: U256) -> Result<BigDecimal> {
    u256_to_decimal_with_prec(u256, 18)
}

pub fn i256_to_decimal(i256: I256) -> Result<BigDecimal> {
    i256_to_decimal_with_prec(i256, 18)
}

pub fn to_e18(value: &BigDecimal) -> Result<U256> {
    // we scale to an e18 representation, which is the standard for ERC20 tokens
    decimal_to_u256_with_prec(value, 18)
}
