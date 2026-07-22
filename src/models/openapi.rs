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
///`AccountFeeInfo`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "base_fee_discount",
///    "rfq_maker_discount",
///    "rfq_taker_discount"
///  ],
///  "properties": {
///    "base_fee_discount": {
///      "type": "string"
///    },
///    "option_maker_fee": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "option_taker_fee": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "perp_maker_fee": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "perp_taker_fee": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "rfq_maker_discount": {
///      "type": "string"
///    },
///    "rfq_taker_discount": {
///      "type": "string"
///    },
///    "spot_maker_fee": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "spot_taker_fee": {
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
pub struct AccountFeeInfo {
    pub base_fee_discount: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub option_maker_fee: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub option_taker_fee: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub perp_maker_fee: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub perp_taker_fee: ::std::option::Option<::std::string::String>,
    pub rfq_maker_discount: ::std::string::String,
    pub rfq_taker_discount: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub spot_maker_fee: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub spot_taker_fee: ::std::option::Option<::std::string::String>,
}
///EIP-712 typed-data Action struct that you sign; its fields are defined below.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "EIP-712 typed-data Action struct that you sign; its fields are defined below.",
///  "type": "object",
///  "required": [
///    "data",
///    "expiry",
///    "module",
///    "nonce",
///    "owner",
///    "signer",
///    "subaccount_id"
///  ],
///  "properties": {
///    "data": {
///      "description": "ABI-encoded action data (`bytes`).",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint8",
///        "minimum": 0.0
///      }
///    },
///    "expiry": {
///      "description": "Unix timestamp after which the action is invalid (`uint256`).",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "module": {
///      "description": "Module address that will process this action (`address`).",
///      "$ref": "#/definitions/Address"
///    },
///    "nonce": {
///      "description": "Replay-protection nonce (`uint256`), interpreted as a UTC-**nanosecond** timestamp: it must fall within the nonce validity window (one day) of the action's timestamp.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "owner": {
///      "description": "Owner of the subaccount (`address`).",
///      "$ref": "#/definitions/Address"
///    },
///    "signer": {
///      "description": "Address that signed this action (`address`).",
///      "$ref": "#/definitions/Address"
///    },
///    "subaccount_id": {
///      "description": "Subaccount identifier (`uint256`).",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Action {
    ///ABI-encoded action data (`bytes`).
    pub data: ::std::vec::Vec<u8>,
    ///Unix timestamp after which the action is invalid (`uint256`).
    pub expiry: u64,
    ///Module address that will process this action (`address`).
    pub module: Address,
    ///Replay-protection nonce (`uint256`), interpreted as a UTC-**nanosecond** timestamp: it must fall within the nonce validity window (one day) of the action's timestamp.
    pub nonce: u64,
    ///Owner of the subaccount (`address`).
    pub owner: Address,
    ///Address that signed this action (`address`).
    pub signer: Address,
    ///Subaccount identifier (`uint256`).
    pub subaccount_id: u64,
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
///`AggregatedOrdersResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "orders",
///    "subaccount_id"
///  ],
///  "properties": {
///    "orders": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Order"
///      }
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AggregatedOrdersResult {
    pub orders: ::std::vec::Vec<Order>,
    pub subaccount_id: i64,
}
///`AggregatedTriggerOrdersResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "orders",
///    "subaccount_id"
///  ],
///  "properties": {
///    "orders": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Order"
///      }
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AggregatedTriggerOrdersResult {
    pub orders: ::std::vec::Vec<Order>,
    pub subaccount_id: i64,
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
///`Asset`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "address",
///    "asset_id",
///    "asset_name",
///    "asset_type",
///    "currency",
///    "is_collateral",
///    "is_position",
///    "sub_id"
///  ],
///  "properties": {
///    "address": {
///      "type": "string"
///    },
///    "asset_id": {
///      "type": "string"
///    },
///    "asset_name": {
///      "type": "string"
///    },
///    "asset_type": {
///      "$ref": "#/definitions/AssetType"
///    },
///    "currency": {
///      "type": "string"
///    },
///    "erc20_details": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/SpotPublicDetails"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "is_collateral": {
///      "type": "boolean"
///    },
///    "is_position": {
///      "type": "boolean"
///    },
///    "option_details": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/OptionDetails"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "perp_details": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/PerpDetails"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "sub_id": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Asset {
    pub address: ::std::string::String,
    pub asset_id: ::std::string::String,
    pub asset_name: ::std::string::String,
    pub asset_type: AssetType,
    pub currency: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub erc20_details: ::std::option::Option<SpotPublicDetails>,
    pub is_collateral: bool,
    pub is_position: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub option_details: ::std::option::Option<OptionDetails>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub perp_details: ::std::option::Option<PerpDetails>,
    pub sub_id: ::std::string::String,
}
///A non-spot asset (option or perp) and its per-universe risk.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A non-spot asset (option or perp) and its per-universe risk.",
///  "type": "object",
///  "required": [
///    "address",
///    "name",
///    "universes"
///  ],
///  "properties": {
///    "address": {
///      "description": "EIP-55 checksummed protocol asset address.",
///      "type": "string"
///    },
///    "name": {
///      "description": "Registered asset name (e.g. \"ETH-OPTION\", \"ETH-PERP\").",
///      "type": "string"
///    },
///    "universes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/AssetUniverse"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct AssetEntry {
    ///EIP-55 checksummed protocol asset address.
    pub address: ::std::string::String,
    ///Registered asset name (e.g. "ETH-OPTION", "ETH-PERP").
    pub name: ::std::string::String,
    pub universes: ::std::vec::Vec<AssetUniverse>,
}
///Asset type of the instrument: `"option"`, `"perp"`, or `"erc20"`.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Asset type of the instrument: `\"option\"`, `\"perp\"`, or `\"erc20\"`.",
///  "type": "string",
///  "enum": [
///    "option",
///    "perp",
///    "erc20"
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
pub enum AssetType {
    #[serde(rename = "option")]
    Option,
    #[serde(rename = "perp")]
    Perp,
    #[serde(rename = "erc20")]
    Erc20,
}
impl ::std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Option => f.write_str("option"),
            Self::Perp => f.write_str("perp"),
            Self::Erc20 => f.write_str("erc20"),
        }
    }
}
impl ::std::str::FromStr for AssetType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "option" => Ok(Self::Option),
            "perp" => Ok(Self::Perp),
            "erc20" => Ok(Self::Erc20),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AssetType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AssetType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AssetType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Per-`(asset, universe)` open interest for an option/perp asset.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Per-`(asset, universe)` open interest for an option/perp asset.",
