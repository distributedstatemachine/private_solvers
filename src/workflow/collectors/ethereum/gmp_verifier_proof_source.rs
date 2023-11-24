use std::sync::Arc;

use anyhow::Result;
use artemis_core::types::CollectorStream;
use async_stream::__private::AsyncStream;
use async_trait::async_trait;
use bindings_khalani::gmp_event_verifier::GMPEventVerifier;
use ethers::middleware::Middleware;
use ethers::types::H256;
use futures::StreamExt;

use crate::config::addresses::AddressesConfig;
use crate::config::chain::ChainId;
use crate::connectors::connector::{Connector, RpcClient};
use crate::types::proof_id::ProofId;
use crate::workflow::collectors::proofs_collector::ProofSource;
use tracing::info;

pub struct GmpEventVerifierProofSource {
    event_verifier: GMPEventVerifier<RpcClient>,
    rpc_client: Arc<RpcClient>,
    chain_id: ChainId,
}

impl GmpEventVerifierProofSource {
    pub fn new(
        connector: Arc<Connector>,
        chain_id: ChainId,
        addresses_config: AddressesConfig,
    ) -> Self {
        let rpc_client = connector.get_rpc_client(chain_id).unwrap();
        let event_verifier = GMPEventVerifier::new(
            addresses_config.khalani_chain_event_verifier_address,
            rpc_client.clone(),
        );
        Self {
            rpc_client: rpc_client.clone(),
            event_verifier,
            chain_id,
        }
    }
}

#[async_trait]
impl ProofSource for GmpEventVerifierProofSource {
    async fn get_proof_ids_stream(&self) -> Result<CollectorStream<'_, ProofId>> {
        let event_stream: AsyncStream<Result<ProofId>, _> = async_stream::try_stream! {
            let starting_block_number = self.rpc_client.get_block_number().await?;
            let chain_id: ChainId = self.chain_id;
            info!(current_block_number = ?starting_block_number, ?chain_id, "Current block number");
            let event = self
                .event_verifier
                .new_event_registered_filter()
                .from_block(starting_block_number);
            let mut event_stream = event.stream().await?;

            while let Some(Ok(new_event)) = event_stream.next().await {
               let proof_id: ProofId = H256::from(new_event.event_hash);
               let chain_id: ChainId = self.chain_id;
               info!(?proof_id, ?chain_id, "GMP Event Verifier received a new proof");
               yield proof_id;
            }
        };
        let event_stream = Box::pin(event_stream);
        let event_stream = event_stream.filter_map(|result| async move {
            match result {
                Ok(proof_id) => Some(proof_id),
                Err(_) => None,
            }
        });
        Ok(Box::pin(event_stream))
    }

    fn get_chain_id(&self) -> ChainId {
        self.chain_id
    }
}
