use dashmap::DashMap;

use bytes::Bytes;
use serde::de::DeserializeOwned;
use tokio::{
    sync::oneshot,
    task::JoinHandle,
    time::{Duration, Instant, MissedTickBehavior, interval, sleep},
};

use anyhow::anyhow;
use alloy::signers::{Signer, local::PrivateKeySigner};
use futures_util::{SinkExt, StreamExt};
use std::env::var;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};
use tokio::sync::{Mutex, mpsc, watch};
use tracing::{error, info, warn};
use yawc::{Frame, OpCode};

use crate::{
    models::{
        asyncapi_rpc::{SetCancelOnDisconnectRequest, SetCancelOnDisconnectResponse},
        openapi::{AssetType, GetAllInstrumentsRequest, Instrument},
    },
    namespaces::orders::OrdersNamespace,
    routing::{extract_channel, extract_id, extract_id_tail},
    rpc::Rpc,
    signing::sign_ws_login,
    subscriptions::Subscriptions,
    types::{
        ChannelResponse, ChannelSpec, ClientError, DispatchResult, Environment, Error, EventStream,
        ExternalEvent, InternalCommand, RequestScope, ResponseSender, RpcError, RpcResult,
        SubscriptionRoute, WsStream,
    },
};

const PING_INTERVAL: Duration = Duration::from_secs(5);
const READ_TIMEOUT: Duration = Duration::from_secs(7);

#[inline(always)]
pub fn deserialise_to_type<T>(s: &Bytes) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    match serde_json::from_slice::<T>(s) {
        Ok(val) => Ok(val),
        Err(e) => {
            error!("Deserialization error: {e:?}");
            error!("Raw response: {}", String::from_utf8_lossy(s));
            Err(e)
        }
    }
}

pub struct WsClient {
    write_tx: mpsc::UnboundedSender<InternalCommand>,
    pending_requests: Arc<DashMap<u64, ResponseSender>>,
    pub public_subscriptions: Arc<DashMap<String, SubscriptionRoute>>,
    pub private_subscriptions: Arc<DashMap<String, SubscriptionRoute>>,
    next_id: Arc<AtomicU64>,
    shutdown_tx: watch::Sender<bool>,
    pub instruments_cache: Arc<DashMap<String, Instrument>>,
    connection_state_rx: watch::Receiver<ExternalEvent>,
    current_connection_state: Arc<Mutex<ExternalEvent>>,
    supervisor_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    subscription_tasks: Arc<Mutex<Vec<JoinHandle<()>>>>,
    pub wallet: Option<PrivateKeySigner>,
    pub public_address: Option<String>,
    pub smart_contract_wallet_address: Option<String>,
    pub subaccount_id: Option<i64>,
    pub environment: Environment,
}

