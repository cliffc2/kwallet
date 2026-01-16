```markdown
# kaspa-wallet (scaffold)

This repository is a minimal Rust scaffold for a Kaspa wallet:

What is implemented
- Create a new 12-word BIP-39 mnemonic
- Restore from a mnemonic
- Derive secp256k1 private keys via BIP-32 derivation path
- Export compressed public key (hex)
- Encrypt and store the mnemonic using Argon2id + AES-GCM
- RPC client skeleton to query balances and broadcast transactions (you must adapt to your Kaspa node API)
- CLI commands: new, restore, address, export-pub, balance, send (send is a TODO skeleton)

Why a scaffold?
- Kaspa's transaction and address encoding differ from Bitcoin; to create valid transactions you should either:
  - Use a Kaspa Rust crate that implements transaction serialization/signing, or
  - Implement Kaspa transaction serialization according to Kaspa protocol, then plug it into `src/main.rs` and `src/rpc_client.rs`.

How to build
1. Install Rust (stable).
2. cargo build --release

Example usage
- Create a wallet:
  cargo run --release -- --file wallet.dat new --passphrase "my strong pass"

- Restore a wallet:
  cargo run --release -- --file wallet.dat restore --mnemonic "seed words ..." --passphrase "my strong pass"

- Show derived pubkey (you must implement address encoding):
  cargo run --release -- --file wallet.dat address --path "m/44'/0'/0'/0/0" --passphrase "my strong pass"

Next steps to finish a working Kaspa wallet
1. Implement Kaspa address encoding from the derived public key (Kaspa uses a specific format).
2. Implement UTXO discovery (node RPC may provide address UTXOs) and coin selection.
3. Implement Kaspa transaction construction and signing, producing the raw tx hex.
4. Use `RpcClient::broadcast(tx_hex)` to publish signed txs.
5. Consider adding tests, integration with a Kaspa testnet node, and hardened derivation and account management (BIP-44 style).

Security notes
- Protect your passphrase and encrypted seed file.
- Consider hardware wallet integration for long-term storage.
- Tune Argon2 params to match your machine's capabilities and desired hardness.

If you want, I can:
- Implement Kaspa-specific address encoding and transaction serialization if you point me to the Kaspa transaction spec or a Kaspa Rust crate to reuse.
- Add UTXO fetching and a simple coin-selection algorithm.
- Add unit tests and CI.

Which of those next steps shall I implement now?
```
