use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use ethers::middleware::{MiddlewareBuilder, NonceManagerMiddleware, SignerMiddleware};
use ethers::providers::{Http, Middleware, Provider, Ws};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::Address;

use crate::config::chain::{ChainConfig, ChainId};
use crate::config::config::Config;

pub type RpcClient = SignerMiddleware<NonceManagerMiddleware<Provider<Http>>, LocalWallet>;
pub type WsClient = Provider<Ws>;

pub struct Connector {
    address: Address,
    rpc_clients: HashMap<ChainId, Arc<RpcClient>>,
    ws_clients: HashMap<ChainId, Arc<WsClient>>,
}

impl Connector {
    pub fn get_address(&self) -> Address {
        self.address
    }

    pub fn get_rpc_client(&self, chain_id: ChainId) -> Option<Arc<RpcClient>> {
        self.rpc_clients.get(&chain_id).cloned()
    }

    pub fn get_ws_client(&self, chain_id: ChainId) -> Option<Arc<WsClient>> {
        self.ws_clients.get(&chain_id).cloned()
    }
}

impl Connector {
    pub async fn new(config: Config, wallet: LocalWallet) -> Result<Self> {
        let mut rpc_clients: HashMap<ChainId, Arc<RpcClient>> = HashMap::new();
        let mut ws_clients: HashMap<ChainId, Arc<WsClient>> = HashMap::new();
        for chain_config in &config.chains {
            let rpc_client = Self::create_rpc_client(chain_config, wallet.clone())
                .await
                .context(format!(
                    "Failed to create a client for chain {}",
                    chain_config.name
                ))?;
            rpc_clients.insert(chain_config.chain_id, rpc_client);

            let ws_client = Self::create_ws_client(chain_config).await.context(format!(
                "Failed to create a client for chain {}",
                chain_config.name
            ))?;
            ws_clients.insert(chain_config.chain_id, ws_client);
        }
        Ok(Connector {
            address: wallet.address(),
            rpc_clients,
            ws_clients,
        })
    }

    async fn create_rpc_client(
        chain_config: &ChainConfig,
        wallet: LocalWallet,
    ) -> Result<Arc<RpcClient>> {
        let client = Provider::<Http>::try_from(chain_config.rpc_url.clone()).expect(
            format!(
                "Failed to create HTTP client for chain {}",
                chain_config.name
            )
            .as_str(),
        );
        let chain_id = client.get_chainid().await?.as_u64();
        if chain_id != chain_config.chain_id {
            return Err(anyhow!(
                "Chain {} has chain ID '{}' but the configuration set '{}'",
                chain_config.name,
                chain_id,
                chain_config.chain_id
            ));
        }
        let wallet: LocalWallet = wallet.with_chain_id(chain_id);
        let address = wallet.address();
        let client: RpcClient = client.nonce_manager(address).with_signer(wallet);
        Ok(Arc::new(client))
    }

    async fn create_ws_client(chain_config: &ChainConfig) -> Result<Arc<WsClient>> {
        let ws_client: WsClient = Provider::<Ws>::connect(chain_config.ws_url.clone()).await?;
        Ok(Arc::new(ws_client))
    }
}