impl WsClient {
    pub fn subscriptions(&self) -> Subscriptions<'_> {
        Subscriptions { client: self }
    }
    pub fn rpc(&self) -> Rpc<'_> {
        Rpc { client: self }
    }

    pub fn orders(&self) -> OrdersNamespace<'_> {
        OrdersNamespace { ws_client: self }
    }

    pub async fn from_env(environment: Environment) -> Result<Self, ClientError> {
        let private_key = match var("DERIVE_PRIVATE_KEY") {
            Ok(v) => v,
            Err(e) => return Err(ClientError::EnvVar(e)),
        };
        let smart_contract_wallet_address = match var("DERIVE_SMART_CONTRACT_WALLET_ADDRESS") {
            Ok(v) => v,
            Err(e) => return Err(ClientError::EnvVar(e)),
        };
        let subaccount_id = match var("DERIVE_SUBACCOUNT_ID") {
            Ok(s) => match s.parse::<i64>() {
                Ok(id) => id,
                Err(e) => return Err(ClientError::Anyhow(anyhow!(e))),
            },
            Err(e) => return Err(ClientError::EnvVar(e)),
        };

        let client = WsClient::new(
            environment,
            Some(private_key),
            Some(smart_contract_wallet_address),
            Some(subaccount_id),
        )
        .await?;
        Ok(client)
    }

    // pub fn account(&self) -> AccountNamespace<'_> {
    //     AccountNamespace { ws_client: self }
    // }

    // pub fn collaterals(&self) -> CollateralsNamespace<'_> {
    //     CollateralsNamespace { ws_client: self }
    // }

    // pub fn orders(&self) -> OrdersNamespace<'_> {
    //     OrdersNamespace { ws_client: self }
    // }

    // pub fn positions(&self) -> PositionsNamespace<'_> {
    //     PositionsNamespace { ws_client: self }
    // }

    // pub fn subaccount(&self) -> SubaccountNamespace<'_> {
    //     SubaccountNamespace { ws_client: self }
    // }

    pub async fn new_public(environment: Environment) -> Result<Self, ClientError> {
        let client = WsClient::new(environment, None, None, None).await?;
        client.wait_for_connection().await;
        Ok(client)
    }

    pub async fn new(
        env: Environment,
        private_key: Option<String>,
        smart_contract_wallet_address: Option<String>,
        subaccount_id: Option<i64>,
    ) -> Result<Self, ClientError> {
        let url = env.get_url().to_string();
        let mut wallet = None;
        let mut public_address = None;
        match &private_key {
            Some(key) => {
                wallet = Some(key.parse::<PrivateKeySigner>().expect("Invalid private key"));
                public_address = Some(format!("{:?}", wallet.as_ref().unwrap().address()));
                info!(
                    "Creating WsClient in private mode with address: {}",
                    public_address.as_ref().unwrap()
                );
            }
            None => {
                info!("Creating WsClient in public mode");
            }
        }
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<InternalCommand>();
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let pending_requests = Arc::new(DashMap::new());
        let public_subscriptions: Arc<DashMap<String, SubscriptionRoute>> =
            Arc::new(DashMap::new());
        let private_subscriptions: Arc<DashMap<String, SubscriptionRoute>> =
            Arc::new(DashMap::new());
        let next_id = Arc::new(AtomicU64::new(1));

        let (connection_state_tx, connection_state_rx) =
            watch::channel(ExternalEvent::Disconnected);

        let _ = connection_state_tx.send(ExternalEvent::Disconnected);

        let supervisor_handle = tokio::spawn(connection_supervisor(
            url,
            cmd_rx,
            shutdown_rx,
            pending_requests.clone(),
            public_subscriptions.clone(),
            private_subscriptions.clone(),
            connection_state_tx,
        ));

        let instruments_cache = Arc::new(DashMap::new());

        let client = WsClient {
            write_tx: cmd_tx.clone(),
            pending_requests: pending_requests.clone(),
            public_subscriptions: public_subscriptions.clone(),
            private_subscriptions: private_subscriptions.clone(),
            next_id: next_id.clone(),
            shutdown_tx: shutdown_tx.clone(),
            connection_state_rx,
            current_connection_state: Arc::new(Mutex::new(ExternalEvent::Disconnected)),
            supervisor_handle: Arc::new(Mutex::new(Some(supervisor_handle))),
            subscription_tasks: Arc::new(Mutex::new(Vec::new())),
            wallet,
            public_address,
            smart_contract_wallet_address,
            // .and_then(|addr| addr.parse::<Address>().ok()),
            subaccount_id,
            instruments_cache,
            environment: env,
        };
        client.cache_instruments().await?;
        // if private_key.is_some() {
        //     client.wait_for_connection().await;
        // }
        Ok(client)
    }

    pub async fn send_rpc<T>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T, ClientError>
    where
        T: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);

        let (tx, rx) = oneshot::channel::<Bytes>();
        self.pending_requests.insert(id, tx);

        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        });

        let text = request.to_string();

        if let Err(e) = self.write_tx.send(InternalCommand::Send(Frame::text(text))) {
            error!("Failed to send RPC request: {e:?}");
            self.pending_requests.remove(&id);
            return Err(ClientError::Transport(Box::new(e)));
        }

        let response = rx.await?;
        match serde_json::from_slice::<RpcResult<T>>(&response) {
            Ok(result) => Ok(result.result),
            Err(parse_err) => {
                if let Ok(rpc_error) = serde_json::from_slice::<RpcError>(&response) {
                    error!(
                        "RPC error response: {:?}; raw: {}",
                        rpc_error.error,
                        String::from_utf8_lossy(&response)
                    );
                    Err(ClientError::RpcError {
                        error: rpc_error.error,
                    })
                } else {
                    println!(
                        "Failed to parse RPC response; raw: {}",
                        String::from_utf8_lossy(&response)
                    );
                    Err(ClientError::Parse(parse_err))
                }
            }
        }
    }

    pub async fn shutdown(&self, reason: &'static str) -> Result<(), ClientError> {
        info!("Shutdown requested: {reason}");
        self.public_subscriptions.clear();
        self.private_subscriptions.clear();
        let _ = self.shutdown_tx.send(true);
        let _ = self.write_tx.send(InternalCommand::Close);
        if let Some(handle) = self.supervisor_handle.lock().await.take() {
            match tokio::time::timeout(Duration::from_secs(5), handle).await {
                Ok(Ok(())) => {
                    info!("Supervisor task completed successfully");
                }
                Ok(Err(e)) => {
                    error!("Supervisor task panicked: {e:?}");
                    return Err(ClientError::Transport(Box::new(e)));
                }
                Err(e) => {
                    error!("Supervisor task timeout after 5s");
                    return Err(ClientError::Transport(Box::new(e)));
                }
            }
        }
        for task in self.subscription_tasks.lock().await.drain(..) {
            task.abort();
        }
        Ok(())
    }

    pub async fn subscribe<C>(&self, spec: C) -> Result<EventStream<C::Output>, ClientError>
    where
        C: ChannelSpec,
    {
        let channel = spec.channel();
        let scope = spec.scope();

        let _sub_result: ChannelResponse = self
            .send_rpc(
                "subscribe",
                serde_json::json!({
                    "channels": [channel.clone()]
                }),
            )
            .await?;

        let (tx, rx) = tokio::sync::broadcast::channel::<C::Output>(100);
        let route = SubscriptionRoute {
            type_name: std::any::type_name::<C::Output>(),
            dispatch: Arc::new(move |bytes: &Bytes| match C::decode(bytes) {
                Ok(parsed) => {
                    if tx.send(parsed).is_ok() {
                        DispatchResult::Delivered
                    } else {
                        DispatchResult::NoReceivers
                    }
                }
                Err(e) => {
                    warn!("Failed to decode subscription payload: {e:?}");
                    DispatchResult::DecodeError
                }
            }),
        };

        {
            match scope {
                RequestScope::Public => {
                    self.public_subscriptions.insert(channel.clone(), route);
                    info!("Subscribed to public channel: {channel}");
                }
                RequestScope::Private => {
                    self.private_subscriptions.insert(channel.clone(), route);
                    info!("Subscribed to private channel: {channel}");
                }
            }
        }
        Ok(tokio_stream::wrappers::BroadcastStream::new(rx))
    }

    pub async fn unsubscribe(&self, channel: &str) -> Result<(), ClientError> {
        let channel = channel.to_string();
        {
            if self.public_subscriptions.remove(&channel).is_some() {
                let _: ChannelResponse = self
                    .send_rpc(
                        "unsubscribe",
                        serde_json::json!({
                            "channels": [channel.clone()]
                        }),
                    )
                    .await?;
                info!("Unsubscribed from public channel: {channel}");
                return Ok(());
            }
        }
        {
            if self.private_subscriptions.remove(&channel).is_some() {
                let _: ChannelResponse = self
                    .send_rpc(
                        "unsubscribe",
                        serde_json::json!({
                            "channels": [channel.clone()]
                        }),
                    )
                    .await?;
                info!("Unsubscribed from private channel: {channel}");
                return Ok(());
            }
        }
        warn!("No active subscription found for channel: {channel}");
        Err(ClientError::Rpc(serde_json::json!({})))
    }

    // pub async fn resubscribe_all(&self) -> Result<(), ClientError> {
    //     let public_channels: Vec<String> = self
    //         .public_subscriptions
    //         .iter()
    //         .map(|e| e.key().clone())
    //         .collect();
    //     let private_channels: Vec<String> = self
    //         .private_subscriptions
    //         .iter()
    //         .map(|e| e.key().clone())
    //         .collect();
    //     let all_channels: Vec<String> = public_channels
    //         .iter()
    //         .chain(private_channels.iter())
    //         .cloned()
    //         .collect();
    //     for attempt in 1..=5 {
    //         let res = self
    //             .send_rpc::<ChannelResponse>(
    //                 "subscribe",
    //                 serde_json::json!({
    //                     "channels": all_channels
    //                 }),
    //             )
    //             .await;
    //         match res {
    //             Ok(res) => {
    //                 info!(
    //                     "Re-subscribed to all channels: {all_channels:?}: response: {res:?} on attempt {attempt}"
    //                 );
    //                 return Ok(());
    //             }
    //             Err(e) => {
    //                 warn!("Failed to re-subscribe to channels: {e:?}; attempt {attempt}");
    //                 tokio::time::sleep(Duration::from_secs(2)).await;
    //             }
    //         }
    //     }
    //     Err(ClientError::Rpc(serde_json::json!({
    //         "message": "Failed to re-subscribe to channels after multiple attempts"
    //     })))
    // }

    pub async fn run_till_event(&self) -> ExternalEvent {
        let mut rx = self.connection_state_rx.clone();
        loop {
            if rx.changed().await.is_ok() {
                let state = *rx.borrow_and_update();
                if state != *self.current_connection_state.lock().await {
                    let mut current_state = self.current_connection_state.lock().await;
                    *current_state = state;
                    return state;
                }
            }
        }
    }

    pub async fn login(&self) -> Result<Vec<u64>, ClientError> {
        if self.wallet.is_none() {
            warn!("No wallet available for login");
            return Err(ClientError::Rpc(serde_json::json!({
                "message": "No wallet available for login"
            })));
        }
        if self.smart_contract_wallet_address.is_none() {
            warn!("No smart contract wallet available for login");
            return Err(ClientError::Rpc(serde_json::json!({
                "message": "No smart contract wallet available for login"
            })));
        }
        let scw = self.smart_contract_wallet_address.as_ref().unwrap();
        let wallet = self.wallet.as_ref().unwrap();
        let login_data = sign_ws_login(scw, wallet).await;

        self.send_rpc("public/login", login_data).await
    }

    pub async fn set_cancel_on_disconnect(
        &self,
        enabled: bool,
    ) -> Result<SetCancelOnDisconnectResponse, ClientError> {
        let scw = match &self.smart_contract_wallet_address {
            Some(addr) => addr.clone(),
            None => {
                warn!("No smart contract wallet available for set_cancel_on_disconnect");
                return Err(ClientError::Rpc(serde_json::json!({
                    "message": "No smart contract wallet available for set_cancel_on_disconnect"
                })));
            }
        };
        let msg = SetCancelOnDisconnectRequest {
            enabled: Some(enabled),
            wallet: Some(scw),
        };
        self.send_rpc(
            "private/set_cancel_on_disconnect",
            serde_json::to_value(msg).map_err(ClientError::Parse)?,
        )
        .await
    }

    pub fn is_connected(&self) -> bool {
        *self.connection_state_rx.borrow() == ExternalEvent::Connected
    }

    pub async fn wait_for_connection(&self) {
        let mut rx = self.connection_state_rx.clone();

        if *rx.borrow_and_update() == ExternalEvent::Connected {
            let mut current_state = self.current_connection_state.lock().await;
            *current_state = ExternalEvent::Connected;
            return;
        }

        while rx.changed().await.is_ok() {
            if *rx.borrow_and_update() == ExternalEvent::Connected {
                let mut current_state = self.current_connection_state.lock().await;
                *current_state = ExternalEvent::Connected;
                return;
            }
        }
    }

    // async fn get_instruments(&self) -> Result<PublicGetAllInstrumentsResultSchema, ClientError> {
    //     let result: PublicGetAllInstrumentsResultSchema = self
    //         .send_rpc(
    //             "public/get_all_instruments",
    //             serde_json::json!({
    //                 "expired": false,
    //                 "instrument_type": "perp"
    //             }),
    //         )
    //         .await?;
    //     Ok(result)
    // }

    async fn cache_instruments(&self) -> Result<(), ClientError> {
        let params = GetAllInstrumentsRequest::builder()
            .expired(false)
            .instrument_type(AssetType::Perp)
            .try_into()?;
        let instruments = self.rpc().market_data().get_all_instruments(params).await?;
        self.instruments_cache.clear();
        for instrument in &instruments.instruments {
            self.instruments_cache
                .insert(instrument.instrument_name.clone(), instrument.clone());
        }
        Ok(())
    }
}

