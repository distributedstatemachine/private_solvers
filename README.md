### Overview
Khalani solver off-chain bot built with [Artemis](https://github.com/paradigmxyz/artemis).

### Startup
```shell
export SEPOLIA_RPC_URL=<sepolia RPC URL>
export SEPOLIA_WS_URL=<sepolia WS URL>
export FUJI_RPC_URL=<fuji RPC URL>
export FUJI_WS_URL=<fuji WS URL>
cargo run -- --config-file <config file> --private-key <private key>
```

- `<sepolia RPC URL>` — JSON RPC provider URL 
- `<sepolia WS URL>` — WebSocket provider URL
- `<private key>` — the solver's wallet
- `<config file>` — path to `./config/token_config.json`

### End-to-end test of the Intents Mempool
- Start the solver to listen to Sepolia.
- Check out the Khalani SDK (`intents` branch)
- Run the `intents.e2e.test.ts`. The SDK will publish an intent to the Intent Mempool on Sepolia.

### Running tests
```shell
export SEPOLIA_RPC_URL=<sepolia RPC URL>
export SEPOLIA_WS_URL=<sepolia WS URL>
export FUJI_RPC_URL=<fuji RPC URL>
export FUJI_WS_URL=<fuji WS URL>
cargo test
```

### Example environment variables
```
export SEPOLIA_RPC_URL=https://ethereum-sepolia.publicnode.com; export SEPOLIA_WS_URL=wss://ethereum-sepolia.publicnode.com; export FUJI_RPC_URL=https://avalanche-fuji-c-chain.publicnode.com; export FUJI_WS_URL=wss://avalanche-fuji-c-chain.publicnode.com;
```