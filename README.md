<div align="center">

[![Derive Protocol](https://github.com/derivexyz/derive-rs/raw/master/header.png)](https://derive.xyz)


[![Crates.io](https://img.shields.io/crates/v/derive-rs.svg)](https://crates.io/crates/derive-rs)
[![Tests](https://github.com/derivexyz/derive-rs/actions/workflows/common.yaml/badge.svg)](https://github.com/derivexyz/derive-rs/actions/workflows/common.yaml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/derivexyz/derive-rs/blob/master/LICENSE.md)

**Official Rust SDK for the Derive Protocol**

A type-safe, async Rust client for trading, market data, account management, RFQs, and protocol interactions over WebSocket.

[Documentation](https://v3.docs.derive.xyz/) ·
[Examples](#examples) ·
[API coverage](#api-coverage) ·
[Contributing](#contributing)

</div>

---

## Installation

```bash
cargo add derive-rs
```

## Quick start

### Public market data

```rust
// examples/get_all_instruments.rs
use derive_rs::{
    Environment, WsClient,
    models::{AssetType, GetAllInstrumentsRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WsClient::new_public(Environment::Testnet).await?;

    let params = GetAllInstrumentsRequest::builder()
        .expired(false)
        .instrument_type(AssetType::Option)
        .try_into()?;

    let instruments = client
        .rpc()
        .market_data()
        .get_all_instruments(params)
        .await?;

    println!("Available instruments: {:#?}", instruments);
    Ok(())
}

```

### Authenticated trading

```rust
// examples/simple_order.rs
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

```

### Streaming market data

```rust
// examples/ws_stream_ticker.rs
mod common;
use derive_rs::{models::TickerSlimNotification, types::ExternalEvent};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

use crate::common::get_test_ws_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws_client = get_test_ws_client().await;

    let mut eth_ticker_stream: BroadcastStream<TickerSlimNotification> = ws_client
        .subscriptions()
        .market_data()
        .ticker_slim("ETH-USDC", "100")
        .await?;

    loop {
        tokio::select! {
            Some(eth_ticker) = eth_ticker_stream.next() => {
                match eth_ticker {
                    Ok(ticker) => println!("ETH Ticker: {:?}", ticker),
                    Err(e) => {
                        eprintln!("Error receiving ETH ticker: {:?}", e);
                        break; // Exit the loop on error
                    }
                }
            }
            event = ws_client.run_till_event() => {
                match event {
                    ExternalEvent::Connected => {
                        let _ = ws_client.login().await;
                        let _ = ws_client.resubscribe_all().await;
                        println!("WebSocket connected and resubscribed to all channels.");
                    }
                    ExternalEvent::Disconnected => {
                        println!("WebSocket disconnected");
                    }
                    ExternalEvent::Exited => {
                        println!("WebSocket exited");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

```

## API design

### Signable Actions 

The SDK groups functionality by signing domain rather than exposing a single flat client API.

```rust
// Orders
client.orders().place(order_args).await?;
client.orders().replace(replace_args).await?;

// RFQs
client.rfqs().send_rfq(rfq_request).await?;
client.rfqs().execute_best_quote(quote_args).await?;

// Fund movements
client.fund_movements().deposit(deposit_args).await?;
client.fund_movements().withdraw(withdraw_args).await?;

// Session keys
client.session_keys().add(session_key_args).await?;
```

Signable actions use typed argument structures and EIP-712 signing.

```rust
use derive_rs::actions::{ExecuteQuoteArgs, OrderArgs, ReplaceArgs};
```

### RPC requests

The SDK provides typed RPC requests for market data, account management, and other protocol interactions. Every RPC request is strongly typed and returns a typed response.

The RPC client is accessible via `client.rpc()` and is grouped by namespace.



## Capabilities

| Area | Supported functionality |
|---|---|
| Market data | Instruments, order books, trades, tickers |
| Streaming | Public and private WebSocket subscriptions |
| Orders | Place, replace, cancel, post-only and reduce-only orders |
| RFQ | Create RFQs, submit quotes, execute quotes |
| Accounts | Subaccounts, positions, collateral and balances |
| Authentication | Wallet authentication and session keys |
| Risk controls | Cancel-on-disconnect and market-maker protection |
| Fund movements | Deposits, withdrawals and transfers |

The client also handles WebSocket heartbeats, reconnection, and subscription recovery.

## Examples

The repository contains runnable examples and integration tests covering common workflows.

| Example | Description |
|---|---|
| [`get_all_currencies`](examples/get_all_currencies.rs) | Fetch supported currencies and ERC-20 details |
| [`ws_stream_tickers`](examples/ws_stream_tickers.rs) | Stream ticker updates for multiple instruments |
| [`ws_rfq_subscriber`](examples/ws_rfq_subscriber.rs) | Subscribe to RFQ updates |
| [`order_lifecycle`](tests/order_lifecycle.rs) | Create, replace, and cancel orders |
| [`rfq`](tests/rfq.rs) | End-to-end RFQ workflow |

Run an example with:

```bash
cargo run --example ws_stream_tickers
```

## API coverage

<details>
<summary><strong>Market data RPC</strong></summary>

- `get_all_instruments`
- `get_instrument`
- `get_all_currencies`
- `get_ticker`
- `get_orderbook`
- `get_trade_history`

</details>

<details>
<summary><strong>Trading RPC</strong></summary>

- `order`
- `replace_order`
- `cancel_order`
- `cancel_all_orders`
- `get_order`
- `get_open_orders`

</details>

<details>
<summary><strong>RFQ RPC</strong></summary>

- `send_rfq`
- `poll_rfqs`
- `send_quote`
- `execute_best_quote`
- `cancel_batch_rfqs`

</details>

<details>
<summary><strong>Account RPC</strong></summary>

- `get_subaccount`
- `get_subaccounts`
- `get_positions`
- `get_collateral`
- `set_cancel_on_disconnect`

</details>

<details>
<summary><strong>Public subscriptions</strong></summary>

- `ticker`
- `ticker_slim`
- `orderbook`
- `trades`
- `instrument`

</details>

<details>
<summary><strong>Private subscriptions</strong></summary>

- `orders`
- `positions`
- `account_summary`
- `trades`
- `rfqs`

</details>

## Configuration

`WsClient::from_env()` reads client configuration from environment variables.

```rust
// examples/env_login.rs
use derive_rs::WsClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WsClient::from_env().await?;

    client.login().await?;

    Ok(())
}

```

The current client expects:

```text
DERIVE_PRIVATE_KEY
DERIVE_WALLET
DERIVE_SUBACCOUNT_ID
DERIVE_ENVIRONMENT
```

## Environments

| Environment | WebSocket URL | Network |
|---|---|---|
| Testnet | `wss://testnet.api.derive.xyz/v3/ws` | Sepolia |
| Mainnet | `wss://api.lyra.finance/ws` | Ethereum |

## Architecture

```text
Your application
      │
      ▼
┌───────────────────────────────────────┐
│              derive-rs                │
│                                       │
│  Actions       RPC      Subscriptions │
│  (signing)  (request)     (streaming) │
│                                       │
│          WebSocket client             │
└──────────────────┬────────────────────┘
                   │
                   ▼
          Derive Protocol API
```

## Testing

```bash
cargo test
```

Run a specific test:

```bash
cargo test order_lifecycle
```

Enable debug logging:

```bash
RUST_LOG=debug cargo test
```

## Development

```bash
git clone https://github.com/derivexyz/derive-rs.git
cd derive-rs

make build
make test
make fmt
make lint
```

## Contributing

Any issues and feature requests can be submitted via GitHub issues. Pull requests are welcome!

1. Fork the repository.
2. Create a feature branch.
3. Make and test your changes.
4. Push the branch.
5. Open a pull request into `dev` branch.
6. The PR will be reviewed and merged by the maintainers.

## Resources

- [Derive](https://derive.xyz)
- [Protocol documentation](https://docs.derive.xyz)
- [Rust API documentation](https://docs.rs/derive-rs)
- [Discord](https://discord.gg/derive)
- [X / Twitter](https://twitter.com/derivexyz)

## License

Licensed under the MIT License. See [LICENSE](LICENSE).

## Disclaimer

This software is provided as-is, without warranty. Trading derivatives involves substantial risk of loss.
