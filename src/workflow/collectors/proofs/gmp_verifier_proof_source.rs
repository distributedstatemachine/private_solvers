use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use artemis_core::types::CollectorStream;
use async_stream::__private::AsyncStream;
use async_trait::async_trait;
use bindings_khalani::gmp_event_verifier::GMPEventVerifier;
use ethers::middleware::Middleware;
use ethers::types::ValueOrArray;
use futures::StreamExt;

use crate::config::addresses::VerifierConfig;
use crate::connectors::{Connector, RpcClient};
use crate::types::proof_id::ProofId;
use crate::workflow::collectors::proofs::proofs_collector::ProofSource;

use tracing::{debug, error, info};

pub struct GmpEventVerifierProofSource {
    event_verifier: GMPEventVerifier<RpcClient>,
    rpc_client: Arc<RpcClient>,
    verifier_config: VerifierConfig,
}

impl GmpEventVerifierProofSource {
    pub fn new(connector: Arc<Connector>, verifier_config: VerifierConfig) -> Self {
        let rpc_client = connector
            .get_rpc_client(verifier_config.verifier_chain_id)
            .unwrap();
        let event_verifier =
            GMPEventVerifier::new(verifier_config.verifier_address, rpc_client.clone());
        Self {
            rpc_client: rpc_client.clone(),
            event_verifier,
            verifier_config,
        }
    }
}

#[async_trait]
impl ProofSource for GmpEventVerifierProofSource {
    async fn get_proof_ids_stream(&self) -> Result<CollectorStream<'_, ProofId>> {
        let verifier_config = self.verifier_config.clone();
        let mut previous_block_number = match self.rpc_client.get_block_number().await {
            Ok(block_number) => block_number,
            Err(e) => {
                error!(?verifier_config, ?e, "Error fetching block");
                return Err(e.into());
            }
        };
        info!(
            ?previous_block_number,
            ?verifier_config,
            "Starting block number"
        );
        let mut logged_last_indexed_block_number = previous_block_number;

        let event_stream: AsyncStream<Result<ProofId>, _> = async_stream::try_stream! {
            // TODO: use sub graphs connection.
            loop {
                let current_block_number = match self.rpc_client.get_block_number().await {
                    Ok(block_number) => block_number,
                    Err(e) => {
                        error!(?verifier_config, ?e, "Error fetching block");
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                };

                if previous_block_number >= current_block_number {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    continue
                }

                if logged_last_indexed_block_number + 50 < current_block_number {
                    debug!(address = ?self.event_verifier.address(), ?previous_block_number, ?current_block_number, ?verifier_config, "Indexing the GMP verifier");
                    logged_last_indexed_block_number = current_block_number;
                }

                let event = self
                    .event_verifier
                    .new_event_registered_filter()
                    .address(ValueOrArray::Value(verifier_config.verifier_address))
                    .from_block(previous_block_number)
                    .to_block(current_block_number);

                let events = match event.query().await {
                    Ok(events) => events,
                    Err(e) => {
                        error!(?e, ?verifier_config, "Error querying events");
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        continue;
                    }
                };

                for event in events {
                    let proof_id: ProofId = event.event_hash.into();
                    info!(?proof_id, ?verifier_config, "GMP Event Verifier received a new proof");
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

    fn get_verifier_config(&self) -> VerifierConfig {
        self.verifier_config.clone()
    }
}
