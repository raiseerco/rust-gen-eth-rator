# Rust gen-eth-rator

A simple Ethereum wallet generator written in Rust, starting from a mnemonic phrase (BIP-39) with BIP-44 std derivation  (`m/44'/60'/0'/0/0`)

## Requirements

- Linux / macOS
- Rust and Cargo installed (https://rustup.rs)

## Installation

Clone and compiling:

```bash
git clone https://github.com/usuario/walletgen.git
cd walletgen
cargo build --release
```

Running from the binary:

```bash
./target/release/walletgen
```

Running from code
```bash
cargo run
```
