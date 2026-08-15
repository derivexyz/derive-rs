use bigdecimal::{BigDecimal, FromPrimitive};
use derive_rs::{
    actions::{OrderArgs, ReplaceArgs},
    models::{
        CancelOrderRequest, Direction, GetInstrumentRequest, GetTickerRequest, OrderStatus,
        OrderType, TimeInForce,
    },
};
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
        .amount(instrument.minimum_amount.clone())
        .limit_price(ticker.i.clone() * BigDecimal::from_f64(0.99).expect("Failed to make offset"))
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

    // we fetch the order from the rpc to be sure it was created correctly.
    let order_id = order_result.as_ref().unwrap().order.order_id.clone();
    let existing_order = ws_client.orders().get_order(order_id).await;
    assert!(
        existing_order.is_ok(),
        "Get Order by ID failed: {:?}",
        existing_order.err()
    );
    // we ensure it is is open;
    let order = existing_order.unwrap();
    assert!(
        order.order_status == OrderStatus::Open,
        "Order is not open: {:?}",
        order.order_status
    );
    // we now update the order.

    let order = order_result.unwrap();

    println!("Order created: {:?}", order);
    // sleep(std::time::Duration::from_secs(1)).await;

    let replace_params = ReplaceArgs::builder()
        .instrument_name(instrument_name.clone())
        .amount(
            instrument.minimum_amount * BigDecimal::from_f64(2.0).expect("Failed to make offset"),
        )
        .limit_price(ticker.i.clone() - BigDecimal::from_f64(1.0).expect("Failed to parse 1.01"))
        .direction(Direction::Buy)
        .order_type(OrderType::Limit)
        .time_in_force(TimeInForce::Gtc)
        .order_id_to_cancel(
            order
                .order
                .order_id
                .clone()
                .parse()
                .expect("Failed to parse order_id"),
        )
        // .nonce_to_cancel(order.order.nonce.parse().expect("Nonce is incorrect"))
        .build();

    let replace_result = ws_client.orders().replace(replace_params).await;
    assert!(
        replace_result.is_ok(),
        "Replace Order failed: {:?}",
        replace_result.err()
    );

    let replacement_order = replace_result.unwrap();

    // we now have an order, we can cancel it
    let cancel_params = CancelOrderRequest::builder()
        .instrument_name(instrument_name)
        .order_id(
            replacement_order
                .order
                .expect("We should have a replacement order")
                .order_id
                .clone(),
        )
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
