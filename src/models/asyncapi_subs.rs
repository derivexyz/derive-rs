// This file is generated from ws_asyncapi_rpc.json.
// Do not edit manually.

#![allow(clippy::derivable_impls)]
         /// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///20-byte Ethereum address as a 0x-prefixed lowercase hex string.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "20-byte Ethereum address as a 0x-prefixed lowercase hex string.",
///  "type": "string",
///  "maxLength": 42,
///  "minLength": 42,
///  "pattern": "^0x[0-9a-fA-F]{40}$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Address(::std::string::String);
impl ::std::ops::Deref for Address {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Address> for ::std::string::String {
    fn from(value: Address) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Address {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 42usize {
            return Err("longer than 42 characters".into());
        }
        if value.chars().count() < 42usize {
            return Err("shorter than 42 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^0x[0-9a-fA-F]{40}$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^0x[0-9a-fA-F]{40}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Address {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Address {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Address {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`AlgoType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "twap"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum AlgoType {
    #[serde(rename = "twap")]
    Twap,
}
impl ::std::fmt::Display for AlgoType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Twap => f.write_str("twap"),
        }
    }
}
impl ::std::str::FromStr for AlgoType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "twap" => Ok(Self::Twap),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AlgoType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AlgoType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AlgoType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`AuctionDetails`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "estimated_bid_price",
///    "estimated_discount_pnl",
///    "estimated_mtm",
///    "estimated_percent_bid",
///    "last_seen_trade_id",
///    "margin_type",
///    "min_cash_transfer",
///    "min_price_limit",
///    "subaccount_balances"
///  ],
///  "properties": {
///    "currency": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "estimated_bid_price": {
///      "type": "string"
///    },
///    "estimated_discount_pnl": {
///      "type": "string"
///    },
///    "estimated_mtm": {
///      "type": "string"
///    },
///    "estimated_percent_bid": {
///      "type": "string"
///    },
///    "last_seen_trade_id": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "margin_type": {
///      "type": "string"
///    },
///    "min_cash_transfer": {
///      "type": "string"
///    },
///    "min_price_limit": {
///      "type": "string"
///    },
///    "subaccount_balances": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AuctionDetails {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub currency: ::std::option::Option<::std::string::String>,
    pub estimated_bid_price: ::std::string::String,
    pub estimated_discount_pnl: ::std::string::String,
    pub estimated_mtm: ::std::string::String,
    pub estimated_percent_bid: ::std::string::String,
    pub last_seen_trade_id: i64,
    pub margin_type: ::std::string::String,
    pub min_cash_transfer: ::std::string::String,
    pub min_price_limit: ::std::string::String,
    pub subaccount_balances: ::std::string::String,
}
///Payload for `auctions.watch`.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Payload for `auctions.watch`.",
///  "type": "object",
///  "required": [
///    "state",
///    "subaccount_id",
///    "timestamp"
///  ],
///  "properties": {
///    "details": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/AuctionDetails"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "state": {
///      "$ref": "#/definitions/AuctionStateType"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AuctionResult {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub details: ::std::option::Option<AuctionDetails>,
    pub state: AuctionStateType,
    pub subaccount_id: i64,
    pub timestamp: i64,
}
///`AuctionStateType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "ongoing",
///    "ended"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum AuctionStateType {
    #[serde(rename = "ongoing")]
    Ongoing,
    #[serde(rename = "ended")]
    Ended,
}
impl ::std::fmt::Display for AuctionStateType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ongoing => f.write_str("ongoing"),
            Self::Ended => f.write_str("ended"),
        }
    }
}
impl ::std::str::FromStr for AuctionStateType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ongoing" => Ok(Self::Ongoing),
            "ended" => Ok(Self::Ended),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AuctionStateType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AuctionStateType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AuctionStateType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`AuctionsWatchNotification`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/definitions/AuctionResult"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct AuctionsWatchNotification(pub AuctionResult);
impl ::std::ops::Deref for AuctionsWatchNotification {
    type Target = AuctionResult;
    fn deref(&self) -> &AuctionResult {
        &self.0
    }
}
impl ::std::convert::From<AuctionsWatchNotification> for AuctionResult {
    fn from(value: AuctionsWatchNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<AuctionResult> for AuctionsWatchNotification {
    fn from(value: AuctionResult) -> Self {
        Self(value)
    }
}
///`BalanceUpdate`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "name",
///    "new_balance",
///    "previous_balance",
///    "update_type"
///  ],
///  "properties": {
///    "name": {
///      "type": "string"
///    },
///    "new_balance": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "previous_balance": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "update_type": {
///      "$ref": "#/definitions/BalanceUpdateType"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BalanceUpdate {
    pub name: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub new_balance: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub previous_balance: ::std::string::String,
    pub update_type: BalanceUpdateType,
}
///`BalanceUpdateType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "trade",
///    "asset_deposit",
///    "asset_withdrawal",
///    "transfer",
///    "subaccount_deposit",
///    "subaccount_withdrawal",
///    "liquidation",
///    "liquidator",
///    "onchain_drift_fix",
///    "perp_settlement",
///    "option_settlement",
///    "interest_accrual",
///    "onchain_revert",
///    "double_revert"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum BalanceUpdateType {
    #[serde(rename = "trade")]
    Trade,
    #[serde(rename = "asset_deposit")]
    AssetDeposit,
    #[serde(rename = "asset_withdrawal")]
    AssetWithdrawal,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "subaccount_deposit")]
    SubaccountDeposit,
    #[serde(rename = "subaccount_withdrawal")]
    SubaccountWithdrawal,
    #[serde(rename = "liquidation")]
    Liquidation,
    #[serde(rename = "liquidator")]
    Liquidator,
    #[serde(rename = "onchain_drift_fix")]
    OnchainDriftFix,
    #[serde(rename = "perp_settlement")]
    PerpSettlement,
    #[serde(rename = "option_settlement")]
    OptionSettlement,
    #[serde(rename = "interest_accrual")]
    InterestAccrual,
    #[serde(rename = "onchain_revert")]
    OnchainRevert,
    #[serde(rename = "double_revert")]
    DoubleRevert,
}
impl ::std::fmt::Display for BalanceUpdateType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Trade => f.write_str("trade"),
            Self::AssetDeposit => f.write_str("asset_deposit"),
            Self::AssetWithdrawal => f.write_str("asset_withdrawal"),
            Self::Transfer => f.write_str("transfer"),
            Self::SubaccountDeposit => f.write_str("subaccount_deposit"),
            Self::SubaccountWithdrawal => f.write_str("subaccount_withdrawal"),
            Self::Liquidation => f.write_str("liquidation"),
            Self::Liquidator => f.write_str("liquidator"),
            Self::OnchainDriftFix => f.write_str("onchain_drift_fix"),
            Self::PerpSettlement => f.write_str("perp_settlement"),
            Self::OptionSettlement => f.write_str("option_settlement"),
            Self::InterestAccrual => f.write_str("interest_accrual"),
            Self::OnchainRevert => f.write_str("onchain_revert"),
            Self::DoubleRevert => f.write_str("double_revert"),
        }
    }
}
impl ::std::str::FromStr for BalanceUpdateType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "trade" => Ok(Self::Trade),
            "asset_deposit" => Ok(Self::AssetDeposit),
            "asset_withdrawal" => Ok(Self::AssetWithdrawal),
            "transfer" => Ok(Self::Transfer),
            "subaccount_deposit" => Ok(Self::SubaccountDeposit),
            "subaccount_withdrawal" => Ok(Self::SubaccountWithdrawal),
            "liquidation" => Ok(Self::Liquidation),
            "liquidator" => Ok(Self::Liquidator),
            "onchain_drift_fix" => Ok(Self::OnchainDriftFix),
            "perp_settlement" => Ok(Self::PerpSettlement),
            "option_settlement" => Ok(Self::OptionSettlement),
            "interest_accrual" => Ok(Self::InterestAccrual),
            "onchain_revert" => Ok(Self::OnchainRevert),
            "double_revert" => Ok(Self::DoubleRevert),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BalanceUpdateType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BalanceUpdateType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BalanceUpdateType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Batch lifecycle status — the single source of lifecycle truth for every operation in the batch (individual ops carry no status of their own). Each stage has a healthy variant and a corresponding `...Error` variant meaning that stage failed. Serialized as the variant name (string) in API responses.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Batch lifecycle status — the single source of lifecycle truth for every operation in the batch (individual ops carry no status of their own). Each stage has a healthy variant and a corresponding `...Error` variant meaning that stage failed. Serialized as the variant name (string) in API responses.",
