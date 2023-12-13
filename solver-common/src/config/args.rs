use crate::config::Config;
use anyhow::{Context, Result};
use clap::Parser;
use ethers::prelude::Signer;
use ethers::signers::LocalWallet;
use tracing::info;

#[derive(Parser, Debug)]
pub struct Args {
    // TODO: move to the config file too.
    /// Private key for sending txs.
    #[arg(long)]
    pub private_key: String,

    #[arg(long)]
    pub config_file: String,
}

impl Args {
    pub fn get_config_and_wallet() -> Result<(Config, LocalWallet)> {
        let args = Args::parse();

        let config =
            Config::read_config(args.config_file.as_str()).context("Failed to read config file")?;
        info!(?config, "Config");

        let wallet: LocalWallet = args
            .private_key
            .parse::<LocalWallet>()
            .expect("Failed to parse private key");
        let address = wallet.address();
        info!(?address, "Solver address");

        Ok((config, wallet))
    }
}