async fn connection_supervisor(
    url: String,
    mut cmd_rx: mpsc::UnboundedReceiver<InternalCommand>,
    mut shutdown_rx: watch::Receiver<bool>,
    pending_requests: Arc<DashMap<u64, ResponseSender>>,
    public_subscriptions: Arc<DashMap<String, SubscriptionRoute>>,
    private_subscriptions: Arc<DashMap<String, SubscriptionRoute>>,
    connection_state_tx: watch::Sender<ExternalEvent>,
) {
    info!("Connection supervisor started for {url}");

    let mut attempts: u64 = 1;
    loop {
        if *shutdown_rx.borrow() {
            info!("Supervisor sees shutdown for {url}");
            connection_state_tx.send(ExternalEvent::Disconnected).ok();
            break;
        }

        match yawc::WebSocket::connect(url.parse().unwrap()).await {
            Ok(ws_stream) => {
                connection_state_tx.send(ExternalEvent::Connected).ok();
                attempts = 1;
                info!("Connected to {url}");
                let result = run_single_connection(
                    &url,
                    ws_stream,
                    &mut cmd_rx,
                    &mut shutdown_rx,
                    &pending_requests,
                    &public_subscriptions,
                    &private_subscriptions,
                )
                .await;
                info!("Connection to {url} ended with result: {result:?}");
                connection_state_tx.send(ExternalEvent::Disconnected).ok();

                for key in pending_requests
                    .iter()
                    .map(|e| *e.key())
                    .collect::<Vec<u64>>()
                {
                    if let Some((_, tx)) = pending_requests.remove(&key) {
                        let _ = tx.send(r#"{"error":"connection closed"}"#.into());
                    }
                }

                if *shutdown_rx.borrow() {
                    connection_state_tx.send(ExternalEvent::Exited).ok();
                    info!("Shutdown after connection end for {url}");
                    break;
                }

                if cmd_rx.is_closed() {
                    connection_state_tx.send(ExternalEvent::Exited).ok();
                    info!("Command channel closed for {url}, stopping supervisor");
                    break;
                }

                let cooldown_secs = attempts * 3;
                info!("Reconnecting to {url} in {cooldown_secs}s (attempt {attempts})");
                tokio::time::sleep(std::time::Duration::from_secs(cooldown_secs.min(60))).await;
                attempts += 1;
            }
            Err(e) => {
                error!("Failed to connect to {url}: {e} on attempt {attempts}");
                if *shutdown_rx.borrow() || cmd_rx.is_closed() {
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_secs(attempts * 3)).await;
                attempts += 1;
                connection_state_tx.send(ExternalEvent::Disconnected).ok();
            }
        }
    }

    info!("Connection supervisor exited for {url}");
}

async fn run_single_connection(
    url: &str,
    mut ws: WsStream,
    cmd_rx: &mut mpsc::UnboundedReceiver<InternalCommand>,
    shutdown_rx: &mut watch::Receiver<bool>,
    pending_requests: &Arc<DashMap<u64, ResponseSender>>,
    public_subscriptions: &Arc<DashMap<String, SubscriptionRoute>>,
    private_subscriptions: &Arc<DashMap<String, SubscriptionRoute>>,
) -> Result<(), Error> {
    let mut ping_interval = interval(PING_INTERVAL);
    ping_interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

    let read_deadline = sleep(READ_TIMEOUT);
    tokio::pin!(read_deadline);

    loop {
        tokio::select! {
            _ = ping_interval.tick() => {
                if let Err(e) = ws.send(Frame::ping(Vec::default())).await {
                    warn!("Failed to send ping for {url}: {e}");
                    return Err(Box::new(e));
                }
            }

            _ = shutdown_rx.changed() => {
                if *shutdown_rx.borrow() {
                    info!("Shutdown requested for {url}");
                    let _ = ws.close().await;
                    return Ok(());
                }
            }

            maybe_cmd = cmd_rx.recv() => {
                match maybe_cmd {
                    Some(InternalCommand::Send(msg)) => {
                        ws.send(msg).await?;
                    }
                    Some(InternalCommand::Close) => {
                        info!("Close command received for {url}");
                        let _ = ws.close().await;
                        return Ok(());
                    }
                    None => {
                        info!("Command channel closed for {url}");
                        let _ = ws.close().await;
                        return Ok(());
                    }
                }
            }

            msg = ws.next() => {
                read_deadline.as_mut().reset(Instant::now() + READ_TIMEOUT);
                let Some(frame): Option<yawc::Frame> = msg else {
                    warn!("WebSocket stream ended for {url}");
                    return Ok(());
                };

                match frame.opcode() {
                    OpCode::Text | OpCode::Binary => {
                        handle_incoming(
                            frame.into_payload(),
                            pending_requests,
                            public_subscriptions,
                            private_subscriptions,
                        );
                    }
                    OpCode::Ping => {
                        ws.send(Frame::pong(Vec::default())).await?;
                    }
                    OpCode::Pong => {}
                    OpCode::Close => {
                        warn!("WebSocket closed for {url}");
                        return Ok(());
                    }
                    OpCode::Continuation => {}
                }
            }

            _ = &mut read_deadline => {
                warn!("WebSocket read timeout for {url} - connection appears dead");
                return Err("websocket read timeout".into());
            }
        }
    }
}

#[inline(always)]
pub fn handle_incoming(
    bytes: Bytes,
    pending_requests: &Arc<DashMap<u64, ResponseSender>>,
    public_subscriptions: &Arc<DashMap<String, SubscriptionRoute>>,
    private_subscriptions: &Arc<DashMap<String, SubscriptionRoute>>,
) {
    // println!("Received message: {}", String::from_utf8_lossy(&bytes));
    if let Some(id) = extract_id(&bytes)
        && let Some((_, tx)) = pending_requests.remove(&id)
    {
        let _ = tx.send(bytes);
        return;
    }

    if let Some(channel) = extract_channel(&bytes) {
        println!("Received message for channel: {channel}");
        for routes in [private_subscriptions, public_subscriptions] {
            if let Some(route) = routes.get(channel) {
                match (route.dispatch)(&bytes) {
                    DispatchResult::Delivered => {}
                    DispatchResult::DecodeError => {
                        warn!(
                            "Decode error for channel {channel} as type {}",
                            route.type_name
                        );
                    }
                    DispatchResult::NoReceivers => {
                        warn!("No receivers for channel {channel}, removing subscription");
                        routes.remove(channel);
                    }
                }
                return;
            }
        }
        warn!("No subscription handler for channel: {channel}");
        return;
    }

    // fallback to the extract_tail_id
    if let Some(id) = extract_id_tail(&bytes)
        && let Some((_, tx)) = pending_requests.remove(&id)
    {
        let _ = tx.send(bytes);
        return;
    }
    warn!(
        "Received unhandled message: {}",
        String::from_utf8_lossy(&bytes)
    );
}