///  "type": "string",
///  "enum": [
///    "Batching",
///    "Executing",
///    "Proving",
///    "Settling",
///    "Settled",
///    "BatchingError",
///    "ExecutingError",
///    "ProvingError",
///    "SettlingError",
///    "SettledError"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum BatchStatus {
    Batching,
    Executing,
    Proving,
    Settling,
    Settled,
    BatchingError,
    ExecutingError,
    ProvingError,
    SettlingError,
    SettledError,
}
impl ::std::fmt::Display for BatchStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Batching => f.write_str("Batching"),
            Self::Executing => f.write_str("Executing"),
            Self::Proving => f.write_str("Proving"),
            Self::Settling => f.write_str("Settling"),
            Self::Settled => f.write_str("Settled"),
            Self::BatchingError => f.write_str("BatchingError"),
            Self::ExecutingError => f.write_str("ExecutingError"),
            Self::ProvingError => f.write_str("ProvingError"),
            Self::SettlingError => f.write_str("SettlingError"),
            Self::SettledError => f.write_str("SettledError"),
        }
    }
}
impl ::std::str::FromStr for BatchStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Batching" => Ok(Self::Batching),
            "Executing" => Ok(Self::Executing),
            "Proving" => Ok(Self::Proving),
            "Settling" => Ok(Self::Settling),
            "Settled" => Ok(Self::Settled),
            "BatchingError" => Ok(Self::BatchingError),
            "ExecutingError" => Ok(Self::ExecutingError),
            "ProvingError" => Ok(Self::ProvingError),
            "SettlingError" => Ok(Self::SettlingError),
            "SettledError" => Ok(Self::SettledError),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BatchStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BatchStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BatchStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Payload for `{subaccount_id}.best.quotes`.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Payload for `{subaccount_id}.best.quotes`.",
