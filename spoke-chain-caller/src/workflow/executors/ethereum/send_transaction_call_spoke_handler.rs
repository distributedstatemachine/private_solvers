use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use bindings_khalani::spoke_chain_executor::SpokeChainExecutor;
use ethers::contract::ContractCall;
use tracing::info;

use crate::types::spoke_chain_call::SpokeChainCall;
use crate::workflow::executors::call_spoke_executor::{CallSpokeHandler, CallSpokeHandlerResult};
use solver_common::config::addresses::AddressesConfig;
use solver_common::connectors::{Connector, RpcClient};
use solver_common::ethereum::transaction::submit_transaction;

pub struct SendTransactionCallSpokeHandler {
    connector: Arc<Connector>,
    addresses_config: AddressesConfig,
}

impl SendTransactionCallSpokeHandler {
    pub fn new(addresses_config: AddressesConfig, connector: Arc<Connector>) -> Self {
        Self {
            addresses_config,
            connector,
        }
    }
}

#[async_trait]
impl CallSpokeHandler for SendTransactionCallSpokeHandler {
    async fn call_spoke(&self, spoke_chain_call: SpokeChainCall) -> Result<CallSpokeHandlerResult> {
        info!(?spoke_chain_call, "Calling spoke chain contract");
        let transaction = self.build_call_spoke_chain_tx(&spoke_chain_call)?;
        let receipt = submit_transaction(transaction).await?;
        let tx_hash = receipt.transaction_hash;
        info!(
            ?spoke_chain_call,
            ?tx_hash,
            "SpokeChainExecutor executed successfully"
        );
        Ok(CallSpokeHandlerResult {
            spoke_chain_call,
            calling_tx_hash: tx_hash,
        })
    }
}

impl SendTransactionCallSpokeHandler {
    fn build_call_spoke_chain_tx(
        &self,
        spoke_chain_call: &SpokeChainCall,
    ) -> Result<ContractCall<RpcClient, ()>> {
        let rpc_client = self.connector.get_rpc_client(spoke_chain_call.chain_id)?;
        let spoke_chain_executor_address = self.addresses_config.spoke_chain_executor_address;
        let spoke_chain_executor =
            SpokeChainExecutor::new(spoke_chain_executor_address, rpc_client);
        let mut call = spoke_chain_executor.call_spoke(
            spoke_chain_call.intent_id.into(),
            spoke_chain_call.contract_to_call,
            spoke_chain_call.call_data.clone(),
            spoke_chain_call.token,
            spoke_chain_call.amount,
        );
        call.tx
            .set_chain_id(Into::<u32>::into(spoke_chain_call.chain_id));
        Ok(call)
    }
}
