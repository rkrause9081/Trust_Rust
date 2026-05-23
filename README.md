# T.R.U.S.T Auction Protocol

A full-stack decentralized auction and escrow platform built with:

* Rust
* Axum
* Alloy
* Solidity
* Hardhat
* JavaScript frontend

This project implements an Ethereum-based auction system with:

* on-chain bidding
* escrow settlement flows
* SIWE (Sign-In With Ethereum) authentication
* auction registry queries
* refund/withdrawal handling
* Rust backend APIs
* Solidity smart contract testing
* modular blockchain client architecture

---

# Architecture

```text
Frontend (HTML / JS)
        ↓
Axum Web Server (trust_rust_web)
        ↓
Route Handlers + AppState
        ↓
Blockchain Client Layer (trust_rust_client)
        ↓
Alloy Provider
        ↓
Ethereum / Hardhat
        ↓
Solidity Smart Contracts
```

---

# Project Structure

```text
TRUST_RUST/
│
├── HH Blockchain node/
│   ├── contracts/
│   │   ├── AuctionFactory.sol
│   │   ├── AuctionRegistry.sol
│   │   └── Auction/
│   │       ├── AuctionState.sol
│   │       ├── AuctionEscrow.sol
│   │       ├── AuctionSettlement.sol
│   │       └── SimpleAuction.sol
│   │
│   ├── scripts/
│   │   ├── deploy_factory.js
│   │   └── create_auction.js
│   │
│   ├── test/
│   │   ├── auctionFactory.test.js
│   │   └── auctionProtocol.test.js
│   │
│   ├── hardhat.config.js
│   └── package.json
│
├── trust_rust_client/
│   ├── src/
│   │   ├── bidding.rs
│   │   ├── create_auction.rs
│   │   ├── escrow.rs
│   │   ├── registry.rs
│   │   ├── withdraw.rs
│   │   ├── config.rs
│   │   └── auction_loader.rs
│   │
│   └── tests/
│
├── trust_rust_web/
│   ├── src/
│   │   ├── auth.rs
│   │   ├── state.rs
│   │   ├── routes/
│   │   └── main.rs
│   │
│   └── static/
│       ├── js/
│       ├── css/
│       └── index.html
│
└── README.md
```

---

# Features

## Smart Contract Features

* AuctionFactory contract
* Dynamic auction deployment
* Auction registry system
* On-chain bidding
* Highest bid tracking
* Escrow settlement system
* Refund support
* Withdrawal pattern for safe refunds
* Confirmation timeout handling
* Seller timeout claim flow
* Solidity unit/integration tests

---

## Rust Backend Features

* Axum HTTP API server
* Shared application state with `Arc` + `Mutex`
* Alloy Ethereum provider integration
* Structured module separation
* Async blockchain interaction
* SIWE authentication
* Session cookie management
* JSON REST API responses
* Rust integration test suite

---

## Frontend Features

* Wallet authentication
* Active auction list
* Bid placement UI
* Withdraw flow
* Escrow interaction controls
* Live frontend refresh

---

# Technology Stack

| Layer                  | Technology              |
| ---------------------- | ----------------------- |
| Smart Contracts        | Solidity                |
| Ethereum Tooling       | Hardhat                 |
| Backend                | Rust                    |
| Web Framework          | Axum                    |
| Ethereum Library       | Alloy                   |
| Async Runtime          | Tokio                   |
| Frontend               | HTML / CSS / JavaScript |
| Authentication         | SIWE                    |
| Smart Contract Testing | Mocha / Chai            |

---

# Getting Started

## Prerequisites

Install:

* Rust
* Cargo
* Node.js
* npm

---

## Clone Repository

```bash
git clone <your-repo-url>
cd TRUST_RUST
```

---

# Smart Contract Setup

## Install Dependencies

```bash
cd "HH Blockchain node"
npm install
```

---

## Compile Contracts

```bash
rm -rf artifacts cache
npx hardhat compile
```

---

## Start Local Hardhat Node

Open a terminal:

```bash
npx hardhat node
```

---

## Deploy AuctionFactory Contract

Open another terminal:

```bash
npx hardhat run scripts/deploy_factory.js --network localhost
```

Optional:

```bash
npx hardhat run scripts/create_auction.js --network localhost
```

After deployment:

1. Copy the deployed factory address
2. Update `.env`

