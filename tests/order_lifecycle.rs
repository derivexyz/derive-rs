use std::str::FromStr;

use derive_rs::{
    actions::OrderArgs,
    models::openapi::{
        CancelOrderRequest, Direction, GetInstrumentRequest, GetTickerRequest, OrderType,
        TimeInForce,
    },
};

use bigdecimal::BigDecimal;
mod common;

#[tokio::test]
async fn test_ws_order_lifecycle() {
    let ws_client = common::get_test_ws_client().await;
    ws_client.login().await.expect("Login failed");

    let instrument_name = "ETH-PERP".to_string();

    let params = GetTickerRequest::builder()
        .instrument_name(instrument_name.clone())
        .try_into()
        .expect("Failed to build GetInstrumentRequest");

    let result = ws_client.rpc().market_data().get_ticker(params).await;
    assert!(result.is_ok(), "Get Ticker failed: {:?}", result.err());
    let ticker = result.unwrap();

    let instrument_params = GetInstrumentRequest::builder()
        .instrument_name(instrument_name.clone())
        .try_into()
        .expect("Failed to build GetInstrumentRequest");
    let instrument_result = ws_client
        .rpc()
        .market_data()
        .get_instrument(instrument_params)
        .await;
    assert!(
        instrument_result.is_ok(),
        "Get Instrument failed: {:?}",
        instrument_result.err()
    );
    let instrument = instrument_result.unwrap();

    let order_args = OrderArgs::builder()
        .instrument_name(instrument_name.clone())
        .amount(
            BigDecimal::from_str(&instrument.minimum_amount)
                .expect("Failed to parse minimum amount"),
        )
        .limit_price(ticker.i.clone())
        .direction(Direction::Buy)
        .order_type(OrderType::Limit)
        .time_in_force(TimeInForce::Gtc)
        .build();

    let order_result = ws_client.orders().place(order_args).await;

    assert!(
        order_result.is_ok(),
        "Order failed: {:?}",
        order_result.err()
    );

    // we now have an order, we can cancel it
    let order = order_result.unwrap();
    let cancel_params = CancelOrderRequest::builder()
        .instrument_name(instrument_name)
        .order_id(order.order.order_id.clone())
        .subaccount_id(ws_client.subaccount_id.expect("Must have a subaccount_id"))
        .try_into()
        .expect("Failed to build CancelOrderRequest");

    let cancel_result = ws_client.rpc().orderbook().cancel(cancel_params).await;
    assert!(
        cancel_result.is_ok(),
        "Cancel Order failed: {:?}",
        cancel_result.err()
    );
}