///  "type": "object",
///  "required": [
///    "rfq_id"
///  ],
///  "properties": {
///    "error": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/RPCError"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "result": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/RfqGetBestQuoteResponse"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "rfq_id": {
///      "description": "UUID v4 string",
///      "type": "string",
///      "format": "uuid"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BestQuoteChannelResult {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<RpcError>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub result: ::std::option::Option<RfqGetBestQuoteResponse>,
    ///UUID v4 string
    pub rfq_id: ::uuid::Uuid,
}
///`CancelReason`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "",
///    "user_request",
///    "mmp_trigger",
///    "insufficient_margin",
///    "signed_max_fee_too_low",
///    "cancel_on_disconnect",
///    "ioc_or_market_partial_fill",
///    "session_key_deregistered",
///    "subaccount_withdrawn",
///    "compliance",
///    "trigger_failed",
///    "validation_failed",
///    "algo_completed"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CancelReason {
    #[serde(rename = "")]
    X,
    #[serde(rename = "user_request")]
    UserRequest,
    #[serde(rename = "mmp_trigger")]
    MmpTrigger,
    #[serde(rename = "insufficient_margin")]
    InsufficientMargin,
    #[serde(rename = "signed_max_fee_too_low")]
    SignedMaxFeeTooLow,
    #[serde(rename = "cancel_on_disconnect")]
    CancelOnDisconnect,
    #[serde(rename = "ioc_or_market_partial_fill")]
    IocOrMarketPartialFill,
    #[serde(rename = "session_key_deregistered")]
    SessionKeyDeregistered,
    #[serde(rename = "subaccount_withdrawn")]
    SubaccountWithdrawn,
    #[serde(rename = "compliance")]
    Compliance,
    #[serde(rename = "trigger_failed")]
    TriggerFailed,
    #[serde(rename = "validation_failed")]
    ValidationFailed,
    #[serde(rename = "algo_completed")]
    AlgoCompleted,
}
impl ::std::fmt::Display for CancelReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X => f.write_str(""),
            Self::UserRequest => f.write_str("user_request"),
            Self::MmpTrigger => f.write_str("mmp_trigger"),
            Self::InsufficientMargin => f.write_str("insufficient_margin"),
            Self::SignedMaxFeeTooLow => f.write_str("signed_max_fee_too_low"),
            Self::CancelOnDisconnect => f.write_str("cancel_on_disconnect"),
            Self::IocOrMarketPartialFill => f.write_str("ioc_or_market_partial_fill"),
            Self::SessionKeyDeregistered => f.write_str("session_key_deregistered"),
            Self::SubaccountWithdrawn => f.write_str("subaccount_withdrawn"),
            Self::Compliance => f.write_str("compliance"),
            Self::TriggerFailed => f.write_str("trigger_failed"),
            Self::ValidationFailed => f.write_str("validation_failed"),
            Self::AlgoCompleted => f.write_str("algo_completed"),
        }
    }
}
impl ::std::str::FromStr for CancelReason {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "" => Ok(Self::X),
            "user_request" => Ok(Self::UserRequest),
            "mmp_trigger" => Ok(Self::MmpTrigger),
            "insufficient_margin" => Ok(Self::InsufficientMargin),
            "signed_max_fee_too_low" => Ok(Self::SignedMaxFeeTooLow),
            "cancel_on_disconnect" => Ok(Self::CancelOnDisconnect),
            "ioc_or_market_partial_fill" => Ok(Self::IocOrMarketPartialFill),
            "session_key_deregistered" => Ok(Self::SessionKeyDeregistered),
            "subaccount_withdrawn" => Ok(Self::SubaccountWithdrawn),
            "compliance" => Ok(Self::Compliance),
            "trigger_failed" => Ok(Self::TriggerFailed),
            "validation_failed" => Ok(Self::ValidationFailed),
            "algo_completed" => Ok(Self::AlgoCompleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CancelReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CancelReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CancelReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DailyTradingStatistics`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "c",
///    "h",
///    "l",
///    "n",
///    "oi",
///    "p",
///    "pr",
///    "v"
///  ],
///  "properties": {
///    "c": {
///      "description": "Number of contracts traded during last 24 hours",
///      "type": "string",
///      "format": "decimal"
///    },
///    "h": {
///      "description": "Highest trade price during last 24h",
///      "type": "string",
///      "format": "decimal"
///    },
///    "l": {
///      "description": "Lowest trade price during last 24h",
///      "type": "string",
///      "format": "decimal"
///    },
///    "n": {
///      "description": "Number of trades during last 24h",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "oi": {
///      "description": "Current total open interest",
///      "type": "string",
///      "format": "decimal"
///    },
///    "p": {
///      "description": "Options: 24hr percent change in premium; Perps: 24hr percent change in mark price",
///      "type": "string",
///      "format": "decimal"
///    },
///    "pr": {
///      "description": "Premium volume traded during last 24 hours",
///      "type": "string",
///      "format": "decimal"
///    },
///    "v": {
///      "description": "Notional volume traded during last 24 hours",
///      "type": "string",
///      "format": "decimal"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DailyTradingStatistics {
    ///Number of contracts traded during last 24 hours
    pub c: ::std::string::String,
    ///Highest trade price during last 24h
    pub h: ::std::string::String,
    ///Lowest trade price during last 24h
    pub l: ::std::string::String,
    ///Number of trades during last 24h
    pub n: u64,
    ///Current total open interest
    pub oi: ::std::string::String,
    ///Options: 24hr percent change in premium; Perps: 24hr percent change in mark price
    pub p: ::std::string::String,
    ///Premium volume traded during last 24 hours
    pub pr: ::std::string::String,
    ///Notional volume traded during last 24 hours
    pub v: ::std::string::String,
}
///`Direction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "buy",
///    "sell"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum Direction {
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "sell")]
    Sell,
}
impl ::std::fmt::Display for Direction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Buy => f.write_str("buy"),
            Self::Sell => f.write_str("sell"),
        }
    }
}
impl ::std::str::FromStr for Direction {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "buy" => Ok(Self::Buy),
            "sell" => Ok(Self::Sell),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Direction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Direction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Direction {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`JsonRpcId`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "anyOf": [
///    {
///      "type": "string"
///    },
///    {
///      "type": "number"
///    },
///    {
///      "type": "null"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum JsonRpcId {
    String(::std::string::String),
    Number(f64),
    Null,
}
impl ::std::convert::From<f64> for JsonRpcId {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
///`LegUnpricedParams`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "amount",
///    "direction",
///    "instrument_name"
///  ],
///  "properties": {
///    "amount": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "instrument_name": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LegUnpricedParams {
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub amount: ::std::string::String,
    pub direction: Direction,
    pub instrument_name: ::std::string::String,
}
///`LiquidityRole`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "maker",
///    "taker"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum LiquidityRole {
    #[serde(rename = "maker")]
    Maker,
    #[serde(rename = "taker")]
    Taker,
}
impl ::std::fmt::Display for LiquidityRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Maker => f.write_str("maker"),
            Self::Taker => f.write_str("taker"),
        }
    }
}
impl ::std::str::FromStr for LiquidityRole {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "maker" => Ok(Self::Maker),
            "taker" => Ok(Self::Taker),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LiquidityRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for LiquidityRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LiquidityRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MarginWatchNotification`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/definitions/MarginWatchResult"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct MarginWatchNotification(pub MarginWatchResult);
impl ::std::ops::Deref for MarginWatchNotification {
    type Target = MarginWatchResult;
    fn deref(&self) -> &MarginWatchResult {
        &self.0
    }
}
impl ::std::convert::From<MarginWatchNotification> for MarginWatchResult {
    fn from(value: MarginWatchNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<MarginWatchResult> for MarginWatchNotification {
    fn from(value: MarginWatchResult) -> Self {
        Self(value)
    }
}
///Payload for `margin.watch`.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Payload for `margin.watch`.",
///  "type": "object",
///  "required": [
///    "collaterals",
///    "currency",
///    "initial_margin",
///    "maintenance_margin",
///    "margin_type",
///    "positions",
///    "subaccount_id",
///    "subaccount_value",
///    "valuation_timestamp"
///  ],
///  "properties": {
///    "collaterals": {
///      "type": "array",
///      "items": true
///    },
///    "currency": {
///      "type": "string"
///    },
///    "initial_margin": {
///      "type": "string"
///    },
///    "maintenance_margin": {
///      "type": "string"
///    },
///    "margin_type": {
///      "type": "string"
///    },
///    "positions": {
///      "type": "array",
///      "items": true
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "subaccount_value": {
///      "type": "string"
///    },
///    "valuation_timestamp": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct MarginWatchResult {
    pub collaterals: ::std::vec::Vec<::serde_json::Value>,
    pub currency: ::std::string::String,
    pub initial_margin: ::std::string::String,
    pub maintenance_margin: ::std::string::String,
    pub margin_type: ::std::string::String,
    pub positions: ::std::vec::Vec<::serde_json::Value>,
    pub subaccount_id: i64,
    pub subaccount_value: ::std::string::String,
    pub valuation_timestamp: i64,
}
///`OptionPricing`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "ai",
///    "bi",
///    "d",
///    "df",
///    "f",
///    "g",
///    "i",
///    "m",
///    "r",
///    "t",
///    "v"
///  ],
///  "properties": {
///    "ai": {
///      "description": "Implied volatility of the current best ask.",
///      "type": "string"
///    },
///    "bi": {
///      "description": "Implied volatility of the current best bid.",
///      "type": "string"
///    },
///    "d": {
///      "description": "Delta of the option.",
///      "type": "string"
///    },
///    "df": {
///      "description": "Discount factor used to calculate the option premium.",
///      "type": "string"
///    },
///    "f": {
///      "description": "Forward price used to calculate the option premium.",
///      "type": "string"
///    },
///    "g": {
///      "description": "Gamma of the option.",
///      "type": "string"
///    },
///    "i": {
///      "description": "Implied volatility of the option.",
///      "type": "string"
///    },
///    "m": {
///      "description": "Mark price of the option.",
///      "type": "string"
///    },
///    "r": {
///      "description": "Rho of the option.",
///      "type": "string"
///    },
///    "t": {
///      "description": "Theta of the option.",
///      "type": "string"
///    },
///    "v": {
///      "description": "Vega of the option.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OptionPricing {
    ///Implied volatility of the current best ask.
    pub ai: ::std::string::String,
    ///Implied volatility of the current best bid.
    pub bi: ::std::string::String,
    ///Delta of the option.
    pub d: ::std::string::String,
    ///Discount factor used to calculate the option premium.
    pub df: ::std::string::String,
    ///Forward price used to calculate the option premium.
    pub f: ::std::string::String,
    ///Gamma of the option.
    pub g: ::std::string::String,
    ///Implied volatility of the option.
    pub i: ::std::string::String,
    ///Mark price of the option.
    pub m: ::std::string::String,
    ///Rho of the option.
    pub r: ::std::string::String,
    ///Theta of the option.
    pub t: ::std::string::String,
    ///Vega of the option.
    pub v: ::std::string::String,
}
///`Order`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "amount",
///    "average_price",
///    "creation_timestamp",
///    "direction",
///    "extra_fee",
///    "filled_amount",
///    "instrument_name",
///    "is_transfer",
///    "last_update_timestamp",
///    "limit_price",
///    "max_fee",
///    "mmp",
///    "nonce",
///    "order_fee",
///    "order_id",
///    "order_status",
///    "order_type",
///    "quote_id",
///    "replaced_order_id",
///    "signature",
///    "signature_expiry_sec",
///    "signed_limit_price",
///    "signer",
///    "subaccount_id",
///    "time_in_force",
///    "trigger_price"
///  ],
///  "properties": {
///    "algo_duration_sec": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int32"
///    },
///    "algo_num_slices": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int32"
///    },
///    "algo_slices_completed": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int32"
///    },
///    "algo_type": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/AlgoType"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "amount": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "average_price": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "cancel_reason": {
///      "default": "",
///      "$ref": "#/definitions/CancelReason"
///    },
///    "creation_timestamp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "extra_fee": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "filled_amount": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "instrument_name": {
///      "type": "string"
///    },
///    "is_transfer": {
///      "type": "boolean"
///    },
///    "label": {
///      "default": "",
///      "type": "string"
///    },
///    "last_update_timestamp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "limit_price": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "max_fee": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "mmp": {
///      "type": "boolean"
///    },
///    "nonce": {
///      "type": "string"
///    },
///    "order_fee": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "order_id": {
///      "type": "string"
///    },
///    "order_status": {
///      "$ref": "#/definitions/OrderStatus"
///    },
///    "order_type": {
///      "$ref": "#/definitions/OrderType"
///    },
///    "quote_id": {
///      "description": "Optional UUID v4 string",
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "uuid"
///    },
///    "replaced_order_id": {
///      "description": "Optional UUID v4 string",
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "uuid"
///    },
///    "signature": {
///      "type": "string"
///    },
///    "signature_expiry_sec": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "signed_limit_price": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "decimal"
///    },
///    "signer": {
///      "type": "string"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "time_in_force": {
///      "$ref": "#/definitions/TimeInForce"
///    },
///    "trigger_price": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "decimal"
///    },
///    "trigger_price_type": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/TriggerPriceType"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "trigger_reject_message": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "trigger_type": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/TriggerType"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Order {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub algo_duration_sec: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub algo_num_slices: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub algo_slices_completed: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub algo_type: ::std::option::Option<AlgoType>,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub amount: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub average_price: ::std::string::String,
    #[serde(default = "defaults::order_cancel_reason")]
    pub cancel_reason: CancelReason,
    pub creation_timestamp: i64,
    pub direction: Direction,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub extra_fee: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub filled_amount: ::std::string::String,
    pub instrument_name: ::std::string::String,
    pub is_transfer: bool,
    #[serde(default)]
    pub label: ::std::string::String,
    pub last_update_timestamp: i64,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub limit_price: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub max_fee: ::std::string::String,
    pub mmp: bool,
    pub nonce: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub order_fee: ::std::string::String,
    pub order_id: ::std::string::String,
    pub order_status: OrderStatus,
    pub order_type: OrderType,
    ///Optional UUID v4 string
    pub quote_id: ::std::option::Option<::uuid::Uuid>,
    ///Optional UUID v4 string
    pub replaced_order_id: ::std::option::Option<::uuid::Uuid>,
    pub signature: ::std::string::String,
    pub signature_expiry_sec: i64,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub signed_limit_price: ::std::option::Option<::std::string::String>,
    pub signer: ::std::string::String,
    pub subaccount_id: i64,
    pub time_in_force: TimeInForce,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub trigger_price: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trigger_price_type: ::std::option::Option<TriggerPriceType>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trigger_reject_message: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trigger_type: ::std::option::Option<TriggerType>,
}
///`OrderSnapshot`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "amount",
///    "price"
///  ],
///  "properties": {
///    "amount": {
///      "type": "string"
///    },
///    "price": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OrderSnapshot {
    pub amount: ::std::string::String,
    pub price: ::std::string::String,
}
///`OrderStatus`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "open",
///    "filled",
///    "rejected",
///    "cancelled",
///    "expired",
///    "untriggered",
///    "algo_active"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum OrderStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "filled")]
    Filled,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "untriggered")]
    Untriggered,
    #[serde(rename = "algo_active")]
    AlgoActive,
}
impl ::std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Open => f.write_str("open"),
            Self::Filled => f.write_str("filled"),
            Self::Rejected => f.write_str("rejected"),
            Self::Cancelled => f.write_str("cancelled"),
            Self::Expired => f.write_str("expired"),
            Self::Untriggered => f.write_str("untriggered"),
            Self::AlgoActive => f.write_str("algo_active"),
        }
    }
}
impl ::std::str::FromStr for OrderStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "open" => Ok(Self::Open),
            "filled" => Ok(Self::Filled),
            "rejected" => Ok(Self::Rejected),
            "cancelled" => Ok(Self::Cancelled),
            "expired" => Ok(Self::Expired),
            "untriggered" => Ok(Self::Untriggered),
            "algo_active" => Ok(Self::AlgoActive),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrderStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for OrderStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OrderStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`OrderType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "limit",
///    "market"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum OrderType {
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "market")]
    Market,
}
impl ::std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Limit => f.write_str("limit"),
            Self::Market => f.write_str("market"),
        }
    }
}
impl ::std::str::FromStr for OrderType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "limit" => Ok(Self::Limit),
            "market" => Ok(Self::Market),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OrderType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for OrderType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OrderType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`OrderbookNotification`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/definitions/OrderbookSnapshot"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct OrderbookNotification(pub OrderbookSnapshot);
