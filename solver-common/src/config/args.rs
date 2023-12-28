use crate::config::Config;
use anyhow::{Context, Result};
use clap::Parser;
use ethers::prelude::Signer;
use ethers::signers::{AwsSigner, LocalWallet};
use ethers::types::Address;
use rusoto_core::Client;
use rusoto_kms::{Kms, KmsClient};
use tracing::info;

pub enum WalletOrSigner {
    Wallet(LocalWallet),
    Signer(AwsSigner),
}

impl WalletOrSigner {
    fn address(&self) -> Address {
        match self {
            WalletOrSigner::Wallet(wallet) => wallet.address(),
            WalletOrSigner::Signer(signer) => signer.address(),
        }
    }
}

#[derive(Parser, Debug)]
pub struct Args {
    // TODO: move to the config file too.
    /// Private key for sending txs.
    #[arg(long)]
    pub private_key: Option<String>,
    /// KMS ID or alias
    #[arg(long)]
    pub kms_id: Option<String>,

    #[arg(long)]
    pub config_file: String,
}

impl Args {
    pub async fn get_config_and_wallet() -> Result<(Config, WalletOrSigner)> {
        let args = Args::parse();

        let config =
            Config::read_config(args.config_file.as_str()).context("Failed to read config file")?;
        info!(?config, "Config");

        let wallet_or_signer = match (args.private_key, args.kms_id) {
            (Some(private_key), None) => {
                let wallet = private_key
                    .parse::<LocalWallet>()
                    .expect("Failed to parse private key");
                WalletOrSigner::Wallet(wallet)
            }
            (None, Some(kms_id)) => {
                let kms_client = KmsClient::new(Region::UsWest1);
                let signer = AwsSigner::new(kms_client, kms_id, chain_id)
                    .await
                    .expect("Failed to create AWS signer");
                WalletOrSigner::Signer(signer)
            }
            _ => panic!("Either private_key or kms_id must be provided, but not both"),
        };

        let address = wallet_or_signer.address();
        info!(?address, "Solver address");

        Ok((config, wallet_or_signer))
    }
}