Example:

```env
FACTORY_ADDRESS=0x...
```

3. Restart the Rust web server if already running

The Rust backend uses this deployed factory contract for:

* auction creation
* registry queries
* escrow lifecycle operations

---

# Rust Setup

## Build Project

```bash
cargo build
```

---

## Format Rust Code

```bash
cargo fmt --all
```

---

## Run Clippy

```bash
cargo clippy
```

---

# Environment Variables

Example `.env`:

```env
RPC_URL=http://127.0.0.1:8545

FACTORY_ADDRESS=0x...
SELLER_ADDRESS=0x...
ADMIN_ADDRESS=0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266
BIDDER_ADDRESS=0xa0ee7a142d267c1f36714e4a8f75612f20a79720
BUYER_ADDRESS=0xbcd4042de499d14e55001ccbb24a551f3b954096

BIDDING_TIME_SECONDS=3600
STARTING_BID_WEI=1000000000000000000
CONFIRMATION_WINDOW=259200
REGISTRY_PAGE_LIMIT=20
```

---

# Running the Web Server

```bash
cargo run -p trust_rust_web
```

Server runs at:

```text
http://localhost:3000
```

---

# Running Tests

## Solidity Smart Contract Tests

Run all Solidity tests:

```bash
npm test
```

or:

```bash
npx hardhat test
```

These tests cover:

* auction creation
* registry queries
* bidding logic
* pending returns
* withdraw pattern
* escrow settlement
* timeout claims
* refund handling

---

## Rust Integration Tests

Run all Rust tests:

```bash
cargo test
```

Run with output:

```bash
cargo test -- --nocapture
```

Example targeted tests:

```bash
cargo test test_create_auction_emits_created_event -- --nocapture
cargo test test_registry_reads_created_auction -- --nocapture
cargo test test_place_bid_updates_highest_bid -- --nocapture
cargo test test_escrow_interactive -- --nocapture
```

Rust integration tests automatically:

* create fresh auctions
* place setup bids
* manipulate Hardhat blockchain time
* verify escrow settlement flows

---

# API Endpoints

## Authentication

| Method | Endpoint       | Description           |
| ------ | -------------- | --------------------- |
| GET    | `/auth/nonce`  | Generate SIWE nonce   |
| POST   | `/auth/verify` | Verify SIWE signature |

---

## Auctions

| Method | Endpoint              | Description     |
| ------ | --------------------- | --------------- |
| POST   | `/api/create-auction` | Create auction  |
| GET    | `/api/auctions`       | List auctions   |
| POST   | `/api/bid`            | Place bid       |
| POST   | `/api/withdraw`       | Withdraw refund |

---

## Escrow

| Method | Endpoint                               | Description          |
| ------ | -------------------------------------- | -------------------- |
| GET    | `/api/escrow/status/{auction_address}` | Escrow state         |
| POST   | `/api/escrow/end`                      | End auction          |
| POST   | `/api/escrow/confirm`                  | Confirm receipt      |
| POST   | `/api/escrow/claim-timeout`            | Seller timeout claim |
| POST   | `/api/escrow/refund`                   | Admin refund         |

---

# Rust Concepts Used

This project heavily uses:

* async/await
* Futures
* Result-based error handling
* `Arc` shared ownership
* `Mutex` interior mutability
* trait abstraction
* modular architecture
* builder patterns
* integration testing
* layered error propagation

---

# Security Notes

Current implementation is intended for local development/testing.

Production improvements would include:

* persistent session storage
* HTTPS-only secure cookies
* Redis/database-backed state
* stronger authorization rules
* rate limiting
* production Ethereum wallet integration
* hardened contract audits

---

# Future Improvements

* WebSocket event subscriptions
* Real-time auction updates
* Database persistence
* Production wallet support
* IPFS metadata storage
* Better frontend UX
* Advanced auction filtering
* Multi-network deployment support
* Event indexing
* Gas snapshot testing
* Fuzz testing

---

# Learning Goals

This project was originally inspired by a group project written in Python.
It was later fully refactored and expanded into Rust to explore:

* systems programming concepts
* async Rust
* backend architecture
* Ethereum smart contract integration
* blockchain infrastructure development
* full-stack Web3 engineering
* smart contract testing
* protocol-level escrow design

---

# License

MIT License