impl ::std::ops::Deref for OrderbookNotification {
    type Target = OrderbookSnapshot;
    fn deref(&self) -> &OrderbookSnapshot {
        &self.0
    }
}
impl ::std::convert::From<OrderbookNotification> for OrderbookSnapshot {
    fn from(value: OrderbookNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<OrderbookSnapshot> for OrderbookNotification {
    fn from(value: OrderbookSnapshot) -> Self {
        Self(value)
    }
}
///`OrderbookSnapshot`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "asks",
///    "bids",
///    "instrument_name",
///    "publish_id",
///    "timestamp"
///  ],
///  "properties": {
///    "asks": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/OrderSnapshot"
///      }
///    },
///    "bids": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/OrderSnapshot"
///      }
///    },
///    "instrument_name": {
///      "type": "string"
///    },
///    "publish_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OrderbookSnapshot {
    pub asks: ::std::vec::Vec<OrderSnapshot>,
    pub bids: ::std::vec::Vec<OrderSnapshot>,
    pub instrument_name: ::std::string::String,
    pub publish_id: u64,
    pub timestamp: i64,
}
///`PricedLegParamsAndResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "amount",
///    "direction",
///    "instrument_name",
///    "price"
///  ],
///  "properties": {
///    "amount": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "instrument_name": {
///      "type": "string"
///    },
///    "price": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PricedLegParamsAndResponse {
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub amount: ::std::string::String,
    pub direction: Direction,
    pub instrument_name: ::std::string::String,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub price: ::std::string::String,
}
///`PublicQuote`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "cancel_reason",
///    "creation_timestamp",
///    "direction",
///    "fill_pct",
///    "last_update_timestamp",
///    "legs",
///    "legs_hash",
///    "liquidity_role",
///    "quote_id",
///    "rfq_id",
///    "status",
///    "subaccount_id",
///    "wallet"
///  ],
///  "properties": {
///    "cancel_reason": {
///      "$ref": "#/definitions/RFQCancelReason"
///    },
///    "creation_timestamp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "fill_pct": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "last_update_timestamp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "legs": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PricedLegParamsAndResponse"
///      }
///    },
///    "legs_hash": {
///      "type": "string"
///    },
///    "liquidity_role": {
///      "$ref": "#/definitions/LiquidityRole"
///    },
///    "quote_id": {
///      "description": "UUID v4 string",
///      "type": "string",
///      "format": "uuid"
///    },
///    "rfq_id": {
///      "description": "UUID v4 string",
///      "type": "string",
///      "format": "uuid"
///    },
///    "status": {
///      "$ref": "#/definitions/RFQStatus"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "wallet": {
///      "$ref": "#/definitions/Address"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PublicQuote {
    pub cancel_reason: RfqCancelReason,
    pub creation_timestamp: i64,
    pub direction: Direction,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub fill_pct: ::std::string::String,
    pub last_update_timestamp: i64,
    pub legs: ::std::vec::Vec<PricedLegParamsAndResponse>,
    pub legs_hash: ::std::string::String,
    pub liquidity_role: LiquidityRole,
    ///UUID v4 string
    pub quote_id: ::uuid::Uuid,
    ///UUID v4 string
    pub rfq_id: ::uuid::Uuid,
    pub status: RfqStatus,
    pub subaccount_id: i64,
    pub wallet: Address,
}
///`PublicRfq`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "cancel_reason",
///    "creation_timestamp",
///    "fill_rate",
///    "filled_pct",
///    "last_update_timestamp",
///    "legs",
///    "partial_fill_step",
///    "recent_fill_rate",
///    "rfq_id",
///    "status",
///    "subaccount_id",
///    "total_cost",
///    "valid_until",
///    "wallet"
///  ],
///  "properties": {
///    "cancel_reason": {
///      "$ref": "#/definitions/RFQCancelReason"
///    },
///    "creation_timestamp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "fill_rate": {
///      "description": "Lifetime taker fill rate as a decimal string; null until the wallet has enough lifetime RFQs for the rate to be meaningful.",
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "decimal"
///    },
///    "filled_direction": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/Direction"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "filled_pct": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "last_update_timestamp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "legs": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/LegUnpricedParams"
///      }
///    },
///    "partial_fill_step": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "recent_fill_rate": {
///      "description": "Decayed-recent taker fill rate as a decimal string; null until enough recent RFQ activity has accumulated for the rate to be meaningful.",
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "decimal"
///    },
///    "rfq_id": {
///      "description": "UUID v4 string",
///      "type": "string",
///      "format": "uuid"
///    },
///    "status": {
///      "$ref": "#/definitions/RFQStatus"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "total_cost": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "decimal"
///    },
///    "valid_until": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "wallet": {
///      "$ref": "#/definitions/Address"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PublicRfq {
    pub cancel_reason: RfqCancelReason,
    pub creation_timestamp: i64,
    ///Lifetime taker fill rate as a decimal string; null until the wallet has enough lifetime RFQs for the rate to be meaningful.
    pub fill_rate: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub filled_direction: ::std::option::Option<Direction>,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub filled_pct: ::std::string::String,
    pub last_update_timestamp: i64,
    pub legs: ::std::vec::Vec<LegUnpricedParams>,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub partial_fill_step: ::std::string::String,
    ///Decayed-recent taker fill rate as a decimal string; null until enough recent RFQ activity has accumulated for the rate to be meaningful.
    pub recent_fill_rate: ::std::option::Option<::std::string::String>,
    ///UUID v4 string
    pub rfq_id: ::uuid::Uuid,
    pub status: RfqStatus,
    pub subaccount_id: i64,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub total_cost: ::std::option::Option<::std::string::String>,
    pub valid_until: i64,
    pub wallet: Address,
}
///`PublicTrade`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "direction",
///    "index_price",
///    "instrument_name",
///    "mark_price",
///    "timestamp",
///    "trade_amount",
///    "trade_id",
///    "trade_price"
///  ],
///  "properties": {
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "index_price": {
///      "type": "string"
///    },
///    "instrument_name": {
///      "type": "string"
///    },
///    "mark_price": {
///      "type": "string"
///    },
///    "quote_id": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "rfq_id": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "trade_amount": {
///      "type": "string"
///    },
///    "trade_id": {
///      "type": "string"
///    },
///    "trade_price": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PublicTrade {
    pub direction: Direction,
    pub index_price: ::std::string::String,
    pub instrument_name: ::std::string::String,
    pub mark_price: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quote_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rfq_id: ::std::option::Option<::std::string::String>,
    pub timestamp: i64,
    pub trade_amount: ::std::string::String,
    pub trade_id: ::std::string::String,
    pub trade_price: ::std::string::String,
}
///`QuotePublishResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "creation_timestamp",
///    "direction",
///    "extra_fee",
///    "fee",
///    "fill_pct",
///    "is_transfer",
///    "label",
///    "last_update_timestamp",
///    "legs",
///    "legs_hash",
///    "liquidity_role",
///    "max_fee",
///    "mmp",
///    "nonce",
///    "quote_id",
///    "rfq_id",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "status",
///    "subaccount_id"
///  ],
///  "properties": {
///    "batch_status": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/BatchStatus"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "cancel_reason": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/RFQCancelReason"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "creation_timestamp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "extra_fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "fill_pct": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "is_transfer": {
///      "type": "boolean"
///    },
///    "label": {
///      "type": "string"
///    },
///    "last_update_timestamp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "legs": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PricedLegParamsAndResponse"
///      }
///    },
///    "legs_hash": {
///      "type": "string"
///    },
///    "liquidity_role": {
///      "$ref": "#/definitions/LiquidityRole"
///    },
///    "max_fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "mmp": {
///      "type": "boolean"
///    },
///    "nonce": {
///      "type": "string"
///    },
///    "quote_id": {
///      "description": "UUID v4 string",
///      "type": "string",
///      "format": "uuid"
///    },
///    "rfq_id": {
///      "description": "UUID v4 string",
///      "type": "string",
///      "format": "uuid"
///    },
///    "signature": {
///      "type": "string"
///    },
///    "signature_expiry_sec": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "signer": {
///      "type": "string"
///    },
///    "status": {
///      "$ref": "#/definitions/RFQStatus"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "tx_hash": {
///      "type": [
///        "string",
///        "null"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct QuotePublishResult {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub batch_status: ::std::option::Option<BatchStatus>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cancel_reason: ::std::option::Option<RfqCancelReason>,
    pub creation_timestamp: i64,
    pub direction: Direction,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub extra_fee: ::std::string::String,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub fee: ::std::string::String,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub fill_pct: ::std::string::String,
    pub is_transfer: bool,
    pub label: ::std::string::String,
    pub last_update_timestamp: i64,
    pub legs: ::std::vec::Vec<PricedLegParamsAndResponse>,
    pub legs_hash: ::std::string::String,
    pub liquidity_role: LiquidityRole,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub max_fee: ::std::string::String,
    pub mmp: bool,
    pub nonce: ::std::string::String,
    ///UUID v4 string
    pub quote_id: ::uuid::Uuid,
    ///UUID v4 string
    pub rfq_id: ::uuid::Uuid,
    pub signature: ::std::string::String,
    pub signature_expiry_sec: i64,
    pub signer: ::std::string::String,
    pub status: RfqStatus,
    pub subaccount_id: i64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tx_hash: ::std::option::Option<::std::string::String>,
}
///`RfqCancelReason`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "",
///    "user_request",
///    "insufficient_margin",
///    "signed_max_fee_too_low",
///    "mmp_trigger",
///    "cancel_on_disconnect",
///    "session_key_deregistered",
///    "subaccount_withdrawn",
///    "rfq_no_longer_open",
///    "compliance"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum RfqCancelReason {
    #[serde(rename = "")]
    X,
    #[serde(rename = "user_request")]
    UserRequest,
    #[serde(rename = "insufficient_margin")]
    InsufficientMargin,
    #[serde(rename = "signed_max_fee_too_low")]
    SignedMaxFeeTooLow,
    #[serde(rename = "mmp_trigger")]
    MmpTrigger,
    #[serde(rename = "cancel_on_disconnect")]
    CancelOnDisconnect,
    #[serde(rename = "session_key_deregistered")]
    SessionKeyDeregistered,
    #[serde(rename = "subaccount_withdrawn")]
    SubaccountWithdrawn,
    #[serde(rename = "rfq_no_longer_open")]
    RfqNoLongerOpen,
    #[serde(rename = "compliance")]
    Compliance,
}
impl ::std::fmt::Display for RfqCancelReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X => f.write_str(""),
            Self::UserRequest => f.write_str("user_request"),
            Self::InsufficientMargin => f.write_str("insufficient_margin"),
            Self::SignedMaxFeeTooLow => f.write_str("signed_max_fee_too_low"),
            Self::MmpTrigger => f.write_str("mmp_trigger"),
            Self::CancelOnDisconnect => f.write_str("cancel_on_disconnect"),
            Self::SessionKeyDeregistered => f.write_str("session_key_deregistered"),
            Self::SubaccountWithdrawn => f.write_str("subaccount_withdrawn"),
            Self::RfqNoLongerOpen => f.write_str("rfq_no_longer_open"),
            Self::Compliance => f.write_str("compliance"),
        }
    }
}
impl ::std::str::FromStr for RfqCancelReason {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "" => Ok(Self::X),
            "user_request" => Ok(Self::UserRequest),
            "insufficient_margin" => Ok(Self::InsufficientMargin),
            "signed_max_fee_too_low" => Ok(Self::SignedMaxFeeTooLow),
            "mmp_trigger" => Ok(Self::MmpTrigger),
            "cancel_on_disconnect" => Ok(Self::CancelOnDisconnect),
            "session_key_deregistered" => Ok(Self::SessionKeyDeregistered),
            "subaccount_withdrawn" => Ok(Self::SubaccountWithdrawn),
            "rfq_no_longer_open" => Ok(Self::RfqNoLongerOpen),
            "compliance" => Ok(Self::Compliance),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RfqCancelReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RfqCancelReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RfqCancelReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RfqGetBestQuoteResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "direction",
///    "down_liquidation_price",
///    "estimated_fee",
///    "estimated_realized_pnl",
///    "estimated_realized_pnl_excl_fees",
///    "estimated_total_cost",
///    "filled_pct",
///    "is_valid",
///    "orderbook_total_cost",
///    "post_initial_margin",
///    "post_liquidation_price",
///    "pre_initial_margin",
///    "suggested_max_fee",
///    "up_liquidation_price"
///  ],
///  "properties": {
///    "best_quote": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/PublicQuote"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "down_liquidation_price": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "decimal"
///    },
///    "estimated_fee": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "estimated_realized_pnl": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "estimated_realized_pnl_excl_fees": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "estimated_total_cost": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "filled_pct": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "invalid_reason": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "is_valid": {
///      "type": "boolean"
///    },
///    "orderbook_total_cost": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "decimal"
///    },
///    "post_initial_margin": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "post_liquidation_price": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "decimal"
///    },
///    "pre_initial_margin": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "suggested_max_fee": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "up_liquidation_price": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "decimal"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RfqGetBestQuoteResponse {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub best_quote: ::std::option::Option<PublicQuote>,
    pub direction: Direction,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub down_liquidation_price: ::std::option::Option<::std::string::String>,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub estimated_fee: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub estimated_realized_pnl: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub estimated_realized_pnl_excl_fees: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub estimated_total_cost: ::std::string::String,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub filled_pct: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub invalid_reason: ::std::option::Option<::std::string::String>,
    pub is_valid: bool,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub orderbook_total_cost: ::std::option::Option<::std::string::String>,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub post_initial_margin: ::std::string::String,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub post_liquidation_price: ::std::option::Option<::std::string::String>,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub pre_initial_margin: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub suggested_max_fee: ::std::string::String,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub up_liquidation_price: ::std::option::Option<::std::string::String>,
}
///`RfqStatus`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "open",
///    "filled",
///    "cancelled",
///    "expired"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum RfqStatus {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "filled")]
    Filled,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "expired")]
    Expired,
}
impl ::std::fmt::Display for RfqStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Open => f.write_str("open"),
            Self::Filled => f.write_str("filled"),
            Self::Cancelled => f.write_str("cancelled"),
            Self::Expired => f.write_str("expired"),
        }
    }
}
impl ::std::str::FromStr for RfqStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "open" => Ok(Self::Open),
            "filled" => Ok(Self::Filled),
            "cancelled" => Ok(Self::Cancelled),
            "expired" => Ok(Self::Expired),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RfqStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RfqStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RfqStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`RpcError`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "code",
