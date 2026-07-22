use crate::models::asyncapi_subs::*;
use crate::types::{ChannelSpec, RequestScope};
pub mod accounting {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    pub struct SubaccountBalancesChannelSpec {
        pub subaccount_id: String,
    }
    impl SubaccountBalancesChannelSpec {
        pub fn new(subaccount_id: impl Into<String>) -> Self {
            Self {
                subaccount_id: subaccount_id.into(),
            }
        }
    }
    impl ChannelSpec for SubaccountBalancesChannelSpec {
        type Output = SubaccountBalancesNotification;
        fn scope(&self) -> RequestScope {
            RequestScope::Private
        }
        #[allow(clippy::needless_return, clippy::useless_format)]
        fn channel(&self) -> String {
            return format!(
                "{subaccount_id}.balances", subaccount_id = self.subaccount_id.as_str()
            );
        }
    }
}
pub mod liquidations {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    pub struct AuctionsWatchChannelSpec {}
    impl AuctionsWatchChannelSpec {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl ChannelSpec for AuctionsWatchChannelSpec {
        type Output = AuctionsWatchNotification;
        fn scope(&self) -> RequestScope {
            RequestScope::Public
        }
        #[allow(clippy::needless_return, clippy::useless_format)]
        fn channel(&self) -> String {
            return format!("auctions.watch",);
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    pub struct MarginWatchChannelSpec {}
    impl MarginWatchChannelSpec {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl ChannelSpec for MarginWatchChannelSpec {
        type Output = MarginWatchNotification;
        fn scope(&self) -> RequestScope {
            RequestScope::Public
        }
        #[allow(clippy::needless_return, clippy::useless_format)]
        fn channel(&self) -> String {
            return format!("margin.watch",);
        }
    }
}
pub mod market_data {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    pub struct OrderbookChannelSpec {
        pub instrument_name: String,
        pub group: String,
        pub depth: String,
    }
    impl OrderbookChannelSpec {
        pub fn new(
            instrument_name: impl Into<String>,
            group: impl Into<String>,
            depth: impl Into<String>,
        ) -> Self {
            Self {
                instrument_name: instrument_name.into(),
                group: group.into(),
                depth: depth.into(),
            }
        }
    }
    impl ChannelSpec for OrderbookChannelSpec {
        type Output = OrderbookNotification;
        fn scope(&self) -> RequestScope {
            RequestScope::Public
        }
        #[allow(clippy::needless_return, clippy::useless_format)]
        fn channel(&self) -> String {
            return format!(
                "orderbook.{instrument_name}.{group}.{depth}", instrument_name = self
                .instrument_name.as_str(), group = self.group.as_str(), depth = self
                .depth.as_str()
            );
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    pub struct SpotFeedChannelSpec {
        pub currency: String,
    }
    impl SpotFeedChannelSpec {
        pub fn new(currency: impl Into<String>) -> Self {
            Self { currency: currency.into() }
        }
    }
    impl ChannelSpec for SpotFeedChannelSpec {
        type Output = SpotFeedNotification;
        fn scope(&self) -> RequestScope {
            RequestScope::Public
        }
        #[allow(clippy::needless_return, clippy::useless_format)]
        fn channel(&self) -> String {
            return format!("spot_feed.{currency}", currency = self.currency.as_str());
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    pub struct TickerSlimChannelSpec {
        pub instrument_name: String,
        pub interval: String,
    }
    impl TickerSlimChannelSpec {
        pub fn new(
            instrument_name: impl Into<String>,
            interval: impl Into<String>,
        ) -> Self {
            Self {
                instrument_name: instrument_name.into(),
                interval: interval.into(),
            }
        }
    }
    impl ChannelSpec for TickerSlimChannelSpec {
        type Output = TickerSlimNotification;
        fn scope(&self) -> RequestScope {
            RequestScope::Public
        }
        #[allow(clippy::needless_return, clippy::useless_format)]
        fn channel(&self) -> String {
            return format!(
                "ticker_slim.{instrument_name}.{interval}", instrument_name = self
                .instrument_name.as_str(), interval = self.interval.as_str()
            );
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    pub struct TradesByInstrumentChannelSpec {
        pub instrument_name: String,
    }
    impl TradesByInstrumentChannelSpec {
        pub fn new(instrument_name: impl Into<String>) -> Self {
            Self {
                instrument_name: instrument_name.into(),
            }
        }
    }
    impl ChannelSpec for TradesByInstrumentChannelSpec {
        type Output = TradesByInstrumentNotification;
        fn scope(&self) -> RequestScope {
            RequestScope::Public
        }
        #[allow(clippy::needless_return, clippy::useless_format)]
        fn channel(&self) -> String {
            return format!(
                "trades.{instrument_name}", instrument_name = self.instrument_name
                .as_str()
            );
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    pub struct TradesByInstrumentTypeCurrencyChannelSpec {
        pub instrument_type: String,
        pub currency: String,
    }
    impl TradesByInstrumentTypeCurrencyChannelSpec {
        pub fn new(
            instrument_type: impl Into<String>,
            currency: impl Into<String>,
        ) -> Self {
            Self {
                instrument_type: instrument_type.into(),
                currency: currency.into(),
            }
        }
    }
    impl ChannelSpec for TradesByInstrumentTypeCurrencyChannelSpec {
        type Output = TradesByInstrumentTypeCurrencyNotification;
        fn scope(&self) -> RequestScope {
            RequestScope::Public
        }
        #[allow(clippy::needless_return, clippy::useless_format)]
        fn channel(&self) -> String {
            return format!(
                "trades.{instrument_type}.{currency}", instrument_type = self
                .instrument_type.as_str(), currency = self.currency.as_str()
            );
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    pub struct TradesByInstrumentTypeCurrencyTxStatusChannelSpec {
        pub instrument_type: String,
        pub currency: String,
        pub tx_status: String,
    }
    impl TradesByInstrumentTypeCurrencyTxStatusChannelSpec {
        pub fn new(
            instrument_type: impl Into<String>,
            currency: impl Into<String>,
            tx_status: impl Into<String>,
        ) -> Self {
            Self {
                instrument_type: instrument_type.into(),
                currency: currency.into(),
                tx_status: tx_status.into(),
            }
        }
    }
    impl ChannelSpec for TradesByInstrumentTypeCurrencyTxStatusChannelSpec {
        type Output = TradesByInstrumentTypeCurrencyTxStatusNotification;
        fn scope(&self) -> RequestScope {
            RequestScope::Public
        }
        #[allow(clippy::needless_return, clippy::useless_format)]
        fn channel(&self) -> String {
            return format!(
                "trades.{instrument_type}.{currency}.{tx_status}", instrument_type = self
                .instrument_type.as_str(), currency = self.currency.as_str(), tx_status =
                self.tx_status.as_str()
            );
        }
    }
}
pub mod rfqs {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    pub struct SubaccountBestQuotesChannelSpec {
        pub subaccount_id: String,
    }
    impl SubaccountBestQuotesChannelSpec {
        pub fn new(subaccount_id: impl Into<String>) -> Self {
            Self {
                subaccount_id: subaccount_id.into(),
            }
        }
    }
    impl ChannelSpec for SubaccountBestQuotesChannelSpec {
        type Output = SubaccountBestQuotesNotification;
        fn scope(&self) -> RequestScope {
            RequestScope::Private
        }
        #[allow(clippy::needless_return, clippy::useless_format)]
        fn channel(&self) -> String {
            return format!(
                "{subaccount_id}.best.quotes", subaccount_id = self.subaccount_id
                .as_str()
            );
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    pub struct SubaccountQuotesChannelSpec {
        pub subaccount_id: String,
    }
    impl SubaccountQuotesChannelSpec {
        pub fn new(subaccount_id: impl Into<String>) -> Self {
            Self {
                subaccount_id: subaccount_id.into(),
            }
        }
    }
    impl ChannelSpec for SubaccountQuotesChannelSpec {
        type Output = SubaccountQuotesNotification;
        fn scope(&self) -> RequestScope {
            RequestScope::Private
        }
        #[allow(clippy::needless_return, clippy::useless_format)]
        fn channel(&self) -> String {
            return format!(
                "{subaccount_id}.quotes", subaccount_id = self.subaccount_id.as_str()
            );
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    pub struct WalletRfqsChannelSpec {
        pub wallet: String,
    }
    impl WalletRfqsChannelSpec {
        pub fn new(wallet: impl Into<String>) -> Self {
            Self { wallet: wallet.into() }
        }
    }
    impl ChannelSpec for WalletRfqsChannelSpec {
        type Output = WalletRfqsNotification;
        fn scope(&self) -> RequestScope {
            RequestScope::Private
        }
        #[allow(clippy::needless_return, clippy::useless_format)]
        fn channel(&self) -> String {
            return format!("{wallet}.rfqs", wallet = self.wallet.as_str());
        }
    }
}
pub mod trading {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    pub struct SubaccountOrdersChannelSpec {
        pub subaccount_id: String,
    }
    impl SubaccountOrdersChannelSpec {
        pub fn new(subaccount_id: impl Into<String>) -> Self {
            Self {
                subaccount_id: subaccount_id.into(),
            }
        }
    }
    impl ChannelSpec for SubaccountOrdersChannelSpec {
        type Output = SubaccountOrdersNotification;
        fn scope(&self) -> RequestScope {
            RequestScope::Private
        }
        #[allow(clippy::needless_return, clippy::useless_format)]
        fn channel(&self) -> String {
            return format!(
                "{subaccount_id}.orders", subaccount_id = self.subaccount_id.as_str()
            );
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    pub struct SubaccountTradesChannelSpec {
        pub subaccount_id: String,
    }
    impl SubaccountTradesChannelSpec {
        pub fn new(subaccount_id: impl Into<String>) -> Self {
            Self {
                subaccount_id: subaccount_id.into(),
            }
        }
    }
    impl ChannelSpec for SubaccountTradesChannelSpec {
        type Output = SubaccountTradesNotification;
        fn scope(&self) -> RequestScope {
            RequestScope::Private
        }
        #[allow(clippy::needless_return, clippy::useless_format)]
        fn channel(&self) -> String {
            return format!(
                "{subaccount_id}.trades", subaccount_id = self.subaccount_id.as_str()
            );
        }
    }
    #[derive(Clone, Debug, PartialEq, Eq, Default)]
    pub struct SubaccountTradesTxStatusChannelSpec {
        pub subaccount_id: String,
        pub tx_status: String,
    }
    impl SubaccountTradesTxStatusChannelSpec {
        pub fn new(
            subaccount_id: impl Into<String>,
            tx_status: impl Into<String>,
        ) -> Self {
            Self {
                subaccount_id: subaccount_id.into(),
                tx_status: tx_status.into(),
            }
        }
    }
    impl ChannelSpec for SubaccountTradesTxStatusChannelSpec {
        type Output = SubaccountTradesTxStatusNotification;
        fn scope(&self) -> RequestScope {
            RequestScope::Private
        }
        #[allow(clippy::needless_return, clippy::useless_format)]
        fn channel(&self) -> String {
            return format!(
                "{subaccount_id}.trades.{tx_status}", subaccount_id = self.subaccount_id
                .as_str(), tx_status = self.tx_status.as_str()
            );
        }
    }
}
