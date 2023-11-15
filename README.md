### Overview
Khalani solver off-chain bot built with [Artemis](https://github.com/paradigmxyz/artemis).

### Startup
```shell
cargo run -- --rpc <provider RPC url> --wss <websocket RPC url> --private-key <private key>
```

- `<provider RPC url>` — RPC provider's URL 
- `<websocket RPC url>` — RPC provider's WebSocket URL
- `<private key>` — the solver's wallet

### End-to-end test of the Intents Mempool
- Start the solver to listen to Sepolia.
- Check out the Khalani SDK (`intents` branch)
- Run the `intents.e2e.test.ts`. The SDK will publish an intent to the Intent Mempool on Sepolia.