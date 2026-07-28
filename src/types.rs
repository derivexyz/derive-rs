use bytes::Bytes;
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::Value;
use std::{error::Error as StdError, fmt, str::FromStr, sync::Arc};
use thiserror::Error;
use tokio::sync::oneshot;
use yawc::Frame;

pub type WsStream = yawc::TcpWebSocket;
pub type ResponseSender = oneshot::Sender<Bytes>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub enum InternalCommand {
    Send(Frame),
    Close,
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum ExternalEvent {
    Connected,
    Disconnected,
    Exited,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RequestScope {
    Public,
    Private,
}

impl fmt::Display for RequestScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestScope::Public => write!(f, "public"),
            RequestScope::Private => write!(f, "private"),
        }
    }
}

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("RPC error: {0:?}")]
    Rpc(Value),
    #[error("transport error")]
    Transport(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("JSON parse error")]
    Parse(#[from] serde_json::Error),
    #[error("oneshot receive error")]
    Recv(#[from] oneshot::error::RecvError),
    #[error("Anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),
    #[error("Environment variable error: {0}")]
    EnvVar(#[from] std::env::VarError),
    #[error("Rpc error {error:?}")]
    RpcError { error: Value },
    #[error("Conversion error: {0}")]
    Conversion(#[from] crate::models::openapi::error::ConversionError),
    #[error("String error: {0}")]
    StringError(#[from] Box<dyn StdError>),
    #[error("Approval error: {0}")]
    ApprovalError(String),
}

// environment enum
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Environment {
    Mainnet,
    #[default]
    Testnet,
}
impl Environment {
    pub fn get_url(&self) -> &str {
        match self {
            Environment::Mainnet => "wss://api.lyra.finance/ws",
            Environment::Testnet => "wss://testnet.api.derive.xyz/v3/ws",
        }
    }
    pub fn get_default_rpc(&self) -> String {
        match self {
            Environment::Testnet => "https://sepolia.drpc.org".to_string(),
            Environment::Mainnet => "https://eth.drpc.org".to_string(),
        }
    }
}
impl FromStr for Environment {
    type Err = ();
    fn from_str(env: &str) -> Result<Self, Self::Err> {
        match env.to_lowercase().as_str() {
            "mainnet" => Ok(Environment::Mainnet),
            "testnet" => Ok(Environment::Testnet),
            _ => Err(()),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ChannelResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_subscriptions: Option<Vec<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_subscriptions: Option<Vec<String>>,

    #[serde(default)]
    pub status: Value,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SubscribeResponse {
    Ok { id: u64, result: ChannelResponse },
    Err { id: u64, error: Value },
}

#[derive(Deserialize, Debug)]
pub struct Pagination {
    #[allow(dead_code)]
    count: u64,
    #[allow(dead_code)]
    num_pages: u64,
}
// a genertic Response struct allowing a type parameter T
#[derive(Deserialize, Debug)]
pub struct RpcResult<T> {
    pub id: u64,
    pub result: T,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}
#[derive(Deserialize, Debug)]
pub struct RpcError {
    pub id: u64,
    pub error: Value,
    #[serde(default)]
    pub jsonrpc: Option<String>,
}
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum RpcResponse<T> {
    Ok(RpcResult<T>),
    // Err(RpcError),
}

pub type RpcResponseResult<T> = Result<RpcResult<T>, ClientError>;

pub type EventSender<T> = tokio::sync::broadcast::Sender<T>;
pub type EventStream<T> = tokio_stream::wrappers::BroadcastStream<T>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DispatchResult {
    Delivered,
    NoReceivers,
    DecodeError,
}

pub struct SubscriptionRoute {
    pub type_name: &'static str,
    pub dispatch: Arc<dyn Fn(&Bytes) -> DispatchResult + Send + Sync>,
}

#[derive(Deserialize, Debug)]
pub struct SubscriptionNotification<T> {
    pub params: SubscriptionParams<T>,
}

#[derive(Deserialize, Debug)]
pub struct SubscriptionParams<T> {
    #[allow(dead_code)]
    pub channel: String,
    pub data: T,
}

pub fn decode_subscription_data<T>(bytes: &Bytes) -> Result<T, ClientError>
where
    T: DeserializeOwned,
{
    let notification: SubscriptionNotification<T> = serde_json::from_slice(bytes)?;
    Ok(notification.params.data)
}

pub trait ChannelSpec {
    type Output: DeserializeOwned + Send + Clone + 'static;

    fn scope(&self) -> RequestScope;
    fn channel(&self) -> String;

    fn decode(bytes: &Bytes) -> Result<Self::Output, ClientError> {
        decode_subscription_data(bytes)
    }
}
