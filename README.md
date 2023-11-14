### Overview
Khalani solver off-chain bot built with [Artemis](https://github.com/paradigmxyz/artemis).

### Startup
```shell
cargo run -- --wss <websocket RPC url> --private-key <private key>
```

- `<websocket RPC url>` is the RPC provider's WebSocket URL. The Infura provides those.
- `<private key>` is the solver's wallet containing funds

### End-to-end test of the Intents Mempool
- Start the solver to listen to Sepolia.
- Check out the Khalani SDK (`intents` branch)
- Run the `intents.e2e.test.ts`. The SDK will publish an intent to the Intent Mempool on Sepolia.