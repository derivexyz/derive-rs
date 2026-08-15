use bigdecimal::BigDecimal;
use derive_rs::{
    WsClient,
    actions::OrderArgs,
    models::{CancelOrderRequest, Direction, OrderType, TimeInForce},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WsClient::from_env().await?;

    client.login().await?;

    let order = OrderArgs::builder()
        .instrument_name("ETH-PERP".to_string())
        .amount(BigDecimal::from(1))
        .limit_price(BigDecimal::from(1500))
        .direction(Direction::Buy)
        .order_type(OrderType::Limit)
        .time_in_force(TimeInForce::Gtc)
        .build();

    let result = client.orders().place(order).await?;

    println!("Order placed: {result:#?}");

    let cancel_params = CancelOrderRequest::builder()
        .order_id(result.order.order_id)
        .instrument_name("ETH-PERP".to_string())
        .subaccount_id(client.subaccount_id.unwrap())
        .try_into()?;
    let cancel_result = client.orders().cancel_order(cancel_params).await?;

    println!("Order cancelled: {cancel_result:#?}");

    Ok(())
}
