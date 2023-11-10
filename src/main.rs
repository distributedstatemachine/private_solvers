use clap::Parser;
use ethers::{
    signers::{LocalWallet, Signer}
};

#[derive(Parser, Debug)]
pub struct Args {
    /// Private key for sending txs.
    #[arg(long)]
    pub private_key: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let wallet: LocalWallet = args
        .private_key
        .parse::<LocalWallet>()
        .unwrap();

    let address = wallet.address();
    println!("Solver address: {}", address);
}
