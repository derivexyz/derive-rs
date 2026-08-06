<div align="center"># Derive Rust Sdk

# 🦀 derive-rs

This is the official Rust SDK for the Derive Protocol. It is designed to interact with the Derive Protocol's smart contracts and provides a set of tools and utilities for developers to interact with the protocol in a Rust environment.

### Official Rust SDK for Derive Protocol

We expose Websocket as the transport layer. The SDK is designed to be easy to use and integrate into existing Rust projects.

[![Crates.io](https://img.shields.io/crates/v/derive-rs.svg)](https://crates.io/crates/derive-rs)

[![Documentation](https://docs.rs/derive-rs/badge.svg)](https://docs.rs/derive-rs)## Installation

[![License](https://img.shields.io/crates/l/derive-rs.svg)](https://github.com/derivexyz/derive-rs/blob/master/LICENSE)

[![Rust](https://img.shields.io/badge/rust-2024%2B-orange.svg)](https://www.rust-lang.org)```

[![Build Status](https://img.shields.io/github/actions/workflow/status/derivexyz/derive-rs/ci.yml?branch=master)](https://github.com/derivexyz/derive-rs/actions)cargo add derive-rs


**High-performance, type-safe Rust client for interacting with Derive Protocol's perpetual futures and options platform**

## Examples

[Getting Started](#-getting-started) •

[Documentation](https://docs.rs/derive-rs) •

[Examples](#-examples) •

[Features](#-features) •

[Contributing](#-contributing)

</div>


## ✨ Overview


`derive-rs` is a comprehensive Rust SDK for [Derive Protocol](https://derive.xyz), providing seamless access to decentralized derivatives trading. Built with modern async Rust, it offers a robust, production-ready solution for algorithmic trading, market making, and DeFi integrations.

### Why derive-rs?

- 🚀 **Blazingly Fast** - Built on Tokio with async/await for maximum concurrency
- 🔒 **Type-Safe** - Compile-time guarantees with comprehensive type definitions
- 🔌 **WebSocket & REST** - Real-time market data and reliable REST endpoints
- 📦 **Zero Config** - Works out of the box with sensible defaults
- 🧪 **Battle-Tested** - Extensive test coverage and production-ready
- 🛠️ **Developer Friendly** - Intuitive API design with builder patterns

---

## 🚀 Getting Started

### Installation

Use cargo:

```bash
cargo add derive-rs
```

### Quick Start

#### Public Market Data

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

#### Authenticated Trading

```
use derive_rs::{WsClient, Environment};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client with credentials
    let client = WsClient::new(
        Environment::Testnet,
        Some("YOUR_PRIVATE_KEY".to_string()),
        Some("YOUR_WALLET_ADDRESS".to_string()),
        Some(1234), // subaccount_id
    ).await?;
    
    // Login to the WebSocket
    client.login().await?;
    
    // Place your first order
    use derive_rs::actions::OrderArgs;
    use derive_rs::models::{Direction, OrderType, TimeInForce};
    use bigdecimal::BigDecimal;
    
    let order = OrderArgs::builder()
        .instrument_name("ETH-PERP")
        .amount(BigDecimal::from(1))
        .limit_price(BigDecimal::from(3000))
        .direction(Direction::Buy)
        .order_type(OrderType::Limit)
        .time_in_force(TimeInForce::GTC)
        .build();
    
    let result = client.orders().place(order).await?;
    println!("Order placed: {:#?}", result);
    
    Ok(())
}
```

#### Real-Time Market Data Streams

```
use derive_rs::{WsClient, Environment, types::ExternalEvent};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = WsClient::new_public(Environment::Testnet).await?;
    
    // Subscribe to ticker updates
    let mut ticker_stream = client.subscriptions()
        .market_data()
        .ticker_slim("ETH-USDC", "100")
        .await?;
    
    // Process tickers in real-time
    loop {
        tokio::select! {
            Some(ticker) = ticker_stream.next() => {
                match ticker {
                    Ok(data) => println!("ETH Price: {:?}", data),
                    Err(e) => eprintln!("Error: {:?}", e),
                }
            }
            event = client.run_till_event() => {
                match event {
                    ExternalEvent::Connected => {
                        println!("WebSocket connected");
                        client.resubscribe_all().await?;
                    }
                    ExternalEvent::Disconnected => println!("Disconnected"),
                    ExternalEvent::Exited => break,
                }
            }
        }
    }
    
    Ok(())
}
```

---

## 📚 Examples

The repository includes comprehensive examples demonstrating various features:

| Example | Description |
|---------|-------------|
| [`get_all_currencies`](examples/get_all_currencies.rs) | Fetch all supported currencies and ERC-20 details |
| [`ws_stream_tickers`](examples/ws_stream_tickers.rs) | Real-time ticker streaming for multiple instruments |
| [`ws_rfq_subscriber`](examples/ws_rfq_subscriber.rs) | Subscribe to Request for Quote (RFQ) updates |
| [`order_lifecycle`](tests/order_lifecycle.rs) | Complete order management: create, replace, cancel |
| [`rfq`](tests/rfq.rs) | Request for Quote workflow for large trades |

Run any example with:

```bash
cargo run --example ws_stream_tickers
```

---

## ✨ Features

### Core Capabilities

- ✅ **WebSocket API**
  - Real-time market data streaming
  - Private account updates
  - Automatic reconnection with state recovery
  - Heartbeat/ping-pong handling

- ✅ **Order Management**
  - Place, replace, and cancel orders
  - Support for all order types (limit, market, stop-loss)
  - Time-in-force options (GTC, IOC, FOK)
  - Post-only and reduce-only orders

- ✅ **Request for Quote (RFQ)**
  - Multi-leg RFQ creation
  - Quote execution
  - Position transfers between subaccounts

- ✅ **Market Data**
  - Instrument details and specifications
  - Order book snapshots and updates
  - Trade history
  - Ticker data (full and slim)

- ✅ **Account Management**
  - Subaccount operations
  - Session key authentication
  - Position tracking
  - Balance queries

- ✅ **Risk Management**
  - Market maker protection
  - Cancel on disconnect
  - Portfolio margining

- ✅ **Fund Movements**
  - Deposits and withdrawals
  - Spot transfers
  - Position transfers

### Type-Safe Actions

All trading actions are EIP-712 compliant with cryptographic signing:

```
use derive_rs::actions::{OrderArgs, ReplaceArgs, ExecuteQuoteArgs};
```

### Namespace API

Organized API surface for intuitive usage:

```
// Orders
client.orders().place(order_args).await?;
client.orders().replace(replace_args).await?;

// RFQs
client.rfqs().send_rfq(rfq_request).await?;
client.rfqs().execute_best_quote(quote_args).await?;

// Fund Movements
client.fund_movements().deposit(deposit_args).await?;
client.fund_movements().withdraw(withdraw_args).await?;

// Session Keys
client.session_keys().add(session_key_args).await?;
```

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         Your Application                     │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────┴──────────────────────────────────┐
│                        derive-rs SDK                         │
├──────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐ │
│  │   Actions   │  │     RPC     │  │   Subscriptions     │ │
│  │  (Signing)  │  │  (Request)  │  │   (Streaming)       │ │
│  └─────────────┘  └─────────────┘  └─────────────────────┘ │
│  ┌──────────────────────────────────────────────────────────┤
│  │              WebSocket Client (yawc)                     │
│  └──────────────────────────────────────────────────────────┤
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────┴──────────────────────────────────┐
│                    Derive Protocol API                       │
│           wss://testnet.api.derive.xyz/v3/ws                │
└──────────────────────────────────────────────────────────────┘
```

---

## 🧪 Testing

Run the test suite:

```bash
# All tests
cargo test

# Specific test
cargo test order_lifecycle

# With logging
RUST_LOG=debug cargo test
```

---

## 📖 API Coverage

### RPC Methods

<details>
<summary><b>Market Data</b></summary>

- `get_all_instruments` - List all available instruments
- `get_instrument` - Get specific instrument details
- `get_all_currencies` - List supported currencies
- `get_ticker` - Get current ticker data
- `get_orderbook` - Fetch order book snapshot
- `get_trade_history` - Query historical trades

</details>

<details>
<summary><b>Trading</b></summary>

- `order` - Place new order
- `replace_order` - Replace existing order
- `cancel_order` - Cancel order
- `cancel_all_orders` - Cancel all orders
- `get_order` - Get order details
- `get_open_orders` - List open orders

</details>

<details>
<summary><b>RFQ</b></summary>

- `send_rfq` - Create RFQ
- `poll_rfqs` - Poll for RFQs
- `send_quote` - Submit quote
- `execute_best_quote` - Execute quote
- `cancel_batch_rfqs` - Cancel multiple RFQs

</details>

<details>
<summary><b>Account</b></summary>

- `get_subaccount` - Get subaccount details
- `get_subaccounts` - List all subaccounts
- `get_positions` - Get open positions
- `get_collateral` - Get collateral balances
- `set_cancel_on_disconnect` - Configure cancel on disconnect

</details>

### WebSocket Subscriptions

<details>
<summary><b>Public Channels</b></summary>

- `ticker` / `ticker_slim` - Real-time price updates
- `orderbook` - Order book updates
- `trades` - Trade feed
- `instrument` - Instrument updates

</details>

<details>
<summary><b>Private Channels</b></summary>

- `orders` - Order updates
- `positions` - Position updates
- `account_summary` - Account balance updates
- `trades` - User trade executions
- `rfqs` - RFQ notifications

</details>

---

## 🛠️ Advanced Usage

### Environment Variables

Load configuration from environment:

```rust
// examples/env_login.rs
use derive_rs::WsClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Reads DERIVE_PRIVATE_KEY, DERIVE_WALLET, DERIVE_SUBACCOUNT_ID, DERIVE_ENVIRONMENT from the environment
    let client = WsClient::from_env().await?;
    client.login().await?;
    Ok(())
}

```

---

## 🌍 Environments

| Environment | WebSocket URL | Network |
|-------------|---------------|---------|
| **Testnet** | `wss://testnet.api.derive.xyz/v3/ws` | Sepolia |
| **Mainnet** | `wss://api.lyra.finance/ws` | Ethereum |

---

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Setup

```bash
# Clone the repository
git clone https://github.com/derivexyz/derive-rs.git
cd derive-rs

# Build the project
make build

# Run tests
make test

# Check formatting
make fmt

# Run linter
make lint
```

---

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🔗 Resources

- **Website:** [https://derive.xyz](https://derive.xyz)
- **Documentation:** [https://docs.derive.xyz](https://docs.derive.xyz)
- **API Docs:** [https://docs.rs/derive-rs](https://docs.rs/derive-rs)
- **Discord:** [Join our community](https://discord.gg/derive)
- **Twitter:** [@derivexyz](https://twitter.com/derivexyz)

---

## ⚠️ Disclaimer

This software is provided "as is" without warranty of any kind. Trading derivatives involves substantial risk of loss. Use at your own risk.

---

## 🙏 Acknowledgments

Built with ❤️ by the Derive team and contributors.

Special thanks to:
- [Tokio](https://tokio.rs) for async runtime
- [Alloy](https://github.com/alloy-rs) for Ethereum integrations
- The Rust community for amazing tooling

---

<div align="center">

**[⬆ back to top](#-derive-rs)**

Made with 🦀 and ☕

</div>
