use bigdecimal::BigDecimal;
use derive_rs::{
    WsClient,
    actions::OrderArgs,
    models::{CancelOrderRequest, Direction, OrderType, TimeInForce},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::INFO)
            .finish(),
    )?;
    let client = WsClient::from_env().await?;

    client.login().await?;

    let order = OrderArgs::builder()
        .instrument_name("HYPE-PERP".to_string())
        .amount(BigDecimal::from(1))
        .limit_price(BigDecimal::from(50))
        .direction(Direction::Buy)
        .order_type(OrderType::Limit)
        .time_in_force(TimeInForce::Gtc)
        .build();

    let result = client.orders().place(order).await?;

    println!("Order placed successfully.");

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let cancel_params = CancelOrderRequest::builder()
        .order_id(result.order.order_id)
        .instrument_name("HYPE-PERP".to_string())
        .subaccount_id(client.subaccount_id.unwrap())
        .try_into()?;
    let cancel_result = client.orders().cancel_order(cancel_params).await;
    match cancel_result {
        Ok(_) => {
            println!("Order cancelled!");
        }
        Err(_err) => {
            println!("Order cancellation failed.");
        }
    }
    Ok(())
}
