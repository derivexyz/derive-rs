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
///Login params. The wallet/timestamp/signature fields are typically supplied via headers (`X-Derive*`) for REST and via the JSON body for websocket; all are optional on the wire and validated server-side.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Login params. The wallet/timestamp/signature fields are typically supplied via headers (`X-Derive*`) for REST and via the JSON body for websocket; all are optional on the wire and validated server-side.",
///  "type": "object",
///  "properties": {
///    "signature": {
///      "description": "EIP-191 signature of the `timestamp` string, signed by the wallet or session key.",
///      "default": null,
///      "type": [
///        "string",
///        "null"
///      ]
///    },
///    "timestamp": {
///      "description": "Milliseconds since Unix epoch. Accepted as either a JSON number or a string-encoded integer.",
///      "default": null,
///      "type": [
///        "integer",
///        "null"
///      ],
///      "format": "uint64",
///      "minimum": 0.0
///    },
///    "wallet": {
///      "description": "Owner of account (not the session key).",
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
pub struct LoginRequest {
    ///EIP-191 signature of the `timestamp` string, signed by the wallet or session key.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub signature: ::std::option::Option<::std::string::String>,
    ///Milliseconds since Unix epoch. Accepted as either a JSON number or a string-encoded integer.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub timestamp: ::std::option::Option<u64>,
    ///Owner of account (not the session key).
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wallet: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for LoginRequest {
    fn default() -> Self {
        Self {
            signature: Default::default(),
            timestamp: Default::default(),
            wallet: Default::default(),
        }
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
///`private/set_cancel_on_disconnect` params. `wallet` is captured by the auth context.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "`private/set_cancel_on_disconnect` params. `wallet` is captured by the auth context.",
///  "type": "object",
///  "properties": {
///    "enabled": {
///      "description": "Whether to enable or disable cancel on disconnect.",
///      "default": null,
///      "type": [
///        "boolean",
///        "null"
///      ]
///    },
///    "wallet": {
///      "description": "Wallet address.",
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
pub struct SetCancelOnDisconnectRequest {
    ///Whether to enable or disable cancel on disconnect.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub enabled: ::std::option::Option<bool>,
    ///Wallet address.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub wallet: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for SetCancelOnDisconnectRequest {
    fn default() -> Self {
        Self {
            enabled: Default::default(),
            wallet: Default::default(),
        }
    }
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
pub enum SetCancelOnDisconnectResponse {
    #[serde(rename = "ok")]
    Ok,
}
impl ::std::fmt::Display for SetCancelOnDisconnectResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ok => f.write_str("ok"),
        }
    }
}
impl ::std::str::FromStr for SetCancelOnDisconnectResponse {
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
impl ::std::convert::TryFrom<&str> for SetCancelOnDisconnectResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SetCancelOnDisconnectResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SetCancelOnDisconnectResponse {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
///Result for `subscribe`. `status` maps each requested channel to `"ok"` or an error string; `current_subscriptions` is the full set of channels the connection is subscribed to after the operation.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Result for `subscribe`. `status` maps each requested channel to `\"ok\"` or an error string; `current_subscriptions` is the full set of channels the connection is subscribed to after the operation.",
///  "type": "object",
///  "required": [
///    "current_subscriptions",
///    "status"
///  ],
///  "properties": {
///    "current_subscriptions": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "status": {
///      "type": "object",
///      "additionalProperties": true
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct SubscribeResult {
    pub current_subscriptions: Vec<::std::string::String>,
    pub status: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
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
///Result for `unsubscribe`. `status` maps each requested channel to `"ok"` or an error string; `remaining_subscriptions` is the set of channels the connection is still subscribed to after the operation.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Result for `unsubscribe`. `status` maps each requested channel to `\"ok\"` or an error string; `remaining_subscriptions` is the set of channels the connection is still subscribed to after the operation.",
///  "type": "object",
///  "required": [
///    "remaining_subscriptions",
///    "status"
///  ],
///  "properties": {
///    "remaining_subscriptions": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "status": {
///      "type": "object",
///      "additionalProperties": true
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct UnsubscribeResult {
    pub remaining_subscriptions: Vec<::std::string::String>,
    pub status: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