///    "message"
///  ],
///  "properties": {
///    "code": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "data": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "message": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RpcError {
    pub code: i64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub data: ::std::option::Option<::std::string::String>,
    pub message: ::std::string::String,
}
///A settled trade returned by `public/get_trade_history`. Anonymized: no `order_id` or `label`. `wallet` is included since settled trades are visible on-chain.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A settled trade returned by `public/get_trade_history`. Anonymized: no `order_id` or `label`. `wallet` is included since settled trades are visible on-chain.",
///  "type": "object",
///  "required": [
///    "direction",
///    "expected_rebate",
///    "extra_fee",
///    "index_price",
///    "instrument_name",
///    "liquidity_role",
///    "mark_price",
///    "quote_id",
///    "realized_pnl",
///    "realized_pnl_excl_fees",
///    "rfq_id",
///    "subaccount_id",
///    "timestamp",
///    "trade_amount",
///    "trade_fee",
///    "trade_id",
///    "trade_price",
///    "tx_hash",
///    "wallet"
///  ],
///  "properties": {
///    "batch_status": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/BatchStatus"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "expected_rebate": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "extra_fee": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "index_price": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "instrument_name": {
///      "type": "string"
///    },
///    "liquidity_role": {
///      "$ref": "#/definitions/LiquidityRole"
///    },
///    "mark_price": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "quote_id": {
///      "description": "Optional UUID v4 string",
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "uuid"
///    },
///    "realized_pnl": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "realized_pnl_excl_fees": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "rfq_id": {
///      "description": "Optional UUID v4 string",
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "uuid"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "trade_amount": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "trade_fee": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "trade_id": {
///      "type": "string"
///    },
///    "trade_price": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "tx_hash": {
///      "type": "string"
///    },
///    "wallet": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SettledTrade {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub batch_status: ::std::option::Option<BatchStatus>,
    pub direction: Direction,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub expected_rebate: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub extra_fee: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub index_price: ::std::string::String,
    pub instrument_name: ::std::string::String,
    pub liquidity_role: LiquidityRole,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub mark_price: ::std::string::String,
    ///Optional UUID v4 string
    pub quote_id: ::std::option::Option<::uuid::Uuid>,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub realized_pnl: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub realized_pnl_excl_fees: ::std::string::String,
    ///Optional UUID v4 string
    pub rfq_id: ::std::option::Option<::uuid::Uuid>,
    pub subaccount_id: i64,
    pub timestamp: i64,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub trade_amount: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub trade_fee: ::std::string::String,
    pub trade_id: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub trade_price: ::std::string::String,
    pub tx_hash: ::std::string::String,
    pub wallet: ::std::string::String,
}
///`SpotFeedEntry`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "confidence",
///    "confidence_prev_daily",
///    "price",
///    "price_prev_daily",
///    "timestamp_prev_daily"
///  ],
///  "properties": {
///    "confidence": {
///      "type": "string"
///    },
///    "confidence_prev_daily": {
///      "type": "string"
///    },
///    "price": {
///      "type": "string"
///    },
///    "price_prev_daily": {
///      "type": "string"
///    },
///    "timestamp_prev_daily": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SpotFeedEntry {
    pub confidence: ::std::string::String,
    pub confidence_prev_daily: ::std::string::String,
    pub price: ::std::string::String,
    pub price_prev_daily: ::std::string::String,
    pub timestamp_prev_daily: i64,
}
///`SpotFeedNotification`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/definitions/SpotFeedPayload"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SpotFeedNotification(pub SpotFeedPayload);
impl ::std::ops::Deref for SpotFeedNotification {
    type Target = SpotFeedPayload;
    fn deref(&self) -> &SpotFeedPayload {
        &self.0
    }
}
impl ::std::convert::From<SpotFeedNotification> for SpotFeedPayload {
    fn from(value: SpotFeedNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<SpotFeedPayload> for SpotFeedNotification {
    fn from(value: SpotFeedPayload) -> Self {
        Self(value)
    }
}
///Payload for `spot_feed.{currency}` (and `spot_feed.ALL`).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Payload for `spot_feed.{currency}` (and `spot_feed.ALL`).",
///  "type": "object",
///  "required": [
///    "feeds",
///    "timestamp"
///  ],
///  "properties": {
///    "feeds": {
///      "type": "object",
///      "properties": {
///        "{key}": {
///          "$ref": "#/definitions/SpotFeedEntry"
///        }
///      },
///      "additionalProperties": {
///        "$ref": "#/definitions/SpotFeedEntry"
///      }
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SpotFeedPayload {
    pub feeds: SpotFeedPayloadFeeds,
    pub timestamp: i64,
}
///`SpotFeedPayloadFeeds`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "{key}": {
///      "$ref": "#/definitions/SpotFeedEntry"
///    }
///  },
///  "additionalProperties": {
///    "$ref": "#/definitions/SpotFeedEntry"
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SpotFeedPayloadFeeds {
    #[serde(
        rename = "{key}",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub key: ::std::option::Option<SpotFeedEntry>,
    #[serde(flatten)]
    pub extra: ::std::collections::HashMap<::std::string::String, SpotFeedEntry>,
}
///`SubaccountBalancesNotification`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/definitions/BalanceUpdate"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SubaccountBalancesNotification(pub BalanceUpdate);
impl ::std::ops::Deref for SubaccountBalancesNotification {
    type Target = BalanceUpdate;
    fn deref(&self) -> &BalanceUpdate {
        &self.0
    }
}
impl ::std::convert::From<SubaccountBalancesNotification> for BalanceUpdate {
    fn from(value: SubaccountBalancesNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<BalanceUpdate> for SubaccountBalancesNotification {
    fn from(value: BalanceUpdate) -> Self {
        Self(value)
    }
}
///`SubaccountBestQuotesNotification`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/definitions/BestQuoteChannelResult"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SubaccountBestQuotesNotification(pub BestQuoteChannelResult);
impl ::std::ops::Deref for SubaccountBestQuotesNotification {
    type Target = BestQuoteChannelResult;
    fn deref(&self) -> &BestQuoteChannelResult {
        &self.0
    }
}
impl ::std::convert::From<SubaccountBestQuotesNotification> for BestQuoteChannelResult {
    fn from(value: SubaccountBestQuotesNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<BestQuoteChannelResult> for SubaccountBestQuotesNotification {
    fn from(value: BestQuoteChannelResult) -> Self {
        Self(value)
    }
}
///`SubaccountOrdersNotification`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/definitions/Order"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SubaccountOrdersNotification(pub Order);
impl ::std::ops::Deref for SubaccountOrdersNotification {
    type Target = Order;
    fn deref(&self) -> &Order {
        &self.0
    }
}
impl ::std::convert::From<SubaccountOrdersNotification> for Order {
    fn from(value: SubaccountOrdersNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<Order> for SubaccountOrdersNotification {
    fn from(value: Order) -> Self {
        Self(value)
    }
}
///`SubaccountQuotesNotification`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/definitions/QuotePublishResult"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SubaccountQuotesNotification(pub QuotePublishResult);
impl ::std::ops::Deref for SubaccountQuotesNotification {
    type Target = QuotePublishResult;
    fn deref(&self) -> &QuotePublishResult {
        &self.0
    }
}
impl ::std::convert::From<SubaccountQuotesNotification> for QuotePublishResult {
    fn from(value: SubaccountQuotesNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<QuotePublishResult> for SubaccountQuotesNotification {
    fn from(value: QuotePublishResult) -> Self {
        Self(value)
    }
}
///`SubaccountTradesBatchStatusNotification`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/definitions/Trade"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SubaccountTradesBatchStatusNotification(pub Trade);
impl ::std::ops::Deref for SubaccountTradesBatchStatusNotification {
    type Target = Trade;
    fn deref(&self) -> &Trade {
        &self.0
    }
}
impl ::std::convert::From<SubaccountTradesBatchStatusNotification> for Trade {
    fn from(value: SubaccountTradesBatchStatusNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<Trade> for SubaccountTradesBatchStatusNotification {
    fn from(value: Trade) -> Self {
        Self(value)
    }
}
///`SubaccountTradesNotification`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/definitions/Trade"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SubaccountTradesNotification(pub Trade);
impl ::std::ops::Deref for SubaccountTradesNotification {
    type Target = Trade;
    fn deref(&self) -> &Trade {
        &self.0
    }
}
impl ::std::convert::From<SubaccountTradesNotification> for Trade {
    fn from(value: SubaccountTradesNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<Trade> for SubaccountTradesNotification {
    fn from(value: Trade) -> Self {
        Self(value)
    }
}
///Params for `subscribe`. `channels` is the required list of channel names to subscribe to.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Params for `subscribe`. `channels` is the required list of channel names to subscribe to.",
///  "type": "object",
///  "required": [
///    "channels"
///  ],
///  "properties": {
///    "channels": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SubscribeParams {
    pub channels: ::std::vec::Vec<::std::string::String>,
}
///`SubscribeRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "id",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "headers": {
///      "description": "Non-standard; used by `auth/login`.",
///      "type": [
///        "object",
///        "null"
///      ],
///      "additionalProperties": true
///    },
///    "id": {
///      "$ref": "#/definitions/JsonRpcId"
///    },
///    "method": {
///      "type": "string",
///      "const": "subscribe"
///    },
///    "params": {
///      "$ref": "#/definitions/SubscribeParams"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SubscribeRequest {
    ///Non-standard; used by `auth/login`.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub headers: ::std::option::Option<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    pub id: JsonRpcId,
    pub method: ::std::string::String,
    pub params: SubscribeParams,
}
///`TickerSlimNotification`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/definitions/TickerSlimPayload"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct TickerSlimNotification(pub TickerSlimPayload);
impl ::std::ops::Deref for TickerSlimNotification {
    type Target = TickerSlimPayload;
    fn deref(&self) -> &TickerSlimPayload {
        &self.0
    }
}
impl ::std::convert::From<TickerSlimNotification> for TickerSlimPayload {
    fn from(value: TickerSlimNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<TickerSlimPayload> for TickerSlimNotification {
    fn from(value: TickerSlimPayload) -> Self {
        Self(value)
    }
}
///Payload for `ticker_slim.{instrument}.{interval}`.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Payload for `ticker_slim.{instrument}.{interval}`.",
///  "type": "object",
///  "required": [
///    "instrument_ticker",
///    "timestamp"
///  ],
///  "properties": {
///    "instrument_ticker": {
///      "$ref": "#/definitions/TickerSlimSnapshot"
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TickerSlimPayload {
    pub instrument_ticker: crate::models::ticker_slim_schema::TickerSlimSchema,
    pub timestamp: i64,
}
///`TimeInForce`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "gtc",
///    "post_only",
///    "fok",
///    "ioc"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum TimeInForce {
    #[serde(rename = "gtc")]
    Gtc,
    #[serde(rename = "post_only")]
    PostOnly,
    #[serde(rename = "fok")]
    Fok,
    #[serde(rename = "ioc")]
    Ioc,
}
impl ::std::fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Gtc => f.write_str("gtc"),
            Self::PostOnly => f.write_str("post_only"),
            Self::Fok => f.write_str("fok"),
            Self::Ioc => f.write_str("ioc"),
        }
    }
}
impl ::std::str::FromStr for TimeInForce {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "gtc" => Ok(Self::Gtc),
            "post_only" => Ok(Self::PostOnly),
            "fok" => Ok(Self::Fok),
            "ioc" => Ok(Self::Ioc),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TimeInForce {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TimeInForce {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TimeInForce {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Info that the user expects to get delivered when requesting trade info.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Info that the user expects to get delivered when requesting trade info.",
///  "type": "object",
///  "required": [
///    "direction",
///    "expected_rebate",
///    "extra_fee",
///    "index_price",
///    "instrument_name",
///    "is_transfer",
///    "liquidity_role",
///    "mark_price",
///    "op_uuid",
///    "order_id",
///    "quote_id",
///    "realized_pnl",
///    "realized_pnl_excl_fees",
///    "rfq_id",
///    "subaccount_id",
///    "timestamp",
///    "trade_amount",
///    "trade_fee",
///    "trade_id",
///    "trade_price"
///  ],
///  "properties": {
///    "batch_status": {
///      "description": "Settlement batch status; `null` if processed by sequencer.",
///      "anyOf": [
///        {
///          "$ref": "#/definitions/BatchStatus"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "expected_rebate": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "extra_fee": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "index_price": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "instrument_name": {
///      "type": "string"
///    },
///    "is_transfer": {
///      "type": "boolean"
///    },
///    "label": {
///      "default": "",
///      "type": "string"
///    },
///    "liquidity_role": {
///      "$ref": "#/definitions/LiquidityRole"
///    },
///    "mark_price": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "op_uuid": {
///      "type": "string"
///    },
///    "order_id": {
///      "type": "string"
///    },
///    "quote_id": {
///      "description": "Optional UUID v4 string",
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "uuid"
///    },
///    "realized_pnl": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "realized_pnl_excl_fees": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "rfq_id": {
///      "description": "Optional UUID v4 string",
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "uuid"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "trade_amount": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "trade_fee": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "trade_id": {
///      "type": "string"
///    },
///    "trade_price": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal"
///    },
///    "tx_hash": {
///      "type": [
///        "string",
///        "null"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Trade {
    ///Settlement batch status; `null` if processed by sequencer.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub batch_status: ::std::option::Option<BatchStatus>,
    pub direction: Direction,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub expected_rebate: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub extra_fee: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub index_price: ::std::string::String,
    pub instrument_name: ::std::string::String,
    pub is_transfer: bool,
    #[serde(default)]
    pub label: ::std::string::String,
    pub liquidity_role: LiquidityRole,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub mark_price: ::std::string::String,
    pub op_uuid: ::std::string::String,
    pub order_id: ::std::string::String,
    ///Optional UUID v4 string
    pub quote_id: ::std::option::Option<::uuid::Uuid>,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub realized_pnl: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub realized_pnl_excl_fees: ::std::string::String,
    ///Optional UUID v4 string
    pub rfq_id: ::std::option::Option<::uuid::Uuid>,
    pub subaccount_id: i64,
    pub timestamp: i64,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub trade_amount: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub trade_fee: ::std::string::String,
    pub trade_id: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub trade_price: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tx_hash: ::std::option::Option<::std::string::String>,
}
///`TradesByInstrumentNotification`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/definitions/PublicTrade"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct TradesByInstrumentNotification(pub PublicTrade);
impl ::std::ops::Deref for TradesByInstrumentNotification {
    type Target = PublicTrade;
    fn deref(&self) -> &PublicTrade {
        &self.0
    }
}
impl ::std::convert::From<TradesByInstrumentNotification> for PublicTrade {
    fn from(value: TradesByInstrumentNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<PublicTrade> for TradesByInstrumentNotification {
    fn from(value: PublicTrade) -> Self {
        Self(value)
    }
}
///`TradesByInstrumentTypeCurrencyBatchStatusNotification`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/definitions/SettledTrade"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct TradesByInstrumentTypeCurrencyBatchStatusNotification(pub SettledTrade);
impl ::std::ops::Deref for TradesByInstrumentTypeCurrencyBatchStatusNotification {
    type Target = SettledTrade;
    fn deref(&self) -> &SettledTrade {
        &self.0
    }
}
impl ::std::convert::From<TradesByInstrumentTypeCurrencyBatchStatusNotification>
for SettledTrade {
    fn from(value: TradesByInstrumentTypeCurrencyBatchStatusNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<SettledTrade>
for TradesByInstrumentTypeCurrencyBatchStatusNotification {
    fn from(value: SettledTrade) -> Self {
        Self(value)
    }
}
///`TradesByInstrumentTypeCurrencyNotification`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/definitions/PublicTrade"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct TradesByInstrumentTypeCurrencyNotification(pub PublicTrade);
impl ::std::ops::Deref for TradesByInstrumentTypeCurrencyNotification {
    type Target = PublicTrade;
    fn deref(&self) -> &PublicTrade {
        &self.0
    }
}
impl ::std::convert::From<TradesByInstrumentTypeCurrencyNotification> for PublicTrade {
    fn from(value: TradesByInstrumentTypeCurrencyNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<PublicTrade> for TradesByInstrumentTypeCurrencyNotification {
    fn from(value: PublicTrade) -> Self {
        Self(value)
    }
}
///`TriggerPriceType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "mark",
///    "index"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum TriggerPriceType {
    #[serde(rename = "mark")]
    Mark,
    #[serde(rename = "index")]
    Index,
}
impl ::std::fmt::Display for TriggerPriceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mark => f.write_str("mark"),
            Self::Index => f.write_str("index"),
        }
    }
}
impl ::std::str::FromStr for TriggerPriceType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "mark" => Ok(Self::Mark),
            "index" => Ok(Self::Index),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TriggerPriceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TriggerPriceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TriggerPriceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`TriggerType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "stoploss",
///    "takeprofit"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum TriggerType {
    #[serde(rename = "stoploss")]
    Stoploss,
    #[serde(rename = "takeprofit")]
    Takeprofit,
}
impl ::std::fmt::Display for TriggerType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Stoploss => f.write_str("stoploss"),
            Self::Takeprofit => f.write_str("takeprofit"),
        }
    }
}
impl ::std::str::FromStr for TriggerType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "stoploss" => Ok(Self::Stoploss),
            "takeprofit" => Ok(Self::Takeprofit),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TriggerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TriggerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TriggerType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/**Params for `unsubscribe`. `channels` is optional.

When omitted (or null) the connection unsubscribes from all channels.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Params for `unsubscribe`. `channels` is optional.\n\nWhen omitted (or null) the connection unsubscribes from all channels.",
///  "type": "object",
///  "properties": {
///    "channels": {
///      "default": null,
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "type": "string"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UnsubscribeParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub channels: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ::std::default::Default for UnsubscribeParams {
    fn default() -> Self {
        Self {
            channels: Default::default(),
        }
    }
}
///`UnsubscribeRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "id",
///    "method",
///    "params"
///  ],
///  "properties": {
///    "headers": {
///      "description": "Non-standard; used by `auth/login`.",
///      "type": [
///        "object",
///        "null"
///      ],
///      "additionalProperties": true
///    },
///    "id": {
///      "$ref": "#/definitions/JsonRpcId"
///    },
///    "method": {
///      "type": "string",
///      "const": "unsubscribe"
///    },
///    "params": {
///      "$ref": "#/definitions/UnsubscribeParams"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UnsubscribeRequest {
    ///Non-standard; used by `auth/login`.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub headers: ::std::option::Option<
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    >,
    pub id: JsonRpcId,
    pub method: ::std::string::String,
    pub params: UnsubscribeParams,
}
///`WalletRfqsNotification`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/definitions/PublicRfq"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct WalletRfqsNotification(pub PublicRfq);
impl ::std::ops::Deref for WalletRfqsNotification {
    type Target = PublicRfq;
    fn deref(&self) -> &PublicRfq {
        &self.0
    }
}
impl ::std::convert::From<WalletRfqsNotification> for PublicRfq {
    fn from(value: WalletRfqsNotification) -> Self {
        value.0
    }
}
impl ::std::convert::From<PublicRfq> for WalletRfqsNotification {
    fn from(value: PublicRfq) -> Self {
        Self(value)
    }
}
/// Generation of default values for serde.
pub mod defaults {
    pub(super) fn order_cancel_reason() -> super::CancelReason {
        super::CancelReason::X
    }
}
