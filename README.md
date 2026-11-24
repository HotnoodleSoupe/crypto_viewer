# crypto_viewer

A fast, lightweight Rust CLI utility to track and manage cryptocurrency transactions locally using SQLite, featuring live market pricing and blockchain data via CoinGecko and Mempool APIs.

## Features

- **Local SQLite Storage:** Fast, offline-capable transaction database.
- **CSV Import & Export:** Bulk import/export of transaction histories.
- **Live Price & Block Data:** Fetches current market prices and block information from CoinGecko and Mempool APIs.

## Build from Source

Requires Rust and Cargo (`rustup`):

    git clone https://github.com/HotnoodleSoupe/crypto_viewer.git
    cd crypto_viewer
    cargo build --release

The compiled binary is located at `target/release/crypto_viewer`.

## Usage

    ./crypto_viewer [OPTIONS]

### Options

| Flag | Description |
| :--- | :--- |
| `-i` | Import transactions from CSV |
| `-e` | Export transactions to CSV |
| `-I` | Interactive mode / Inspect entries |
| `-n` | Add a new transaction |
| `-d` | Delete a transaction |
| `-h`, `--help` | Print help information |