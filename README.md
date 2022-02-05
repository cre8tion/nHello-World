# Near Hello World

## Available commands

### Building contract
To run build:
```bash
cargo build --target wasm32-unknown-unknown --release
```

### Testing contract
To test run:
```bash
cargo test --package near_hello_world -- --nocapture
```

### Deploy contract
To run deployment:
```bash
near deploy --wasmFile target/wasm32-unknown-unknown/release/near_hello_world.wasm --accountId YOUR_ACCOUNT_HERE
```