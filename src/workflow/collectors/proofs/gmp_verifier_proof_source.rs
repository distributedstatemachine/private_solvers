use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use artemis_core::types::CollectorStream;
use async_stream::__private::AsyncStream;
use async_trait::async_trait;
use bindings_khalani::gmp_event_verifier::GMPEventVerifier;
use ethers::middleware::Middleware;
use futures::StreamExt;

use crate::config::addresses::AddressesConfig;
use crate::config::chain::ChainId;
use crate::connectors::{Connector, RpcClient};
use crate::types::proof_id::ProofId;
use crate::workflow::collectors::proofs::proofs_collector::ProofSource;
use tracing::{debug, error, info};

pub struct GmpEventVerifierProofSource {
    event_verifier: GMPEventVerifier<RpcClient>,
    rpc_client: Arc<RpcClient>,
    verifier_chain_id: ChainId,
    prover_chain_id: ChainId,
}

impl GmpEventVerifierProofSource {
    pub fn new(
        connector: Arc<Connector>,
        verifier_chain_id: ChainId,
        prover_chain_id: ChainId,
        addresses_config: AddressesConfig,
    ) -> Self {
        let rpc_client = connector.get_rpc_client(verifier_chain_id).unwrap();
        let verifier_address = addresses_config
            .verifiers
            .iter()
            .find(|verifier_address| {
                verifier_address.prover_chain_id == prover_chain_id
                    && verifier_address.verifier_chain_id == verifier_chain_id
            })
            .unwrap()
            .verifier_address;
        let event_verifier = GMPEventVerifier::new(verifier_address, rpc_client.clone());
        Self {
            rpc_client: rpc_client.clone(),
            event_verifier,
            verifier_chain_id,
            prover_chain_id,
        }
    }
}

#[async_trait]
impl ProofSource for GmpEventVerifierProofSource {
    async fn get_proof_ids_stream(&self) -> Result<CollectorStream<'_, ProofId>> {
        let prover_chain_id: ChainId = self.prover_chain_id;
        let verifier_chain_id: ChainId = self.prover_chain_id;
        let mut previous_block_number = match self.rpc_client.get_block_number().await {
            Ok(block_number) => block_number,
            Err(e) => {
                error!(
                    ?prover_chain_id,
                    ?verifier_chain_id,
                    ?e,
                    "Error fetching block"
                );
                return Err(e.into());
            }
        };
        info!(
            ?previous_block_number,
            ?prover_chain_id,
            "Starting block number"
        );
        let mut logged_last_indexed_block_number = previous_block_number;

        let event_stream: AsyncStream<Result<ProofId>, _> = async_stream::try_stream! {
            // TODO: use sub graphs connection.
            loop {
                let current_block_number = match self.rpc_client.get_block_number().await {
                    Ok(block_number) => block_number,
                    Err(e) => {
                        error!(?prover_chain_id, ?verifier_chain_id, ?e, "Error fetching block");
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                };

                if previous_block_number >= current_block_number {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue
                }

                if logged_last_indexed_block_number + 50 < current_block_number {
                    debug!(address = ?self.event_verifier.address(), ?previous_block_number, ?current_block_number, ?prover_chain_id, ?verifier_chain_id, "Indexing the GMP verifier");
                    logged_last_indexed_block_number = current_block_number;
                }

                let event = self
                    .event_verifier
                    .new_event_registered_filter()
                    .from_block(previous_block_number)
                    .to_block(current_block_number);

                let events = match event.query().await {
                    Ok(events) => events,
                    Err(e) => {
                        error!(?e, ?prover_chain_id, ?verifier_chain_id, "Error querying events");
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                };

                for event in events {
                    let proof_id: ProofId = event.event_hash.into();
                    info!(?proof_id, ?prover_chain_id, ?verifier_chain_id, "GMP Event Verifier received a new proof");
                    yield proof_id;
                }

                previous_block_number = current_block_number + 1;
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

    fn get_prover_chain_id(&self) -> ChainId {
        self.prover_chain_id
    }

    fn get_verifier_chain_id(&self) -> ChainId {
        self.verifier_chain_id
    }
}
