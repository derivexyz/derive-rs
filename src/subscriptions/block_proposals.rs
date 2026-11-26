use crate::{
    models::*, subscriptions::channel_specs::block_proposals::*,
    types::{ClientError, EventStream},
    ws_client::WsClient,
};
pub struct BlockProposalsSubscriptions<'a> {
    client: &'a WsClient,
}
impl<'a> BlockProposalsSubscriptions<'a> {
    pub fn new(client: &'a WsClient) -> Self {
        Self { client }
    }
    pub async fn block_proposals(
        &self,
    ) -> Result<EventStream<BlockProposalsNotification>, ClientError> {
        self.client.subscribe(BlockProposalsChannelSpec {}).await
    }
    pub async fn wallet_block_proposals(
        &self,
        wallet: &str,
    ) -> Result<EventStream<WalletBlockProposalsNotification>, ClientError> {
        self.client
            .subscribe(WalletBlockProposalsChannelSpec {
                wallet: wallet.to_owned(),
            })
            .await
    }
}
