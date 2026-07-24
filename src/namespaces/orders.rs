use crate::{
    actions::{ActionData, ModuleType, OrderArgs, TradeData},
    models::openapi::{Direction, OrderCreatedResponse},
    types::ClientError,
    ws_client::WsClient,
};
// use ethers::prelude::Signer;

pub struct SignableRequest {
    // pub params: CreateOrderRequest,
    // pub signature: String,
    // pub signer_address: String,
}
impl Default for SignableRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl SignableRequest {
    pub fn new() -> Self {
        Self {}
    }

    pub fn sign(
        &self,
        // signer: &impl Signer,
    ) -> Result<SignableRequest, ClientError> {
        // let signature = signer.sign_message(&self.params.to_string()).await?;
        // let signer_address = signer.address().to_string();
        Ok(SignableRequest {
            // params: self.params.clone(),
            // signature: signature.to_string(),
            // signer_address,
        })
    }

    pub async fn send(
        &self,
        // ws_client: &WsClient,
    ) -> Result<OrderCreatedResponse, ClientError> {
        // let params_json = serde_json::to_value(&self.params)?;
        // let signature_json = serde_json::json!({
        //     "signature": self.signature,
        //     "signer_address": self.signer_address,
        // });
        // let request_json = serde_json::json!({
        //     "params": params_json,
        //     "signature": signature_json,
        // });
        // ws_client.send_rpc("private/order", request_json).await
        panic!("Send method not implemented yet");
    }
}

pub struct OrdersNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> OrdersNamespace<'a> {
    pub async fn place(&self, order_args: OrderArgs) -> Result<OrderCreatedResponse, ClientError> {
        let subaccount_id = self.ws_client.subaccount_id.unwrap();
        let signer = self.ws_client.wallet.clone().unwrap();
        let instrument = self
            .ws_client
            .instruments_cache
            .get(&order_args.instrument_name)
            .ok_or_else(|| {
                format!(
                    "Instrument {} not found in cache",
                    order_args.instrument_name
                )
            })
            .expect("Couldnt collect instrument.");
        let trade_data = TradeData::new(
            &instrument,
            // ticker,
            subaccount_id,
            order_args.limit_price.clone(),
            order_args.amount.clone(),
            order_args.direction == Direction::Buy,
        )?;

        let order_action = ActionData::new(
            trade_data,
            subaccount_id,
            signer.address(),
            &self
                .ws_client
                .smart_contract_wallet_address
                .clone()
                .unwrap()
                .parse()
                .expect("Couldnt unwrap."),
            &self.ws_client.environment,
            ModuleType::Trade,
        )?;
        let params = order_action.populate_order_params(
            &signer,
            order_args.clone(),
            &self.ws_client.environment,
        )?;
        // let params_json = serde_json::to_value(&params)?;

        // we pretty print the params for debugging purposes
        println!("Order params: {:?}", params);

        self.ws_client.rpc().orderbook().order(params).await
        // Ok(SignableRequest::new())
    }

    // pub async fn replace_order(
    //     &self,
    //     replace_params: PrivateReplaceParamsSchema,
    // ) -> Result<PrivateReplaceResultSchema, ClientError> {
    //     let subaccount_id = self.ws_client.subaccount_id.unwrap();
    //     let signer = self.ws_client.wallet.clone().unwrap();
    //     let instrument = self
    //         .ws_client
    //         .instruments_cache
    //         .get(&replace_params.instrument_name)
    //         .ok_or_else(|| {
    //             format!(
    //                 "Instrument {} not found in cache",
    //                 replace_params.instrument_name
    //             )
    //         })
    //         .unwrap();
    //     let trade_data = TradeData::new(
    //         &instrument,
    //         // ticker,
    //         subaccount_id,
    //         replace_params.limit_price.clone(),
    //         replace_params.amount.clone(),
    //         replace_params.direction == DirectionEnum::Buy,
    //     )?;

    //     let order_action = ActionData::new(
    //         trade_data,
    //         subaccount_id,
    //         signer.address(),
    //         &self.ws_client.smart_contract_wallet_address.unwrap(),
    //         &self.ws_client.environment,
    //         ModuleType::Trade,
    //     )?;
    //     let params = order_action.populate_replace_params(
    //         &signer,
    //         replace_params.clone(),
    //         &self.ws_client.environment,
    //     )?;
    //     let params_json = serde_json::to_value(&params)?;
    //     self.ws_client
    //         .send_rpc("private/replace", params_json)
    //         .await
    // }
    // pub async fn cancel_order(
    //     &self,
    //     order_id: String,
    //     instrument_name: String,
    // ) -> Result<PrivateCancelResultSchema, ClientError> {
    //     let params_json = serde_json::json!({
    //         "order_id": order_id,
    //         "instrument_name": instrument_name,
    //         "subaccount_id": self.ws_client.subaccount_id.unwrap(),
    //     });
    //     self.ws_client.send_rpc("private/cancel", params_json).await
    // }

    // pub async fn get_order(
    //     &self,
    //     order_id: String,
    // ) -> Result<PrivateGetOrderResultSchema, ClientError> {
    //     let params_json = serde_json::json!({
    //         "order_id": order_id,
    //         "subaccount_id": self.ws_client.subaccount_id.unwrap(),
    //     });
    //     self.ws_client
    //         .send_rpc("private/get_order", params_json)
    //         .await
    // }

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
