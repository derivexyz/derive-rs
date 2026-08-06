use crate::{
    actions::{
        ActionData, ExecuteQuoteArgs, ModuleType, RfqExecuteData, SendQuoteArgs,
        TransferPositionsData,
    },
    models::{
        CancelBatchRfqsRequest, CancelBatchRfqsResponse, Direction, Instrument, PollRfqsRequest,
        Quote, QuoteExecuteResponse, Rfq, RfqGetBestQuoteRequest, RfqGetBestQuoteResponse,
        RfqPollResponse, SendRfqRequest,
    },
    types::ClientError,
    ws_client::WsClient,
};
pub struct RfqsNamespace<'a> {
    pub ws_client: &'a WsClient,
}
impl<'a> RfqsNamespace<'a> {
    pub async fn send_rfq(&self, request: SendRfqRequest) -> Result<Rfq, ClientError> {
        self.ws_client.rpc().rfq().send_rfq(request).await
    }

    pub async fn poll_rfqs(
        &self,
        request: PollRfqsRequest,
    ) -> Result<RfqPollResponse, ClientError> {
        self.ws_client.rpc().rfq().poll_rfqs(request).await
    }

    pub async fn send_quote(&self, request: SendQuoteArgs) -> Result<Quote, ClientError> {
        let instrument_map = self
            .ws_client
            .instruments_cache
            .iter()
            .map(|entry| {
                let instrument = entry.value();
                (instrument.instrument_name.clone(), instrument.clone())
            })
            .collect::<std::collections::HashMap<String, Instrument>>();

        let rfq_data = TransferPositionsData::from_send_quote_args(
            request.clone(),
            Direction::Sell,
            1,
            &instrument_map,
        )?;
        let action_data = ActionData::new(
            rfq_data,
            self.ws_client.subaccount_id.unwrap(),
            self.ws_client.wallet.clone().unwrap().address(),
            &self
                .ws_client
                .smart_contract_wallet_address
                .clone()
                .unwrap()
                .parse()
                .expect("Couldnt unwrap."),
            &self.ws_client.environment,
            ModuleType::RfqPositionTransfer,
        )?;

        let params = action_data.populate_send_quote(
            &self.ws_client.wallet.clone().unwrap(),
            request.clone(),
            &self.ws_client.environment,
            self.ws_client.subaccount_id.unwrap(),
        )?;

        println!("Params: {:?}", params);
        self.ws_client.rpc().rfq().send_quote(params).await
    }

    pub async fn execute_best_quote(
        &self,
        request: ExecuteQuoteArgs,
    ) -> Result<QuoteExecuteResponse, ClientError> {
        let instrument_map = self
            .ws_client
            .instruments_cache
            .iter()
            .map(|entry| {
                let instrument = entry.value();
                (instrument.instrument_name.clone(), instrument.clone())
            })
            .collect::<std::collections::HashMap<String, Instrument>>();

        let rfq_data = RfqExecuteData::from_execute_quote_args(
            request.clone(),
            &instrument_map,
            Direction::Buy,
            -1,
        )?;
        let action_data = ActionData::new(
            rfq_data.clone(),
            self.ws_client.subaccount_id.unwrap(),
            self.ws_client.wallet.clone().unwrap().address(),
            &self
                .ws_client
                .smart_contract_wallet_address
                .clone()
                .unwrap()
                .parse()
                .expect("Couldnt unwrap."),
            &self.ws_client.environment,
            ModuleType::RfqPositionTransfer,
        )?;

        let params = action_data.populate_execute_quote(
            &self.ws_client.wallet.clone().unwrap(),
            request.clone(),
            &self.ws_client.environment,
            self.ws_client.subaccount_id.unwrap(),
        )?;

        println!("Params: {:?}", params);
        self.ws_client.rpc().rfq().execute_quote(params).await
    }

    pub async fn cancel_batch_rfqs(
        &self,
        request: CancelBatchRfqsRequest,
    ) -> Result<CancelBatchRfqsResponse, ClientError> {
        self.ws_client.rpc().rfq().cancel_batch_rfqs(request).await
    }

    pub async fn get_best_quote(
        &self,
        request: RfqGetBestQuoteRequest,
    ) -> Result<RfqGetBestQuoteResponse, ClientError> {
        self.ws_client.rpc().rfq().rfq_get_best_quote(request).await
    }
}
