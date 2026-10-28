mod common;

use bigdecimal::{BigDecimal, FromPrimitive};
use bytes::Bytes;
use derive_rs::actions::OrderArgs;
use derive_rs::models::{
    CancelOrderRequest, Direction, GetInstrumentRequest, GetTickerRequest, OrderStatus, OrderType,
    SubaccountBalancesNotification, SubaccountOrdersNotification, TimeInForce,
};
use derive_rs::subscriptions::channel_specs::accounting::SubaccountBalancesChannelSpec;
use derive_rs::subscriptions::channel_specs::trading::SubaccountOrdersChannelSpec;
use derive_rs::types::ChannelSpec;
use serde_json::{Value, json};

fn order(order_id: &str, status: &str) -> Value {
    json!({
        "amount": "0.5",
        "average_price": "0",
        "cancel_reason": "",
        "creation_timestamp": 1761600000000i64,
        "direction": "buy",
        "extra_fee": "0",
        "filled_amount": "0",
        "instrument_name": "ETH-PERP",
        "is_transfer": false,
        "label": "quoter",
        "last_update_timestamp": 1761600000000i64,
        "limit_price": "2500.5",
        "max_fee": "10",
        "mmp": false,
        "nonce": "1761600000000123",
        "order_fee": "0",
        "order_id": order_id,
        "order_status": status,
        "order_type": "limit",
        "quote_id": null,
        "replaced_order_id": null,
        "signature": "0xabc",
        "signature_expiry_sec": 1761600600i64,
        "signed_limit_price": null,
        "signer": "0xdef",
        "subaccount_id": 62060,
        "time_in_force": "post_only",
        "trigger_price": null
    })
}

fn notification(data: Value) -> Bytes {
    Bytes::from(
        json!({
            "jsonrpc": "2.0",
            "method": "subscription",
            "params": { "channel": "62060.orders", "data": data }
        })
        .to_string(),
    )
}

#[test]
fn subaccount_orders_decodes_the_array_derive_pushes() {
    let payload = notification(json!([order("o-1", "open"), order("o-2", "filled")]));
    let decoded: SubaccountOrdersNotification =
        SubaccountOrdersChannelSpec::decode(&payload).expect("array payload decodes");
    assert_eq!(decoded.len(), 2);
    assert_eq!(decoded[0].order_id, "o-1");
    assert_eq!(decoded[0].subaccount_id, 62060);
    assert_eq!(decoded[1].order_status, OrderStatus::Filled);
}

// The shape behind staging's "invalid type: map, expected a string" on 62060.balances.
#[test]
fn subaccount_balances_decodes_the_array_derive_pushes() {
    let payload = notification(json!([{
        "name": "USDC",
        "new_balance": "1000.5",
        "previous_balance": "1000",
        "update_type": "trade"
    }]));
    let decoded: SubaccountBalancesNotification =
        SubaccountBalancesChannelSpec::decode(&payload).expect("array payload decodes");
    assert_eq!(decoded.len(), 1);
    assert_eq!(decoded[0].name, "USDC");
}

// Live, against testnet: a decode failure is only logged by the client and the update dropped, so
// it shows here as no update arriving for the order we placed.
#[tokio::test]
async fn subaccount_orders_stream_delivers_a_placed_order() {
    use std::time::Duration;
    use tokio_stream::StreamExt;

    let ws_client = common::get_test_ws_client().await;
    ws_client.login().await.expect("Login failed");
    let subaccount_id = ws_client.subaccount_id.expect("Must have a subaccount_id");

    let mut orders = ws_client
        .subscriptions()
        .trading()
        .subaccount_orders(&subaccount_id.to_string())
        .await
        .expect("subscribe to orders");

    let instrument_name = "ETH-PERP".to_string();
    let ticker = ws_client
        .rpc()
        .market_data()
        .get_ticker(
            GetTickerRequest::builder()
                .instrument_name(instrument_name.clone())
                .try_into()
                .expect("ticker request"),
        )
        .await
        .expect("get ticker");
    let instrument = ws_client
        .rpc()
        .market_data()
        .get_instrument(
            GetInstrumentRequest::builder()
                .instrument_name(instrument_name.clone())
                .try_into()
                .expect("instrument request"),
        )
        .await
        .expect("get instrument");

    // far below the index so it rests and never fills
    let placed = ws_client
        .orders()
        .place(
            OrderArgs::builder()
                .instrument_name(instrument_name.clone())
                .amount(instrument.minimum_amount.clone())
                .limit_price(ticker.i.clone() * BigDecimal::from_f64(0.9).expect("offset"))
                .direction(Direction::Buy)
                .order_type(OrderType::Limit)
                .time_in_force(TimeInForce::PostOnly)
                .build(),
        )
        .await
        .expect("place order");
    let order_id = placed.order.order_id.clone();

    let received = tokio::time::timeout(Duration::from_secs(15), async {
        while let Some(subaccount_orders) = orders.next().await {
            let Ok(subaccount_orders) = subaccount_orders else {
                continue;
            };
            if let Some(order) = subaccount_orders.iter().find(|o| o.order_id == order_id) {
                return Some(order.clone());
            }
        }
        None
    })
    .await;

    // cancel before asserting, so a failure does not leave the order resting
    ws_client
        .rpc()
        .orderbook()
        .cancel(
            CancelOrderRequest::builder()
                .instrument_name(instrument_name)
                .order_id(order_id.clone())
                .subaccount_id(subaccount_id)
                .try_into()
                .expect("cancel request"),
        )
        .await
        .expect("cancel order");

    let update = received
        .expect("no update for the placed order within 15s; check the log for a decode error")
        .expect("orders stream ended");
    assert_eq!(update.order_status, OrderStatus::Open);
}
