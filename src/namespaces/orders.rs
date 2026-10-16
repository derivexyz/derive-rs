use alloy::signers::local::PrivateKeySigner;

use crate::{
    actions::{ActionData, ModuleType, OrderArgs, ReplaceArgs, TradeData},
    models::{
        CancelOrderRequest, Direction, GetOrderRequest, Instrument, Order, OrderCreatedResponse,
        ReplaceOrderResponse,
    },
    types::ClientError,
    utils::snap_to_instrument_grid,
    ws_client::WsClient,
};

/// Everything a signed order needs, resolved up front.
struct SigningContext {
    subaccount_id: u64,
    signer: PrivateKeySigner,
    instrument: Instrument,
    derive_wallet: String,
}

pub struct OrdersNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> OrdersNamespace<'a> {
    /// Resolves the signing context, or names what is missing. These used to be `unwrap`s and
    /// `expect`s, which took the caller's task down rather than failing the order.
    fn signing_context(&self, instrument_name: &str) -> Result<SigningContext, ClientError> {
        Ok(SigningContext {
            subaccount_id: self.ws_client.subaccount_id.ok_or_else(|| {
                ClientError::InvalidOrder("client has no subaccount_id".to_string())
            })?,
            signer: self
                .ws_client
                .wallet
                .clone()
                .ok_or_else(|| ClientError::InvalidOrder("client has no wallet".to_string()))?,
            derive_wallet: self.ws_client.derive_wallet.clone().ok_or_else(|| {
                ClientError::InvalidOrder("client has no derive wallet address".to_string())
            })?,
            instrument: self
                .ws_client
                .instruments_cache
                .get(instrument_name)
                .ok_or_else(|| {
                    ClientError::InvalidOrder(format!(
                        "Instrument {instrument_name} not found in cache"
                    ))
                })?
                .clone(),
        })
    }

    pub async fn place(&self, order_args: OrderArgs) -> Result<OrderCreatedResponse, ClientError> {
        let SigningContext {
            subaccount_id,
            signer,
            instrument,
            derive_wallet,
        } = self.signing_context(&order_args.instrument_name)?;

        let (rounded_price, rounded_amount) =
            snap_to_instrument_grid(&instrument, &order_args.limit_price, &order_args.amount)?;

        let trade_data = TradeData::new(
            &instrument,
            // ticker,
            subaccount_id,
            &rounded_price,
            &rounded_amount,
            order_args.direction == Direction::Buy,
        )?;

        let order_action = ActionData::new(
            trade_data,
            subaccount_id,
            signer.address(),
            &derive_wallet.parse().map_err(|e| {
                ClientError::InvalidOrder(format!(
                    "derive wallet {derive_wallet} is not an address: {e}"
                ))
            })?,
            &self.ws_client.environment,
            ModuleType::Trade,
        )?;
        let params = order_action.populate_order_params(
            &signer,
            order_args.clone(),
            &self.ws_client.environment,
            rounded_price,
            rounded_amount,
        )?;
        self.ws_client.rpc().orderbook().order(params).await
    }

    pub async fn replace(
        &self,
        replace_args: ReplaceArgs,
    ) -> Result<ReplaceOrderResponse, ClientError> {
        let SigningContext {
            subaccount_id,
            signer,
            instrument,
            derive_wallet,
        } = self.signing_context(&replace_args.instrument_name)?;

        let (rounded_price, rounded_amount) =
            snap_to_instrument_grid(&instrument, &replace_args.limit_price, &replace_args.amount)?;

        let trade_data = TradeData::new(
            &instrument,
            // ticker,
            subaccount_id,
            &rounded_price,
            &rounded_amount,
            replace_args.direction == Direction::Buy,
        )?;

        let order_action = ActionData::new(
            trade_data,
            subaccount_id,
            signer.address(),
            &derive_wallet.parse().map_err(|e| {
                ClientError::InvalidOrder(format!(
                    "derive wallet {derive_wallet} is not an address: {e}"
                ))
            })?,
            &self.ws_client.environment,
            ModuleType::Trade,
        )?;
        let params = order_action.populate_replace_params(
            &signer,
            replace_args.clone(),
            &self.ws_client.environment,
            rounded_price,
            rounded_amount,
        )?;

        self.ws_client.rpc().orderbook().replace(params).await
    }

    pub async fn cancel_order(&self, params: CancelOrderRequest) -> Result<Order, ClientError> {
        self.ws_client.rpc().orderbook().cancel(params).await
    }

    pub async fn get_order(&self, order_id: String) -> Result<Order, ClientError> {
        let subaccount_id = self
            .ws_client
            .subaccount_id
            .ok_or_else(|| ClientError::InvalidOrder("client has no subaccount_id".to_string()))?;
        let params: GetOrderRequest = GetOrderRequest::builder()
            .order_id(order_id)
            .subaccount_id(subaccount_id)
            .try_into()?;
        self.ws_client.rpc().orderbook().get_order(params).await
    }

    // pub async fn cancel_all(&self) -> Result<ResultEnum, ClientError> {
    //     let params_json = serde_json::json!({
    //         "subaccount_id": self.ws_client.subaccount_id.unwrap(),
    //     });
    //     self.ws_client
    //         .send_rpc("private/cancel_all", params_json)
    //         .await
    // }

    // pub async fn cancel_by_label(
    //     &self,
    //     label: String,
    // ) -> Result<PrivateCancelByLabelResultSchema, ClientError> {
    //     let params_json = serde_json::json!({
    //         "label": label,
    //         "subaccount_id": self.ws_client.subaccount_id.unwrap(),
    //     });
    //     self.ws_client
    //         .send_rpc("private/cancel_by_label", params_json)
    //         .await
    // }
}