///  "type": "object",
///  "required": [
///    "oi",
///    "risk_universe_id"
///  ],
///  "properties": {
///    "oi": {
///      "$ref": "#/definitions/OpenInterestStats"
///    },
///    "risk_universe_id": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "risk_universe_name": {
///      "description": "Display name of the universe (uppercase, e.g. \"PRIME\"); absent until set by the exchange.",
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
pub struct AssetUniverse {
    pub oi: OpenInterestStats,
    pub risk_universe_id: u32,
    ///Display name of the universe (uppercase, e.g. "PRIME"); absent until set by the exchange.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub risk_universe_name: ::std::option::Option<::std::string::String>,
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
///Fields common to every signed vault action. They appear alongside each endpoint's action-specific parameters in the same request object.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Fields common to every signed vault action. They appear alongside each endpoint's action-specific parameters in the same request object.",
///  "type": "object",
///  "required": [
///    "nonce",
///    "request_id",
///    "share_price",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "subaccount_id",
///    "withdraw_hash"
///  ],
///  "properties": {
///    "nonce": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "request_id": {
///      "$ref": "#/definitions/VaultRequestId"
///    },
///    "share_price": {
///      "description": "Quoted share price in USD per share, as a decimal string (e.g. `\"1\"`).",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "signature": {
///      "description": "0x-prefixed hex of the 65-byte EOA signature.",
///      "type": "string"
///    },
///    "signature_expiry_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "signer": {
///      "$ref": "#/definitions/Address"
///    },
///    "subaccount_id": {
///      "description": "The subaccount the action is signed on; which subaccount applies depends on the specific method — see that method's reference.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "withdraw_hash": {
///      "description": "0x-prefixed hex of the 32-byte user withdraw-action hash.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct BurnSharesRequest {
    pub nonce: u64,
    pub request_id: VaultRequestId,
    ///Quoted share price in USD per share, as a decimal string (e.g. `"1"`).
    pub share_price: ::bigdecimal::BigDecimal,
    ///0x-prefixed hex of the 65-byte EOA signature.
    pub signature: ::std::string::String,
    pub signature_expiry_sec: u64,
    pub signer: Address,
    ///The subaccount the action is signed on; which subaccount applies depends on the specific method — see that method's reference.
    pub subaccount_id: u64,
    ///0x-prefixed hex of the 32-byte user withdraw-action hash.
    pub withdraw_hash: ::std::string::String,
}
///`CancelAlgoOrderRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "order_id",
///    "subaccount_id"
///  ],
///  "properties": {
///    "order_id": {
///      "description": "UUID v4 string",
///      "type": "string",
///      "format": "uuid"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelAlgoOrderRequest {
    ///UUID v4 string
    pub order_id: ::uuid::Uuid,
    pub subaccount_id: i64,
}
///`CancelAllAlgoOrdersRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelAllAlgoOrdersRequest {
    pub subaccount_id: i64,
}
///The literal string `"ok"` returned on success.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The literal string `\"ok\"` returned on success.",
///  "type": "string",
///  "enum": [
///    "ok"
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
pub enum CancelAllAlgoOrdersResponse {
    #[serde(rename = "ok")]
    Ok,
}
impl ::std::fmt::Display for CancelAllAlgoOrdersResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ok => f.write_str("ok"),
        }
    }
}
impl ::std::str::FromStr for CancelAllAlgoOrdersResponse {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ok" => Ok(Self::Ok),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CancelAllAlgoOrdersResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CancelAllAlgoOrdersResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CancelAllAlgoOrdersResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`CancelAllRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "cancel_algo_orders": {
///      "default": null,
///      "type": [
///        "boolean",
///        "null"
///      ]
///    },
///    "cancel_trigger_orders": {
///      "default": null,
///      "type": [
///        "boolean",
///        "null"
///      ]
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelAllRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cancel_algo_orders: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cancel_trigger_orders: ::std::option::Option<bool>,
    pub subaccount_id: i64,
}
///The literal string `"ok"` returned on success.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The literal string `\"ok\"` returned on success.",
///  "type": "string",
///  "enum": [
///    "ok"
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
pub enum CancelAllResponse {
    #[serde(rename = "ok")]
    Ok,
}
impl ::std::fmt::Display for CancelAllResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ok => f.write_str("ok"),
        }
    }
}
impl ::std::str::FromStr for CancelAllResponse {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ok" => Ok(Self::Ok),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CancelAllResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CancelAllResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CancelAllResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`CancelAllTriggerOrdersRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelAllTriggerOrdersRequest {
    pub subaccount_id: i64,
}
///The literal string `"ok"` returned on success.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The literal string `\"ok\"` returned on success.",
///  "type": "string",
///  "enum": [
///    "ok"
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
pub enum CancelAllTriggerOrdersResponse {
    #[serde(rename = "ok")]
    Ok,
}
impl ::std::fmt::Display for CancelAllTriggerOrdersResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ok => f.write_str("ok"),
        }
    }
}
impl ::std::str::FromStr for CancelAllTriggerOrdersResponse {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ok" => Ok(Self::Ok),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CancelAllTriggerOrdersResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CancelAllTriggerOrdersResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CancelAllTriggerOrdersResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`CancelBatchQuotesRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "label": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "nonce": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "quote_id": {
///      "description": "Optional UUID v4 string",
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "uuid"
///    },
///    "rfq_id": {
///      "description": "Optional UUID v4 string",
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "uuid"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelBatchQuotesRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nonce: ::std::option::Option<i64>,
    ///Optional UUID v4 string
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quote_id: ::std::option::Option<::uuid::Uuid>,
    ///Optional UUID v4 string
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rfq_id: ::std::option::Option<::uuid::Uuid>,
    pub subaccount_id: u64,
}
///`CancelBatchResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "cancelled_ids"
///  ],
///  "properties": {
///    "cancelled_ids": {
///      "type": "array",
///      "items": {
///        "type": "string",
///        "format": "uuid"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelBatchResult {
    pub cancelled_ids: ::std::vec::Vec<::uuid::Uuid>,
}
///`CancelBatchRfqsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "label": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "nonce": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "rfq_id": {
///      "description": "Optional UUID v4 string",
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "uuid"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelBatchRfqsRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nonce: ::std::option::Option<i64>,
    ///Optional UUID v4 string
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rfq_id: ::std::option::Option<::uuid::Uuid>,
    pub subaccount_id: u64,
}
///`CancelBatchRfqsResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "cancelled_ids"
///  ],
///  "properties": {
///    "cancelled_ids": {
///      "type": "array",
///      "items": {
///        "type": "string",
///        "format": "uuid"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelBatchRfqsResponse {
    pub cancelled_ids: ::std::vec::Vec<::uuid::Uuid>,
}
///`CancelByInstrumentRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "instrument_name",
///    "subaccount_id"
///  ],
///  "properties": {
///    "instrument_name": {
///      "type": "string"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelByInstrumentRequest {
    pub instrument_name: ::std::string::String,
    pub subaccount_id: i64,
}
///`CancelByInstrumentResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "cancelled_orders"
///  ],
///  "properties": {
///    "cancelled_orders": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelByInstrumentResponse {
    pub cancelled_orders: i64,
}
///`CancelByLabelRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "label",
///    "subaccount_id"
///  ],
///  "properties": {
///    "instrument_name": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "label": {
///      "type": "string"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelByLabelRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instrument_name: ::std::option::Option<::std::string::String>,
    pub label: ::std::string::String,
    pub subaccount_id: i64,
}
///`CancelByLabelResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "cancelled_orders"
///  ],
///  "properties": {
///    "cancelled_orders": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelByLabelResponse {
    pub cancelled_orders: i64,
}
///`CancelByNonceRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "instrument_name",
///    "nonce",
///    "subaccount_id"
///  ],
///  "properties": {
///    "instrument_name": {
///      "type": "string"
///    },
///    "nonce": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelByNonceRequest {
    pub instrument_name: ::std::string::String,
    pub nonce: i64,
    pub subaccount_id: i64,
}
///`CancelByNonceResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "cancelled_orders"
///  ],
///  "properties": {
///    "cancelled_orders": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelByNonceResponse {
    pub cancelled_orders: i64,
}
///`CancelOrderRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "instrument_name",
///    "order_id",
///    "subaccount_id"
///  ],
///  "properties": {
///    "instrument_name": {
///      "type": "string"
///    },
///    "order_id": {
///      "description": "UUID v4 string",
///      "type": "string",
///      "format": "uuid"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelOrderRequest {
    pub instrument_name: ::std::string::String,
    ///UUID v4 string
    pub order_id: ::uuid::Uuid,
    pub subaccount_id: i64,
}
///`CancelQuoteRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "quote_id",
///    "subaccount_id"
///  ],
///  "properties": {
///    "label": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "nonce": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "quote_id": {
///      "description": "UUID v4 string",
///      "type": "string",
///      "format": "uuid"
///    },
///    "rfq_id": {
///      "description": "Optional UUID v4 string",
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "uuid"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelQuoteRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nonce: ::std::option::Option<i64>,
    ///UUID v4 string
    pub quote_id: ::uuid::Uuid,
    ///Optional UUID v4 string
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rfq_id: ::std::option::Option<::uuid::Uuid>,
    pub subaccount_id: u64,
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
///`CancelRfqRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "rfq_id",
///    "subaccount_id"
///  ],
///  "properties": {
///    "rfq_id": {
///      "description": "UUID v4 string",
///      "type": "string",
///      "format": "uuid"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelRfqRequest {
    ///UUID v4 string
    pub rfq_id: ::uuid::Uuid,
    pub subaccount_id: u64,
}
///The literal string `"ok"` returned on success.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The literal string `\"ok\"` returned on success.",
///  "type": "string",
///  "enum": [
///    "ok"
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
pub enum CancelRfqResponse {
    #[serde(rename = "ok")]
    Ok,
}
impl ::std::fmt::Display for CancelRfqResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ok => f.write_str("ok"),
        }
    }
}
impl ::std::str::FromStr for CancelRfqResponse {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ok" => Ok(Self::Ok),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CancelRfqResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CancelRfqResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CancelRfqResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`CancelTriggerOrderRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "order_id",
///    "subaccount_id"
///  ],
///  "properties": {
///    "order_id": {
///      "description": "UUID v4 string",
///      "type": "string",
///      "format": "uuid"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelTriggerOrderRequest {
    ///UUID v4 string
    pub order_id: ::uuid::Uuid,
    pub subaccount_id: i64,
}
///Fields common to every signed vault action. They appear alongside each endpoint's action-specific parameters in the same request object.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Fields common to every signed vault action. They appear alongside each endpoint's action-specific parameters in the same request object.",
///  "type": "object",
///  "required": [
///    "nonce",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "subaccount_id",
///    "vault_subaccount_id"
///  ],
///  "properties": {
///    "nonce": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "signature": {
///      "description": "0x-prefixed hex of the 65-byte EOA signature.",
///      "type": "string"
///    },
///    "signature_expiry_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "signer": {
///      "$ref": "#/definitions/Address"
///    },
///    "subaccount_id": {
///      "description": "The subaccount the action is signed on; which subaccount applies depends on the specific method — see that method's reference.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "vault_subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CancelVaultRequestRequest {
    pub nonce: u64,
    ///0x-prefixed hex of the 65-byte EOA signature.
    pub signature: ::std::string::String,
    pub signature_expiry_sec: u64,
    pub signer: Address,
    ///The subaccount the action is signed on; which subaccount applies depends on the specific method — see that method's reference.
    pub subaccount_id: u64,
    pub vault_subaccount_id: u64,
}
///`ChangeSubaccountLabelRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "label",
///    "subaccount_id"
///  ],
///  "properties": {
///    "label": {
///      "type": "string"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ChangeSubaccountLabelRequest {
    pub label: ::std::string::String,
    pub subaccount_id: u64,
}
///`Collateral`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "amount",
///    "amount_step",
///    "asset_name",
///    "asset_type",
///    "average_price",
///    "average_price_excl_fees",
///    "creation_timestamp",
///    "cumulative_interest",
///    "currency",
///    "delta",
///    "delta_currency",
///    "initial_margin",
///    "maintenance_margin",
///    "mark_price",
///    "mark_value",
///    "open_orders_margin",
///    "pending_interest",
///    "realized_pnl",
///    "realized_pnl_excl_fees",
///    "total_fees",
///    "unrealized_pnl",
///    "unrealized_pnl_excl_fees"
///  ],
///  "properties": {
///    "amount": {
///      "type": "string"
///    },
///    "amount_step": {
///      "type": "string"
///    },
///    "asset_name": {
///      "type": "string"
///    },
///    "asset_type": {
///      "type": "string"
///    },
///    "average_price": {
///      "type": "string"
///    },
///    "average_price_excl_fees": {
///      "type": "string"
///    },
///    "creation_timestamp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "cumulative_interest": {
///      "type": "string"
///    },
///    "currency": {
///      "type": "string"
///    },
///    "delta": {
///      "type": "string"
///    },
///    "delta_currency": {
///      "type": "string"
///    },
///    "initial_margin": {
///      "type": "string"
///    },
///    "maintenance_margin": {
///      "type": "string"
///    },
///    "mark_price": {
///      "type": "string"
///    },
///    "mark_value": {
///      "type": "string"
///    },
///    "open_orders_margin": {
///      "type": "string"
///    },
///    "pending_interest": {
///      "type": "string"
///    },
///    "realized_pnl": {
///      "type": "string"
///    },
///    "realized_pnl_excl_fees": {
///      "type": "string"
///    },
///    "total_fees": {
///      "type": "string"
///    },
///    "unrealized_pnl": {
///      "type": "string"
///    },
///    "unrealized_pnl_excl_fees": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Collateral {
    pub amount: ::std::string::String,
    pub amount_step: ::std::string::String,
    pub asset_name: ::std::string::String,
    pub asset_type: ::std::string::String,
    pub average_price: ::std::string::String,
    pub average_price_excl_fees: ::std::string::String,
    pub creation_timestamp: i64,
    pub cumulative_interest: ::std::string::String,
    pub currency: ::std::string::String,
    pub delta: ::std::string::String,
    pub delta_currency: ::std::string::String,
    pub initial_margin: ::std::string::String,
    pub maintenance_margin: ::std::string::String,
    pub mark_price: ::std::string::String,
    pub mark_value: ::std::string::String,
    pub open_orders_margin: ::std::string::String,
    pub pending_interest: ::std::string::String,
    pub realized_pnl: ::std::string::String,
    pub realized_pnl_excl_fees: ::std::string::String,
    pub total_fees: ::std::string::String,
    pub unrealized_pnl: ::std::string::String,
    pub unrealized_pnl_excl_fees: ::std::string::String,
}
///`private/order` params.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "`private/order` params.",
///  "type": "object",
///  "required": [
///    "amount",
///    "direction",
///    "instrument_name",
///    "limit_price",
///    "max_fee",
///    "nonce",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "subaccount_id"
///  ],
///  "properties": {
///    "algo_duration_sec": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int32"
///    },
///    "algo_num_slices": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int32"
///    },
///    "algo_type": {
///      "default": null,
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
///      "description": "Order amount in units of the base, as a decimal string (e.g. `\"1.5\"`) or a JSON number.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "client": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "extra_fee": {
///      "description": "Optional extra fee per unit of volume, as a decimal string or JSON number. Defaults to zero.",
///      "default": null,
///      "anyOf": [
///        {
///          "description": "Optional extra fee per unit of volume, as a decimal string or JSON number. Defaults to zero.",
///          "default": null,
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "instrument_name": {
///      "type": "string"
///    },
///    "is_atomic_signing": {
///      "default": null,
///      "type": [
///        "boolean",
///        "null"
///      ]
///    },
///    "label": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "limit_price": {
///      "description": "Limit price in quote currency, as a decimal string (e.g. `\"3100.5\"`) or a JSON number.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "max_fee": {
///      "description": "Max fee per unit of volume in quote currency, as a decimal string or a JSON number.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "mmp": {
///      "default": null,
///      "type": [
///        "boolean",
///        "null"
///      ]
///    },
///    "nonce": {
///      "type": "string"
///    },
///    "order_type": {
///      "default": "limit",
///      "$ref": "#/definitions/OrderType"
///    },
///    "reduce_only": {
///      "default": null,
///      "type": [
///        "boolean",
///        "null"
///      ]
///    },
///    "referral_code": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "reject_post_only": {
///      "default": null,
///      "type": [
///        "boolean",
///        "null"
///      ]
///    },
///    "reject_timestamp": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
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
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "time_in_force": {
///      "default": "gtc",
///      "$ref": "#/definitions/TimeInForce"
///    },
///    "trigger_price": {
///      "description": "Trigger price as a decimal string or JSON number; omit for non-trigger orders.",
///      "default": null,
///      "anyOf": [
///        {
///          "description": "Trigger price as a decimal string or JSON number; omit for non-trigger orders.",
///          "default": null,
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "trigger_price_type": {
///      "default": null,
///      "anyOf": [
///        {
///          "$ref": "#/definitions/TriggerPriceType"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "trigger_type": {
///      "default": null,
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
pub struct CreateOrderRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub algo_duration_sec: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub algo_num_slices: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub algo_type: ::std::option::Option<AlgoType>,
    ///Order amount in units of the base, as a decimal string (e.g. `"1.5"`) or a JSON number.
    pub amount: ::bigdecimal::BigDecimal,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub client: ::std::option::Option<::std::string::String>,
    pub direction: Direction,
    ///Optional extra fee per unit of volume, as a decimal string or JSON number. Defaults to zero.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub extra_fee: ::std::option::Option<::bigdecimal::BigDecimal>,
    pub instrument_name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_atomic_signing: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    ///Limit price in quote currency, as a decimal string (e.g. `"3100.5"`) or a JSON number.
    pub limit_price: ::bigdecimal::BigDecimal,
    ///Max fee per unit of volume in quote currency, as a decimal string or a JSON number.
    pub max_fee: ::bigdecimal::BigDecimal,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mmp: ::std::option::Option<bool>,
    pub nonce: ::std::string::String,
    #[serde(default = "defaults::create_order_request_order_type")]
    pub order_type: OrderType,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reduce_only: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub referral_code: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reject_post_only: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reject_timestamp: ::std::option::Option<i64>,
    pub signature: ::std::string::String,
    pub signature_expiry_sec: i64,
    pub signer: ::std::string::String,
    pub subaccount_id: i64,
    #[serde(default = "defaults::create_order_request_time_in_force")]
    pub time_in_force: TimeInForce,
    ///Trigger price as a decimal string or JSON number; omit for non-trigger orders.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trigger_price: ::std::option::Option<::bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trigger_price_type: ::std::option::Option<TriggerPriceType>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trigger_type: ::std::option::Option<TriggerType>,
}
///Request parameters for registering a scoped session key. Address fields are 0x-prefixed hex strings.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Request parameters for registering a scoped session key. Address fields are 0x-prefixed hex strings.",
///  "type": "object",
///  "required": [
///    "expiry_sec",
///    "nonce",
///    "offchain_scopes",
///    "protocol_scopes",
///    "public_session_key",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "wallet"
///  ],
///  "properties": {
///    "expiry_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "ip_whitelist": {
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "type": "string"
///      }
///    },
///    "label": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "nonce": {
///      "type": "string"
///    },
///    "offchain_scopes": {
///      "description": "Off-chain scopes which are validated in backend only",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "protocol_scopes": {
///      "description": "Scopes granted to the session key, validated by the protocol. Each is a string like `\"trade:orderbook:all\"`.",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "public_session_key": {
///      "description": "Session key address being authorized.",
///      "type": "string"
///    },
///    "signature": {
///      "description": "0x-prefixed hex, 65-byte r||s||v.",
///      "type": "string"
///    },
///    "signature_expiry_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "signer": {
///      "type": "string"
///    },
///    "subaccount_ids": {
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "wallet": {
///      "description": "Wallet the session key is being registered for.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CreateSessionKeyRequest {
    pub expiry_sec: u64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ip_whitelist: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    pub nonce: ::std::string::String,
    ///Off-chain scopes which are validated in backend only
    pub offchain_scopes: ::std::vec::Vec<::std::string::String>,
    ///Scopes granted to the session key, validated by the protocol. Each is a string like `"trade:orderbook:all"`.
    pub protocol_scopes: ::std::vec::Vec<::std::string::String>,
    ///Session key address being authorized.
    pub public_session_key: ::std::string::String,
    ///0x-prefixed hex, 65-byte r||s||v.
    pub signature: ::std::string::String,
    pub signature_expiry_sec: u64,
    pub signer: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subaccount_ids: ::std::option::Option<::std::vec::Vec<u64>>,
    ///Wallet the session key is being registered for.
    pub wallet: ::std::string::String,
}
///Fields common to every signed vault action. They appear alongside each endpoint's action-specific parameters in the same request object.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Fields common to every signed vault action. They appear alongside each endpoint's action-specific parameters in the same request object.",
///  "type": "object",
///  "required": [
///    "cooldown_sec",
///    "deposit_spot_asset",
///    "initial_deposit",
///    "initial_share_price_usd",
///    "management_fee_bps",
///    "manager_id",
///    "max_fee_usd",
///    "max_slippage_bps",
///    "nonce",
///    "performance_fee_bps",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "subaccount_id"
///  ],
///  "properties": {
///    "benchmark_asset": {
///      "description": "Spot asset to denominate the high-water mark in. Omit (or null) for the feed-less USD default; set it to charge performance fees only on outperformance measured against that asset.",
///      "default": null,
///      "anyOf": [
///        {
///          "$ref": "#/definitions/Address"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "cooldown_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "deposit_spot_asset": {
///      "$ref": "#/definitions/Address"
///    },
///    "initial_deposit": {
///      "description": "Initial deposit in the vault's deposit asset, as a USD decimal string (e.g. `\"15000\"`).",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "initial_share_price_usd": {
///      "description": "Initial share price the vault is seeded at, in USD, as a decimal string (e.g. `\"1\"` or `\"100\"`). Must lie within the protocol's permitted range.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "management_fee_bps": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "manager_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "max_fee_usd": {
///      "description": "Maximum sequencer fee the signer authorises, in USD, as a decimal string (e.g. `\"1.5\"`).",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "max_slippage_bps": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "nonce": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "performance_fee_bps": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "signature": {
///      "description": "0x-prefixed hex of the 65-byte EOA signature.",
///      "type": "string"
///    },
///    "signature_expiry_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "signer": {
///      "$ref": "#/definitions/Address"
///    },
///    "subaccount_id": {
///      "description": "The subaccount the action is signed on; which subaccount applies depends on the specific method — see that method's reference.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct CreateVaultRequest {
    ///Spot asset to denominate the high-water mark in. Omit (or null) for the feed-less USD default; set it to charge performance fees only on outperformance measured against that asset.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub benchmark_asset: ::std::option::Option<Address>,
    pub cooldown_sec: u64,
    pub deposit_spot_asset: Address,
    ///Initial deposit in the vault's deposit asset, as a USD decimal string (e.g. `"15000"`).
    pub initial_deposit: ::bigdecimal::BigDecimal,
    ///Initial share price the vault is seeded at, in USD, as a decimal string (e.g. `"1"` or `"100"`). Must lie within the protocol's permitted range.
    pub initial_share_price_usd: ::bigdecimal::BigDecimal,
    pub management_fee_bps: u64,
    pub manager_id: u64,
    ///Maximum sequencer fee the signer authorises, in USD, as a decimal string (e.g. `"1.5"`).
    pub max_fee_usd: ::bigdecimal::BigDecimal,
    pub max_slippage_bps: u64,
    pub nonce: u64,
    pub performance_fee_bps: u64,
    ///0x-prefixed hex of the 65-byte EOA signature.
    pub signature: ::std::string::String,
    pub signature_expiry_sec: u64,
    pub signer: Address,
    ///The subaccount the action is signed on; which subaccount applies depends on the specific method — see that method's reference.
    pub subaccount_id: u64,
}
///A currency's spot price and its assets grouped by type. Managers risking the currency are listed per universe under `managers`; asset-level risk — OI, lending, and discounts — lives under each asset's `universes`.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A currency's spot price and its assets grouped by type. Managers risking the currency are listed per universe under `managers`; asset-level risk — OI, lending, and discounts — lives under each asset's `universes`.",
///  "type": "object",
///  "required": [
///    "currency",
///    "managers",
///    "market_type",
///    "spot",
///    "spot_price"
///  ],
///  "properties": {
///    "currency": {
///      "type": "string"
///    },
///    "managers": {
///      "description": "Managers risk-pricing this currency, one entry per universe where at least one manager covers it.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/UniverseManagers"
///      }
///    },
///    "market_type": {
///      "description": "Which instrument/collateral classes the currency is enabled for.",
///      "$ref": "#/definitions/MarketType"
///    },
///    "option": {
///      "description": "The option asset for this currency, if one is registered.",
///      "anyOf": [
///        {
///          "$ref": "#/definitions/AssetEntry"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "perp": {
///      "description": "The perp asset for this currency, if one is registered.",
///      "anyOf": [
///        {
///          "$ref": "#/definitions/AssetEntry"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "spot": {
///      "description": "Every spot asset registered to this currency (e.g. lending \"USDC\" and non-lending \"USDC-NL\").",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/SpotAssetEntry"
///      }
///    },
///    "spot_price": {
///      "type": "string"
///    },
///    "spot_price_24h": {
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
pub struct Currency {
    pub currency: ::std::string::String,
    ///Managers risk-pricing this currency, one entry per universe where at least one manager covers it.
    pub managers: ::std::vec::Vec<UniverseManagers>,
    ///Which instrument/collateral classes the currency is enabled for.
    pub market_type: MarketType,
    ///The option asset for this currency, if one is registered.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub option: ::std::option::Option<AssetEntry>,
    ///The perp asset for this currency, if one is registered.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub perp: ::std::option::Option<AssetEntry>,
    ///Every spot asset registered to this currency (e.g. lending "USDC" and non-lending "USDC-NL").
    pub spot: ::std::vec::Vec<SpotAssetEntry>,
    pub spot_price: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub spot_price_24h: ::std::option::Option<::std::string::String>,
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "h": {
///      "description": "Highest trade price during last 24h",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "l": {
///      "description": "Lowest trade price during last 24h",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "p": {
///      "description": "Options: 24hr percent change in premium; Perps: 24hr percent change in mark price",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "pr": {
///      "description": "Premium volume traded during last 24 hours",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "v": {
///      "description": "Notional volume traded during last 24 hours",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DailyTradingStatistics {
    ///Number of contracts traded during last 24 hours
    pub c: ::bigdecimal::BigDecimal,
    ///Highest trade price during last 24h
    pub h: ::bigdecimal::BigDecimal,
    ///Lowest trade price during last 24h
    pub l: ::bigdecimal::BigDecimal,
    ///Number of trades during last 24h
    pub n: u64,
    ///Current total open interest
    pub oi: ::bigdecimal::BigDecimal,
    ///Options: 24hr percent change in premium; Perps: 24hr percent change in mark price
    pub p: ::bigdecimal::BigDecimal,
    ///Premium volume traded during last 24 hours
    pub pr: ::bigdecimal::BigDecimal,
    ///Notional volume traded during last 24 hours
    pub v: ::bigdecimal::BigDecimal,
}
///`amount` and `fee` are decimal strings (e.g. `"1.1"`); the net credited amount is `amount - fee`. `operation_id`/`batch_uuid` are stable uuids.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "`amount` and `fee` are decimal strings (e.g. `\"1.1\"`); the net credited amount is `amount - fee`. `operation_id`/`batch_uuid` are stable uuids.",
///  "type": "object",
///  "required": [
///    "amount",
///    "asset",
///    "batch_status",
///    "batch_uuid",
///    "fee",
///    "new_subaccount",
///    "operation_id",
///    "subaccount_id",
///    "timestamp",
///    "wallet"
///  ],
///  "properties": {
///    "action_id": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "amount": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "asset": {
///      "type": "string"
///    },
///    "batch_status": {
///      "$ref": "#/definitions/BatchStatus"
///    },
///    "batch_uuid": {
///      "type": "string"
///    },
///    "fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "new_subaccount": {
///      "type": "boolean"
///    },
///    "operation_id": {
///      "type": "string"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "tx_hash": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "wallet": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DepositEntry {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub action_id: ::std::option::Option<u64>,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub amount: ::bigdecimal::BigDecimal,
    pub asset: ::std::string::String,
    pub batch_status: BatchStatus,
    pub batch_uuid: ::std::string::String,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub fee: ::bigdecimal::BigDecimal,
    pub new_subaccount: bool,
    pub operation_id: ::std::string::String,
    pub subaccount_id: u64,
    pub timestamp: u64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tx_hash: ::std::option::Option<::std::string::String>,
    pub wallet: ::std::string::String,
}
///`DepositHistoryResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "deposits"
///  ],
///  "properties": {
///    "deposits": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/DepositEntry"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct DepositHistoryResult {
    pub deposits: ::std::vec::Vec<DepositEntry>,
}
///How a deposit reached the protocol. `Standard` and `Instant` are the two CEX deposit-address flows: `Standard` (formerly `slow`) credits the depositor's own subaccount on-chain via its factory; `Instant` (formerly `fast`) pools every deposit and credits the owner off-chain from the factory's `DepositProcessed` events. `Direct` is a plain wallet deposit straight into a subaccount — no escrow, no factory. Only `Standard`/`Instant` are factory-routed (and registerable / sweepable); `Direct` is a read-only provenance tag.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "How a deposit reached the protocol. `Standard` and `Instant` are the two CEX deposit-address flows: `Standard` (formerly `slow`) credits the depositor's own subaccount on-chain via its factory; `Instant` (formerly `fast`) pools every deposit and credits the owner off-chain from the factory's `DepositProcessed` events. `Direct` is a plain wallet deposit straight into a subaccount — no escrow, no factory. Only `Standard`/`Instant` are factory-routed (and registerable / sweepable); `Direct` is a read-only provenance tag.",
///  "type": "string",
///  "enum": [
///    "standard",
///    "instant",
///    "direct"
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
pub enum DepositType {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "instant")]
    Instant,
    #[serde(rename = "direct")]
    Direct,
}
impl ::std::fmt::Display for DepositType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Standard => f.write_str("standard"),
            Self::Instant => f.write_str("instant"),
            Self::Direct => f.write_str("direct"),
        }
    }
}
impl ::std::str::FromStr for DepositType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "standard" => Ok(Self::Standard),
            "instant" => Ok(Self::Instant),
            "direct" => Ok(Self::Direct),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DepositType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DepositType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DepositType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
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
///`EditSessionKeyRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "public_session_key",
///    "wallet"
///  ],
///  "properties": {
///    "ip_whitelist": {
///      "default": null,
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "type": "string"
///      }
///    },
///    "label": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "offchain_scopes": {
///      "default": null,
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "type": "string"
///      }
///    },
///    "public_session_key": {
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
pub struct EditSessionKeyRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ip_whitelist: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub offchain_scopes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub public_session_key: ::std::string::String,
    pub wallet: ::std::string::String,
}
///This method takes no parameters; send an empty object `{}`.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "This method takes no parameters; send an empty object `{}`.",
///  "type": "null"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EmptyRequest(pub ());
impl ::std::ops::Deref for EmptyRequest {
    type Target = ();
    fn deref(&self) -> &() {
        &self.0
    }
}
impl ::std::convert::From<EmptyRequest> for () {
    fn from(value: EmptyRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<()> for EmptyRequest {
    fn from(value: ()) -> Self {
        Self(value)
    }
}
///`Erc20Details`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "decimals"
///  ],
///  "properties": {
///    "decimals": {
///      "type": "integer",
///      "format": "uint8",
///      "minimum": 0.0
///    },
///    "underlying_erc20": {
///      "description": "On-chain ERC20 token deposited into the spot asset (informational).",
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
pub struct Erc20Details {
    pub decimals: u8,
    ///On-chain ERC20 token deposited into the spot asset (informational).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub underlying_erc20: ::std::option::Option<::std::string::String>,
}
///`ExecuteQuoteRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "direction",
///    "legs",
///    "max_fee",
///    "nonce",
///    "quote_id",
///    "rfq_id",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "subaccount_id"
///  ],
///  "properties": {
///    "client": {
///      "default": "",
///      "type": "string"
///    },
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "enable_taker_protection": {
///      "default": false,
///      "type": "boolean"
///    },
///    "label": {
///      "default": "",
///      "type": "string"
///    },
///    "legs": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PricedLegParamsAndResponse"
///      }
///    },
///    "max_fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "nonce": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "quote_id": {
///      "description": "UUID v4 string",
///      "type": "string",
///      "format": "uuid"
///    },
///    "referral_code": {
///      "default": "",
///      "type": "string"
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
///      "$ref": "#/definitions/Address"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ExecuteQuoteRequest {
    #[serde(default)]
    pub client: ::std::string::String,
    pub direction: Direction,
    #[serde(default)]
    pub enable_taker_protection: bool,
    #[serde(default)]
    pub label: ::std::string::String,
    pub legs: ::std::vec::Vec<PricedLegParamsAndResponse>,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub max_fee: ::bigdecimal::BigDecimal,
    pub nonce: i64,
    ///UUID v4 string
    pub quote_id: ::uuid::Uuid,
    #[serde(default)]
    pub referral_code: ::std::string::String,
    ///UUID v4 string
    pub rfq_id: ::uuid::Uuid,
    pub signature: ::std::string::String,
    pub signature_expiry_sec: i64,
    pub signer: Address,
    pub subaccount_id: u64,
}
///One settled expiry and its settlement price.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "One settled expiry and its settlement price.",
///  "type": "object",
///  "required": [
///    "expiry_date",
///    "utc_expiry_sec"
///  ],
///  "properties": {
///    "expiry_date": {
///      "description": "Expiry date in `YYYYMMDD` format.",
///      "type": "string"
///    },
///    "price": {
///      "description": "Settlement price. Only settled expiries are returned, so never null.",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "utc_expiry_sec": {
///      "description": "UTC timestamp of expiry, unix seconds.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ExpirySettlementPrice {
    ///Expiry date in `YYYYMMDD` format.
    pub expiry_date: ::std::string::String,
    ///Settlement price. Only settled expiries are returned, so never null.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub price: ::std::option::Option<::std::string::String>,
    ///UTC timestamp of expiry, unix seconds.
    pub utc_expiry_sec: u64,
}
///`ForceBurnRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "holder",
///    "subaccount_id"
///  ],
///  "properties": {
///    "holder": {
///      "description": "The shareholder being exited.",
///      "$ref": "#/definitions/Address"
///    },
///    "subaccount_id": {
///      "description": "The vault's subaccount ID.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ForceBurnRequest {
    ///The shareholder being exited.
    pub holder: Address,
    ///The vault's subaccount ID.
    pub subaccount_id: u64,
}
///`ForwardFeedDataResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "confidence",
///    "currency",
///    "deadline",
///    "expiry",
///    "fwd_diff",
///    "signatures",
///    "spot_aggregate_latest",
///    "spot_aggregate_start",
///    "timestamp"
///  ],
///  "properties": {
///    "confidence": {
///      "type": "string"
///    },
///    "currency": {
///      "type": "string"
///    },
///    "deadline": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "expiry": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "fwd_diff": {
///      "type": "string"
///    },
///    "signatures": {
///      "$ref": "#/definitions/OracleSignatureDataResponse"
///    },
///    "spot_aggregate_latest": {
///      "type": "string"
///    },
///    "spot_aggregate_start": {
///      "type": "string"
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ForwardFeedDataResponse {
    pub confidence: ::std::string::String,
    pub currency: ::std::string::String,
    pub deadline: u64,
    pub expiry: u64,
    pub fwd_diff: ::std::string::String,
    pub signatures: OracleSignatureDataResponse,
    pub spot_aggregate_latest: ::std::string::String,
    pub spot_aggregate_start: ::std::string::String,
    pub timestamp: u64,
}
///Single funding-rate OHLC candle. `funding_rate` mirrors `close` for backward compatibility with the pre-OHLC response shape; `timestamp` is the bucket start in UTC milliseconds.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Single funding-rate OHLC candle. `funding_rate` mirrors `close` for backward compatibility with the pre-OHLC response shape; `timestamp` is the bucket start in UTC milliseconds.",
///  "type": "object",
///  "required": [
///    "close",
///    "currency",
///    "funding_rate",
///    "high",
///    "low",
///    "open",
///    "risk_universe_id",
///    "timestamp"
///  ],
///  "properties": {
///    "close": {
///      "type": "string"
///    },
///    "currency": {
///      "type": "string"
///    },
///    "funding_rate": {
///      "type": "string"
///    },
///    "high": {
///      "type": "string"
///    },
///    "low": {
///      "type": "string"
///    },
///    "open": {
///      "type": "string"
///    },
///    "risk_universe_id": {
///      "type": "integer",
///      "format": "uint32",
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
pub struct FundingRateCandle {
    pub close: ::std::string::String,
    pub currency: ::std::string::String,
    pub funding_rate: ::std::string::String,
    pub high: ::std::string::String,
    pub low: ::std::string::String,
    pub open: ::std::string::String,
    pub risk_universe_id: u32,
    pub timestamp: i64,
}
///Response shape for `public/get_funding_rate_history`.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Response shape for `public/get_funding_rate_history`.",
///  "type": "object",
///  "required": [
///    "funding_rate_history"
///  ],
///  "properties": {
///    "funding_rate_history": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/FundingRateCandle"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct FundingRateHistoryResult {
    pub funding_rate_history: ::std::vec::Vec<FundingRateCandle>,
}
///`GetAccountRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "wallet"
///  ],
///  "properties": {
///    "wallet": {
///      "$ref": "#/definitions/Address"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetAccountRequest {
    pub wallet: Address,
}
///`GetAlgoOrdersRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetAlgoOrdersRequest {
    pub subaccount_id: i64,
}
///`GetAllInstrumentsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "expired",
///    "instrument_type"
///  ],
///  "properties": {
///    "currency": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "expired": {
///      "type": "boolean"
///    },
///    "instrument_type": {
///      "$ref": "#/definitions/AssetType"
///    },
///    "page": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "page_size": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "risk_universe_id": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetAllInstrumentsRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub currency: ::std::option::Option<::std::string::String>,
    pub expired: bool,
    pub instrument_type: AssetType,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page_size: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub risk_universe_id: ::std::option::Option<u32>,
}
///`GetAllInstrumentsResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "instruments",
///    "pagination"
///  ],
///  "properties": {
///    "instruments": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Instrument"
///      }
///    },
///    "pagination": {
///      "$ref": "#/definitions/Pagination"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetAllInstrumentsResponse {
    pub instruments: ::std::vec::Vec<Instrument>,
    pub pagination: Pagination,
}
///`GetAllPortfoliosRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "wallet"
///  ],
///  "properties": {
///    "wallet": {
///      "$ref": "#/definitions/Address"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetAllPortfoliosRequest {
    pub wallet: Address,
}
///`GetAllReferralCodesParams`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct GetAllReferralCodesParams(
    pub ::serde_json::Map<::std::string::String, ::serde_json::Value>,
);
impl ::std::ops::Deref for GetAllReferralCodesParams {
    type Target = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    fn deref(&self) -> &::serde_json::Map<::std::string::String, ::serde_json::Value> {
        &self.0
    }
}
impl ::std::convert::From<GetAllReferralCodesParams>
for ::serde_json::Map<::std::string::String, ::serde_json::Value> {
    fn from(value: GetAllReferralCodesParams) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for GetAllReferralCodesParams {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self(value)
    }
}
///`GetAssetsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "asset_type",
///    "currency",
///    "expired"
///  ],
///  "properties": {
///    "asset_type": {
///      "$ref": "#/definitions/AssetType"
///    },
///    "currency": {
///      "type": "string"
///    },
///    "expired": {
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetAssetsRequest {
    pub asset_type: AssetType,
    pub currency: ::std::string::String,
    pub expired: bool,
}
///`GetCollateralsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetCollateralsRequest {
    pub subaccount_id: u64,
}
///`GetCuratedVaultsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "wallet"
///  ],
///  "properties": {
///    "wallet": {
///      "description": "The curator wallet (0x hex).",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetCuratedVaultsRequest {
    ///The curator wallet (0x hex).
    pub wallet: ::std::string::String,
}
///`GetCurrencyRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "currency"
///  ],
///  "properties": {
///    "currency": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetCurrencyRequest {
    pub currency: ::std::string::String,
}
///Parameters for `private/get_deposit_history`. Takes `subaccount_id` plus an optional `[start_timestamp, end_timestamp]` window in unix milliseconds. `wallet`, when set, takes precedence over `subaccount_id` and returns the whole wallet's deposits. Results are capped at 1000 rows.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Parameters for `private/get_deposit_history`. Takes `subaccount_id` plus an optional `[start_timestamp, end_timestamp]` window in unix milliseconds. `wallet`, when set, takes precedence over `subaccount_id` and returns the whole wallet's deposits. Results are capped at 1000 rows.",
///  "type": "object",
///  "properties": {
///    "end_timestamp": {
///      "description": "End of the window, unix milliseconds (default: unbounded / now).",
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "start_timestamp": {
///      "description": "Start of the window, unix milliseconds (default 0).",
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "subaccount_id": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "wallet": {
///      "default": null,
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
pub struct GetDepositHistoryRequest {
    ///End of the window, unix milliseconds (default: unbounded / now).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end_timestamp: ::std::option::Option<u64>,
    ///Start of the window, unix milliseconds (default 0).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub start_timestamp: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subaccount_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wallet: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for GetDepositHistoryRequest {
    fn default() -> Self {
        Self {
            end_timestamp: Default::default(),
            start_timestamp: Default::default(),
            subaccount_id: Default::default(),
            wallet: Default::default(),
        }
    }
}
///Parameters for `private/get_erc20_transfer_history`. Matches `get_withdrawal_history`: `subaccount_id` plus an optional `[start_timestamp, end_timestamp]` window in unix milliseconds. `wallet`, when set, takes precedence over `subaccount_id` and returns the whole wallet's transfers. Results are capped at 1000 rows.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Parameters for `private/get_erc20_transfer_history`. Matches `get_withdrawal_history`: `subaccount_id` plus an optional `[start_timestamp, end_timestamp]` window in unix milliseconds. `wallet`, when set, takes precedence over `subaccount_id` and returns the whole wallet's transfers. Results are capped at 1000 rows.",
///  "type": "object",
///  "properties": {
///    "end_timestamp": {
///      "description": "End of the window, unix milliseconds (default: unbounded / now).",
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "start_timestamp": {
///      "description": "Start of the window, unix milliseconds (default 0).",
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "subaccount_id": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "wallet": {
///      "default": null,
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
pub struct GetErc20TransferHistoryRequest {
    ///End of the window, unix milliseconds (default: unbounded / now).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end_timestamp: ::std::option::Option<u64>,
    ///Start of the window, unix milliseconds (default 0).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub start_timestamp: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subaccount_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wallet: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for GetErc20TransferHistoryRequest {
    fn default() -> Self {
        Self {
            end_timestamp: Default::default(),
            start_timestamp: Default::default(),
            subaccount_id: Default::default(),
            wallet: Default::default(),
        }
    }
}
/**Parameters for `private/get_funding_history`.

`subaccount_id`, `start_timestamp`, and `end_timestamp` accept either a JSON number or a string-encoded integer. Pagination is `page` >= 1, `page_size` in 1..=1000.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Parameters for `private/get_funding_history`.\n\n`subaccount_id`, `start_timestamp`, and `end_timestamp` accept either a JSON number or a string-encoded integer. Pagination is `page` >= 1, `page_size` in 1..=1000.",
///  "type": "object",
///  "properties": {
///    "end_timestamp": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "instrument_name": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "page": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "page_size": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "start_timestamp": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "subaccount_id": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "wallet": {
///      "default": null,
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
pub struct GetFundingHistoryRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end_timestamp: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instrument_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<u32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page_size: ::std::option::Option<u32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub start_timestamp: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subaccount_id: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wallet: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for GetFundingHistoryRequest {
    fn default() -> Self {
        Self {
            end_timestamp: Default::default(),
            instrument_name: Default::default(),
            page: Default::default(),
            page_size: Default::default(),
            start_timestamp: Default::default(),
            subaccount_id: Default::default(),
            wallet: Default::default(),
        }
    }
}
///Parameters for `public/get_funding_rate_history`. Timestamp fields accept either a JSON number or a string-encoded integer, and are UTC milliseconds.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Parameters for `public/get_funding_rate_history`. Timestamp fields accept either a JSON number or a string-encoded integer, and are UTC milliseconds.",
///  "type": "object",
///  "required": [
///    "instrument_name"
///  ],
///  "properties": {
///    "end_timestamp": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "instrument_name": {
///      "type": "string"
///    },
///    "period": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "start_timestamp": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetFundingRateHistoryRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end_timestamp: ::std::option::Option<i64>,
    pub instrument_name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub period: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub start_timestamp: ::std::option::Option<i64>,
}
///Parameters for `public/get_index_chart_data`. Numeric fields accept either a JSON number or a string-encoded integer. Timestamps are UTC seconds.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Parameters for `public/get_index_chart_data`. Numeric fields accept either a JSON number or a string-encoded integer. Timestamps are UTC seconds.",
///  "type": "object",
///  "required": [
///    "currency",
///    "end_timestamp",
///    "period",
///    "start_timestamp"
///  ],
///  "properties": {
///    "currency": {
///      "type": "string"
///    },
///    "end_timestamp": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "period": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "start_timestamp": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetIndexChartDataRequest {
    pub currency: ::std::string::String,
    pub end_timestamp: u64,
    pub period: u64,
    pub start_timestamp: u64,
}
///`GetInstrumentRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "instrument_name"
///  ],
///  "properties": {
///    "instrument_name": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetInstrumentRequest {
    pub instrument_name: ::std::string::String,
}
///Parameters for `private/get_interest_history`. `wallet`, when set, takes precedence over `subaccount_id` and returns the whole wallet's settlements. `[start_timestamp, end_timestamp]` is a unix-millisecond window. Results are capped at 1000 rows, newest first.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Parameters for `private/get_interest_history`. `wallet`, when set, takes precedence over `subaccount_id` and returns the whole wallet's settlements. `[start_timestamp, end_timestamp]` is a unix-millisecond window. Results are capped at 1000 rows, newest first.",
///  "type": "object",
///  "properties": {
///    "end_timestamp": {
///      "description": "End of the window, unix milliseconds (default: now / unbounded).",
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "start_timestamp": {
///      "description": "Start of the window, unix milliseconds (default 0).",
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "subaccount_id": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "wallet": {
///      "default": null,
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
pub struct GetInterestHistoryRequest {
    ///End of the window, unix milliseconds (default: now / unbounded).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end_timestamp: ::std::option::Option<u64>,
    ///Start of the window, unix milliseconds (default 0).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub start_timestamp: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subaccount_id: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wallet: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for GetInterestHistoryRequest {
    fn default() -> Self {
        Self {
            end_timestamp: Default::default(),
            start_timestamp: Default::default(),
            subaccount_id: Default::default(),
            wallet: Default::default(),
        }
    }
}
///Parameters for `public/get_interest_rate_history`. Timestamp fields accept either a JSON number or a string-encoded integer, and are UTC milliseconds.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Parameters for `public/get_interest_rate_history`. Timestamp fields accept either a JSON number or a string-encoded integer, and are UTC milliseconds.",
///  "type": "object",
///  "required": [
///    "currency"
///  ],
///  "properties": {
///    "currency": {
///      "type": "string"
///    },
///    "end_timestamp": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "period": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "risk_universe_id": {
///      "description": "Restrict to one risk universe's pool. Omit to return every universe's candles for the currency (each candle is tagged with its universe).",
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "start_timestamp": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetInterestRateHistoryRequest {
    pub currency: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end_timestamp: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub period: ::std::option::Option<i64>,
    ///Restrict to one risk universe's pool. Omit to return every universe's candles for the currency (each candle is tagged with its universe).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub risk_universe_id: ::std::option::Option<u32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub start_timestamp: ::std::option::Option<i64>,
}
///`GetLatestSignedFeedsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "currency": {
///      "description": "Currency filter, (defaults to all currencies)",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "expiry": {
///      "description": "Expiry filter for options and forward data (defaults to all expiries). Use `0` to get data only for spot and perpetuals.",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetLatestSignedFeedsRequest {
    ///Currency filter, (defaults to all currencies)
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub currency: ::std::option::Option<::std::string::String>,
    ///Expiry filter for options and forward data (defaults to all expiries). Use `0` to get data only for spot and perpetuals.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub expiry: ::std::option::Option<u64>,
}
impl ::std::default::Default for GetLatestSignedFeedsRequest {
    fn default() -> Self {
        Self {
            currency: Default::default(),
            expiry: Default::default(),
        }
    }
}
///`GetLatestSignedFeedsResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "fwd_data",
///    "perp_data",
///    "rate_data",
///    "spot_data",
///    "vol_data"
///  ],
///  "properties": {
///    "fwd_data": {
///      "description": "currency -> expiry -> latest forward feed data",
///      "type": "object",
///      "additionalProperties": {
///        "type": "object",
///        "additionalProperties": {
///          "$ref": "#/definitions/ForwardFeedDataResponse"
///        }
///      }
///    },
///    "perp_data": {
///      "description": "currency -> feed type -> latest perp feed data",
///      "type": "object",
///      "additionalProperties": {
///        "type": "object",
///        "additionalProperties": {
///          "$ref": "#/definitions/PerpFeedDataResponse"
///        }
///      }
///    },
///    "rate_data": {
///      "description": "currency -> expiry -> latest rate feed data",
///      "type": "object",
///      "additionalProperties": {
///        "type": "object",
///        "additionalProperties": {
///          "$ref": "#/definitions/RateFeedDataResponse"
///        }
///      }
///    },
///    "spot_data": {
///      "description": "currency -> latest spot feed data",
///      "type": "object",
///      "additionalProperties": {
///        "$ref": "#/definitions/SpotFeedDataResponse"
///      }
///    },
///    "vol_data": {
///      "description": "currency -> expiry -> latest vol feed data",
///      "type": "object",
///      "additionalProperties": {
///        "type": "object",
///        "additionalProperties": {
///          "$ref": "#/definitions/VolFeedDataResponse"
///        }
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetLatestSignedFeedsResponse {
    ///currency -> expiry -> latest forward feed data
    pub fwd_data: ::std::collections::HashMap<
        ::std::string::String,
        ::std::collections::HashMap<::std::string::String, ForwardFeedDataResponse>,
    >,
    ///currency -> feed type -> latest perp feed data
    pub perp_data: ::std::collections::HashMap<
        ::std::string::String,
        ::std::collections::HashMap<::std::string::String, PerpFeedDataResponse>,
    >,
    ///currency -> expiry -> latest rate feed data
    pub rate_data: ::std::collections::HashMap<
        ::std::string::String,
        ::std::collections::HashMap<::std::string::String, RateFeedDataResponse>,
    >,
    ///currency -> latest spot feed data
    pub spot_data: ::std::collections::HashMap<
        ::std::string::String,
        SpotFeedDataResponse,
    >,
    ///currency -> expiry -> latest vol feed data
    pub vol_data: ::std::collections::HashMap<
        ::std::string::String,
        ::std::collections::HashMap<::std::string::String, VolFeedDataResponse>,
    >,
}
///`GetLiveBurnRequestsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "limit",
///    "subaccount_id"
///  ],
///  "properties": {
///    "limit": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetLiveBurnRequestsRequest {
    pub limit: u64,
    pub subaccount_id: u64,
}
///`GetLiveMintRequestsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "limit",
///    "subaccount_id"
///  ],
///  "properties": {
///    "limit": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetLiveMintRequestsRequest {
    pub limit: u64,
    pub subaccount_id: u64,
}
///`GetLiveVaultRequestsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "wallet"
///  ],
///  "properties": {
///    "wallet": {
///      "description": "The shareholder wallet (0x hex).",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetLiveVaultRequestsRequest {
    ///The shareholder wallet (0x hex).
    pub wallet: ::std::string::String,
}
///`GetOnchainActionHistoryParams`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "action_type": {
///      "description": "On-chain `actionType` id.",
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "end_timestamp": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "page": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "page_size": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "start_timestamp": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "wallet": {
///      "description": "L1 sender of the action (any hex casing).",
///      "default": null,
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
pub struct GetOnchainActionHistoryParams {
    ///On-chain `actionType` id.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub action_type: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end_timestamp: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<u32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page_size: ::std::option::Option<u32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub start_timestamp: ::std::option::Option<i64>,
    ///L1 sender of the action (any hex casing).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wallet: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for GetOnchainActionHistoryParams {
    fn default() -> Self {
        Self {
            action_type: Default::default(),
            end_timestamp: Default::default(),
            page: Default::default(),
            page_size: Default::default(),
            start_timestamp: Default::default(),
            wallet: Default::default(),
        }
    }
}
///`GetOnchainActionHistoryResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "actions",
///    "pagination"
///  ],
///  "properties": {
///    "actions": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/OnchainActionHistoryEntry"
///      }
///    },
///    "pagination": {
///      "$ref": "#/definitions/Pagination"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetOnchainActionHistoryResponse {
    pub actions: ::std::vec::Vec<OnchainActionHistoryEntry>,
    pub pagination: Pagination,
}
///`GetOpenOrdersRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetOpenOrdersRequest {
    pub subaccount_id: i64,
}
///Parameters for `private/get_option_settlement_history`. One of `wallet` or `subaccount_id` must be provided; `wallet` returns every subaccount's settlements.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Parameters for `private/get_option_settlement_history`. One of `wallet` or `subaccount_id` must be provided; `wallet` returns every subaccount's settlements.",
///  "type": "object",
///  "properties": {
///    "subaccount_id": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "wallet": {
///      "default": null,
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
pub struct GetOptionSettlementHistoryParams {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subaccount_id: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wallet: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for GetOptionSettlementHistoryParams {
    fn default() -> Self {
        Self {
            subaccount_id: Default::default(),
            wallet: Default::default(),
        }
    }
}
///Parameters for `public/get_option_settlement_prices`.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Parameters for `public/get_option_settlement_prices`.",
///  "type": "object",
///  "required": [
///    "currency"
///  ],
///  "properties": {
///    "currency": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetOptionSettlementPricesRequest {
    pub currency: ::std::string::String,
}
///`GetOrderHistoryRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "from_timestamp": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "page": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "page_size": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "subaccount_id": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "to_timestamp": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "wallet": {
///      "default": null,
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
pub struct GetOrderHistoryRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub from_timestamp: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<u32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page_size: ::std::option::Option<u32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subaccount_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub to_timestamp: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wallet: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for GetOrderHistoryRequest {
    fn default() -> Self {
        Self {
            from_timestamp: Default::default(),
            page: Default::default(),
            page_size: Default::default(),
            subaccount_id: Default::default(),
            to_timestamp: Default::default(),
            wallet: Default::default(),
        }
    }
}
///`GetOrderRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "order_id",
///    "subaccount_id"
///  ],
///  "properties": {
///    "order_id": {
///      "description": "UUID v4 string",
///      "type": "string",
///      "format": "uuid"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetOrderRequest {
    ///UUID v4 string
    pub order_id: ::uuid::Uuid,
    pub subaccount_id: i64,
}
///`GetPendingDepositsParams`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "wallet"
///  ],
///  "properties": {
///    "wallet": {
///      "description": "Wallet the pending deposits credit.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetPendingDepositsParams {
    ///Wallet the pending deposits credit.
    pub wallet: ::std::string::String,
}
///`GetPendingDepositsResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "pending_deposits",
///    "wallet"
///  ],
///  "properties": {
///    "pending_deposits": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PendingDepositEntry"
///      }
///    },
///    "wallet": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetPendingDepositsResult {
    pub pending_deposits: ::std::vec::Vec<PendingDepositEntry>,
    pub wallet: ::std::string::String,
}
///`GetPositionsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetPositionsRequest {
    pub subaccount_id: u64,
}
///`GetPublicTradeHistoryRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "currency": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "from_timestamp": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "instrument_name": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "instrument_type": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "page": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "page_size": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "subaccount_id": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "to_timestamp": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "trade_id": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "tx_status": {
///      "default": null,
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
pub struct GetPublicTradeHistoryRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub currency: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub from_timestamp: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instrument_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instrument_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<u32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page_size: ::std::option::Option<u32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subaccount_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub to_timestamp: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trade_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tx_status: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for GetPublicTradeHistoryRequest {
    fn default() -> Self {
        Self {
            currency: Default::default(),
            from_timestamp: Default::default(),
            instrument_name: Default::default(),
            instrument_type: Default::default(),
            page: Default::default(),
            page_size: Default::default(),
            subaccount_id: Default::default(),
            to_timestamp: Default::default(),
            trade_id: Default::default(),
            tx_status: Default::default(),
        }
    }
}
///`GetQuotesRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "from_timestamp": {
///      "default": 0,
///      "type": "integer",
///      "format": "int64"
///    },
///    "page": {
///      "default": 1,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "page_size": {
///      "default": 20,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "quote_id": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "rfq_id": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "status": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "to_timestamp": {
///      "default": 9223372036854775807,
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetQuotesRequest {
    #[serde(default)]
    pub from_timestamp: i64,
    #[serde(default = "defaults::default_u64::<u64, 1>")]
    pub page: u64,
    #[serde(default = "defaults::default_u64::<u64, 20>")]
    pub page_size: u64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quote_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rfq_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    pub subaccount_id: i64,
    #[serde(default = "defaults::default_u64::<i64, 9223372036854775807>")]
    pub to_timestamp: i64,
}
///`GetReferralPerformanceParams`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "end_ms",
///    "start_ms"
///  ],
///  "properties": {
///    "end_ms": {
///      "description": "End timestamp in UTC milliseconds.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "referral_code": {
///      "description": "(Optional) referral code.",
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "start_ms": {
///      "description": "Start timestamp in UTC milliseconds.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "wallet": {
///      "description": "(Optional) wallet of the referrer.",
///      "default": null,
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
pub struct GetReferralPerformanceParams {
    ///End timestamp in UTC milliseconds.
    pub end_ms: i64,
    ///(Optional) referral code.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub referral_code: ::std::option::Option<::std::string::String>,
    ///Start timestamp in UTC milliseconds.
    pub start_ms: i64,
    ///(Optional) wallet of the referrer.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wallet: ::std::option::Option<::std::string::String>,
}
///`GetReferralPerformanceResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "fee_share_percentage",
///    "referral_code",
///    "rewards",
///    "stdrv_balance",
///    "total_builder_fee_collected",
///    "total_fee_rewards",
///    "total_notional_volume",
///    "total_referred_fees"
///  ],
///  "properties": {
///    "fee_share_percentage": {
///      "type": "string"
///    },
///    "referral_code": {
///      "type": "string"
///    },
///    "rewards": {
///      "description": "Performance by liquidity role / currency / instrument type.",
///      "type": "object",
///      "additionalProperties": {
///        "type": "object",
///        "additionalProperties": {
///          "type": "object",
///          "additionalProperties": {
///            "$ref": "#/definitions/ReferralPerformanceByInstrumentType"
///          }
///        }
///      }
///    },
///    "stdrv_balance": {
///      "type": "string"
///    },
///    "total_builder_fee_collected": {
///      "type": "string"
///    },
///    "total_fee_rewards": {
///      "type": "string"
///    },
///    "total_notional_volume": {
///      "type": "string"
///    },
///    "total_referred_fees": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetReferralPerformanceResult {
    pub fee_share_percentage: ::std::string::String,
    pub referral_code: ::std::string::String,
    ///Performance by liquidity role / currency / instrument type.
    pub rewards: ::std::collections::HashMap<
        ::std::string::String,
        ::std::collections::HashMap<
            ::std::string::String,
            ::std::collections::HashMap<
                ::std::string::String,
                ReferralPerformanceByInstrumentType,
            >,
        >,
    >,
    pub stdrv_balance: ::std::string::String,
    pub total_builder_fee_collected: ::std::string::String,
    pub total_fee_rewards: ::std::string::String,
    pub total_notional_volume: ::std::string::String,
    pub total_referred_fees: ::std::string::String,
}
///`GetRfqsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "from_timestamp": {
///      "default": 0,
///      "type": "integer",
///      "format": "int64"
///    },
///    "page": {
///      "default": 1,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "page_size": {
///      "default": 20,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "rfq_id": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "status": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "to_timestamp": {
///      "default": 9223372036854775807,
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetRfqsRequest {
    #[serde(default)]
    pub from_timestamp: i64,
    #[serde(default = "defaults::default_u64::<u64, 1>")]
    pub page: u64,
    #[serde(default = "defaults::default_u64::<u64, 20>")]
    pub page_size: u64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rfq_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    pub subaccount_id: i64,
    #[serde(default = "defaults::default_u64::<i64, 9223372036854775807>")]
    pub to_timestamp: i64,
}
///`GetShareholderVaultsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "wallet"
///  ],
///  "properties": {
///    "wallet": {
///      "description": "The shareholder wallet (0x hex).",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetShareholderVaultsRequest {
    ///The shareholder wallet (0x hex).
    pub wallet: ::std::string::String,
}
///`GetSubaccountRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetSubaccountRequest {
    pub subaccount_id: u64,
}
///`GetSubaccountsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "wallet"
///  ],
///  "properties": {
///    "wallet": {
///      "$ref": "#/definitions/Address"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetSubaccountsRequest {
    pub wallet: Address,
}
///`GetTickerRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "instrument_name"
///  ],
///  "properties": {
///    "instrument_name": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetTickerRequest {
    pub instrument_name: ::std::string::String,
}
///Parameters for `public/get_tickers`. `expiry_date` accepts either a JSON number or a string-encoded 8-digit date (e.g. `20260511`).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Parameters for `public/get_tickers`. `expiry_date` accepts either a JSON number or a string-encoded 8-digit date (e.g. `20260511`).",
///  "type": "object",
///  "required": [
///    "instrument_type"
///  ],
///  "properties": {
///    "currency": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "expiry_date": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "instrument_type": {
///      "$ref": "#/definitions/AssetType"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetTickersRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub currency: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub expiry_date: ::std::option::Option<i64>,
    pub instrument_type: AssetType,
}
///A map of instrument name to ticker: `{"tickers": {<instrument_name>: <ticker_data>}}`. Each ticker value matches the payload of a `ticker_slim.{instrument_name}.{interval}` subscription update.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A map of instrument name to ticker: `{\"tickers\": {<instrument_name>: <ticker_data>}}`. Each ticker value matches the payload of a `ticker_slim.{instrument_name}.{interval}` subscription update.",
///  "type": "object",
///  "required": [
///    "tickers"
///  ],
///  "properties": {
///    "tickers": {
///      "type": "object",
///      "additionalProperties": {
///        "$ref": "#/definitions/TickerSlimSnapshot"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetTickersResponse {
    pub tickers: ::std::collections::HashMap<
        ::std::string::String,
        crate::models::ticker_slim_schema::TickerSlimSchema,
    >,
}
///`GetTradeHistoryRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "from_timestamp": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "instrument_name": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "order_id": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "page": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "page_size": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "quote_id": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "subaccount_id": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "to_timestamp": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "wallet": {
///      "default": null,
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
pub struct GetTradeHistoryRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub from_timestamp: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub instrument_name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub order_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<u32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page_size: ::std::option::Option<u32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quote_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subaccount_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub to_timestamp: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wallet: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for GetTradeHistoryRequest {
    fn default() -> Self {
        Self {
            from_timestamp: Default::default(),
            instrument_name: Default::default(),
            order_id: Default::default(),
            page: Default::default(),
            page_size: Default::default(),
            quote_id: Default::default(),
            subaccount_id: Default::default(),
            to_timestamp: Default::default(),
            wallet: Default::default(),
        }
    }
}
///Parameters for `public/get_tradingview_chart_data`. Numeric fields accept either a JSON number or a string-encoded integer. Timestamps are UTC seconds.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Parameters for `public/get_tradingview_chart_data`. Numeric fields accept either a JSON number or a string-encoded integer. Timestamps are UTC seconds.",
///  "type": "object",
///  "required": [
///    "end_timestamp",
///    "instrument_name",
///    "period",
///    "start_timestamp"
///  ],
///  "properties": {
///    "end_timestamp": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "instrument_name": {
///      "type": "string"
///    },
///    "period": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "start_timestamp": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetTradingviewChartDataRequest {
    pub end_timestamp: u64,
    pub instrument_name: ::std::string::String,
    pub period: u64,
    pub start_timestamp: u64,
}
///`GetTransactionParams`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "op_uuid"
///  ],
///  "properties": {
///    "op_uuid": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetTransactionParams {
    pub op_uuid: ::std::string::String,
}
///`status` is the operation's batch status; `null` until the operation has been picked up for settlement.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "`status` is the operation's batch status; `null` until the operation has been picked up for settlement.",
///  "type": "object",
///  "required": [
///    "data"
///  ],
///  "properties": {
///    "data": {
///      "type": "string"
///    },
///    "error_log": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "status": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/BatchStatus"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "transaction_hash": {
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
pub struct GetTransactionResult {
    pub data: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error_log: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<BatchStatus>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub transaction_hash: ::std::option::Option<::std::string::String>,
}
///`GetTriggerOrdersRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetTriggerOrdersRequest {
    pub subaccount_id: i64,
}
///`GetVaultActionHistoryRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "event_type": {
///      "description": "Optional: \"vault_deposit\" | \"vault_withdraw\" | \"vault_fee_accrual\".",
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "page": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "page_size": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetVaultActionHistoryRequest {
    ///Optional: "vault_deposit" | "vault_withdraw" | "vault_fee_accrual".
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub event_type: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<u32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page_size: ::std::option::Option<u32>,
    pub subaccount_id: u64,
}
///`GetVaultPerformanceHistoryRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "resolution",
///    "subaccount_id"
///  ],
///  "properties": {
///    "from": {
///      "description": "Optional inclusive lower bound (unix seconds).",
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "limit": {
///      "description": "Max points to return, newest first (default 1000, capped at 10000).",
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "resolution": {
///      "$ref": "#/definitions/PerformanceResolution"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "to": {
///      "description": "Optional exclusive upper bound (unix seconds).",
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetVaultPerformanceHistoryRequest {
    ///Optional inclusive lower bound (unix seconds).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub from: ::std::option::Option<i64>,
    ///Max points to return, newest first (default 1000, capped at 10000).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub limit: ::std::option::Option<u64>,
    pub resolution: PerformanceResolution,
    pub subaccount_id: u64,
    ///Optional exclusive upper bound (unix seconds).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub to: ::std::option::Option<i64>,
}
///`GetVaultRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetVaultRequest {
    pub subaccount_id: u64,
}
///`GetVaultRequestHistoryRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "wallet"
///  ],
///  "properties": {
///    "page": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "page_size": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "wallet": {
///      "description": "The shareholder wallet (0x hex).",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetVaultRequestHistoryRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page: ::std::option::Option<u32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub page_size: ::std::option::Option<u32>,
    ///The shareholder wallet (0x hex).
    pub wallet: ::std::string::String,
}
///`GetVaultSharesRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "wallet"
///  ],
///  "properties": {
///    "wallet": {
///      "description": "The shareholder wallet (0x hex).",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetVaultSharesRequest {
    ///The shareholder wallet (0x hex).
    pub wallet: ::std::string::String,
}
///`GetVaultsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "page": {
///      "default": 1,
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    },
///    "page_size": {
///      "default": 100,
///      "type": "integer",
///      "format": "uint",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct GetVaultsRequest {
    #[serde(default = "defaults::default_u64::<u32, 1>")]
    pub page: u32,
    #[serde(default = "defaults::default_u64::<u32, 100>")]
    pub page_size: u32,
}
impl ::std::default::Default for GetVaultsRequest {
    fn default() -> Self {
        Self {
            page: defaults::default_u64::<u32, 1>(),
            page_size: defaults::default_u64::<u32, 100>(),
        }
    }
}
///`GetWalletsFromSessionKeyRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "public_session_key"
///  ],
///  "properties": {
///    "public_session_key": {
///      "type": "string"
///    },
///    "scope": {
///      "default": null,
///      "anyOf": [
///        {
///          "$ref": "#/definitions/OffchainKeyScope"
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
pub struct GetWalletsFromSessionKeyRequest {
    pub public_session_key: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub scope: ::std::option::Option<OffchainKeyScope>,
}
///Parameters for `private/get_withdrawal_history`. Matches `get_deposit_history`: `subaccount_id` plus an optional `[start_timestamp, end_timestamp]` window in unix milliseconds. `wallet`, when set, takes precedence over `subaccount_id` and returns the whole wallet's withdrawals. Results are capped at 1000 rows.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Parameters for `private/get_withdrawal_history`. Matches `get_deposit_history`: `subaccount_id` plus an optional `[start_timestamp, end_timestamp]` window in unix milliseconds. `wallet`, when set, takes precedence over `subaccount_id` and returns the whole wallet's withdrawals. Results are capped at 1000 rows.",
///  "type": "object",
///  "properties": {
///    "end_timestamp": {
///      "description": "End of the window, unix milliseconds (default: unbounded / now).",
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "start_timestamp": {
///      "description": "Start of the window, unix milliseconds (default 0).",
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "subaccount_id": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "wallet": {
///      "default": null,
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
pub struct GetWithdrawalHistoryRequest {
    ///End of the window, unix milliseconds (default: unbounded / now).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub end_timestamp: ::std::option::Option<u64>,
    ///Start of the window, unix milliseconds (default 0).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub start_timestamp: ::std::option::Option<u64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subaccount_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wallet: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for GetWithdrawalHistoryRequest {
    fn default() -> Self {
        Self {
            end_timestamp: Default::default(),
            start_timestamp: Default::default(),
            subaccount_id: Default::default(),
            wallet: Default::default(),
        }
    }
}
///Single spot OHLC candle in `public/get_index_chart_data`. `price` mirrors the close; `timestamp` and `timestamp_bucket` are the bucket start in UTC seconds.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Single spot OHLC candle in `public/get_index_chart_data`. `price` mirrors the close; `timestamp` and `timestamp_bucket` are the bucket start in UTC seconds.",
///  "type": "object",
///  "required": [
///    "close_price",
///    "high_price",
///    "low_price",
///    "open_price",
///    "price",
///    "timestamp",
///    "timestamp_bucket"
///  ],
///  "properties": {
///    "close_price": {
///      "type": "string"
///    },
///    "high_price": {
///      "type": "string"
///    },
///    "low_price": {
///      "type": "string"
///    },
///    "open_price": {
///      "type": "string"
///    },
///    "price": {
///      "type": "string"
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "timestamp_bucket": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct IndexCandle {
    pub close_price: ::std::string::String,
    pub high_price: ::std::string::String,
    pub low_price: ::std::string::String,
    pub open_price: ::std::string::String,
    pub price: ::std::string::String,
    pub timestamp: i64,
    pub timestamp_bucket: i64,
}
///`Instrument`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "amount_step",
///    "base_asset_address",
///    "base_asset_sub_id",
///    "base_currency",
///    "base_fee",
///    "fifo_min_allocation",
///    "instrument_name",
///    "instrument_type",
///    "is_active",
///    "maker_fee_rate",
///    "maximum_amount",
///    "minimum_amount",
///    "pro_rata_amount_step",
///    "pro_rata_fraction",
///    "quote_currency",
///    "scheduled_activation",
///    "scheduled_deactivation",
///    "taker_fee_rate",
///    "tick_size"
///  ],
///  "properties": {
///    "amount_step": {
///      "type": "string"
///    },
///    "base_asset_address": {
///      "description": "The traded asset's contract address — word 0 (`asset`) of the order action `data`. Use this (not `erc20_details`) when signing an order.",
///      "type": "string"
///    },
///    "base_asset_sub_id": {
///      "description": "The traded asset's sub-id — word 1 (`subId`) of the order action `data`. Decimal string; `\"0\"` for perps and spot.",
///      "type": "string"
///    },
///    "base_currency": {
///      "type": "string"
///    },
///    "base_fee": {
///      "type": "string"
///    },
///    "erc20_details": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/SpotPublicDetails"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "fifo_min_allocation": {
///      "type": "string"
///    },
///    "instrument_name": {
///      "type": "string"
///    },
///    "instrument_type": {
///      "$ref": "#/definitions/AssetType"
///    },
///    "is_active": {
///      "type": "boolean"
///    },
///    "maker_fee_rate": {
///      "type": "string"
///    },
///    "mark_price_fee_rate_cap": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "maximum_amount": {
///      "type": "string"
///    },
///    "minimum_amount": {
///      "type": "string"
///    },
///    "option_details": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/OptionDetails"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "perp_details": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/PerpDetails"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "pro_rata_amount_step": {
///      "type": "string"
///    },
///    "pro_rata_fraction": {
///      "type": "string"
///    },
///    "quote_currency": {
///      "type": "string"
///    },
///    "scheduled_activation": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "scheduled_deactivation": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "taker_fee_rate": {
///      "type": "string"
///    },
///    "tick_size": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Instrument {
    pub amount_step: ::std::string::String,
    ///The traded asset's contract address — word 0 (`asset`) of the order action `data`. Use this (not `erc20_details`) when signing an order.
    pub base_asset_address: ::std::string::String,
    ///The traded asset's sub-id — word 1 (`subId`) of the order action `data`. Decimal string; `"0"` for perps and spot.
    pub base_asset_sub_id: ::std::string::String,
    pub base_currency: ::std::string::String,
    pub base_fee: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub erc20_details: ::std::option::Option<SpotPublicDetails>,
    pub fifo_min_allocation: ::std::string::String,
    pub instrument_name: ::std::string::String,
    pub instrument_type: AssetType,
    pub is_active: bool,
    pub maker_fee_rate: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mark_price_fee_rate_cap: ::std::option::Option<::std::string::String>,
    pub maximum_amount: ::std::string::String,
    pub minimum_amount: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub option_details: ::std::option::Option<OptionDetails>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub perp_details: ::std::option::Option<PerpDetails>,
    pub pro_rata_amount_step: ::std::string::String,
    pub pro_rata_fraction: ::std::string::String,
    pub quote_currency: ::std::string::String,
    pub scheduled_activation: i64,
    pub scheduled_deactivation: i64,
    pub taker_fee_rate: ::std::string::String,
    pub tick_size: ::std::string::String,
}
///`InterestHistoryResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "events"
///  ],
///  "properties": {
///    "events": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/InterestPayment"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct InterestHistoryResult {
    pub events: ::std::vec::Vec<InterestPayment>,
}
///One realized interest settlement. `interest` is a decimal string; negative = paid, positive = received. `timestamp` is unix ms.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "One realized interest settlement. `interest` is a decimal string; negative = paid, positive = received. `timestamp` is unix ms.",
///  "type": "object",
///  "required": [
///    "interest",
///    "subaccount_id",
///    "timestamp"
///  ],
///  "properties": {
///    "interest": {
///      "type": "string"
///    },
///    "subaccount_id": {
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
pub struct InterestPayment {
    pub interest: ::std::string::String,
    pub subaccount_id: u64,
    pub timestamp: i64,
}
///One interest-rate candle: OHLC of the annualized borrow and supply APY plus the pool totals at the bucket close. `timestamp` is the bucket start in UTC milliseconds.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "One interest-rate candle: OHLC of the annualized borrow and supply APY plus the pool totals at the bucket close. `timestamp` is the bucket start in UTC milliseconds.",
///  "type": "object",
///  "required": [
///    "borrow_apy",
///    "risk_universe_id",
///    "supply_apy",
///    "timestamp",
///    "total_borrow",
///    "total_supply"
///  ],
///  "properties": {
///    "borrow_apy": {
///      "$ref": "#/definitions/Ohlc"
///    },
///    "risk_universe_id": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "supply_apy": {
///      "$ref": "#/definitions/Ohlc"
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "total_borrow": {
///      "type": "string"
///    },
///    "total_supply": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct InterestRateCandle {
    pub borrow_apy: Ohlc,
    pub risk_universe_id: u32,
    pub supply_apy: Ohlc,
    pub timestamp: i64,
    pub total_borrow: ::std::string::String,
    pub total_supply: ::std::string::String,
}
///Response shape for `public/get_interest_rate_history`.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Response shape for `public/get_interest_rate_history`.",
///  "type": "object",
///  "required": [
///    "interest_rate_history"
///  ],
///  "properties": {
///    "interest_rate_history": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/InterestRateCandle"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct InterestRateHistoryResult {
    pub interest_rate_history: ::std::vec::Vec<InterestRateCandle>,
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
    pub amount: ::bigdecimal::BigDecimal,
    pub direction: Direction,
    pub instrument_name: ::std::string::String,
}
///Lending pool stats for a `(spot_asset, universe)` pair. The pool's total supply is reported in the `current_open_interest` field.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Lending pool stats for a `(spot_asset, universe)` pair. The pool's total supply is reported in the `current_open_interest` field.",
///  "type": "object",
///  "required": [
///    "borrow_apy",
///    "supply_apy",
///    "total_borrow",
///    "total_borrow_cap"
///  ],
///  "properties": {
///    "borrow_apy": {
///      "type": "string"
///    },
///    "supply_apy": {
///      "type": "string"
///    },
///    "total_borrow": {
///      "type": "string"
///    },
///    "total_borrow_cap": {
///      "description": "Borrow ceiling for this lending asset in this universe.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct LendingDetails {
    pub borrow_apy: ::std::string::String,
    pub supply_apy: ::std::string::String,
    pub total_borrow: ::std::string::String,
    ///Borrow ceiling for this lending asset in this universe.
    pub total_borrow_cap: ::std::string::String,
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
///A spot asset the manager accepts as collateral, with deposit metadata and the manager's margin discounts for it ("1" = full credit, cash).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A spot asset the manager accepts as collateral, with deposit metadata and the manager's margin discounts for it (\"1\" = full credit, cash).",
///  "type": "object",
///  "required": [
///    "address",
///    "erc20",
///    "im_discount",
///    "min_deposit_usd",
///    "mm_discount",
///    "name"
///  ],
///  "properties": {
///    "address": {
///      "description": "EIP-55 checksummed protocol asset address — the `asset` argument when depositing.",
///      "type": "string"
///    },
///    "erc20": {
///      "$ref": "#/definitions/Erc20Details"
///    },
///    "im_discount": {
///      "description": "Initial-margin credit for the asset under this manager.",
///      "type": "string"
///    },
///    "min_deposit_usd": {
///      "type": "string"
///    },
///    "mm_discount": {
///      "description": "Maintenance-margin credit for the asset under this manager.",
///      "type": "string"
///    },
///    "name": {
///      "description": "Registered asset name (e.g. \"USDC\").",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ManagerCollateral {
    ///EIP-55 checksummed protocol asset address — the `asset` argument when depositing.
    pub address: ::std::string::String,
    pub erc20: Erc20Details,
    ///Initial-margin credit for the asset under this manager.
    pub im_discount: ::std::string::String,
    pub min_deposit_usd: ::std::string::String,
    ///Maintenance-margin credit for the asset under this manager.
    pub mm_discount: ::std::string::String,
    ///Registered asset name (e.g. "USDC").
    pub name: ::std::string::String,
}
///Margin model applied by a manager: `SM` (standard, cross-collateral) or `PM2` (portfolio, scenario-based netting).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Margin model applied by a manager: `SM` (standard, cross-collateral) or `PM2` (portfolio, scenario-based netting).",
///  "type": "string",
///  "enum": [
///    "SM",
///    "PM2"
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
pub enum MarginType {
    #[serde(rename = "SM")]
    Sm,
    #[serde(rename = "PM2")]
    Pm2,
}
impl ::std::fmt::Display for MarginType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Sm => f.write_str("SM"),
            Self::Pm2 => f.write_str("PM2"),
        }
    }
}
impl ::std::str::FromStr for MarginType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SM" => Ok(Self::Sm),
            "PM2" => Ok(Self::Pm2),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MarginType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MarginType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MarginType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`MarketType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "ALL",
///    "SRM_BASE_ONLY",
///    "SRM_OPTION_ONLY",
///    "SRM_PERP_ONLY",
///    "CASH"
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
pub enum MarketType {
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "SRM_BASE_ONLY")]
    SrmBaseOnly,
    #[serde(rename = "SRM_OPTION_ONLY")]
    SrmOptionOnly,
    #[serde(rename = "SRM_PERP_ONLY")]
    SrmPerpOnly,
    #[serde(rename = "CASH")]
    Cash,
}
impl ::std::fmt::Display for MarketType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::All => f.write_str("ALL"),
            Self::SrmBaseOnly => f.write_str("SRM_BASE_ONLY"),
            Self::SrmOptionOnly => f.write_str("SRM_OPTION_ONLY"),
            Self::SrmPerpOnly => f.write_str("SRM_PERP_ONLY"),
            Self::Cash => f.write_str("CASH"),
        }
    }
}
impl ::std::str::FromStr for MarketType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ALL" => Ok(Self::All),
            "SRM_BASE_ONLY" => Ok(Self::SrmBaseOnly),
            "SRM_OPTION_ONLY" => Ok(Self::SrmOptionOnly),
            "SRM_PERP_ONLY" => Ok(Self::SrmPerpOnly),
            "CASH" => Ok(Self::Cash),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MarketType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MarketType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MarketType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Fields common to every signed vault action. They appear alongside each endpoint's action-specific parameters in the same request object.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Fields common to every signed vault action. They appear alongside each endpoint's action-specific parameters in the same request object.",
///  "type": "object",
///  "required": [
///    "deposit_hash",
///    "nonce",
///    "request_id",
///    "share_price",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "subaccount_id"
///  ],
///  "properties": {
///    "deposit_hash": {
///      "description": "0x-prefixed hex of the 32-byte user deposit-action hash.",
///      "type": "string"
///    },
///    "nonce": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "request_id": {
///      "$ref": "#/definitions/VaultRequestId"
///    },
///    "share_price": {
///      "description": "Quoted share price in USD per share, as a decimal string (e.g. `\"1.02\"`).",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "signature": {
///      "description": "0x-prefixed hex of the 65-byte EOA signature.",
///      "type": "string"
///    },
///    "signature_expiry_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "signer": {
///      "$ref": "#/definitions/Address"
///    },
///    "subaccount_id": {
///      "description": "The subaccount the action is signed on; which subaccount applies depends on the specific method — see that method's reference.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct MintSharesRequest {
    ///0x-prefixed hex of the 32-byte user deposit-action hash.
    pub deposit_hash: ::std::string::String,
    pub nonce: u64,
    pub request_id: VaultRequestId,
    ///Quoted share price in USD per share, as a decimal string (e.g. `"1.02"`).
    pub share_price: ::bigdecimal::BigDecimal,
    ///0x-prefixed hex of the 65-byte EOA signature.
    pub signature: ::std::string::String,
    pub signature_expiry_sec: u64,
    pub signer: Address,
    ///The subaccount the action is signed on; which subaccount applies depends on the specific method — see that method's reference.
    pub subaccount_id: u64,
}
///`MmpConfigResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "currency",
///    "is_frozen",
///    "mmp_amount_limit",
///    "mmp_delta_limit",
///    "mmp_frozen_time",
///    "mmp_interval",
///    "mmp_unfreeze_time",
///    "subaccount_id"
///  ],
///  "properties": {
///    "currency": {
///      "type": "string"
///    },
///    "is_frozen": {
///      "type": "boolean"
///    },
///    "mmp_amount_limit": {
///      "type": "string"
///    },
///    "mmp_delta_limit": {
///      "type": "string"
///    },
///    "mmp_frozen_time": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "mmp_interval": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "mmp_unfreeze_time": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct MmpConfigResult {
    pub currency: ::std::string::String,
    pub is_frozen: bool,
    pub mmp_amount_limit: ::std::string::String,
    pub mmp_delta_limit: ::std::string::String,
    pub mmp_frozen_time: u64,
    pub mmp_interval: u64,
    pub mmp_unfreeze_time: i64,
    pub subaccount_id: u64,
}
///`MmpScopeRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "currency": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct MmpScopeRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub currency: ::std::option::Option<::std::string::String>,
    pub subaccount_id: u64,
}
///Returned by `get_live_mint_requests` / `get_live_burn_requests`: a FIFO page plus the queue's total live length (so the caller can paginate).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Returned by `get_live_mint_requests` / `get_live_burn_requests`: a FIFO page plus the queue's total live length (so the caller can paginate).",
///  "type": "object",
///  "required": [
///    "requests",
///    "total"
///  ],
///  "properties": {
///    "requests": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/VaultRequestResponse"
///      }
///    },
///    "total": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct MultipleVaultRequestsResponse {
    pub requests: ::std::vec::Vec<VaultRequestResponse>,
    pub total: u64,
}
///A generic `{ "status": "ok" }` acknowledgement returned by fire-and-forget operations that have no data to return (e.g. updating account settings, MMP config, or the active currency).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A generic `{ \"status\": \"ok\" }` acknowledgement returned by fire-and-forget operations that have no data to return (e.g. updating account settings, MMP config, or the active currency).",
///  "type": "object",
///  "required": [
///    "status"
///  ],
///  "properties": {
///    "status": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OffchainAckResponse {
    pub status: ::std::string::String,
}
///Additional permission scopes enforced off-chain (not part of the on-chain session-key scopes).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Additional permission scopes enforced off-chain (not part of the on-chain session-key scopes).",
///  "oneOf": [
///    {
///      "type": "string",
///      "enum": [
///        "account_info"
///      ]
///    },
///    {
///      "description": "Permission to delete a session key. This is an off-chain-only permission, not part of the on-chain protocol scopes.",
///      "type": "string",
///      "enum": [
///        "delete_session_key"
///      ]
///    }
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
pub enum OffchainKeyScope {
    #[serde(rename = "account_info")]
    AccountInfo,
    ///Permission to delete a session key. This is an off-chain-only permission, not part of the on-chain protocol scopes.
    #[serde(rename = "delete_session_key")]
    DeleteSessionKey,
}
impl ::std::fmt::Display for OffchainKeyScope {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::AccountInfo => f.write_str("account_info"),
            Self::DeleteSessionKey => f.write_str("delete_session_key"),
        }
    }
}
impl ::std::str::FromStr for OffchainKeyScope {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "account_info" => Ok(Self::AccountInfo),
            "delete_session_key" => Ok(Self::DeleteSessionKey),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for OffchainKeyScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for OffchainKeyScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for OffchainKeyScope {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///Open, high, low, and close for one series, each a decimal string.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Open, high, low, and close for one series, each a decimal string.",
///  "type": "object",
///  "required": [
///    "close",
///    "high",
///    "low",
///    "open"
///  ],
///  "properties": {
///    "close": {
///      "type": "string"
///    },
///    "high": {
///      "type": "string"
///    },
///    "low": {
///      "type": "string"
///    },
///    "open": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Ohlc {
    pub close: ::std::string::String,
    pub high: ::std::string::String,
    pub low: ::std::string::String,
    pub open: ::std::string::String,
}
///`OnchainActionHistoryEntry`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "acc",
///    "action_id",
///    "action_type",
///    "action_type_label",
///    "block_number",
///    "data",
///    "l1_sender",
///    "queue",
///    "status",
///    "updated_at"
///  ],
///  "properties": {
///    "acc": {
///      "description": "Rolling accumulator from the ActionQueued event, 0x-hex.",
///      "type": "string"
///    },
///    "action_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "action_type": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "action_type_label": {
///      "type": "string"
///    },
///    "block_number": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "data": {
///      "description": "Raw action calldata, 0x-hex.",
///      "type": "string"
///    },
///    "error_code": {
///      "description": "Most recent submit rejection; null if the action never failed. Kept on applied/fallback rows as the reason the action struggled. The verbose error text stays internal (ClickHouse only).",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "error_message": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "fallback_at": {
///      "description": "Unix ms the action was escalated with `fallback=true`; null if never escalated.",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "first_failed_at": {
///      "description": "Unix ms of the first failed submit; null if the action never failed.",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "l1_sender": {
///      "type": "string"
///    },
///    "last_failed_at": {
///      "description": "Unix ms of the most recent failed submit; null if never failed.",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "op_uuid": {
///      "description": "Operation uuid assigned by the sequencer; null until applied.",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "queue": {
///      "description": "Onchain queue: 'public' or 'admin'.",
///      "type": "string"
///    },
///    "status": {
///      "description": "Sequencer applied states: 'applied' | 'applied_with_fallback' Failed States: 'instant_fallback' | 'retry_then_fallback' | 'never_escalate'",
///      "type": "string"
///    },
///    "tx_hash": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "updated_at": {
///      "description": "Unix ms of the latest state change.",
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OnchainActionHistoryEntry {
    ///Rolling accumulator from the ActionQueued event, 0x-hex.
    pub acc: ::std::string::String,
    pub action_id: u64,
    pub action_type: u64,
    pub action_type_label: ::std::string::String,
    pub block_number: u64,
    ///Raw action calldata, 0x-hex.
    pub data: ::std::string::String,
    ///Most recent submit rejection; null if the action never failed. Kept on applied/fallback rows as the reason the action struggled. The verbose error text stays internal (ClickHouse only).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error_code: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error_message: ::std::option::Option<::std::string::String>,
    ///Unix ms the action was escalated with `fallback=true`; null if never escalated.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fallback_at: ::std::option::Option<i64>,
    ///Unix ms of the first failed submit; null if the action never failed.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub first_failed_at: ::std::option::Option<i64>,
    pub l1_sender: ::std::string::String,
    ///Unix ms of the most recent failed submit; null if never failed.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub last_failed_at: ::std::option::Option<i64>,
    ///Operation uuid assigned by the sequencer; null until applied.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub op_uuid: ::std::option::Option<::std::string::String>,
    ///Onchain queue: 'public' or 'admin'.
    pub queue: ::std::string::String,
    ///Sequencer applied states: 'applied' | 'applied_with_fallback' Failed States: 'instant_fallback' | 'retry_then_fallback' | 'never_escalate'
    pub status: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tx_hash: ::std::option::Option<::std::string::String>,
    ///Unix ms of the latest state change.
    pub updated_at: i64,
}
///`OpenInterestStats`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "current_open_interest",
///    "interest_cap"
///  ],
///  "properties": {
///    "current_open_interest": {
///      "type": "string"
///    },
///    "interest_cap": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OpenInterestStats {
    pub current_open_interest: ::std::string::String,
    pub interest_cap: ::std::string::String,
}
///`OptionDetails`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "expiry",
///    "index",
///    "option_type",
///    "strike"
///  ],
///  "properties": {
///    "expiry": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "index": {
///      "type": "string"
///    },
///    "option_type": {
///      "type": "string"
///    },
///    "settlement_price": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "strike": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OptionDetails {
    pub expiry: u64,
    pub index: ::std::string::String,
    pub option_type: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub settlement_price: ::std::option::Option<::std::string::String>,
    pub strike: ::std::string::String,
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
///`OptionSettlementHistoryResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "settlements"
///  ],
///  "properties": {
///    "settlements": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/OptionSettlementResponse"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OptionSettlementHistoryResponse {
    pub settlements: ::std::vec::Vec<OptionSettlementResponse>,
}
///`OptionSettlementPricesResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "expiries"
///  ],
///  "properties": {
///    "expiries": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ExpirySettlementPrice"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OptionSettlementPricesResult {
    pub expiries: ::std::vec::Vec<ExpirySettlementPrice>,
}
///One settled option position.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "One settled option position.",
///  "type": "object",
///  "required": [
///    "amount",
///    "expiry",
///    "instrument_name",
///    "settlement_price",
///    "settlement_value",
///    "subaccount_id"
///  ],
///  "properties": {
///    "amount": {
///      "description": "Settled balance (negative = short, positive = long).",
///      "type": "string"
///    },
///    "expiry": {
///      "description": "Expiry of the option, unix seconds.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "instrument_name": {
///      "type": "string"
///    },
///    "settlement_price": {
///      "type": "string"
///    },
///    "settlement_value": {
///      "description": "Crystallized cash value = intrinsic value * amount.",
///      "type": "string"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OptionSettlementResponse {
    ///Settled balance (negative = short, positive = long).
    pub amount: ::std::string::String,
    ///Expiry of the option, unix seconds.
    pub expiry: u64,
    pub instrument_name: ::std::string::String,
    pub settlement_price: ::std::string::String,
    ///Crystallized cash value = intrinsic value * amount.
    pub settlement_value: ::std::string::String,
    pub subaccount_id: u64,
}
///`OracleSignatureDataResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "signatures",
///    "signers"
///  ],
///  "properties": {
///    "signatures": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "signers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Address"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OracleSignatureDataResponse {
    pub signatures: ::std::vec::Vec<::std::string::String>,
    pub signers: ::std::vec::Vec<Address>,
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "average_price": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "filled_amount": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "max_fee": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "anyOf": [
///        {
///          "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
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
///      "anyOf": [
///        {
///          "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
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
    pub amount: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub average_price: ::bigdecimal::BigDecimal,
    #[serde(default = "defaults::order_cancel_reason")]
    pub cancel_reason: CancelReason,
    pub creation_timestamp: i64,
    pub direction: Direction,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub extra_fee: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub filled_amount: ::bigdecimal::BigDecimal,
    pub instrument_name: ::std::string::String,
    pub is_transfer: bool,
    #[serde(default)]
    pub label: ::std::string::String,
    pub last_update_timestamp: i64,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub limit_price: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub max_fee: ::bigdecimal::BigDecimal,
    pub mmp: bool,
    pub nonce: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub order_fee: ::bigdecimal::BigDecimal,
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
    pub signed_limit_price: ::std::option::Option<::bigdecimal::BigDecimal>,
    pub signer: ::std::string::String,
    pub subaccount_id: i64,
    pub time_in_force: TimeInForce,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub trigger_price: ::std::option::Option<::bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trigger_price_type: ::std::option::Option<TriggerPriceType>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trigger_reject_message: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trigger_type: ::std::option::Option<TriggerType>,
}
///`OrderActionDataResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "asset_address",
///    "asset_sub_id",
///    "desired_amount",
///    "is_bid",
///    "limit_price",
///    "recipient_id",
///    "worst_fee"
///  ],
///  "properties": {
///    "asset_address": {
///      "type": "string"
///    },
///    "asset_sub_id": {
///      "type": "string"
///    },
///    "desired_amount": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "is_bid": {
///      "type": "boolean"
///    },
///    "limit_price": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "recipient_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "worst_fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OrderActionDataResponse {
    pub asset_address: ::std::string::String,
    pub asset_sub_id: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub desired_amount: ::bigdecimal::BigDecimal,
    pub is_bid: bool,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub limit_price: ::bigdecimal::BigDecimal,
    pub recipient_id: u64,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub worst_fee: ::bigdecimal::BigDecimal,
}
///The `Action` envelope a debug route rebuilt from the request inputs, plus its decoded action data — i.e. what the signature commits to.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The `Action` envelope a debug route rebuilt from the request inputs, plus its decoded action data — i.e. what the signature commits to.",
///  "type": "object",
///  "required": [
///    "data",
///    "expiry",
///    "module",
///    "nonce",
///    "owner",
///    "signer",
///    "subaccount_id"
///  ],
///  "properties": {
///    "data": {
///      "$ref": "#/definitions/OrderActionDataResponse"
///    },
///    "expiry": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "module": {
///      "type": "string"
///    },
///    "nonce": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "owner": {
///      "type": "string"
///    },
///    "signer": {
///      "type": "string"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OrderActionInputData {
    pub data: OrderActionDataResponse,
    pub expiry: u64,
    pub module: ::std::string::String,
    pub nonce: u64,
    pub owner: ::std::string::String,
    pub signer: ::std::string::String,
    pub subaccount_id: u64,
}
///`OrderCreatedResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "order",
///    "trades"
///  ],
///  "properties": {
///    "order": {
///      "$ref": "#/definitions/Order"
///    },
///    "trades": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Trade"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OrderCreatedResponse {
    pub order: Order,
    pub trades: ::std::vec::Vec<Trade>,
}
///Debug-route payload for one rebuilt action: the EIP-712 hashes plus the action input fields.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Debug-route payload for one rebuilt action: the EIP-712 hashes plus the action input fields.",
///  "type": "object",
///  "required": [
///    "action_hash",
///    "action_typehash",
///    "domain_separator",
///    "encoded_data",
///    "encoded_data_hashed",
///    "expected_signer",
///    "input_data",
///    "module",
///    "owner",
///    "typed_data_hash"
///  ],
///  "properties": {
///    "action_hash": {
///      "description": "EIP-712 struct hash of the `Action`: `keccak256(abi.encode(action_typehash, …, encoded_data_hashed, …))`.",
///      "type": "string"
///    },
///    "action_typehash": {
///      "description": "`ACTION_TYPEHASH` — keccak of the `Action` struct type string; invariant across deployments.",
///      "type": "string"
///    },
///    "domain_separator": {
///      "description": "EIP-712 domain separator of the Matching contract for this deployment.",
///      "type": "string"
///    },
///    "encoded_data": {
///      "description": "ABI-encoded, module-specific action payload (the `data` bytes), 0x-hex.",
///      "type": "string"
///    },
///    "encoded_data_hashed": {
///      "description": "`keccak256(encoded_data)` — the value packed into the struct hash.",
///      "type": "string"
///    },
///    "expected_signer": {
///      "description": "The signer the signature is checked against.",
///      "type": "string"
///    },
///    "input_data": {
///      "description": "The rebuilt `Action` envelope and its decoded module-specific `data`.",
///      "$ref": "#/definitions/OrderActionInputData"
///    },
///    "module": {
///      "description": "Per-action module contract address bound into the signed struct.",
///      "type": "string"
///    },
///    "owner": {
///      "description": "Wallet that owns the subaccount the action applies to.",
///      "type": "string"
///    },
///    "recovered_signer": {
///      "description": "null on the debug routes (no signature is checked there); on a signature-mismatch error this is the address actually recovered.",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "typed_data_hash": {
///      "description": "Final EIP-712 digest the client signs: `keccak256(0x1901 || domain_separator || action_hash)`.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OrderDebugResponse {
    ///EIP-712 struct hash of the `Action`: `keccak256(abi.encode(action_typehash, …, encoded_data_hashed, …))`.
    pub action_hash: ::std::string::String,
    ///`ACTION_TYPEHASH` — keccak of the `Action` struct type string; invariant across deployments.
    pub action_typehash: ::std::string::String,
    ///EIP-712 domain separator of the Matching contract for this deployment.
    pub domain_separator: ::std::string::String,
    ///ABI-encoded, module-specific action payload (the `data` bytes), 0x-hex.
    pub encoded_data: ::std::string::String,
    ///`keccak256(encoded_data)` — the value packed into the struct hash.
    pub encoded_data_hashed: ::std::string::String,
    ///The signer the signature is checked against.
    pub expected_signer: ::std::string::String,
    ///The rebuilt `Action` envelope and its decoded module-specific `data`.
    pub input_data: OrderActionInputData,
    ///Per-action module contract address bound into the signed struct.
    pub module: ::std::string::String,
    ///Wallet that owns the subaccount the action applies to.
    pub owner: ::std::string::String,
    ///null on the debug routes (no signature is checked there); on a signature-mismatch error this is the address actually recovered.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub recovered_signer: ::std::option::Option<::std::string::String>,
    ///Final EIP-712 digest the client signs: `keccak256(0x1901 || domain_separator || action_hash)`.
    pub typed_data_hash: ::std::string::String,
}
///Parameters for `public/order_quote` and `private/order_quote`.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Parameters for `public/order_quote` and `private/order_quote`.",
///  "type": "object",
///  "required": [
///    "amount",
///    "direction",
///    "instrument_name",
///    "limit_price",
///    "max_fee",
///    "nonce",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "subaccount_id"
///  ],
///  "properties": {
///    "amount": {
///      "description": "Order amount in units of the base, as a decimal string (e.g. `\"1.5\"`) or a JSON number.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "client": {
///      "default": "",
///      "type": "string"
///    },
///    "direction": {
///      "description": "Order direction",
///      "$ref": "#/definitions/Direction"
///    },
///    "extra_fee": {
///      "description": "Optional extra fee per unit of volume, as a decimal string or JSON number. Defaults to zero.",
///      "default": null,
///      "anyOf": [
///        {
///          "description": "Optional extra fee per unit of volume, as a decimal string or JSON number. Defaults to zero.",
///          "default": null,
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "instrument_name": {
///      "description": "Instrument name",
///      "type": "string"
///    },
///    "is_atomic_signing": {
///      "default": false,
///      "type": "boolean"
///    },
///    "label": {
///      "default": "",
///      "type": "string"
///    },
///    "limit_price": {
///      "description": "Limit price in quote currency, as a decimal string or a JSON number.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "max_fee": {
///      "description": "Max fee per unit of volume in quote currency, as a decimal string or a JSON number.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "mmp": {
///      "default": false,
///      "type": "boolean"
///    },
///    "nonce": {
///      "description": "Nonce.",
///      "type": "string"
///    },
///    "order_type": {
///      "default": "limit",
///      "$ref": "#/definitions/OrderType"
///    },
///    "reduce_only": {
///      "default": false,
///      "type": "boolean"
///    },
///    "referral_code": {
///      "default": "",
///      "type": "string"
///    },
///    "reject_post_only": {
///      "default": true,
///      "type": "boolean"
///    },
///    "reject_timestamp": {
///      "default": 9223372036854775807,
///      "type": "integer",
///      "format": "int64"
///    },
///    "signature": {
///      "description": "Ethereum signature of the order.",
///      "type": "string"
///    },
///    "signature_expiry_sec": {
///      "description": "Signature expiry (unix seconds).",
///      "type": "integer",
///      "format": "int64"
///    },
///    "signer": {
///      "description": "Owner wallet or session key that signed the order.",
///      "type": "string"
///    },
///    "subaccount_id": {
///      "description": "Subaccount ID",
///      "type": "integer",
///      "format": "int64"
///    },
///    "time_in_force": {
///      "default": "gtc",
///      "$ref": "#/definitions/TimeInForce"
///    },
///    "trigger_price": {
///      "description": "Trigger price as a decimal string or JSON number; omit for non-trigger orders.",
///      "default": null,
///      "anyOf": [
///        {
///          "description": "Trigger price as a decimal string or JSON number; omit for non-trigger orders.",
///          "default": null,
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
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
pub struct OrderQuoteRequest {
    ///Order amount in units of the base, as a decimal string (e.g. `"1.5"`) or a JSON number.
    pub amount: ::bigdecimal::BigDecimal,
    #[serde(default)]
    pub client: ::std::string::String,
    ///Order direction
    pub direction: Direction,
    ///Optional extra fee per unit of volume, as a decimal string or JSON number. Defaults to zero.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub extra_fee: ::std::option::Option<::bigdecimal::BigDecimal>,
    ///Instrument name
    pub instrument_name: ::std::string::String,
    #[serde(default)]
    pub is_atomic_signing: bool,
    #[serde(default)]
    pub label: ::std::string::String,
    ///Limit price in quote currency, as a decimal string or a JSON number.
    pub limit_price: ::bigdecimal::BigDecimal,
    ///Max fee per unit of volume in quote currency, as a decimal string or a JSON number.
    pub max_fee: ::bigdecimal::BigDecimal,
    #[serde(default)]
    pub mmp: bool,
    ///Nonce.
    pub nonce: ::std::string::String,
    #[serde(default = "defaults::order_quote_request_order_type")]
    pub order_type: OrderType,
    #[serde(default)]
    pub reduce_only: bool,
    #[serde(default)]
    pub referral_code: ::std::string::String,
    #[serde(default = "defaults::default_bool::<true>")]
    pub reject_post_only: bool,
    #[serde(default = "defaults::default_u64::<i64, 9223372036854775807>")]
    pub reject_timestamp: i64,
    ///Ethereum signature of the order.
    pub signature: ::std::string::String,
    ///Signature expiry (unix seconds).
    pub signature_expiry_sec: i64,
    ///Owner wallet or session key that signed the order.
    pub signer: ::std::string::String,
    ///Subaccount ID
    pub subaccount_id: i64,
    #[serde(default = "defaults::order_quote_request_time_in_force")]
    pub time_in_force: TimeInForce,
    ///Trigger price as a decimal string or JSON number; omit for non-trigger orders.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trigger_price: ::std::option::Option<::bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trigger_price_type: ::std::option::Option<TriggerPriceType>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trigger_type: ::std::option::Option<TriggerType>,
}
///Estimated fill price, fees, and resulting margin for a proposed order — a non-binding dry-run. All price, fee, and margin fields are decimal strings.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Estimated fill price, fees, and resulting margin for a proposed order — a non-binding dry-run. All price, fee, and margin fields are decimal strings.",
///  "type": "object",
///  "required": [
///    "estimated_fee",
///    "estimated_fill_amount",
///    "estimated_fill_price",
///    "estimated_order_status",
///    "estimated_realized_pnl",
///    "estimated_realized_pnl_excl_fees",
///    "is_valid",
///    "post_initial_margin",
///    "pre_initial_margin",
///    "suggested_max_fee"
///  ],
///  "properties": {
///    "estimated_fee": {
///      "type": "string"
///    },
///    "estimated_fill_amount": {
///      "type": "string"
///    },
///    "estimated_fill_price": {
///      "type": "string"
///    },
///    "estimated_order_status": {
///      "$ref": "#/definitions/OrderStatus"
///    },
///    "estimated_realized_pnl": {
///      "type": "string"
///    },
///    "estimated_realized_pnl_excl_fees": {
///      "type": "string"
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
///    "max_amount": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "post_initial_margin": {
///      "type": "string"
///    },
///    "post_liquidation_price": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "pre_initial_margin": {
///      "type": "string"
///    },
///    "suggested_max_fee": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct OrderQuoteResponse {
    pub estimated_fee: ::std::string::String,
    pub estimated_fill_amount: ::std::string::String,
    pub estimated_fill_price: ::std::string::String,
    pub estimated_order_status: OrderStatus,
    pub estimated_realized_pnl: ::std::string::String,
    pub estimated_realized_pnl_excl_fees: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub invalid_reason: ::std::option::Option<::std::string::String>,
    pub is_valid: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_amount: ::std::option::Option<::std::string::String>,
    pub post_initial_margin: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub post_liquidation_price: ::std::option::Option<::std::string::String>,
    pub pre_initial_margin: ::std::string::String,
    pub suggested_max_fee: ::std::string::String,
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
///`PaginatedOrdersResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "orders",
///    "pagination",
///    "subaccount_id"
///  ],
///  "properties": {
///    "orders": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Order"
///      }
///    },
///    "pagination": {
///      "$ref": "#/definitions/Pagination"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PaginatedOrdersResult {
    pub orders: ::std::vec::Vec<Order>,
    pub pagination: Pagination,
    pub subaccount_id: i64,
}
///`PaginatedTradesResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "pagination",
///    "subaccount_id",
///    "trades"
///  ],
///  "properties": {
///    "pagination": {
///      "$ref": "#/definitions/Pagination"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "trades": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/TradeHistoryResponse"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PaginatedTradesResult {
    pub pagination: Pagination,
    pub subaccount_id: i64,
    pub trades: ::std::vec::Vec<TradeHistoryResponse>,
}
///`PaginatedVaultActionHistory`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "events",
///    "pagination",
///    "subaccount_id"
///  ],
///  "properties": {
///    "events": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PublicVaultActionResponse"
///      }
///    },
///    "pagination": {
///      "$ref": "#/definitions/Pagination"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PaginatedVaultActionHistory {
    pub events: ::std::vec::Vec<PublicVaultActionResponse>,
    pub pagination: Pagination,
    pub subaccount_id: u64,
}
///`PaginatedVaultRequestHistory`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "actions",
///    "pagination",
///    "wallet"
///  ],
///  "properties": {
///    "actions": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/VaultActionResponse"
///      }
///    },
///    "pagination": {
///      "$ref": "#/definitions/Pagination"
///    },
///    "wallet": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PaginatedVaultRequestHistory {
    pub actions: ::std::vec::Vec<VaultActionResponse>,
    pub pagination: Pagination,
    pub wallet: ::std::string::String,
}
///Response envelope for paginated RPCs.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Response envelope for paginated RPCs.",
///  "type": "object",
///  "required": [
///    "count",
///    "num_pages"
///  ],
///  "properties": {
///    "count": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "num_pages": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Pagination {
    pub count: u64,
    pub num_pages: u64,
}
///`PendingDepositEntry`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "action_id",
///    "action_type",
///    "amount",
///    "asset",
///    "block_number",
///    "deposit_type",
///    "log_index",
///    "manager_id",
///    "status",
///    "subaccount_id",
///    "timestamp",
///    "tx_hash",
///    "updated_at_ms"
///  ],
///  "properties": {
///    "action_id": {
///      "description": "Per-queue id assigned by the `OnchainActionManager`; `0` for fast deposits (indexed from the factory event, not the pooled OAM action).",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "action_type": {
///      "description": "`Deposit` (existing subaccount), `DepositToNewSubaccount`, or `FastDeposit` (pooled, credited off-chain).",
///      "type": "string"
///    },
///    "amount": {
///      "description": "Amount in the asset's native ERC-20 units, as a decimal string.",
///      "type": "string"
///    },
///    "asset": {
///      "type": "string"
///    },
///    "block_number": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "credit_nonce": {
///      "description": "Fast only: the credit transfer's nonce, disambiguating a deposit's entries. Absent on slow entries and the uncredited remainder. Decimal string — the value exceeds JS number precision.",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "deposit_type": {
///      "description": "`slow` or `fast`.",
///      "type": "string"
///    },
///    "log_index": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "manager_id": {
///      "description": "Manager of the new subaccount; `0` for existing-subaccount deposits.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "status": {
///      "description": "`detected` (transfer seen at the tip, pre-confirmation) `pending` (escrow swept, awaiting confirmation) `confirmed` (sweep tx past the confirmation range) `reverted` (the sweep reorged out — or uncreditable: no valid destination exists) Fast deposits additionally show: `crediting` (transfer in flight), `credited` (paid out) `partial_revert` (reorged out after a chunk was paid)",
///      "type": "string"
///    },
///    "subaccount_id": {
///      "description": "Credited subaccount; `0` for new-subaccount deposits.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "timestamp": {
///      "description": "Milliseconds when the tracker first picked up the deposit - approximates the block timestamp of `block_number`. Shared by all of a deposit's entries.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "tx_hash": {
///      "type": "string"
///    },
///    "updated_at_ms": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PendingDepositEntry {
    ///Per-queue id assigned by the `OnchainActionManager`; `0` for fast deposits (indexed from the factory event, not the pooled OAM action).
    pub action_id: u64,
    ///`Deposit` (existing subaccount), `DepositToNewSubaccount`, or `FastDeposit` (pooled, credited off-chain).
    pub action_type: ::std::string::String,
    ///Amount in the asset's native ERC-20 units, as a decimal string.
    pub amount: ::std::string::String,
    pub asset: ::std::string::String,
    pub block_number: u64,
    ///Fast only: the credit transfer's nonce, disambiguating a deposit's entries. Absent on slow entries and the uncredited remainder. Decimal string — the value exceeds JS number precision.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub credit_nonce: ::std::option::Option<::std::string::String>,
    ///`slow` or `fast`.
    pub deposit_type: ::std::string::String,
    pub log_index: u64,
    ///Manager of the new subaccount; `0` for existing-subaccount deposits.
    pub manager_id: u32,
    ///`detected` (transfer seen at the tip, pre-confirmation) `pending` (escrow swept, awaiting confirmation) `confirmed` (sweep tx past the confirmation range) `reverted` (the sweep reorged out — or uncreditable: no valid destination exists) Fast deposits additionally show: `crediting` (transfer in flight), `credited` (paid out) `partial_revert` (reorged out after a chunk was paid)
    pub status: ::std::string::String,
    ///Credited subaccount; `0` for new-subaccount deposits.
    pub subaccount_id: i64,
    ///Milliseconds when the tracker first picked up the deposit - approximates the block timestamp of `block_number`. Shared by all of a deposit's entries.
    pub timestamp: i64,
    pub tx_hash: ::std::string::String,
    pub updated_at_ms: i64,
}
///Quantization of the hourly samples, requested by the caller.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Quantization of the hourly samples, requested by the caller.",
///  "type": "string",
///  "enum": [
///    "1h",
///    "8h",
///    "24h",
///    "1wk"
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
pub enum PerformanceResolution {
    #[serde(rename = "1h")]
    X1h,
    #[serde(rename = "8h")]
    X8h,
    #[serde(rename = "24h")]
    X24h,
    #[serde(rename = "1wk")]
    X1wk,
}
impl ::std::fmt::Display for PerformanceResolution {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X1h => f.write_str("1h"),
            Self::X8h => f.write_str("8h"),
            Self::X24h => f.write_str("24h"),
            Self::X1wk => f.write_str("1wk"),
        }
    }
}
impl ::std::str::FromStr for PerformanceResolution {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "1h" => Ok(Self::X1h),
            "8h" => Ok(Self::X8h),
            "24h" => Ok(Self::X24h),
            "1wk" => Ok(Self::X1wk),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for PerformanceResolution {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for PerformanceResolution {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for PerformanceResolution {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`PerpDetails`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "aggregate_funding",
///    "funding_rate",
///    "index",
///    "max_rate_per_hour",
///    "min_rate_per_hour",
///    "static_interest_rate"
///  ],
///  "properties": {
///    "aggregate_funding": {
///      "type": "string"
///    },
///    "funding_rate": {
///      "type": "string"
///    },
///    "index": {
///      "type": "string"
///    },
///    "max_rate_per_hour": {
///      "type": "string"
///    },
///    "min_rate_per_hour": {
///      "type": "string"
///    },
///    "static_interest_rate": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PerpDetails {
    pub aggregate_funding: ::std::string::String,
    pub funding_rate: ::std::string::String,
    pub index: ::std::string::String,
    pub max_rate_per_hour: ::std::string::String,
    pub min_rate_per_hour: ::std::string::String,
    pub static_interest_rate: ::std::string::String,
}
///`PerpFeedDataResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "confidence",
///    "currency",
///    "deadline",
///    "signatures",
///    "spot_diff_value",
///    "timestamp",
///    "type"
///  ],
///  "properties": {
///    "confidence": {
///      "type": "string"
///    },
///    "currency": {
///      "type": "string"
///    },
///    "deadline": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "signatures": {
///      "$ref": "#/definitions/OracleSignatureDataResponse"
///    },
///    "spot_diff_value": {
///      "type": "string"
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "type": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PerpFeedDataResponse {
    pub confidence: ::std::string::String,
    pub currency: ::std::string::String,
    pub deadline: u64,
    pub signatures: OracleSignatureDataResponse,
    pub spot_diff_value: ::std::string::String,
    pub timestamp: u64,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
///`PerpSettlementEventResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "batch_status",
///    "batch_uuid",
///    "funding",
///    "instrument_name",
///    "pnl",
///    "subaccount_id",
///    "timestamp"
///  ],
///  "properties": {
///    "batch_status": {
///      "$ref": "#/definitions/BatchStatus"
///    },
///    "batch_uuid": {
///      "description": "UUID of the settlement batch; a settlement in a reverted or errored batch reflects that status.",
///      "type": "string"
///    },
///    "funding": {
///      "type": "string"
///    },
///    "instrument_name": {
///      "type": "string"
///    },
///    "pnl": {
///      "type": "string"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "timestamp": {
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
pub struct PerpSettlementEventResponse {
    pub batch_status: BatchStatus,
    ///UUID of the settlement batch; a settlement in a reverted or errored batch reflects that status.
    pub batch_uuid: ::std::string::String,
    pub funding: ::std::string::String,
    pub instrument_name: ::std::string::String,
    pub pnl: ::std::string::String,
    pub subaccount_id: u64,
    pub timestamp: i64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tx_hash: ::std::option::Option<::std::string::String>,
}
///`PerpSettlementHistoryResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "events",
///    "pagination"
///  ],
///  "properties": {
///    "events": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PerpSettlementEventResponse"
///      }
///    },
///    "pagination": {
///      "$ref": "#/definitions/Pagination"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PerpSettlementHistoryResponse {
    pub events: ::std::vec::Vec<PerpSettlementEventResponse>,
    pub pagination: Pagination,
}
///`PollQuotesRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "from_timestamp": {
///      "default": 0,
///      "type": "integer",
///      "format": "int64"
///    },
///    "page": {
///      "default": 1,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "page_size": {
///      "default": 20,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "quote_id": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "rfq_id": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "status": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "to_timestamp": {
///      "default": 9223372036854775807,
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PollQuotesRequest {
    #[serde(default)]
    pub from_timestamp: i64,
    #[serde(default = "defaults::default_u64::<u64, 1>")]
    pub page: u64,
    #[serde(default = "defaults::default_u64::<u64, 20>")]
    pub page_size: u64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quote_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rfq_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    pub subaccount_id: i64,
    #[serde(default = "defaults::default_u64::<i64, 9223372036854775807>")]
    pub to_timestamp: i64,
}
///`PollRfqsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "from_timestamp": {
///      "default": 0,
///      "type": "integer",
///      "format": "int64"
///    },
///    "page": {
///      "default": 1,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "page_size": {
///      "default": 20,
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "rfq_id": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "rfq_subaccount_id": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "status": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "to_timestamp": {
///      "default": 9223372036854775807,
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PollRfqsRequest {
    #[serde(default)]
    pub from_timestamp: i64,
    #[serde(default = "defaults::default_u64::<u64, 1>")]
    pub page: u64,
    #[serde(default = "defaults::default_u64::<u64, 20>")]
    pub page_size: u64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rfq_id: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rfq_subaccount_id: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub status: ::std::option::Option<::std::string::String>,
    pub subaccount_id: i64,
    #[serde(default = "defaults::default_u64::<i64, 9223372036854775807>")]
    pub to_timestamp: i64,
}
///`Position`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "amount",
///    "amount_step",
///    "average_price",
///    "average_price_excl_fees",
///    "creation_timestamp",
///    "cumulative_funding",
///    "delta",
///    "gamma",
///    "index_price",
///    "initial_margin",
///    "instrument_name",
///    "instrument_type",
///    "maintenance_margin",
///    "mark_price",
///    "mark_value",
///    "net_settlements",
///    "open_orders_margin",
///    "pending_funding",
///    "realized_pnl",
///    "realized_pnl_excl_fees",
///    "theta",
///    "total_fees",
///    "unrealized_pnl",
///    "unrealized_pnl_excl_fees",
///    "vega"
///  ],
///  "properties": {
///    "amount": {
///      "type": "string"
///    },
///    "amount_step": {
///      "type": "string"
///    },
///    "average_price": {
///      "type": "string"
///    },
///    "average_price_excl_fees": {
///      "type": "string"
///    },
///    "creation_timestamp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "cumulative_funding": {
///      "type": "string"
///    },
///    "delta": {
///      "type": "string"
///    },
///    "gamma": {
///      "type": "string"
///    },
///    "index_price": {
///      "type": "string"
///    },
///    "initial_margin": {
///      "type": "string"
///    },
///    "instrument_name": {
///      "type": "string"
///    },
///    "instrument_type": {
///      "type": "string"
///    },
///    "leverage": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "liquidation_price": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "maintenance_margin": {
///      "type": "string"
///    },
///    "mark_price": {
///      "type": "string"
///    },
///    "mark_value": {
///      "type": "string"
///    },
///    "net_settlements": {
///      "type": "string"
///    },
///    "open_orders_margin": {
///      "type": "string"
///    },
///    "pending_funding": {
///      "type": "string"
///    },
///    "realized_pnl": {
///      "type": "string"
///    },
///    "realized_pnl_excl_fees": {
///      "type": "string"
///    },
///    "theta": {
///      "type": "string"
///    },
///    "total_fees": {
///      "type": "string"
///    },
///    "unrealized_pnl": {
///      "type": "string"
///    },
///    "unrealized_pnl_excl_fees": {
///      "type": "string"
///    },
///    "vega": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Position {
    pub amount: ::std::string::String,
    pub amount_step: ::std::string::String,
    pub average_price: ::std::string::String,
    pub average_price_excl_fees: ::std::string::String,
    pub creation_timestamp: i64,
    pub cumulative_funding: ::std::string::String,
    pub delta: ::std::string::String,
    pub gamma: ::std::string::String,
    pub index_price: ::std::string::String,
    pub initial_margin: ::std::string::String,
    pub instrument_name: ::std::string::String,
    pub instrument_type: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub leverage: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub liquidation_price: ::std::option::Option<::std::string::String>,
    pub maintenance_margin: ::std::string::String,
    pub mark_price: ::std::string::String,
    pub mark_value: ::std::string::String,
    pub net_settlements: ::std::string::String,
    pub open_orders_margin: ::std::string::String,
    pub pending_funding: ::std::string::String,
    pub realized_pnl: ::std::string::String,
    pub realized_pnl_excl_fees: ::std::string::String,
    pub theta: ::std::string::String,
    pub total_fees: ::std::string::String,
    pub unrealized_pnl: ::std::string::String,
    pub unrealized_pnl_excl_fees: ::std::string::String,
    pub vega: ::std::string::String,
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PricedLegParamsAndResponse {
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub amount: ::bigdecimal::BigDecimal,
    pub direction: Direction,
    pub instrument_name: ::std::string::String,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub price: ::bigdecimal::BigDecimal,
}
///`PrivateChangeSubaccountLabelResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "label",
///    "subaccount_id"
///  ],
///  "properties": {
///    "label": {
///      "type": "string"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PrivateChangeSubaccountLabelResponse {
    pub label: ::std::string::String,
    pub subaccount_id: u64,
}
///`PrivateCreateSessionKeyResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "expiry_sec",
///    "ip_whitelist",
///    "offchain_scopes",
///    "protocol_scopes",
///    "public_session_key",
///    "subaccount_ids"
///  ],
///  "properties": {
///    "expiry_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "ip_whitelist": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "label": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "offchain_scopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "protocol_scopes": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "public_session_key": {
///      "type": "string"
///    },
///    "subaccount_ids": {
///      "description": "Subaccounts the key may act on. Empty/omitted request (all subaccounts) is expanded to the wallet's current subaccount list.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PrivateCreateSessionKeyResponse {
    pub expiry_sec: u64,
    pub ip_whitelist: ::std::vec::Vec<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    pub offchain_scopes: ::std::vec::Vec<::std::string::String>,
    pub protocol_scopes: ::std::vec::Vec<::std::string::String>,
    pub public_session_key: ::std::string::String,
    ///Subaccounts the key may act on. Empty/omitted request (all subaccounts) is expanded to the wallet's current subaccount list.
    pub subaccount_ids: ::std::vec::Vec<u64>,
}
///`PrivateGetAccountResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "cancel_on_disconnect",
///    "fee_info",
///    "is_rfq_maker",
///    "per_endpoint_tps",
///    "subaccount_ids",
///    "wallet",
///    "websocket_matching_tps",
///    "websocket_non_matching_tps",
///    "websocket_option_tps",
///    "websocket_perp_tps"
///  ],
///  "properties": {
///    "cancel_on_disconnect": {
///      "type": "boolean"
///    },
///    "creation_timestamp_sec": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "fee_info": {
///      "$ref": "#/definitions/AccountFeeInfo"
///    },
///    "is_rfq_maker": {
///      "type": "boolean"
///    },
///    "per_endpoint_tps": {
///      "type": "object",
///      "additionalProperties": {
///        "type": "integer",
///        "format": "uint16",
///        "minimum": 0.0
///      }
///    },
///    "referral_code": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "subaccount_ids": {
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "wallet": {
///      "type": "string"
///    },
///    "websocket_matching_tps": {
///      "type": "integer",
///      "format": "uint16",
///      "minimum": 0.0
///    },
///    "websocket_non_matching_tps": {
///      "type": "integer",
///      "format": "uint16",
///      "minimum": 0.0
///    },
///    "websocket_option_tps": {
///      "type": "integer",
///      "format": "uint16",
///      "minimum": 0.0
///    },
///    "websocket_perp_tps": {
///      "type": "integer",
///      "format": "uint16",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PrivateGetAccountResponse {
    pub cancel_on_disconnect: bool,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub creation_timestamp_sec: ::std::option::Option<u64>,
    pub fee_info: AccountFeeInfo,
    pub is_rfq_maker: bool,
    pub per_endpoint_tps: ::std::collections::HashMap<::std::string::String, u16>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub referral_code: ::std::option::Option<::std::string::String>,
    pub subaccount_ids: ::std::vec::Vec<u64>,
    pub wallet: ::std::string::String,
    pub websocket_matching_tps: u16,
    pub websocket_non_matching_tps: u16,
    pub websocket_option_tps: u16,
    pub websocket_perp_tps: u16,
}
///`PrivateGetCollateralsResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "collaterals",
///    "subaccount_id"
///  ],
///  "properties": {
///    "collaterals": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Collateral"
///      }
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PrivateGetCollateralsResponse {
    pub collaterals: ::std::vec::Vec<Collateral>,
    pub subaccount_id: u64,
}
///`PrivateGetPositionsResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "positions",
///    "subaccount_id"
///  ],
///  "properties": {
///    "positions": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Position"
///      }
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PrivateGetPositionsResponse {
    pub positions: ::std::vec::Vec<Position>,
    pub subaccount_id: u64,
}
///`PrivateGetSubaccountsResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_ids",
///    "wallet"
///  ],
///  "properties": {
///    "subaccount_ids": {
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    },
///    "wallet": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PrivateGetSubaccountsResponse {
    pub subaccount_ids: ::std::vec::Vec<u64>,
    pub wallet: ::std::string::String,
}
///`PrivateSessionKeysResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "public_session_keys"
///  ],
///  "properties": {
///    "public_session_keys": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/SessionKey"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PrivateSessionKeysResponse {
    pub public_session_keys: ::std::vec::Vec<SessionKey>,
}
///`PrivateTransferSpotExternalRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "amount",
///    "asset_name",
///    "max_fee_usd",
///    "new_subaccount_manager",
///    "nonce",
///    "recipient_address",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "sub_id",
///    "subaccount_id",
///    "to_subaccount_id"
///  ],
///  "properties": {
///    "amount": {
///      "description": "Transfer amount in units of the asset, as a decimal string (e.g. `\"1.5\"`) or JSON number.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "asset_name": {
///      "type": "string"
///    },
///    "max_fee_usd": {
///      "description": "Maximum sequencer fee the signer authorises, in USD, as a decimal string (e.g. `\"1.5\"`) or a JSON number. Must cover both the transfer fee and any subaccount-creation fee.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "new_subaccount_manager": {
///      "description": "Manager id for the new subaccount when `to_subaccount_id == 0`.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "nonce": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "recipient_address": {
///      "description": "Owner of the destination account/subaccount.",
///      "type": "string"
///    },
///    "signature": {
///      "type": "string"
///    },
///    "signature_expiry_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "signer": {
///      "type": "string"
///    },
///    "sub_id": {
///      "type": "integer",
///      "format": "uint128",
///      "minimum": 0.0
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "to_subaccount_id": {
///      "description": "Recipient's existing destination subaccount. `0` → create a new subaccount for `recipient_address` under `new_subaccount_manager`.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PrivateTransferSpotExternalRequest {
    ///Transfer amount in units of the asset, as a decimal string (e.g. `"1.5"`) or JSON number.
    pub amount: ::bigdecimal::BigDecimal,
    pub asset_name: ::std::string::String,
    ///Maximum sequencer fee the signer authorises, in USD, as a decimal string (e.g. `"1.5"`) or a JSON number. Must cover both the transfer fee and any subaccount-creation fee.
    pub max_fee_usd: ::bigdecimal::BigDecimal,
    ///Manager id for the new subaccount when `to_subaccount_id == 0`.
    pub new_subaccount_manager: u32,
    pub nonce: i64,
    ///Owner of the destination account/subaccount.
    pub recipient_address: ::std::string::String,
    pub signature: ::std::string::String,
    pub signature_expiry_sec: u64,
    pub signer: ::std::string::String,
    pub sub_id: u64,
    pub subaccount_id: u64,
    ///Recipient's existing destination subaccount. `0` → create a new subaccount for `recipient_address` under `new_subaccount_manager`.
    pub to_subaccount_id: u64,
}
///`PrivateTransferSpotExternalResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "op_uuid",
///    "operation_id"
///  ],
///  "properties": {
///    "op_uuid": {
///      "type": "string"
///    },
///    "operation_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PrivateTransferSpotExternalResponse {
    pub op_uuid: ::std::string::String,
    pub operation_id: u64,
}
///`PrivateTransferSpotRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "amount",
///    "asset_name",
///    "max_fee_usd",
///    "new_subaccount_manager",
///    "nonce",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "sub_id",
///    "subaccount_id",
///    "to_subaccount_id"
///  ],
///  "properties": {
///    "amount": {
///      "description": "Transfer amount in units of the asset, as a decimal string (e.g. `\"1.5\"`) or JSON number.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "asset_name": {
///      "type": "string"
///    },
///    "max_fee_usd": {
///      "description": "Maximum sequencer fee the signer authorises, in USD, as a decimal string (e.g. `\"1.5\"`) or a JSON number.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "new_subaccount_manager": {
///      "description": "Non-zero → create a new sender-owned subaccount under this manager id.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "nonce": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "signature": {
///      "type": "string"
///    },
///    "signature_expiry_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "signer": {
///      "type": "string"
///    },
///    "sub_id": {
///      "type": "integer",
///      "format": "uint128",
///      "minimum": 0.0
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "to_subaccount_id": {
///      "description": "Existing destination subaccount. Ignored when `new_subaccount_manager != 0`.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PrivateTransferSpotRequest {
    ///Transfer amount in units of the asset, as a decimal string (e.g. `"1.5"`) or JSON number.
    pub amount: ::bigdecimal::BigDecimal,
    pub asset_name: ::std::string::String,
    ///Maximum sequencer fee the signer authorises, in USD, as a decimal string (e.g. `"1.5"`) or a JSON number.
    pub max_fee_usd: ::bigdecimal::BigDecimal,
    ///Non-zero → create a new sender-owned subaccount under this manager id.
    pub new_subaccount_manager: u32,
    pub nonce: i64,
    pub signature: ::std::string::String,
    pub signature_expiry_sec: u64,
    pub signer: ::std::string::String,
    pub sub_id: u64,
    pub subaccount_id: u64,
    ///Existing destination subaccount. Ignored when `new_subaccount_manager != 0`.
    pub to_subaccount_id: u64,
}
///`PrivateTransferSpotResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "op_uuid",
///    "operation_id"
///  ],
///  "properties": {
///    "op_uuid": {
///      "type": "string"
///    },
///    "operation_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PrivateTransferSpotResponse {
    pub op_uuid: ::std::string::String,
    pub operation_id: u64,
}
///`PrivateWithdrawRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "amount_in_underlying",
///    "asset_name",
///    "force_batch",
///    "max_fee_usd",
///    "nonce",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "subaccount_id"
///  ],
///  "properties": {
///    "amount_in_underlying": {
///      "type": "string"
///    },
///    "asset_name": {
///      "type": "string"
///    },
///    "force_batch": {
///      "type": "boolean"
///    },
///    "max_fee_usd": {
///      "description": "Maximum sequencer fee the signer authorises, in USD, as a decimal string (e.g. `\"1.5\"`) or a JSON number.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "nonce": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "signature": {
///      "type": "string"
///    },
///    "signature_expiry_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "signer": {
///      "type": "string"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PrivateWithdrawRequest {
    pub amount_in_underlying: ::std::string::String,
    pub asset_name: ::std::string::String,
    pub force_batch: bool,
    ///Maximum sequencer fee the signer authorises, in USD, as a decimal string (e.g. `"1.5"`) or a JSON number.
    pub max_fee_usd: ::bigdecimal::BigDecimal,
    pub nonce: i64,
    pub signature: ::std::string::String,
    pub signature_expiry_sec: u64,
    pub signer: ::std::string::String,
    pub subaccount_id: u64,
}
///`PrivateWithdrawResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "op_uuid",
///    "operation_id"
///  ],
///  "properties": {
///    "op_uuid": {
///      "type": "string"
///    },
///    "operation_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PrivateWithdrawResponse {
    pub op_uuid: ::std::string::String,
    pub operation_id: u64,
}
///`ProtocolVault`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "closed",
///    "config",
///    "global_hwm",
///    "last_fee_settled_at_sec",
///    "protocol_fee_share_bps",
///    "subaccount_id",
///    "total_shares"
///  ],
///  "properties": {
///    "closed": {
///      "type": "boolean"
///    },
///    "config": {
///      "$ref": "#/definitions/VaultConfig"
///    },
///    "global_hwm": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "last_fee_settled_at_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "protocol_fee_share_bps": {
///      "type": "integer",
///      "format": "uint16",
///      "minimum": 0.0
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "total_shares": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ProtocolVault {
    pub closed: bool,
    pub config: VaultConfig,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub global_hwm: ::bigdecimal::BigDecimal,
    pub last_fee_settled_at_sec: u64,
    pub protocol_fee_share_bps: u16,
    pub subaccount_id: u64,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub total_shares: ::bigdecimal::BigDecimal,
}
///`PublicExecuteQuoteDebugRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "direction",
///    "legs",
///    "max_fee",
///    "nonce",
///    "quote_id",
///    "rfq_id",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "subaccount_id"
///  ],
///  "properties": {
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "legs": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PricedLegParamsAndResponse"
///      }
///    },
///    "max_fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "nonce": {
///      "type": "integer",
///      "format": "int64"
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
///      "$ref": "#/definitions/Address"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PublicExecuteQuoteDebugRequest {
    pub direction: Direction,
    pub legs: ::std::vec::Vec<PricedLegParamsAndResponse>,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub max_fee: ::bigdecimal::BigDecimal,
    pub nonce: i64,
    ///UUID v4 string
    pub quote_id: ::uuid::Uuid,
    ///UUID v4 string
    pub rfq_id: ::uuid::Uuid,
    pub signature: ::std::string::String,
    pub signature_expiry_sec: i64,
    pub signer: Address,
    pub subaccount_id: u64,
}
///`PublicGetWalletsFromSessionKeyResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "wallets"
///  ],
///  "properties": {
///    "wallets": {
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
pub struct PublicGetWalletsFromSessionKeyResponse {
    pub wallets: ::std::vec::Vec<::std::string::String>,
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
    pub fill_pct: ::bigdecimal::BigDecimal,
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
///      "anyOf": [
///        {
///          "description": "Lifetime taker fill rate as a decimal string; null until the wallet has enough lifetime RFQs for the rate to be meaningful.",
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "recent_fill_rate": {
///      "description": "Decayed-recent taker fill rate as a decimal string; null until enough recent RFQ activity has accumulated for the rate to be meaningful.",
///      "anyOf": [
///        {
///          "description": "Decayed-recent taker fill rate as a decimal string; null until enough recent RFQ activity has accumulated for the rate to be meaningful.",
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
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
///      "anyOf": [
///        {
///          "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
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
    pub fill_rate: ::std::option::Option<::bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub filled_direction: ::std::option::Option<Direction>,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub filled_pct: ::bigdecimal::BigDecimal,
    pub last_update_timestamp: i64,
    pub legs: ::std::vec::Vec<LegUnpricedParams>,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub partial_fill_step: ::bigdecimal::BigDecimal,
    ///Decayed-recent taker fill rate as a decimal string; null until enough recent RFQ activity has accumulated for the rate to be meaningful.
    pub recent_fill_rate: ::std::option::Option<::bigdecimal::BigDecimal>,
    ///UUID v4 string
    pub rfq_id: ::uuid::Uuid,
    pub status: RfqStatus,
    pub subaccount_id: i64,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub total_cost: ::std::option::Option<::bigdecimal::BigDecimal>,
    pub valid_until: i64,
    pub wallet: Address,
}
///`PublicSendQuoteDebugRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "direction",
///    "legs",
///    "max_fee",
///    "nonce",
///    "rfq_id",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "subaccount_id"
///  ],
///  "properties": {
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "legs": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PricedLegParamsAndResponse"
///      }
///    },
///    "max_fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "nonce": {
///      "type": "integer",
///      "format": "int64"
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
///      "$ref": "#/definitions/Address"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PublicSendQuoteDebugRequest {
    pub direction: Direction,
    pub legs: ::std::vec::Vec<PricedLegParamsAndResponse>,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub max_fee: ::bigdecimal::BigDecimal,
    pub nonce: i64,
    ///UUID v4 string
    pub rfq_id: ::uuid::Uuid,
    pub signature: ::std::string::String,
    pub signature_expiry_sec: i64,
    pub signer: Address,
    pub subaccount_id: u64,
}
///`PublicTradesResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "pagination",
///    "trades"
///  ],
///  "properties": {
///    "pagination": {
///      "$ref": "#/definitions/Pagination"
///    },
///    "trades": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/SettledTrade"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PublicTradesResult {
    pub pagination: Pagination,
    pub trades: ::std::vec::Vec<SettledTrade>,
}
///A vault action at the aggregate, vault level. Per-holder position details (entry/exit price, before/after balances) are omitted — those are available on the private endpoints. Monetary fields (share price, NAV, high-water marks) are decimal strings.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A vault action at the aggregate, vault level. Per-holder position details (entry/exit price, before/after balances) are omitted — those are available on the private endpoints. Monetary fields (share price, NAV, high-water marks) are decimal strings.",
///  "type": "object",
///  "required": [
///    "curator_shares_minted",
///    "event_ts",
///    "event_type",
///    "holder",
///    "management_shares_minted",
///    "nav",
///    "new_high_water_mark",
///    "old_high_water_mark",
///    "operation_uuid",
///    "performance_shares_minted",
///    "protocol_shares_minted",
///    "share_price",
///    "shares_delta",
///    "status",
///    "subaccount_id",
///    "total_shares"
///  ],
///  "properties": {
///    "curator_shares_minted": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "event_ts": {
///      "description": "Timestamp of this event (ms).",
///      "type": "integer",
///      "format": "int64"
///    },
///    "event_type": {
///      "description": "\"vault_deposit\" | \"vault_withdraw\" | \"vault_force_withdraw\".",
///      "type": "string"
///    },
///    "holder": {
///      "type": "string"
///    },
///    "management_shares_minted": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "nav": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "new_high_water_mark": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "old_high_water_mark": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "operation_uuid": {
///      "type": "string"
///    },
///    "performance_shares_minted": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "protocol_shares_minted": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "share_price": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "shares_delta": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "status": {
///      "description": "While the action is in the queue, one of: \"enqueued\" | \"sequencer_applied\" | \"user_cancel\" | \"curator_reject\" | \"protocol_reject\" | \"expired\".\n\nOnce the action is in the protocol, the status takes on `BatchStatus` values: \"Batching\" | \"Executing\" | \"Proving\" | \"Settling\" | \"Settled\" or an error.",
///      "type": "string"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "total_shares": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PublicVaultActionResponse {
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub curator_shares_minted: ::bigdecimal::BigDecimal,
    ///Timestamp of this event (ms).
    pub event_ts: i64,
    ///"vault_deposit" | "vault_withdraw" | "vault_force_withdraw".
    pub event_type: ::std::string::String,
    pub holder: ::std::string::String,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub management_shares_minted: ::bigdecimal::BigDecimal,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub nav: ::bigdecimal::BigDecimal,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub new_high_water_mark: ::bigdecimal::BigDecimal,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub old_high_water_mark: ::bigdecimal::BigDecimal,
    pub operation_uuid: ::std::string::String,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub performance_shares_minted: ::bigdecimal::BigDecimal,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub protocol_shares_minted: ::bigdecimal::BigDecimal,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub share_price: ::bigdecimal::BigDecimal,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub shares_delta: ::bigdecimal::BigDecimal,
    /**While the action is in the queue, one of: "enqueued" | "sequencer_applied" | "user_cancel" | "curator_reject" | "protocol_reject" | "expired".

Once the action is in the protocol, the status takes on `BatchStatus` values: "Batching" | "Executing" | "Proving" | "Settling" | "Settled" or an error.*/
    pub status: ::std::string::String,
    pub subaccount_id: u64,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub total_shares: ::bigdecimal::BigDecimal,
}
///`PublicWithdrawDebugRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "amount_in_underlying",
///    "asset_name",
///    "force_batch",
///    "max_fee_usd",
///    "nonce",
///    "signature_expiry_sec",
///    "signer",
///    "subaccount_id"
///  ],
///  "properties": {
///    "amount_in_underlying": {
///      "type": "string"
///    },
///    "asset_name": {
///      "type": "string"
///    },
///    "force_batch": {
///      "type": "boolean"
///    },
///    "max_fee_usd": {
///      "description": "Maximum sequencer fee the signer authorises, in USD, as a decimal string (e.g. `\"1.5\"`) or a JSON number.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "nonce": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "signature_expiry_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "signer": {
///      "type": "string"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct PublicWithdrawDebugRequest {
    pub amount_in_underlying: ::std::string::String,
    pub asset_name: ::std::string::String,
    pub force_batch: bool,
    ///Maximum sequencer fee the signer authorises, in USD, as a decimal string (e.g. `"1.5"`) or a JSON number.
    pub max_fee_usd: ::bigdecimal::BigDecimal,
    pub nonce: i64,
    pub signature_expiry_sec: u64,
    pub signer: ::std::string::String,
    pub subaccount_id: u64,
}
///`Quote`
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
///    "signature_expiry_sec",
///    "status",
///    "subaccount_id"
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
///    "extra_fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "fill_pct": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///    "signature_expiry_sec": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "status": {
///      "$ref": "#/definitions/RFQStatus"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "tx_hash": {
///      "description": "Blockchain transaction hash (only for executed quotes).",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "tx_status": {
///      "description": "Blockchain transaction status (only for executed quotes).",
///      "anyOf": [
///        {
///          "$ref": "#/definitions/TxStatus"
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
pub struct Quote {
    pub cancel_reason: RfqCancelReason,
    pub creation_timestamp: i64,
    pub direction: Direction,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub extra_fee: ::bigdecimal::BigDecimal,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub fee: ::bigdecimal::BigDecimal,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub fill_pct: ::bigdecimal::BigDecimal,
    pub is_transfer: bool,
    pub label: ::std::string::String,
    pub last_update_timestamp: i64,
    pub legs: ::std::vec::Vec<PricedLegParamsAndResponse>,
    pub legs_hash: ::std::string::String,
    pub liquidity_role: LiquidityRole,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub max_fee: ::bigdecimal::BigDecimal,
    pub mmp: bool,
    pub nonce: ::std::string::String,
    ///UUID v4 string
    pub quote_id: ::uuid::Uuid,
    ///UUID v4 string
    pub rfq_id: ::uuid::Uuid,
    pub signature_expiry_sec: i64,
    pub status: RfqStatus,
    pub subaccount_id: i64,
    ///Blockchain transaction hash (only for executed quotes).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tx_hash: ::std::option::Option<::std::string::String>,
    ///Blockchain transaction status (only for executed quotes).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tx_status: ::std::option::Option<TxStatus>,
}
///`QuoteExecuteDebugResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "action_hash",
///    "encoded_data",
///    "encoded_data_hashed",
///    "encoded_legs",
///    "legs_hash",
///    "typed_data_hash"
///  ],
///  "properties": {
///    "action_hash": {
///      "type": "string"
///    },
///    "encoded_data": {
///      "type": "string"
///    },
///    "encoded_data_hashed": {
///      "type": "string"
///    },
///    "encoded_legs": {
///      "type": "string"
///    },
///    "legs_hash": {
///      "type": "string"
///    },
///    "typed_data_hash": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct QuoteExecuteDebugResult {
    pub action_hash: ::std::string::String,
    pub encoded_data: ::std::string::String,
    pub encoded_data_hashed: ::std::string::String,
    pub encoded_legs: ::std::string::String,
    pub legs_hash: ::std::string::String,
    pub typed_data_hash: ::std::string::String,
}
///`QuoteExecuteResponse`
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
///    "rfq_filled_pct",
///    "rfq_id",
///    "signature_expiry_sec",
///    "status",
///    "subaccount_id"
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
///    "extra_fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "fill_pct": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///    "rfq_filled_pct": {
///      "description": "Cumulative fraction of the RFQ filled after this execution, as a decimal string (e.g. `\"0.5\"` = 50% filled).",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "rfq_id": {
///      "description": "UUID v4 string",
///      "type": "string",
///      "format": "uuid"
///    },
///    "signature_expiry_sec": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "status": {
///      "$ref": "#/definitions/RFQStatus"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct QuoteExecuteResponse {
    pub cancel_reason: RfqCancelReason,
    pub creation_timestamp: i64,
    pub direction: Direction,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub extra_fee: ::bigdecimal::BigDecimal,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub fee: ::bigdecimal::BigDecimal,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub fill_pct: ::bigdecimal::BigDecimal,
    pub is_transfer: bool,
    pub label: ::std::string::String,
    pub last_update_timestamp: i64,
    pub legs: ::std::vec::Vec<PricedLegParamsAndResponse>,
    pub legs_hash: ::std::string::String,
    pub liquidity_role: LiquidityRole,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub max_fee: ::bigdecimal::BigDecimal,
    pub mmp: bool,
    pub nonce: ::std::string::String,
    ///UUID v4 string
    pub quote_id: ::uuid::Uuid,
    ///Cumulative fraction of the RFQ filled after this execution, as a decimal string (e.g. `"0.5"` = 50% filled).
    pub rfq_filled_pct: ::bigdecimal::BigDecimal,
    ///UUID v4 string
    pub rfq_id: ::uuid::Uuid,
    pub signature_expiry_sec: i64,
    pub status: RfqStatus,
    pub subaccount_id: i64,
}
///`QuoteGetResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "pagination",
///    "quotes"
///  ],
///  "properties": {
///    "pagination": {
///      "$ref": "#/definitions/Pagination"
///    },
///    "quotes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Quote"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct QuoteGetResponse {
    pub pagination: Pagination,
    pub quotes: ::std::vec::Vec<Quote>,
}
///`QuotePollResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "pagination",
///    "quotes"
///  ],
///  "properties": {
///    "pagination": {
///      "$ref": "#/definitions/Pagination"
///    },
///    "quotes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PublicQuote"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct QuotePollResponse {
    pub pagination: Pagination,
    pub quotes: ::std::vec::Vec<PublicQuote>,
}
///`QuoteReplaceResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "cancelled_quote"
///  ],
///  "properties": {
///    "cancelled_quote": {
///      "$ref": "#/definitions/Quote"
///    },
///    "create_quote_error": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/RPCError"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "quote": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/Quote"
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
pub struct QuoteReplaceResponse {
    pub cancelled_quote: Quote,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub create_quote_error: ::std::option::Option<RpcError>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quote: ::std::option::Option<Quote>,
}
///`QuoteSendDebugResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "action_hash",
///    "encoded_data",
///    "encoded_data_hashed",
///    "typed_data_hash"
///  ],
///  "properties": {
///    "action_hash": {
///      "type": "string"
///    },
///    "encoded_data": {
///      "type": "string"
///    },
///    "encoded_data_hashed": {
///      "type": "string"
///    },
///    "typed_data_hash": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct QuoteSendDebugResult {
    pub action_hash: ::std::string::String,
    pub encoded_data: ::std::string::String,
    pub encoded_data_hashed: ::std::string::String,
    pub typed_data_hash: ::std::string::String,
}
///`RateFeedDataResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "confidence",
///    "currency",
///    "deadline",
///    "expiry",
///    "rate",
///    "signatures",
///    "timestamp"
///  ],
///  "properties": {
///    "confidence": {
///      "type": "string"
///    },
///    "currency": {
///      "type": "string"
///    },
///    "deadline": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "expiry": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "rate": {
///      "type": "string"
///    },
///    "signatures": {
///      "$ref": "#/definitions/OracleSignatureDataResponse"
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RateFeedDataResponse {
    pub confidence: ::std::string::String,
    pub currency: ::std::string::String,
    pub deadline: u64,
    pub expiry: u64,
    pub rate: ::std::string::String,
    pub signatures: OracleSignatureDataResponse,
    pub timestamp: u64,
}
///`RateLimitInfo`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "consumedPoints",
///    "isFirstInDuration",
///    "msBeforeNext",
///    "remainingPoints"
///  ],
///  "properties": {
///    "consumedPoints": {
///      "description": "Total consumed points.",
///      "type": "integer",
///      "format": "int64"
///    },
///    "isFirstInDuration": {
///      "description": "Whether it's the first request in this duration window.",
///      "type": "boolean"
///    },
///    "msBeforeNext": {
///      "description": "Milliseconds before the next request is allowed (0 if none).",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "remainingPoints": {
///      "description": "Number of remaining points (tokens).",
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RateLimitInfo {
    ///Total consumed points.
    #[serde(rename = "consumedPoints")]
    pub consumed_points: i64,
    ///Whether it's the first request in this duration window.
    #[serde(rename = "isFirstInDuration")]
    pub is_first_in_duration: bool,
    ///Milliseconds before the next request is allowed (0 if none).
    #[serde(rename = "msBeforeNext")]
    pub ms_before_next: u64,
    ///Number of remaining points (tokens).
    #[serde(rename = "remainingPoints")]
    pub remaining_points: i64,
}
///`RateLimitResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "remaining_matching",
///    "remaining_non_matching",
///    "remaining_per_endpoint"
///  ],
///  "properties": {
///    "remaining_connections": {
///      "description": "Remaining WebSocket connection allowance for this account, present only on WebSocket connections.",
///      "anyOf": [
///        {
///          "$ref": "#/definitions/RateLimitInfo"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "remaining_matching": {
///      "$ref": "#/definitions/RateLimitInfo"
///    },
///    "remaining_non_matching": {
///      "$ref": "#/definitions/RateLimitInfo"
///    },
///    "remaining_per_endpoint": {
///      "type": "object",
///      "additionalProperties": {
///        "$ref": "#/definitions/RateLimitInfo"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RateLimitResult {
    ///Remaining WebSocket connection allowance for this account, present only on WebSocket connections.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub remaining_connections: ::std::option::Option<RateLimitInfo>,
    pub remaining_matching: RateLimitInfo,
    pub remaining_non_matching: RateLimitInfo,
    pub remaining_per_endpoint: ::std::collections::HashMap<
        ::std::string::String,
        RateLimitInfo,
    >,
}
///Referral performance for one `(role, currency, instrument_type)` bucket.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Referral performance for one `(role, currency, instrument_type)` bucket.",
///  "type": "object",
///  "required": [
///    "builder_fee",
///    "fee_reward",
///    "notional_volume",
///    "referred_fee",
///    "unique_traders_referred"
///  ],
///  "properties": {
///    "builder_fee": {
///      "type": "string"
///    },
///    "fee_reward": {
///      "type": "string"
///    },
///    "notional_volume": {
///      "type": "string"
///    },
///    "referred_fee": {
///      "type": "string"
///    },
///    "unique_traders_referred": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ReferralPerformanceByInstrumentType {
    pub builder_fee: ::std::string::String,
    pub fee_reward: ::std::string::String,
    pub notional_volume: ::std::string::String,
    pub referred_fee: ::std::string::String,
    pub unique_traders_referred: u64,
}
///`Referrer`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "referral_code",
///    "wallet"
///  ],
///  "properties": {
///    "receiving_wallet": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "referral_code": {
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
pub struct Referrer {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub receiving_wallet: ::std::option::Option<::std::string::String>,
    pub referral_code: ::std::string::String,
    pub wallet: ::std::string::String,
}
///`RegisterDepositAddressParams`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "deposit_type",
///    "wallet"
///  ],
///  "properties": {
///    "deposit_type": {
///      "description": "Which factory to register against. Required: the same `(wallet, subaccount, manager)` has a distinct escrow address per factory, so callers must always be explicit about which flow they want.",
///      "$ref": "#/definitions/DepositType"
///    },
///    "manager_id": {
///      "description": "Manager the deposit routes under. Required (non-zero) when creating a new subaccount (`subaccount_id` omitted or 0); must be 0 or omitted for an existing subaccount — it salts the deposit address, so a nonzero value would mint a distinct escrow for the same destination.",
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "subaccount_id": {
///      "default": 0,
///      "type": "integer",
///      "format": "int64"
///    },
///    "wallet": {
///      "description": "Wallet address to watch for deposits.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RegisterDepositAddressParams {
    ///Which factory to register against. Required: the same `(wallet, subaccount, manager)` has a distinct escrow address per factory, so callers must always be explicit about which flow they want.
    pub deposit_type: DepositType,
    ///Manager the deposit routes under. Required (non-zero) when creating a new subaccount (`subaccount_id` omitted or 0); must be 0 or omitted for an existing subaccount — it salts the deposit address, so a nonzero value would mint a distinct escrow for the same destination.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub manager_id: ::std::option::Option<u32>,
    #[serde(default)]
    pub subaccount_id: i64,
    ///Wallet address to watch for deposits.
    pub wallet: ::std::string::String,
}
///`RegisterDepositAddressResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "deposit_address",
///    "deposit_type",
///    "wallet"
///  ],
///  "properties": {
///    "deposit_address": {
///      "type": "string"
///    },
///    "deposit_type": {
///      "$ref": "#/definitions/DepositType"
///    },
///    "manager_id": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "subaccount_id": {
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "wallet": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RegisterDepositAddressResult {
    pub deposit_address: ::std::string::String,
    pub deposit_type: DepositType,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub manager_id: ::std::option::Option<u32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub subaccount_id: ::std::option::Option<i64>,
    pub wallet: ::std::string::String,
}
///`RejectDepositRequestRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "request_id"
///  ],
///  "properties": {
///    "reason": {
///      "description": "Optional human-readable reason for the rejection.",
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "request_id": {
///      "$ref": "#/definitions/VaultRequestId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RejectDepositRequestRequest {
    ///Optional human-readable reason for the rejection.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<::std::string::String>,
    pub request_id: VaultRequestId,
}
///`private/replace` parameters: a `private/order` payload plus the cancel-target fields (`order_id_to_cancel`, `nonce_to_cancel`, `expected_filled_amount`).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "`private/replace` parameters: a `private/order` payload plus the cancel-target fields (`order_id_to_cancel`, `nonce_to_cancel`, `expected_filled_amount`).",
///  "type": "object",
///  "required": [
///    "amount",
///    "direction",
///    "instrument_name",
///    "limit_price",
///    "max_fee",
///    "nonce",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "subaccount_id"
///  ],
///  "properties": {
///    "algo_duration_sec": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int32"
///    },
///    "algo_num_slices": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int32"
///    },
///    "algo_type": {
///      "default": null,
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
///      "description": "Order amount in units of the base, as a decimal string (e.g. `\"1.5\"`) or a JSON number.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "client": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "expected_filled_amount": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "default": null,
///      "anyOf": [
///        {
///          "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "default": null,
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "extra_fee": {
///      "description": "Optional extra fee per unit of volume, as a decimal string or JSON number. Defaults to zero.",
///      "default": null,
///      "anyOf": [
///        {
///          "description": "Optional extra fee per unit of volume, as a decimal string or JSON number. Defaults to zero.",
///          "default": null,
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "instrument_name": {
///      "type": "string"
///    },
///    "is_atomic_signing": {
///      "default": null,
///      "type": [
///        "boolean",
///        "null"
///      ]
///    },
///    "label": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "limit_price": {
///      "description": "Limit price in quote currency, as a decimal string (e.g. `\"3100.5\"`) or a JSON number.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "max_fee": {
///      "description": "Max fee per unit of volume in quote currency, as a decimal string or a JSON number.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "mmp": {
///      "default": null,
///      "type": [
///        "boolean",
///        "null"
///      ]
///    },
///    "nonce": {
///      "type": "string"
///    },
///    "nonce_to_cancel": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "order_id_to_cancel": {
///      "description": "Optional UUID v4 string",
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "uuid"
///    },
///    "order_type": {
///      "default": "limit",
///      "$ref": "#/definitions/OrderType"
///    },
///    "reduce_only": {
///      "default": null,
///      "type": [
///        "boolean",
///        "null"
///      ]
///    },
///    "referral_code": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "reject_post_only": {
///      "default": null,
///      "type": [
///        "boolean",
///        "null"
///      ]
///    },
///    "reject_timestamp": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
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
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "time_in_force": {
///      "default": "gtc",
///      "$ref": "#/definitions/TimeInForce"
///    },
///    "trigger_price": {
///      "description": "Trigger price as a decimal string or JSON number; omit for non-trigger orders.",
///      "default": null,
///      "anyOf": [
///        {
///          "description": "Trigger price as a decimal string or JSON number; omit for non-trigger orders.",
///          "default": null,
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "trigger_price_type": {
///      "default": null,
///      "anyOf": [
///        {
///          "$ref": "#/definitions/TriggerPriceType"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "trigger_type": {
///      "default": null,
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
pub struct ReplaceOrderRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub algo_duration_sec: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub algo_num_slices: ::std::option::Option<i32>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub algo_type: ::std::option::Option<AlgoType>,
    ///Order amount in units of the base, as a decimal string (e.g. `"1.5"`) or a JSON number.
    pub amount: ::bigdecimal::BigDecimal,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub client: ::std::option::Option<::std::string::String>,
    pub direction: Direction,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub expected_filled_amount: ::std::option::Option<::bigdecimal::BigDecimal>,
    ///Optional extra fee per unit of volume, as a decimal string or JSON number. Defaults to zero.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub extra_fee: ::std::option::Option<::bigdecimal::BigDecimal>,
    pub instrument_name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub is_atomic_signing: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub label: ::std::option::Option<::std::string::String>,
    ///Limit price in quote currency, as a decimal string (e.g. `"3100.5"`) or a JSON number.
    pub limit_price: ::bigdecimal::BigDecimal,
    ///Max fee per unit of volume in quote currency, as a decimal string or a JSON number.
    pub max_fee: ::bigdecimal::BigDecimal,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mmp: ::std::option::Option<bool>,
    pub nonce: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nonce_to_cancel: ::std::option::Option<i64>,
    ///Optional UUID v4 string
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub order_id_to_cancel: ::std::option::Option<::uuid::Uuid>,
    #[serde(default = "defaults::replace_order_request_order_type")]
    pub order_type: OrderType,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reduce_only: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub referral_code: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reject_post_only: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reject_timestamp: ::std::option::Option<i64>,
    pub signature: ::std::string::String,
    pub signature_expiry_sec: i64,
    pub signer: ::std::string::String,
    pub subaccount_id: i64,
    #[serde(default = "defaults::replace_order_request_time_in_force")]
    pub time_in_force: TimeInForce,
    ///Trigger price as a decimal string or JSON number; omit for non-trigger orders.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trigger_price: ::std::option::Option<::bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trigger_price_type: ::std::option::Option<TriggerPriceType>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trigger_type: ::std::option::Option<TriggerType>,
}
///`ReplaceOrderResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "cancelled_order"
///  ],
///  "properties": {
///    "cancelled_order": {
///      "$ref": "#/definitions/Order"
///    },
///    "create_order_error": {
///      "default": null,
///      "anyOf": [
///        {
///          "$ref": "#/definitions/RPCError"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "order": {
///      "default": null,
///      "anyOf": [
///        {
///          "$ref": "#/definitions/Order"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "trades": {
///      "default": null,
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "$ref": "#/definitions/Trade"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ReplaceOrderResponse {
    pub cancelled_order: Order,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub create_order_error: ::std::option::Option<RpcError>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub order: ::std::option::Option<Order>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub trades: ::std::option::Option<::std::vec::Vec<Trade>>,
}
///`ReplaceQuoteRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "direction",
///    "legs",
///    "max_fee",
///    "nonce",
///    "rfq_id",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "subaccount_id"
///  ],
///  "properties": {
///    "client": {
///      "default": "",
///      "type": "string"
///    },
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "extra_fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "default": "0",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "label": {
///      "default": "",
///      "type": "string"
///    },
///    "legs": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PricedLegParamsAndResponse"
///      }
///    },
///    "max_fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "mmp": {
///      "default": false,
///      "type": "boolean"
///    },
///    "nonce": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "nonce_to_cancel": {
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "int64"
///    },
///    "quote_id_to_cancel": {
///      "description": "Optional UUID v4 string",
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "uuid"
///    },
///    "referral_code": {
///      "default": "",
///      "type": "string"
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
///      "$ref": "#/definitions/Address"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ReplaceQuoteRequest {
    #[serde(default)]
    pub client: ::std::string::String,
    pub direction: Direction,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    #[serde(default = "defaults::replace_quote_request_extra_fee")]
    pub extra_fee: ::bigdecimal::BigDecimal,
    #[serde(default)]
    pub label: ::std::string::String,
    pub legs: ::std::vec::Vec<PricedLegParamsAndResponse>,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub max_fee: ::bigdecimal::BigDecimal,
    #[serde(default)]
    pub mmp: bool,
    pub nonce: i64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nonce_to_cancel: ::std::option::Option<i64>,
    ///Optional UUID v4 string
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub quote_id_to_cancel: ::std::option::Option<::uuid::Uuid>,
    #[serde(default)]
    pub referral_code: ::std::string::String,
    ///UUID v4 string
    pub rfq_id: ::uuid::Uuid,
    pub signature: ::std::string::String,
    pub signature_expiry_sec: i64,
    pub signer: Address,
    pub subaccount_id: u64,
}
///Fields common to every signed vault action. They appear alongside each endpoint's action-specific parameters in the same request object.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Fields common to every signed vault action. They appear alongside each endpoint's action-specific parameters in the same request object.",
///  "type": "object",
///  "required": [
///    "amount",
///    "deposit_spot_asset",
///    "nonce",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "subaccount_id",
///    "vault_subaccount_id"
///  ],
///  "properties": {
///    "amount": {
///      "description": "Deposit amount in the vault's deposit asset, as a decimal string (e.g. `\"1\"`).",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "deposit_spot_asset": {
///      "$ref": "#/definitions/Address"
///    },
///    "nonce": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "signature": {
///      "description": "0x-prefixed hex of the 65-byte EOA signature.",
///      "type": "string"
///    },
///    "signature_expiry_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "signer": {
///      "$ref": "#/definitions/Address"
///    },
///    "subaccount_id": {
///      "description": "The subaccount the action is signed on; which subaccount applies depends on the specific method — see that method's reference.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "vault_subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RequestVaultDepositRequest {
    ///Deposit amount in the vault's deposit asset, as a decimal string (e.g. `"1"`).
    pub amount: ::bigdecimal::BigDecimal,
    pub deposit_spot_asset: Address,
    pub nonce: u64,
    ///0x-prefixed hex of the 65-byte EOA signature.
    pub signature: ::std::string::String,
    pub signature_expiry_sec: u64,
    pub signer: Address,
    ///The subaccount the action is signed on; which subaccount applies depends on the specific method — see that method's reference.
    pub subaccount_id: u64,
    pub vault_subaccount_id: u64,
}
///Fields common to every signed vault action. They appear alongside each endpoint's action-specific parameters in the same request object.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Fields common to every signed vault action. They appear alongside each endpoint's action-specific parameters in the same request object.",
///  "type": "object",
///  "required": [
///    "nonce",
///    "shares_to_burn",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "subaccount_id",
///    "vault_subaccount_id"
///  ],
///  "properties": {
///    "nonce": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "shares_to_burn": {
///      "description": "Number of vault shares to burn, as a decimal string.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "signature": {
///      "description": "0x-prefixed hex of the 65-byte EOA signature.",
///      "type": "string"
///    },
///    "signature_expiry_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "signer": {
///      "$ref": "#/definitions/Address"
///    },
///    "subaccount_id": {
///      "description": "The subaccount the action is signed on; which subaccount applies depends on the specific method — see that method's reference.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "vault_subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RequestVaultWithdrawRequest {
    pub nonce: u64,
    ///Number of vault shares to burn, as a decimal string.
    pub shares_to_burn: ::bigdecimal::BigDecimal,
    ///0x-prefixed hex of the 65-byte EOA signature.
    pub signature: ::std::string::String,
    pub signature_expiry_sec: u64,
    pub signer: Address,
    ///The subaccount the action is signed on; which subaccount applies depends on the specific method — see that method's reference.
    pub subaccount_id: u64,
    pub vault_subaccount_id: u64,
}
///`ResetMmpResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "ok"
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
pub enum ResetMmpResponse {
    #[serde(rename = "ok")]
    Ok,
}
impl ::std::fmt::Display for ResetMmpResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ok => f.write_str("ok"),
        }
    }
}
impl ::std::str::FromStr for ResetMmpResponse {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "ok" => Ok(Self::Ok),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ResetMmpResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ResetMmpResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ResetMmpResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`Rfq`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "ask_total_cost",
///    "bid_total_cost",
///    "cancel_reason",
///    "creation_timestamp",
///    "filled_pct",
///    "label",
///    "last_update_timestamp",
///    "legs",
///    "mark_total_cost",
///    "max_total_cost",
///    "min_total_cost",
///    "partial_fill_step",
///    "rfq_id",
///    "status",
///    "subaccount_id",
///    "total_cost",
///    "valid_until",
///    "wallet"
///  ],
///  "properties": {
///    "ask_total_cost": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "anyOf": [
///        {
///          "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "bid_total_cost": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "anyOf": [
///        {
///          "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "cancel_reason": {
///      "$ref": "#/definitions/RFQCancelReason"
///    },
///    "counterparties": {
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "$ref": "#/definitions/Address"
///      }
///    },
///    "creation_timestamp": {
///      "type": "integer",
///      "format": "int64"
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///        "$ref": "#/definitions/LegUnpricedParams"
///      }
///    },
///    "mark_total_cost": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "anyOf": [
///        {
///          "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "max_total_cost": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "anyOf": [
///        {
///          "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "min_total_cost": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "anyOf": [
///        {
///          "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "partial_fill_step": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "anyOf": [
///        {
///          "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
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
pub struct Rfq {
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub ask_total_cost: ::std::option::Option<::bigdecimal::BigDecimal>,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub bid_total_cost: ::std::option::Option<::bigdecimal::BigDecimal>,
    pub cancel_reason: RfqCancelReason,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub counterparties: ::std::option::Option<::std::vec::Vec<Address>>,
    pub creation_timestamp: i64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub filled_direction: ::std::option::Option<Direction>,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub filled_pct: ::bigdecimal::BigDecimal,
    pub label: ::std::string::String,
    pub last_update_timestamp: i64,
    pub legs: ::std::vec::Vec<LegUnpricedParams>,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub mark_total_cost: ::std::option::Option<::bigdecimal::BigDecimal>,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub max_total_cost: ::std::option::Option<::bigdecimal::BigDecimal>,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub min_total_cost: ::std::option::Option<::bigdecimal::BigDecimal>,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub partial_fill_step: ::bigdecimal::BigDecimal,
    ///UUID v4 string
    pub rfq_id: ::uuid::Uuid,
    pub status: RfqStatus,
    pub subaccount_id: i64,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub total_cost: ::std::option::Option<::bigdecimal::BigDecimal>,
    pub valid_until: i64,
    pub wallet: Address,
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
///`RfqGetBestQuoteRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "client": {
///      "default": "",
///      "type": "string"
///    },
///    "direction": {
///      "default": "buy",
///      "$ref": "#/definitions/Direction"
///    },
///    "extra_fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "default": "0",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "legs": {
///      "default": [],
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/LegUnpricedParams"
///      }
///    },
///    "rfq_id": {
///      "description": "RFQ ID to get best quote for.\n\nIf not provided, will return estimates based on mark prices.",
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ],
///      "format": "uuid"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RfqGetBestQuoteRequest {
    #[serde(default)]
    pub client: ::std::string::String,
    #[serde(default = "defaults::rfq_get_best_quote_request_direction")]
    pub direction: Direction,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    #[serde(default = "defaults::rfq_get_best_quote_request_extra_fee")]
    pub extra_fee: ::bigdecimal::BigDecimal,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub legs: ::std::vec::Vec<LegUnpricedParams>,
    /**RFQ ID to get best quote for.

If not provided, will return estimates based on mark prices.*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rfq_id: ::std::option::Option<::uuid::Uuid>,
    pub subaccount_id: u64,
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
///      "anyOf": [
///        {
///          "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "estimated_fee": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "estimated_realized_pnl": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "estimated_realized_pnl_excl_fees": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "estimated_total_cost": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "filled_pct": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "anyOf": [
///        {
///          "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "post_initial_margin": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "post_liquidation_price": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "anyOf": [
///        {
///          "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "pre_initial_margin": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "suggested_max_fee": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "up_liquidation_price": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "anyOf": [
///        {
///          "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
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
pub struct RfqGetBestQuoteResponse {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub best_quote: ::std::option::Option<PublicQuote>,
    pub direction: Direction,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub down_liquidation_price: ::std::option::Option<::bigdecimal::BigDecimal>,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub estimated_fee: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub estimated_realized_pnl: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub estimated_realized_pnl_excl_fees: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub estimated_total_cost: ::bigdecimal::BigDecimal,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub filled_pct: ::bigdecimal::BigDecimal,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub invalid_reason: ::std::option::Option<::std::string::String>,
    pub is_valid: bool,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub orderbook_total_cost: ::std::option::Option<::bigdecimal::BigDecimal>,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub post_initial_margin: ::bigdecimal::BigDecimal,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub post_liquidation_price: ::std::option::Option<::bigdecimal::BigDecimal>,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub pre_initial_margin: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub suggested_max_fee: ::bigdecimal::BigDecimal,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub up_liquidation_price: ::std::option::Option<::bigdecimal::BigDecimal>,
}
///`RfqGetResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "pagination",
///    "rfqs"
///  ],
///  "properties": {
///    "pagination": {
///      "$ref": "#/definitions/Pagination"
///    },
///    "rfqs": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Rfq"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RfqGetResponse {
    pub pagination: Pagination,
    pub rfqs: ::std::vec::Vec<Rfq>,
}
///`RfqPollResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "pagination",
///    "rfqs"
///  ],
///  "properties": {
///    "pagination": {
///      "$ref": "#/definitions/Pagination"
///    },
///    "rfqs": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PublicRfq"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RfqPollResponse {
    pub pagination: Pagination,
    pub rfqs: ::std::vec::Vec<PublicRfq>,
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
///One risk universe: its display metadata, managers (each with what it lets you trade and deposit), and the Security Module absorbing the universe's losses. The universe-first mirror of `public/get_all_currencies` — pick a universe, pick a manager in it, and the `manager_id` is what you pass when depositing to a new subaccount.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "One risk universe: its display metadata, managers (each with what it lets you trade and deposit), and the Security Module absorbing the universe's losses. The universe-first mirror of `public/get_all_currencies` — pick a universe, pick a manager in it, and the `manager_id` is what you pass when depositing to a new subaccount.",
///  "type": "object",
///  "required": [
///    "managers",
///    "risk_universe_id",
///    "security_module"
///  ],
///  "properties": {
///    "description": {
///      "description": "Short description of the universe; null until set by the exchange.",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "managers": {
///      "description": "The universe's managers (at most one Standard and one Portfolio), each with the instruments it risk-prices and the collaterals it accepts.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/RiskUniverseManager"
///      }
///    },
///    "name": {
///      "description": "Display name (e.g. \"PRIME\"), always uppercase; null until set by the exchange.",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "risk_universe_id": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "security_module": {
///      "$ref": "#/definitions/SecurityModuleDetails"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RiskUniverse {
    ///Short description of the universe; null until set by the exchange.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    ///The universe's managers (at most one Standard and one Portfolio), each with the instruments it risk-prices and the collaterals it accepts.
    pub managers: ::std::vec::Vec<RiskUniverseManager>,
    ///Display name (e.g. "PRIME"), always uppercase; null until set by the exchange.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    pub risk_universe_id: u32,
    pub security_module: SecurityModuleDetails,
}
///A manager and what a subaccount created under it can trade and hold.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A manager and what a subaccount created under it can trade and hold.",
///  "type": "object",
///  "required": [
///    "collaterals",
///    "instruments",
///    "manager_id",
///    "margin_type"
///  ],
///  "properties": {
///    "collaterals": {
///      "description": "Assets this manager accepts as deposit collateral.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/ManagerCollateral"
///      }
///    },
///    "instruments": {
///      "description": "Live derivative asset names tradeable under this manager, sorted (e.g. \"BTC-OPTION\", \"BTC-PERP\"). An option entry names the family — fetch its individual listings via `public/get_all_instruments`.",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "manager_id": {
///      "description": "Pass this as `manager_id` when depositing to a new subaccount.",
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "margin_type": {
///      "$ref": "#/definitions/MarginType"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct RiskUniverseManager {
    ///Assets this manager accepts as deposit collateral.
    pub collaterals: ::std::vec::Vec<ManagerCollateral>,
    ///Live derivative asset names tradeable under this manager, sorted (e.g. "BTC-OPTION", "BTC-PERP"). An option entry names the family — fetch its individual listings via `public/get_all_instruments`.
    pub instruments: ::std::vec::Vec<::std::string::String>,
    ///Pass this as `manager_id` when depositing to a new subaccount.
    pub manager_id: u32,
    pub margin_type: MarginType,
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
///The universe's Security Module: the subaccount whose funds absorb insolvent losses before any socialization within the universe.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The universe's Security Module: the subaccount whose funds absorb insolvent losses before any socialization within the universe.",
///  "type": "object",
///  "required": [
///    "cash_asset",
///    "cash_currency",
///    "subaccount_id"
///  ],
///  "properties": {
///    "cash_asset": {
///      "description": "EIP-55 checksummed address of the universe's cash asset.",
///      "type": "string"
///    },
///    "cash_currency": {
///      "description": "Currency of the cash asset (e.g. \"USDC\").",
///      "type": "string"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SecurityModuleDetails {
    ///EIP-55 checksummed address of the universe's cash asset.
    pub cash_asset: ::std::string::String,
    ///Currency of the cash asset (e.g. "USDC").
    pub cash_currency: ::std::string::String,
    pub subaccount_id: u64,
}
///`SendQuoteRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "direction",
///    "legs",
///    "max_fee",
///    "nonce",
///    "rfq_id",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "subaccount_id"
///  ],
///  "properties": {
///    "client": {
///      "default": "",
///      "type": "string"
///    },
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "extra_fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "default": "0",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "label": {
///      "default": "",
///      "type": "string"
///    },
///    "legs": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PricedLegParamsAndResponse"
///      }
///    },
///    "max_fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "mmp": {
///      "default": false,
///      "type": "boolean"
///    },
///    "nonce": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "referral_code": {
///      "default": "",
///      "type": "string"
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
///      "$ref": "#/definitions/Address"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SendQuoteRequest {
    #[serde(default)]
    pub client: ::std::string::String,
    pub direction: Direction,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    #[serde(default = "defaults::send_quote_request_extra_fee")]
    pub extra_fee: ::bigdecimal::BigDecimal,
    #[serde(default)]
    pub label: ::std::string::String,
    pub legs: ::std::vec::Vec<PricedLegParamsAndResponse>,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub max_fee: ::bigdecimal::BigDecimal,
    #[serde(default)]
    pub mmp: bool,
    pub nonce: i64,
    #[serde(default)]
    pub referral_code: ::std::string::String,
    ///UUID v4 string
    pub rfq_id: ::uuid::Uuid,
    pub signature: ::std::string::String,
    pub signature_expiry_sec: i64,
    pub signer: Address,
    pub subaccount_id: u64,
}
///`SendRfqRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "legs",
///    "subaccount_id"
///  ],
///  "properties": {
///    "client": {
///      "default": "",
///      "type": "string"
///    },
///    "counterparties": {
///      "default": null,
///      "type": [
///        "array",
///        "null"
///      ],
///      "items": {
///        "type": "string"
///      }
///    },
///    "extra_fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "default": "0",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "label": {
///      "default": "",
///      "type": "string"
///    },
///    "legs": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/LegUnpricedParams"
///      }
///    },
///    "max_total_cost": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "default": null,
///      "anyOf": [
///        {
///          "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "default": null,
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "min_total_cost": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "default": null,
///      "anyOf": [
///        {
///          "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "default": null,
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "partial_fill_step": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "default": "1",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "referral_code": {
///      "default": "",
///      "type": "string"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SendRfqRequest {
    #[serde(default)]
    pub client: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub counterparties: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    #[serde(default = "defaults::send_rfq_request_extra_fee")]
    pub extra_fee: ::bigdecimal::BigDecimal,
    #[serde(default)]
    pub label: ::std::string::String,
    pub legs: ::std::vec::Vec<LegUnpricedParams>,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_total_cost: ::std::option::Option<::bigdecimal::BigDecimal>,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub min_total_cost: ::std::option::Option<::bigdecimal::BigDecimal>,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    #[serde(default = "defaults::send_rfq_request_partial_fill_step")]
    pub partial_fill_step: ::bigdecimal::BigDecimal,
    #[serde(default)]
    pub referral_code: ::std::string::String,
    pub subaccount_id: u64,
}
///`SessionKey`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "expiry_sec",
///    "ip_whitelist",
///    "label",
///    "offchain_scopes",
///    "protocol_scopes",
///    "public_session_key",
///    "registered_sec",
///    "subaccount_ids"
///  ],
///  "properties": {
///    "expiry_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "ip_whitelist": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "label": {
///      "type": "string"
///    },
///    "offchain_scopes": {
///      "description": "Off-chain scopes (read-tier capabilities).",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "protocol_scopes": {
///      "description": "Protocol-level (on-chain authority) scopes.",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "public_session_key": {
///      "type": "string"
///    },
///    "registered_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "subaccount_ids": {
///      "description": "Subaccounts this key may act on. An empty stored value means \"all subaccounts\" and is expanded to the wallet's current subaccount list.",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SessionKey {
    pub expiry_sec: u64,
    pub ip_whitelist: ::std::vec::Vec<::std::string::String>,
    pub label: ::std::string::String,
    ///Off-chain scopes (read-tier capabilities).
    pub offchain_scopes: ::std::vec::Vec<::std::string::String>,
    ///Protocol-level (on-chain authority) scopes.
    pub protocol_scopes: ::std::vec::Vec<::std::string::String>,
    pub public_session_key: ::std::string::String,
    pub registered_sec: u64,
    ///Subaccounts this key may act on. An empty stored value means "all subaccounts" and is expanded to the wallet's current subaccount list.
    pub subaccount_ids: ::std::vec::Vec<u64>,
}
///`SessionKeysRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "wallet"
///  ],
///  "properties": {
///    "wallet": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SessionKeysRequest {
    pub wallet: ::std::string::String,
}
///`mmp_interval` and `mmp_frozen_time` are specified in milliseconds (rounded down to the nearest whole second).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "`mmp_interval` and `mmp_frozen_time` are specified in milliseconds (rounded down to the nearest whole second).",
///  "type": "object",
///  "required": [
///    "currency",
///    "mmp_frozen_time",
///    "mmp_interval",
///    "subaccount_id"
///  ],
///  "properties": {
///    "currency": {
///      "type": "string"
///    },
///    "mmp_amount_limit": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "default": "0",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "mmp_delta_limit": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "default": "0",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "mmp_frozen_time": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "mmp_interval": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SetMmpConfigRequest {
    pub currency: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    #[serde(default = "defaults::set_mmp_config_request_mmp_amount_limit")]
    pub mmp_amount_limit: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    #[serde(default = "defaults::set_mmp_config_request_mmp_delta_limit")]
    pub mmp_delta_limit: ::bigdecimal::BigDecimal,
    pub mmp_frozen_time: u64,
    pub mmp_interval: u64,
    pub subaccount_id: u64,
}
///Echoes the inbound params back: decimal-string limits, ms intervals.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Echoes the inbound params back: decimal-string limits, ms intervals.",
///  "type": "object",
///  "required": [
///    "currency",
///    "mmp_amount_limit",
///    "mmp_delta_limit",
///    "mmp_frozen_time",
///    "mmp_interval",
///    "subaccount_id"
///  ],
///  "properties": {
///    "currency": {
///      "type": "string"
///    },
///    "mmp_amount_limit": {
///      "type": "string"
///    },
///    "mmp_delta_limit": {
///      "type": "string"
///    },
///    "mmp_frozen_time": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "mmp_interval": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SetMmpConfigResponse {
    pub currency: ::std::string::String,
    pub mmp_amount_limit: ::std::string::String,
    pub mmp_delta_limit: ::std::string::String,
    pub mmp_frozen_time: u64,
    pub mmp_interval: u64,
    pub subaccount_id: u64,
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
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "expected_rebate": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "extra_fee": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "index_price": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "realized_pnl_excl_fees": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "trade_fee": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "trade_id": {
///      "type": "string"
///    },
///    "trade_price": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "tx_hash": {
///      "type": "string"
///    },
///    "tx_status": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/BatchStatus"
///        },
///        {
///          "type": "null"
///        }
///      ]
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
    pub direction: Direction,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub expected_rebate: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub extra_fee: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub index_price: ::bigdecimal::BigDecimal,
    pub instrument_name: ::std::string::String,
    pub liquidity_role: LiquidityRole,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub mark_price: ::bigdecimal::BigDecimal,
    ///Optional UUID v4 string
    pub quote_id: ::std::option::Option<::uuid::Uuid>,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub realized_pnl: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub realized_pnl_excl_fees: ::bigdecimal::BigDecimal,
    ///Optional UUID v4 string
    pub rfq_id: ::std::option::Option<::uuid::Uuid>,
    pub subaccount_id: i64,
    pub timestamp: i64,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub trade_amount: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub trade_fee: ::bigdecimal::BigDecimal,
    pub trade_id: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub trade_price: ::bigdecimal::BigDecimal,
    pub tx_hash: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tx_status: ::std::option::Option<BatchStatus>,
    pub wallet: ::std::string::String,
}
///An Action bundled with its ECDSA signature (65 bytes: r||s||v).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "An Action bundled with its ECDSA signature (65 bytes: r||s||v).",
///  "type": "object",
///  "required": [
///    "action",
///    "signature"
///  ],
///  "properties": {
///    "action": {
///      "description": "The signed action envelope.",
///      "$ref": "#/definitions/Action"
///    },
///    "signature": {
///      "description": "65-byte ECDSA signature (r||s||v).",
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint8",
///        "minimum": 0.0
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SignedAction {
    ///The signed action envelope.
    pub action: Action,
    ///65-byte ECDSA signature (r||s||v).
    pub signature: ::std::vec::Vec<u8>,
}
///Action info that needs to be signed by quoter or RFQ executor for a transfer. The quoter signs the legs hash + max fee; the executor signs the same legs hash + their own max fee.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Action info that needs to be signed by quoter or RFQ executor for a transfer. The quoter signs the legs hash + max fee; the executor signs the same legs hash + their own max fee.",
///  "type": "object",
///  "required": [
///    "direction",
///    "legs",
///    "max_fee",
///    "nonce",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "subaccount_id"
///  ],
///  "properties": {
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "legs": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/PricedLegParamsAndResponse"
///      }
///    },
///    "max_fee": {
///      "description": "Maximum fee the signer authorises, in USD, as a decimal string (e.g. `\"1.5\"`) or a JSON number.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "nonce": {
///      "type": "string"
///    },
///    "signature": {
///      "type": "string"
///    },
///    "signature_expiry_sec": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "signer": {
///      "$ref": "#/definitions/Address"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SignedTransferQuoteRequest {
    pub direction: Direction,
    pub legs: ::std::vec::Vec<PricedLegParamsAndResponse>,
    ///Maximum fee the signer authorises, in USD, as a decimal string (e.g. `"1.5"`) or a JSON number.
    pub max_fee: ::bigdecimal::BigDecimal,
    pub nonce: ::std::string::String,
    pub signature: ::std::string::String,
    pub signature_expiry_sec: i64,
    pub signer: Address,
    pub subaccount_id: i64,
}
///A spot asset and its per-universe risk: collateral discounts, lending, OI.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A spot asset and its per-universe risk: collateral discounts, lending, OI.",
///  "type": "object",
///  "required": [
///    "address",
///    "erc20",
///    "min_deposit_usd",
///    "name",
///    "universes"
///  ],
///  "properties": {
///    "address": {
///      "description": "EIP-55 checksummed protocol asset address.",
///      "type": "string"
///    },
///    "erc20": {
///      "$ref": "#/definitions/Erc20Details"
///    },
///    "min_deposit_usd": {
///      "type": "string"
///    },
///    "name": {
///      "description": "Registered asset name (e.g. \"USDC\", \"USDC-NL\").",
///      "type": "string"
///    },
///    "universes": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/SpotUniverse"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SpotAssetEntry {
    ///EIP-55 checksummed protocol asset address.
    pub address: ::std::string::String,
    pub erc20: Erc20Details,
    pub min_deposit_usd: ::std::string::String,
    ///Registered asset name (e.g. "USDC", "USDC-NL").
    pub name: ::std::string::String,
    pub universes: ::std::vec::Vec<SpotUniverse>,
}
///`SpotFeedDataResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "confidence",
///    "currency",
///    "deadline",
///    "price",
///    "signatures",
///    "timestamp"
///  ],
///  "properties": {
///    "confidence": {
///      "type": "string"
///    },
///    "currency": {
///      "type": "string"
///    },
///    "deadline": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "feed_source_type": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "price": {
///      "type": "string"
///    },
///    "signatures": {
///      "$ref": "#/definitions/OracleSignatureDataResponse"
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SpotFeedDataResponse {
    pub confidence: ::std::string::String,
    pub currency: ::std::string::String,
    pub deadline: u64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub feed_source_type: ::std::option::Option<::std::string::String>,
    pub price: ::std::string::String,
    pub signatures: OracleSignatureDataResponse,
    pub timestamp: u64,
}
///`SpotPublicDetails`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "borrow_index",
///    "decimals",
///    "supply_index",
///    "underlying_erc20_address"
///  ],
///  "properties": {
///    "borrow_index": {
///      "type": "string"
///    },
///    "decimals": {
///      "type": "integer",
///      "format": "uint8",
///      "minimum": 0.0
///    },
///    "supply_index": {
///      "type": "string"
///    },
///    "underlying_erc20_address": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SpotPublicDetails {
    pub borrow_index: ::std::string::String,
    pub decimals: u8,
    pub supply_index: ::std::string::String,
    pub underlying_erc20_address: ::std::string::String,
}
///Risk details for a single `(spot_asset, universe)` pair.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Risk details for a single `(spot_asset, universe)` pair.",
///  "type": "object",
///  "required": [
///    "oi",
///    "pm2_im_discount",
///    "pm2_mm_discount",
///    "risk_universe_id",
///    "srm_im_discount",
///    "srm_mm_discount"
///  ],
///  "properties": {
///    "lending": {
///      "description": "Lending stats; null if the asset has no pool in this universe.",
///      "anyOf": [
///        {
///          "$ref": "#/definitions/LendingDetails"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "oi": {
///      "description": "Spot supply OI cap + current supply (for a lending asset, the current supply is the pool's total supplied collateral).",
///      "$ref": "#/definitions/OpenInterestStats"
///    },
///    "pm2_im_discount": {
///      "description": "PMRM2 collateral haircut (\"0\" if not collateralized).",
///      "type": "string"
///    },
///    "pm2_mm_discount": {
///      "type": "string"
///    },
///    "risk_universe_id": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "srm_im_discount": {
///      "description": "SRM collateral discount (cash asset \"1\"; non-collateral \"0\").",
///      "type": "string"
///    },
///    "srm_mm_discount": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SpotUniverse {
    ///Lending stats; null if the asset has no pool in this universe.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub lending: ::std::option::Option<LendingDetails>,
    ///Spot supply OI cap + current supply (for a lending asset, the current supply is the pool's total supplied collateral).
    pub oi: OpenInterestStats,
    ///PMRM2 collateral haircut ("0" if not collateralized).
    pub pm2_im_discount: ::std::string::String,
    pub pm2_mm_discount: ::std::string::String,
    pub risk_universe_id: u32,
    ///SRM collateral discount (cash asset "1"; non-collateral "0").
    pub srm_im_discount: ::std::string::String,
    pub srm_mm_discount: ::std::string::String,
}
///`Subaccount`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "collaterals",
///    "collaterals_initial_margin",
///    "collaterals_maintenance_margin",
///    "collaterals_value",
///    "currency",
///    "failed_to_fetch",
///    "initial_margin",
///    "is_under_liquidation",
///    "label",
///    "maintenance_margin",
///    "manager_id",
///    "margin_type",
///    "open_orders",
///    "open_orders_margin",
///    "positions",
///    "positions_initial_margin",
///    "positions_maintenance_margin",
///    "positions_value",
///    "projected_margin_change",
///    "risk_universe_id",
///    "subaccount_id",
///    "subaccount_value",
///    "vault_deposit_holds"
///  ],
///  "properties": {
///    "collaterals": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Collateral"
///      }
///    },
///    "collaterals_initial_margin": {
///      "type": "string"
///    },
///    "collaterals_maintenance_margin": {
///      "type": "string"
///    },
///    "collaterals_value": {
///      "type": "string"
///    },
///    "currency": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "failed_to_fetch": {
///      "description": "`true` if this subaccount's portfolio could not be loaded; such subaccounts are returned as placeholders by `get_all_portfolios`.",
///      "type": "boolean"
///    },
///    "initial_margin": {
///      "type": "string"
///    },
///    "is_under_liquidation": {
///      "type": "boolean"
///    },
///    "label": {
///      "type": "string"
///    },
///    "maintenance_margin": {
///      "type": "string"
///    },
///    "manager_id": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "margin_type": {
///      "type": "string"
///    },
///    "open_orders": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Order"
///      }
///    },
///    "open_orders_margin": {
///      "type": "string"
///    },
///    "positions": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Position"
///      }
///    },
///    "positions_initial_margin": {
///      "type": "string"
///    },
///    "positions_maintenance_margin": {
///      "type": "string"
///    },
///    "positions_value": {
///      "type": "string"
///    },
///    "projected_margin_change": {
///      "type": "string"
///    },
///    "risk_universe_id": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "subaccount_value": {
///      "type": "string"
///    },
///    "vault_deposit_holds": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/VaultDepositHold"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Subaccount {
    pub collaterals: ::std::vec::Vec<Collateral>,
    pub collaterals_initial_margin: ::std::string::String,
    pub collaterals_maintenance_margin: ::std::string::String,
    pub collaterals_value: ::std::string::String,
    pub currency: ::std::vec::Vec<::std::string::String>,
    ///`true` if this subaccount's portfolio could not be loaded; such subaccounts are returned as placeholders by `get_all_portfolios`.
    pub failed_to_fetch: bool,
    pub initial_margin: ::std::string::String,
    pub is_under_liquidation: bool,
    pub label: ::std::string::String,
    pub maintenance_margin: ::std::string::String,
    pub manager_id: u32,
    pub margin_type: ::std::string::String,
    pub open_orders: ::std::vec::Vec<Order>,
    pub open_orders_margin: ::std::string::String,
    pub positions: ::std::vec::Vec<Position>,
    pub positions_initial_margin: ::std::string::String,
    pub positions_maintenance_margin: ::std::string::String,
    pub positions_value: ::std::string::String,
    pub projected_margin_change: ::std::string::String,
    pub risk_universe_id: u32,
    pub subaccount_id: u64,
    pub subaccount_value: ::std::string::String,
    pub vault_deposit_holds: ::std::vec::Vec<VaultDepositHold>,
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
///    "trade_price",
///    "tx_status"
///  ],
///  "properties": {
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "expected_rebate": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "extra_fee": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "index_price": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "realized_pnl_excl_fees": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "trade_fee": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "trade_id": {
///      "type": "string"
///    },
///    "trade_price": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "tx_hash": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "tx_status": {
///      "$ref": "#/definitions/TxStatus"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Trade {
    pub direction: Direction,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub expected_rebate: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub extra_fee: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub index_price: ::bigdecimal::BigDecimal,
    pub instrument_name: ::std::string::String,
    pub is_transfer: bool,
    #[serde(default)]
    pub label: ::std::string::String,
    pub liquidity_role: LiquidityRole,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub mark_price: ::bigdecimal::BigDecimal,
    pub op_uuid: ::std::string::String,
    pub order_id: ::std::string::String,
    ///Optional UUID v4 string
    pub quote_id: ::std::option::Option<::uuid::Uuid>,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub realized_pnl: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub realized_pnl_excl_fees: ::bigdecimal::BigDecimal,
    ///Optional UUID v4 string
    pub rfq_id: ::std::option::Option<::uuid::Uuid>,
    pub subaccount_id: i64,
    pub timestamp: i64,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub trade_amount: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub trade_fee: ::bigdecimal::BigDecimal,
    pub trade_id: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub trade_price: ::bigdecimal::BigDecimal,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tx_hash: ::std::option::Option<::std::string::String>,
    pub tx_status: TxStatus,
}
///A trade as returned by `private/get_trade_history`. Mirrors a private trades subscription event, except `tx_status` is the settlement status (`null` until the batch settles).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A trade as returned by `private/get_trade_history`. Mirrors a private trades subscription event, except `tx_status` is the settlement status (`null` until the batch settles).",
///  "type": "object",
///  "required": [
///    "direction",
///    "expected_rebate",
///    "extra_fee",
///    "index_price",
///    "instrument_name",
///    "is_transfer",
///    "label",
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
///    "direction": {
///      "$ref": "#/definitions/Direction"
///    },
///    "expected_rebate": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "extra_fee": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "index_price": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "instrument_name": {
///      "type": "string"
///    },
///    "is_transfer": {
///      "type": "boolean"
///    },
///    "label": {
///      "type": "string"
///    },
///    "liquidity_role": {
///      "$ref": "#/definitions/LiquidityRole"
///    },
///    "mark_price": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "realized_pnl_excl_fees": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
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
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "trade_fee": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "trade_id": {
///      "type": "string"
///    },
///    "trade_price": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "tx_hash": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "tx_status": {
///      "anyOf": [
///        {
///          "$ref": "#/definitions/BatchStatus"
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
pub struct TradeHistoryResponse {
    pub direction: Direction,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub expected_rebate: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub extra_fee: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub index_price: ::bigdecimal::BigDecimal,
    pub instrument_name: ::std::string::String,
    pub is_transfer: bool,
    pub label: ::std::string::String,
    pub liquidity_role: LiquidityRole,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub mark_price: ::bigdecimal::BigDecimal,
    pub op_uuid: ::std::string::String,
    pub order_id: ::std::string::String,
    ///Optional UUID v4 string
    pub quote_id: ::std::option::Option<::uuid::Uuid>,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub realized_pnl: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub realized_pnl_excl_fees: ::bigdecimal::BigDecimal,
    ///Optional UUID v4 string
    pub rfq_id: ::std::option::Option<::uuid::Uuid>,
    pub subaccount_id: i64,
    pub timestamp: i64,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub trade_amount: ::bigdecimal::BigDecimal,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub trade_fee: ::bigdecimal::BigDecimal,
    pub trade_id: ::std::string::String,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub trade_price: ::bigdecimal::BigDecimal,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tx_hash: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tx_status: ::std::option::Option<BatchStatus>,
}
///Single trades OHLCV candle in `public/get_tradingview_chart_data`. `timestamp` and `timestamp_bucket` are the bucket start in UTC seconds.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Single trades OHLCV candle in `public/get_tradingview_chart_data`. `timestamp` and `timestamp_bucket` are the bucket start in UTC seconds.",
///  "type": "object",
///  "required": [
///    "close_price",
///    "high_price",
///    "low_price",
///    "open_price",
///    "timestamp",
///    "timestamp_bucket",
///    "volume_contracts",
///    "volume_usd"
///  ],
///  "properties": {
///    "close_price": {
///      "type": "string"
///    },
///    "high_price": {
///      "type": "string"
///    },
///    "low_price": {
///      "type": "string"
///    },
///    "open_price": {
///      "type": "string"
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "timestamp_bucket": {
///      "type": "integer",
///      "format": "int64"
///    },
///    "volume_contracts": {
///      "type": "string"
///    },
///    "volume_usd": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TradingviewCandle {
    pub close_price: ::std::string::String,
    pub high_price: ::std::string::String,
    pub low_price: ::std::string::String,
    pub open_price: ::std::string::String,
    pub timestamp: i64,
    pub timestamp_bucket: i64,
    pub volume_contracts: ::std::string::String,
    pub volume_usd: ::std::string::String,
}
///`amount` and `fee` are decimal strings (e.g. `"1.1"`). `is_outgoing` marks the caller's side; on the sender's row `amount` is the gross debited and `fee` the in-kind fee, on the receiver's row `amount` is the net credited and `fee` is 0. `operation_id`/`batch_uuid` are stable uuids.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "`amount` and `fee` are decimal strings (e.g. `\"1.1\"`). `is_outgoing` marks the caller's side; on the sender's row `amount` is the gross debited and `fee` the in-kind fee, on the receiver's row `amount` is the net credited and `fee` is 0. `operation_id`/`batch_uuid` are stable uuids.",
///  "type": "object",
///  "required": [
///    "amount",
///    "asset",
///    "batch_status",
///    "batch_uuid",
///    "fee",
///    "from_subaccount_id",
///    "from_wallet",
///    "is_outgoing",
///    "operation_id",
///    "timestamp",
///    "to_subaccount_id",
///    "to_wallet"
///  ],
///  "properties": {
///    "amount": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "asset": {
///      "type": "string"
///    },
///    "batch_status": {
///      "$ref": "#/definitions/BatchStatus"
///    },
///    "batch_uuid": {
///      "type": "string"
///    },
///    "fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "from_subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "from_wallet": {
///      "type": "string"
///    },
///    "is_outgoing": {
///      "type": "boolean"
///    },
///    "operation_id": {
///      "type": "string"
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "to_subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "to_wallet": {
///      "type": "string"
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
pub struct TransferEntry {
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub amount: ::bigdecimal::BigDecimal,
    pub asset: ::std::string::String,
    pub batch_status: BatchStatus,
    pub batch_uuid: ::std::string::String,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub fee: ::bigdecimal::BigDecimal,
    pub from_subaccount_id: u64,
    pub from_wallet: ::std::string::String,
    pub is_outgoing: bool,
    pub operation_id: ::std::string::String,
    pub timestamp: u64,
    pub to_subaccount_id: u64,
    pub to_wallet: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tx_hash: ::std::option::Option<::std::string::String>,
}
///`TransferHistoryResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "transfers"
///  ],
///  "properties": {
///    "transfers": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/TransferEntry"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TransferHistoryResult {
    pub transfers: ::std::vec::Vec<TransferEntry>,
}
///Params for `private/transfer_positions`.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Params for `private/transfer_positions`.",
///  "type": "object",
///  "required": [
///    "maker_params",
///    "taker_params",
///    "wallet"
///  ],
///  "properties": {
///    "maker_params": {
///      "$ref": "#/definitions/SignedTransferQuoteRequest"
///    },
///    "taker_params": {
///      "$ref": "#/definitions/SignedTransferQuoteRequest"
///    },
///    "wallet": {
///      "$ref": "#/definitions/Address"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TransferPositionsRequest {
    pub maker_params: SignedTransferQuoteRequest,
    pub taker_params: SignedTransferQuoteRequest,
    pub wallet: Address,
}
///Result for `private/transfer_positions`. Carries the synthesized maker and taker quote payloads so clients see a response shape parallel to `private/execute_quote`.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Result for `private/transfer_positions`. Carries the synthesized maker and taker quote payloads so clients see a response shape parallel to `private/execute_quote`.",
///  "type": "object",
///  "required": [
///    "maker_quote",
///    "taker_quote"
///  ],
///  "properties": {
///    "maker_quote": {
///      "$ref": "#/definitions/Quote"
///    },
///    "taker_quote": {
///      "$ref": "#/definitions/Quote"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TransferPositionsResponse {
    pub maker_quote: Quote,
    pub taker_quote: Quote,
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
///`TxStatus`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "string",
///      "enum": [
///        "requested",
///        "pending",
///        "settled",
///        "reverted",
///        "ignored",
///        "timed_out"
///      ]
///    },
///    {
///      "description": "Applied off-chain; not yet included in an on-chain settlement batch.",
///      "type": "string",
///      "enum": [
///        "applied"
///      ]
///    },
///    {
///      "description": "Included in a settlement batch and executing.",
///      "type": "string",
///      "enum": [
///        "in_batch"
///      ]
///    },
///    {
///      "description": "Settlement proof in progress.",
///      "type": "string",
///      "enum": [
///        "proving"
///      ]
///    },
///    {
///      "description": "Settlement transaction broadcast on-chain; awaiting confirmation.",
///      "type": "string",
///      "enum": [
///        "submitted"
///      ]
///    }
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
pub enum TxStatus {
    #[serde(rename = "requested")]
    Requested,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "settled")]
    Settled,
    #[serde(rename = "reverted")]
    Reverted,
    #[serde(rename = "ignored")]
    Ignored,
    #[serde(rename = "timed_out")]
    TimedOut,
    ///Applied off-chain; not yet included in an on-chain settlement batch.
    #[serde(rename = "applied")]
    Applied,
    ///Included in a settlement batch and executing.
    #[serde(rename = "in_batch")]
    InBatch,
    ///Settlement proof in progress.
    #[serde(rename = "proving")]
    Proving,
    ///Settlement transaction broadcast on-chain; awaiting confirmation.
    #[serde(rename = "submitted")]
    Submitted,
}
impl ::std::fmt::Display for TxStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Requested => f.write_str("requested"),
            Self::Pending => f.write_str("pending"),
            Self::Settled => f.write_str("settled"),
            Self::Reverted => f.write_str("reverted"),
            Self::Ignored => f.write_str("ignored"),
            Self::TimedOut => f.write_str("timed_out"),
            Self::Applied => f.write_str("applied"),
            Self::InBatch => f.write_str("in_batch"),
            Self::Proving => f.write_str("proving"),
            Self::Submitted => f.write_str("submitted"),
        }
    }
}
impl ::std::str::FromStr for TxStatus {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "requested" => Ok(Self::Requested),
            "pending" => Ok(Self::Pending),
            "settled" => Ok(Self::Settled),
            "reverted" => Ok(Self::Reverted),
            "ignored" => Ok(Self::Ignored),
            "timed_out" => Ok(Self::TimedOut),
            "applied" => Ok(Self::Applied),
            "in_batch" => Ok(Self::InBatch),
            "proving" => Ok(Self::Proving),
            "submitted" => Ok(Self::Submitted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TxStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TxStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TxStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///The managers risk-pricing a currency within one universe.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The managers risk-pricing a currency within one universe.",
///  "type": "object",
///  "required": [
///    "risk_universe_id"
///  ],
///  "properties": {
///    "pm": {
///      "description": "PMRM2 manager id risking this currency in the universe.",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "risk_universe_id": {
///      "type": "integer",
///      "format": "uint32",
///      "minimum": 0.0
///    },
///    "risk_universe_name": {
///      "description": "Display name of the universe (uppercase, e.g. \"PRIME\"); absent until set by the exchange.",
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "sm": {
///      "description": "SRM manager id risking this currency in the universe.",
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint32",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UniverseManagers {
    ///PMRM2 manager id risking this currency in the universe.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pm: ::std::option::Option<u32>,
    pub risk_universe_id: u32,
    ///Display name of the universe (uppercase, e.g. "PRIME"); absent until set by the exchange.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub risk_universe_name: ::std::option::Option<::std::string::String>,
    ///SRM manager id risking this currency in the universe.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub sm: ::std::option::Option<u32>,
}
///`UpdateVaultInfoRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "subaccount_id"
///  ],
///  "properties": {
///    "description": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "mtm_cap": {
///      "description": "Advisory mark-to-market cap in USD, as a decimal string (e.g. `\"1000\"`).",
///      "default": null,
///      "anyOf": [
///        {
///          "description": "Advisory mark-to-market cap in USD, as a decimal string (e.g. `\"1000\"`).",
///          "default": null,
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "name": {
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "subaccount_id": {
///      "description": "The vault's subaccount ID; the caller must be the vault's curator.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "whitelist_only": {
///      "default": null,
///      "type": [
///        "boolean",
///        "null"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UpdateVaultInfoRequest {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    ///Advisory mark-to-market cap in USD, as a decimal string (e.g. `"1000"`).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mtm_cap: ::std::option::Option<::bigdecimal::BigDecimal>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///The vault's subaccount ID; the caller must be the vault's curator.
    pub subaccount_id: u64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub whitelist_only: ::std::option::Option<bool>,
}
///`UpdateWhitelistedRecipientsRequest`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "add",
///    "nonce",
///    "remove",
///    "signature",
///    "signature_expiry_sec",
///    "signer",
///    "wallet"
///  ],
///  "properties": {
///    "add": {
///      "description": "Recipient wallet addresses to add to the whitelist.",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "nonce": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "remove": {
///      "description": "Recipient wallet addresses to remove from the whitelist.",
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    },
///    "signature": {
///      "type": "string"
///    },
///    "signature_expiry_sec": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "signer": {
///      "type": "string"
///    },
///    "wallet": {
///      "description": "Wallet whose whitelist is being updated; becomes the action owner.",
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UpdateWhitelistedRecipientsRequest {
    ///Recipient wallet addresses to add to the whitelist.
    pub add: ::std::vec::Vec<::std::string::String>,
    pub nonce: u64,
    ///Recipient wallet addresses to remove from the whitelist.
    pub remove: ::std::vec::Vec<::std::string::String>,
    pub signature: ::std::string::String,
    pub signature_expiry_sec: u64,
    pub signer: ::std::string::String,
    ///Wallet whose whitelist is being updated; becomes the action owner.
    pub wallet: ::std::string::String,
}
///`UpdateWhitelistedRecipientsResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "op_uuid",
///    "operation_id",
///    "whitelisted_recipients"
///  ],
///  "properties": {
///    "op_uuid": {
///      "type": "string"
///    },
///    "operation_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "whitelisted_recipients": {
///      "description": "The account's whitelisted recipients after the update was applied.",
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
pub struct UpdateWhitelistedRecipientsResponse {
    pub op_uuid: ::std::string::String,
    pub operation_id: u64,
    ///The account's whitelisted recipients after the update was applied.
    pub whitelisted_recipients: ::std::vec::Vec<::std::string::String>,
}
///`Vault`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "curator",
///    "curator_shares",
///    "description",
///    "name",
///    "protocol",
///    "whitelist_only"
///  ],
///  "properties": {
///    "benchmark_price": {
///      "description": "USD spot price of the HWM benchmark asset (`1.0` for feed-less USD vaults). null when a configured benchmark has no resolvable price.",
///      "default": null,
///      "anyOf": [
///        {
///          "description": "USD spot price of the HWM benchmark asset (`1.0` for feed-less USD vaults). null when a configured benchmark has no resolvable price.",
///          "default": null,
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "curator": {
///      "description": "The vault's curator — the owner wallet of the vault subaccount.",
///      "$ref": "#/definitions/Address"
///    },
///    "curator_shares": {
///      "description": "Shares held by the curator (the vault subaccount's owner) — their skin-in-the-game. Same units as `protocol.total_shares`.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "description": {
///      "type": "string"
///    },
///    "mtm_cap": {
///      "description": "Optional non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "default": null,
///      "anyOf": [
///        {
///          "description": "Optional non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "default": null,
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "name": {
///      "type": "string"
///    },
///    "nav_benchmark": {
///      "description": "Live NAV expressed in the HWM benchmark asset's units. null when the vault couldn't be priced or its configured benchmark has no spot price.",
///      "default": null,
///      "anyOf": [
///        {
///          "description": "Live NAV expressed in the HWM benchmark asset's units. null when the vault couldn't be priced or its configured benchmark has no spot price.",
///          "default": null,
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "nav_usd": {
///      "description": "Live mark-to-market NAV (USD) — signed, so an insolvent vault surfaces as a negative value. null only when the vault couldn't be priced (e.g. missing data). Divide by `protocol.total_shares` for the pre-fee share price.",
///      "default": null,
///      "anyOf": [
///        {
///          "description": "Live mark-to-market NAV (USD) — signed, so an insolvent vault surfaces as a negative value. null only when the vault couldn't be priced (e.g. missing data). Divide by `protocol.total_shares` for the pre-fee share price.",
///          "default": null,
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "protocol": {
///      "$ref": "#/definitions/ProtocolVault"
///    },
///    "simulated_share_price_usd": {
///      "description": "Live share price (USD/share) as if curator fees were settled now — the price a depositor/withdrawer faces. null when unpriceable or the vault is insolvent (a fee settlement on negative NAV is meaningless).",
///      "default": null,
///      "anyOf": [
///        {
///          "description": "Live share price (USD/share) as if curator fees were settled now — the price a depositor/withdrawer faces. null when unpriceable or the vault is insolvent (a fee settlement on negative NAV is meaningless).",
///          "default": null,
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "whitelist_only": {
///      "type": "boolean"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct Vault {
    ///USD spot price of the HWM benchmark asset (`1.0` for feed-less USD vaults). null when a configured benchmark has no resolvable price.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub benchmark_price: ::std::option::Option<::bigdecimal::BigDecimal>,
    ///The vault's curator — the owner wallet of the vault subaccount.
    pub curator: Address,
    ///Shares held by the curator (the vault subaccount's owner) — their skin-in-the-game. Same units as `protocol.total_shares`.
    pub curator_shares: ::bigdecimal::BigDecimal,
    pub description: ::std::string::String,
    ///Optional non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub mtm_cap: ::std::option::Option<::bigdecimal::BigDecimal>,
    pub name: ::std::string::String,
    ///Live NAV expressed in the HWM benchmark asset's units. null when the vault couldn't be priced or its configured benchmark has no spot price.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nav_benchmark: ::std::option::Option<::bigdecimal::BigDecimal>,
    ///Live mark-to-market NAV (USD) — signed, so an insolvent vault surfaces as a negative value. null only when the vault couldn't be priced (e.g. missing data). Divide by `protocol.total_shares` for the pre-fee share price.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nav_usd: ::std::option::Option<::bigdecimal::BigDecimal>,
    pub protocol: ProtocolVault,
    ///Live share price (USD/share) as if curator fees were settled now — the price a depositor/withdrawer faces. null when unpriceable or the vault is insolvent (a fee settlement on negative NAV is meaningless).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub simulated_share_price_usd: ::std::option::Option<::bigdecimal::BigDecimal>,
    pub whitelist_only: bool,
}
///A vault action as surfaced to the requesting user. Monetary fields (amounts, prices, share price) are decimal strings.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "A vault action as surfaced to the requesting user. Monetary fields (amounts, prices, share price) are decimal strings.",
///  "type": "object",
///  "required": [
///    "after_shares",
///    "amount",
///    "before_shares",
///    "creation_timestamp_ms",
///    "entry_price",
///    "error_reason",
///    "event_ts",
///    "event_type",
///    "exit_price",
///    "operation_id",
///    "operation_uuid",
///    "share_price",
///    "shares_delta",
///    "shares_requested",
///    "status",
///    "user_action_hash",
///    "vault_nonce",
///    "vault_subaccount_id",
///    "wallet"
///  ],
///  "properties": {
///    "after_shares": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "amount": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "before_shares": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "creation_timestamp_ms": {
///      "description": "Timestamp the request was created (ms).",
///      "type": "integer",
///      "format": "int64"
///    },
///    "entry_price": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "error_reason": {
///      "type": "string"
///    },
///    "event_ts": {
///      "description": "Timestamp of this event (ms).",
///      "type": "integer",
///      "format": "int64"
///    },
///    "event_type": {
///      "description": "\"vault_deposit\" | \"vault_withdraw\" | \"vault_force_withdraw\" | \"vault_cancel\".",
///      "type": "string"
///    },
///    "exit_price": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "operation_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "operation_uuid": {
///      "type": "string"
///    },
///    "share_price": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "shares_delta": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "shares_requested": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "status": {
///      "description": "While the action is in the queue, can be one of the following: \"enqueued\" | \"sequencer_applied\" | \"user_cancel\" | \"curator_reject\" | \"protocol_reject\" | \"expired\".\n\nOnce action is applied in the protocol, the status takes on `BatchStatus` values: \"Batching\" | \"Executing\" | \"Proving\" | \"Settling\" | \"Settled\" or an error.",
///      "type": "string"
///    },
///    "user_action_hash": {
///      "type": "string"
///    },
///    "vault_nonce": {
///      "type": "string"
///    },
///    "vault_subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "wallet": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VaultActionResponse {
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub after_shares: ::bigdecimal::BigDecimal,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub amount: ::bigdecimal::BigDecimal,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub before_shares: ::bigdecimal::BigDecimal,
    ///Timestamp the request was created (ms).
    pub creation_timestamp_ms: i64,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub entry_price: ::bigdecimal::BigDecimal,
    pub error_reason: ::std::string::String,
    ///Timestamp of this event (ms).
    pub event_ts: i64,
    ///"vault_deposit" | "vault_withdraw" | "vault_force_withdraw" | "vault_cancel".
    pub event_type: ::std::string::String,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub exit_price: ::bigdecimal::BigDecimal,
    pub operation_id: u64,
    pub operation_uuid: ::std::string::String,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub share_price: ::bigdecimal::BigDecimal,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub shares_delta: ::bigdecimal::BigDecimal,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub shares_requested: ::bigdecimal::BigDecimal,
    /**While the action is in the queue, can be one of the following: "enqueued" | "sequencer_applied" | "user_cancel" | "curator_reject" | "protocol_reject" | "expired".

Once action is applied in the protocol, the status takes on `BatchStatus` values: "Batching" | "Executing" | "Proving" | "Settling" | "Settled" or an error.*/
    pub status: ::std::string::String,
    pub user_action_hash: ::std::string::String,
    pub vault_nonce: ::std::string::String,
    pub vault_subaccount_id: u64,
    pub wallet: ::std::string::String,
}
///Returned by `cancel_all_vault_requests`: identifiers to track the on-chain `VaultCancel` operation plus the ids of the wallet's pending requests that were cancelled.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Returned by `cancel_all_vault_requests`: identifiers to track the on-chain `VaultCancel` operation plus the ids of the wallet's pending requests that were cancelled.",
///  "type": "object",
///  "required": [
///    "cancelled_request_ids",
///    "op_uuid",
///    "operation_id"
///  ],
///  "properties": {
///    "cancelled_request_ids": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/VaultRequestId"
///      }
///    },
///    "op_uuid": {
///      "type": "string"
///    },
///    "operation_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VaultCancelResponse {
    pub cancelled_request_ids: ::std::vec::Vec<VaultRequestId>,
    pub op_uuid: ::std::string::String,
    pub operation_id: u64,
}
/**Configuration for a vault subaccount's deposit/withdrawal controls.

Every field is immutable after vault creation. Mutable, informational settings (descriptions, an advisory TVL cap) live in off-chain vault state, not here.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Configuration for a vault subaccount's deposit/withdrawal controls.\n\nEvery field is immutable after vault creation. Mutable, informational settings (descriptions, an advisory TVL cap) live in off-chain vault state, not here.",
///  "type": "object",
///  "required": [
///    "cooldown_sec",
///    "deposit_spot_asset",
///    "management_fee_bps",
///    "max_slippage_bps",
///    "performance_fee_bps"
///  ],
///  "properties": {
///    "benchmark_asset": {
///      "description": "Spot asset the high-water mark is denominated in. null is the feed-less default: the HWM is a constant USD unit (reads no feed, so a deposit-asset depeg cannot manufacture a performance fee). An address denominates the HWM in that spot asset's units — performance fees then accrue only on outperformance measured in the benchmark (e.g. an ETH vault charging fees on ETH-outperformance, not on an ETH-price rally).",
///      "anyOf": [
///        {
///          "$ref": "#/definitions/Address"
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "cooldown_sec": {
///      "description": "Min seconds between a holder's last deposit and their withdrawal.",
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "deposit_spot_asset": {
///      "$ref": "#/definitions/Address"
///    },
///    "management_fee_bps": {
///      "type": "integer",
///      "format": "uint16",
///      "minimum": 0.0
///    },
///    "max_slippage_bps": {
///      "description": "Max deviation of the curator's quoted share price from the mtm-derived price, both sides. Bounded at creation by the protocol-wide maximum slippage limit.",
///      "type": "integer",
///      "format": "uint16",
///      "minimum": 0.0
///    },
///    "performance_fee_bps": {
///      "type": "integer",
///      "format": "uint16",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VaultConfig {
    ///Spot asset the high-water mark is denominated in. null is the feed-less default: the HWM is a constant USD unit (reads no feed, so a deposit-asset depeg cannot manufacture a performance fee). An address denominates the HWM in that spot asset's units — performance fees then accrue only on outperformance measured in the benchmark (e.g. an ETH vault charging fees on ETH-outperformance, not on an ETH-price rally).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub benchmark_asset: ::std::option::Option<Address>,
    ///Min seconds between a holder's last deposit and their withdrawal.
    pub cooldown_sec: u64,
    pub deposit_spot_asset: Address,
    pub management_fee_bps: u16,
    ///Max deviation of the curator's quoted share price from the mtm-derived price, both sides. Bounded at creation by the protocol-wide maximum slippage limit.
    pub max_slippage_bps: u16,
    pub performance_fee_bps: u16,
}
///Returned by `create_vault`: identifiers to track the on-chain `CreateVault` operation.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Returned by `create_vault`: identifiers to track the on-chain `CreateVault` operation.",
///  "type": "object",
///  "required": [
///    "op_uuid",
///    "operation_id"
///  ],
///  "properties": {
///    "op_uuid": {
///      "type": "string"
///    },
///    "operation_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VaultCreateResponse {
    pub op_uuid: ::std::string::String,
    pub operation_id: u64,
}
///`VaultDepositHold`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "amount",
///    "asset_name",
///    "currency",
///    "vault_id"
///  ],
///  "properties": {
///    "amount": {
///      "type": "string"
///    },
///    "asset_name": {
///      "type": "string"
///    },
///    "currency": {
///      "type": "string"
///    },
///    "vault_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VaultDepositHold {
    pub amount: ::std::string::String,
    pub asset_name: ::std::string::String,
    pub currency: ::std::string::String,
    pub vault_id: u64,
}
///Returned by `force_burn`: identifiers to track the on-chain `ForceBurn` operation.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Returned by `force_burn`: identifiers to track the on-chain `ForceBurn` operation.",
///  "type": "object",
///  "required": [
///    "op_uuid",
///    "operation_id"
///  ],
///  "properties": {
///    "op_uuid": {
///      "type": "string"
///    },
///    "operation_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VaultForceBurnResponse {
    pub op_uuid: ::std::string::String,
    pub operation_id: u64,
}
///Returned by `get_curated_vaults` / `get_shareholder_vaults`: the vault subaccount ids the caller curates / holds shares in.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Returned by `get_curated_vaults` / `get_shareholder_vaults`: the vault subaccount ids the caller curates / holds shares in.",
///  "type": "object",
///  "required": [
///    "subaccount_ids"
///  ],
///  "properties": {
///    "subaccount_ids": {
///      "type": "array",
///      "items": {
///        "type": "integer",
///        "format": "uint64",
///        "minimum": 0.0
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VaultIdsResponse {
    pub subaccount_ids: ::std::vec::Vec<u64>,
}
///`VaultPerformanceHistoryResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "points",
///    "resolution",
///    "subaccount_id"
///  ],
///  "properties": {
///    "points": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/VaultPerformancePointResponse"
///      }
///    },
///    "resolution": {
///      "$ref": "#/definitions/PerformanceResolution"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VaultPerformanceHistoryResult {
    pub points: ::std::vec::Vec<VaultPerformancePointResponse>,
    pub resolution: PerformanceResolution,
    pub subaccount_id: u64,
}
///One point in a vault's performance series. `ts` is the bucket time (epoch millis); each monetary value is the bucket's last sample as a decimal string. Live prices are nullable (a vault that couldn't be priced).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "One point in a vault's performance series. `ts` is the bucket time (epoch millis); each monetary value is the bucket's last sample as a decimal string. Live prices are nullable (a vault that couldn't be priced).",
///  "type": "object",
///  "required": [
///    "benchmark_price",
///    "curator_shares",
///    "global_hwm",
///    "nav",
///    "nav_benchmark",
///    "share_price",
///    "total_shares",
///    "ts"
///  ],
///  "properties": {
///    "benchmark_price": {
///      "description": "Optional non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "anyOf": [
///        {
///          "description": "Optional non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "curator_shares": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "global_hwm": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "nav": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "anyOf": [
///        {
///          "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "nav_benchmark": {
///      "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///      "anyOf": [
///        {
///          "description": "Optional decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits, or null; a string or JSON number is accepted",
///          "type": "string",
///          "format": "decimal",
///          "x-rust-type": {
///            "crate": "bigdecimal",
///            "path": "bigdecimal::BigDecimal",
///            "version": ">=0.4.0, <0.5.0"
///          }
///        },
///        {
///          "type": "null"
///        }
///      ]
///    },
///    "share_price": {
///      "description": "Decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "total_shares": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "ts": {
///      "type": "integer",
///      "format": "int64"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VaultPerformancePointResponse {
    ///Optional non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub benchmark_price: ::std::option::Option<::bigdecimal::BigDecimal>,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub curator_shares: ::bigdecimal::BigDecimal,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub global_hwm: ::bigdecimal::BigDecimal,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub nav: ::std::option::Option<::bigdecimal::BigDecimal>,
    ///Optional decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits, or null; a string or JSON number is accepted
    pub nav_benchmark: ::std::option::Option<::bigdecimal::BigDecimal>,
    ///Decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub share_price: ::bigdecimal::BigDecimal,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub total_shares: ::bigdecimal::BigDecimal,
    pub ts: i64,
}
///Returned by `request_vault_deposit` / `request_vault_withdraw`: the id the queued request was assigned (the user's handle for polling / cancelling).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Returned by `request_vault_deposit` / `request_vault_withdraw`: the id the queued request was assigned (the user's handle for polling / cancelling).",
///  "type": "object",
///  "required": [
///    "request_id"
///  ],
///  "properties": {
///    "request_id": {
///      "$ref": "#/definitions/VaultRequestId"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VaultRequestAckResponse {
    pub request_id: VaultRequestId,
}
/**Composite vault request id: `(vault_subaccount_id, wallet, vault_nonce)`

Useful as a stable dedup key across off-chain and on-chain vault events.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Composite vault request id: `(vault_subaccount_id, wallet, vault_nonce)`\n\nUseful as a stable dedup key across off-chain and on-chain vault events.",
///  "type": "object",
///  "required": [
///    "vault_nonce",
///    "vault_subaccount_id",
///    "wallet"
///  ],
///  "properties": {
///    "vault_nonce": {
///      "type": "string"
///    },
///    "vault_subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "wallet": {
///      "$ref": "#/definitions/Address"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VaultRequestId {
    pub vault_nonce: ::std::string::String,
    pub vault_subaccount_id: u64,
    pub wallet: Address,
}
///One queued deposit/withdraw request as surfaced to the curator.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "One queued deposit/withdraw request as surfaced to the curator.",
///  "type": "object",
///  "required": [
///    "creation_timestamp_ms",
///    "id",
///    "signed_action",
///    "subaccount_id",
///    "user_action_hash",
///    "wallet"
///  ],
///  "properties": {
///    "creation_timestamp_ms": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "id": {
///      "$ref": "#/definitions/VaultRequestId"
///    },
///    "signed_action": {
///      "description": "The user's signed deposit/withdraw envelope; the curator pairs their `MintShares`/`BurnShares` approval against it.",
///      "$ref": "#/definitions/SignedAction"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "user_action_hash": {
///      "description": "0x-prefixed hex of the 32-byte user-action hash the curator commits to.",
///      "type": "string"
///    },
///    "wallet": {
///      "$ref": "#/definitions/Address"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VaultRequestResponse {
    pub creation_timestamp_ms: u64,
    pub id: VaultRequestId,
    ///The user's signed deposit/withdraw envelope; the curator pairs their `MintShares`/`BurnShares` approval against it.
    pub signed_action: SignedAction,
    pub subaccount_id: u64,
    ///0x-prefixed hex of the 32-byte user-action hash the curator commits to.
    pub user_action_hash: ::std::string::String,
    pub wallet: Address,
}
///Returned by `mint_vault_shares` / `burn_vault_shares`: identifiers to track the on-chain `MintShares` / `BurnShares` settlement.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Returned by `mint_vault_shares` / `burn_vault_shares`: identifiers to track the on-chain `MintShares` / `BurnShares` settlement.",
///  "type": "object",
///  "required": [
///    "op_uuid",
///    "operation_id"
///  ],
///  "properties": {
///    "op_uuid": {
///      "type": "string"
///    },
///    "operation_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VaultSettleResponse {
    pub op_uuid: ::std::string::String,
    pub operation_id: u64,
}
///One row of `get_vault_shares`: the caller's share balance in a vault plus the full enriched vault row (same shape as `public/get_vault`).
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "One row of `get_vault_shares`: the caller's share balance in a vault plus the full enriched vault row (same shape as `public/get_vault`).",
///  "type": "object",
///  "required": [
///    "shares",
///    "vault"
///  ],
///  "properties": {
///    "shares": {
///      "description": "Shares the caller holds. Same units as `vault.protocol.total_shares`.",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "vault": {
///      "$ref": "#/definitions/Vault"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VaultShareEntryResponse {
    ///Shares the caller holds. Same units as `vault.protocol.total_shares`.
    pub shares: ::bigdecimal::BigDecimal,
    pub vault: Vault,
}
///Returned by `get_vault_shares`: one entry per vault the wallet holds shares in.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Returned by `get_vault_shares`: one entry per vault the wallet holds shares in.",
///  "type": "object",
///  "required": [
///    "vaults"
///  ],
///  "properties": {
///    "vaults": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/VaultShareEntryResponse"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VaultSharesResponse {
    pub vaults: ::std::vec::Vec<VaultShareEntryResponse>,
}
///Returned by `public/get_vaults`: one page of vaults plus pagination info. Each row carries its subaccount id under `protocol.subaccount_id`.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Returned by `public/get_vaults`: one page of vaults plus pagination info. Each row carries its subaccount id under `protocol.subaccount_id`.",
///  "type": "object",
///  "required": [
///    "pagination",
///    "vaults"
///  ],
///  "properties": {
///    "pagination": {
///      "$ref": "#/definitions/Pagination"
///    },
///    "vaults": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/Vault"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VaultsResponse {
    pub pagination: Pagination,
    pub vaults: ::std::vec::Vec<Vault>,
}
///`VolFeedDataResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "confidence",
///    "currency",
///    "deadline",
///    "expiry",
///    "signatures",
///    "timestamp",
///    "vol_data"
///  ],
///  "properties": {
///    "confidence": {
///      "type": "string"
///    },
///    "currency": {
///      "type": "string"
///    },
///    "deadline": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "expiry": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "signatures": {
///      "$ref": "#/definitions/OracleSignatureDataResponse"
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "vol_data": {
///      "$ref": "#/definitions/VolSVIParamDataResponse"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VolFeedDataResponse {
    pub confidence: ::std::string::String,
    pub currency: ::std::string::String,
    pub deadline: u64,
    pub expiry: u64,
    pub signatures: OracleSignatureDataResponse,
    pub timestamp: u64,
    pub vol_data: VolSviParamDataResponse,
}
///`VolSviParamDataResponse`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "SVI_a",
///    "SVI_b",
///    "SVI_fwd",
///    "SVI_m",
///    "SVI_refTau",
///    "SVI_rho",
///    "SVI_sigma"
///  ],
///  "properties": {
///    "SVI_a": {
///      "type": "string"
///    },
///    "SVI_b": {
///      "type": "string"
///    },
///    "SVI_fwd": {
///      "type": "string"
///    },
///    "SVI_m": {
///      "type": "string"
///    },
///    "SVI_refTau": {
///      "type": "string"
///    },
///    "SVI_rho": {
///      "type": "string"
///    },
///    "SVI_sigma": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct VolSviParamDataResponse {
    #[serde(rename = "SVI_a")]
    pub svi_a: ::std::string::String,
    #[serde(rename = "SVI_b")]
    pub svi_b: ::std::string::String,
    #[serde(rename = "SVI_fwd")]
    pub svi_fwd: ::std::string::String,
    #[serde(rename = "SVI_m")]
    pub svi_m: ::std::string::String,
    #[serde(rename = "SVI_refTau")]
    pub svi_ref_tau: ::std::string::String,
    #[serde(rename = "SVI_rho")]
    pub svi_rho: ::std::string::String,
    #[serde(rename = "SVI_sigma")]
    pub svi_sigma: ::std::string::String,
}
///`amount` and `fee` are decimal strings (e.g. `"1.1"`); the net sent to the recipient is `amount - fee`. `wallet` is the account owner; `recipient` is the L1 destination. `asset` is the protocol asset (the native-ETH sentinel for ETH); `erc20_address` is the on-chain ERC20 the funds settle against (WETH for native ETH). `operation_id`/`batch_uuid` are the stable uuids.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "`amount` and `fee` are decimal strings (e.g. `\"1.1\"`); the net sent to the recipient is `amount - fee`. `wallet` is the account owner; `recipient` is the L1 destination. `asset` is the protocol asset (the native-ETH sentinel for ETH); `erc20_address` is the on-chain ERC20 the funds settle against (WETH for native ETH). `operation_id`/`batch_uuid` are the stable uuids.",
///  "type": "object",
///  "required": [
///    "amount",
///    "asset",
///    "batch_status",
///    "batch_uuid",
///    "erc20_address",
///    "fee",
///    "operation_id",
///    "recipient",
///    "subaccount_id",
///    "timestamp",
///    "wallet"
///  ],
///  "properties": {
///    "amount": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "asset": {
///      "type": "string"
///    },
///    "batch_status": {
///      "$ref": "#/definitions/BatchStatus"
///    },
///    "batch_uuid": {
///      "type": "string"
///    },
///    "erc20_address": {
///      "type": "string"
///    },
///    "fee": {
///      "description": "Non-negative decimal string of the human value (e.g. `\"1.5\"`), up to 12 fractional digits; a string or JSON number is accepted",
///      "type": "string",
///      "format": "decimal",
///      "x-rust-type": {
///        "crate": "bigdecimal",
///        "path": "bigdecimal::BigDecimal",
///        "version": ">=0.4.0, <0.5.0"
///      }
///    },
///    "operation_id": {
///      "type": "string"
///    },
///    "recipient": {
///      "type": "string"
///    },
///    "subaccount_id": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "timestamp": {
///      "type": "integer",
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "tx_hash": {
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "wallet": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WithdrawalEntry {
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub amount: ::bigdecimal::BigDecimal,
    pub asset: ::std::string::String,
    pub batch_status: BatchStatus,
    pub batch_uuid: ::std::string::String,
    pub erc20_address: ::std::string::String,
    ///Non-negative decimal string of the human value (e.g. `"1.5"`), up to 12 fractional digits; a string or JSON number is accepted
    pub fee: ::bigdecimal::BigDecimal,
    pub operation_id: ::std::string::String,
    pub recipient: ::std::string::String,
    pub subaccount_id: u64,
    pub timestamp: u64,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub tx_hash: ::std::option::Option<::std::string::String>,
    pub wallet: ::std::string::String,
}
///`WithdrawalHistoryResult`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "withdrawals"
///  ],
///  "properties": {
///    "withdrawals": {
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/WithdrawalEntry"
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct WithdrawalHistoryResult {
    pub withdrawals: ::std::vec::Vec<WithdrawalEntry>,
}
/// Generation of default values for serde.
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn default_u64<T, const V: u64>() -> T
    where
        T: ::std::convert::TryFrom<u64>,
        <T as ::std::convert::TryFrom<u64>>::Error: ::std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
    pub(super) fn create_order_request_order_type() -> super::OrderType {
        super::OrderType::Limit
    }
    pub(super) fn create_order_request_time_in_force() -> super::TimeInForce {
        super::TimeInForce::Gtc
    }
    pub(super) fn order_cancel_reason() -> super::CancelReason {
        super::CancelReason::X
    }
    pub(super) fn order_quote_request_order_type() -> super::OrderType {
        super::OrderType::Limit
    }
    pub(super) fn order_quote_request_time_in_force() -> super::TimeInForce {
        super::TimeInForce::Gtc
    }
    pub(super) fn replace_order_request_order_type() -> super::OrderType {
        super::OrderType::Limit
    }
    pub(super) fn replace_order_request_time_in_force() -> super::TimeInForce {
        super::TimeInForce::Gtc
    }
    pub(super) fn replace_quote_request_extra_fee() -> ::bigdecimal::BigDecimal {
        ::serde_json::from_str::<::bigdecimal::BigDecimal>("\"0\"").unwrap()
    }
    pub(super) fn rfq_get_best_quote_request_direction() -> super::Direction {
        super::Direction::Buy
    }
    pub(super) fn rfq_get_best_quote_request_extra_fee() -> ::bigdecimal::BigDecimal {
        ::serde_json::from_str::<::bigdecimal::BigDecimal>("\"0\"").unwrap()
    }
    pub(super) fn send_quote_request_extra_fee() -> ::bigdecimal::BigDecimal {
        ::serde_json::from_str::<::bigdecimal::BigDecimal>("\"0\"").unwrap()
    }
    pub(super) fn send_rfq_request_extra_fee() -> ::bigdecimal::BigDecimal {
        ::serde_json::from_str::<::bigdecimal::BigDecimal>("\"0\"").unwrap()
    }
    pub(super) fn send_rfq_request_partial_fill_step() -> ::bigdecimal::BigDecimal {
        ::serde_json::from_str::<::bigdecimal::BigDecimal>("\"1\"").unwrap()
    }
    pub(super) fn set_mmp_config_request_mmp_amount_limit() -> ::bigdecimal::BigDecimal {
        ::serde_json::from_str::<::bigdecimal::BigDecimal>("\"0\"").unwrap()
    }
    pub(super) fn set_mmp_config_request_mmp_delta_limit() -> ::bigdecimal::BigDecimal {
        ::serde_json::from_str::<::bigdecimal::BigDecimal>("\"0\"").unwrap()
    }
}
