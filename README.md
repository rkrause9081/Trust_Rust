# T.R.U.S.T Auction Protocol

A full-stack decentralized auction and escrow platform built with:

* Rust
* Axum
* Alloy
* Solidity
* Hardhat
* JavaScript

The protocol implements:

* On-chain auction creation
* On-chain bidding
* Escrow settlement flows
* Withdrawal/refund handling
* Auction registry queries
* SIWE-style wallet authentication
* Session cookie management
* Rust backend APIs
* Solidity smart contracts
* Modular frontend architecture

---

# Architecture

```text
Frontend (HTML / CSS / JS)
            │
            ▼
     Axum Web Server
      (trust_rust_web)
            │
            ▼
       Route Handlers
         + AppState
            │
            ▼
 Blockchain Client Layer
   (trust_rust_client)
            │
            ▼
      Alloy Provider
            │
            ▼
 Ethereum / Hardhat Node
            │
            ▼
     Solidity Contracts
```

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
| Authentication         | SIWE-style Signatures   |
| Smart Contract Testing | Mocha / Chai            |

---

# Project Structure

```text
TRUST_RUST/
│
├── HH Blockchain node/
│   ├── contracts/
│   ├── scripts/
│   ├── test/
│   ├── hardhat.config.js
│   └── package.json
│
├── trust_rust_client/
│   ├── src/
│   └── tests/
│
├── trust_rust_web/
│   ├── src/
│   ├── static/
│   │   ├── index.html
│   │   ├── css/
│   │   └── js/
│   └── tests/
│
├── Dockerfile
├── docker-compose.yml
├── .env
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
* Escrow settlement
* Refund support
* Withdrawal pattern
* Confirmation timeout handling
* Seller timeout claim flow
* Solidity unit and integration tests

---

## Rust Backend Features

* Axum HTTP API
* Shared application state
* Alloy Ethereum integration
* Async blockchain interaction
* Session cookie management
* SIWE-style authentication
* Modular route architecture
* JSON REST APIs
* Integration testing

---

## Frontend Features

* Wallet authentication
* Dashboard statistics
* Active auction list
* Auction creation
* Bid placement
* Withdrawal workflow
* Escrow interaction controls
* Shared auction state cache
* Modular API/state/render architecture

---

# Quick Start

The project supports two workflows.

| Branch | Purpose            |
| ------ | ------------------ |
| `main` | Docker deployment  |
| `dev`  | Manual development |

---

# Main Branch (Docker)

Recommended for users, evaluators, demonstrations, and deployment testing.

## Prerequisites

Install:

* Docker
* Docker Compose

Verify installation:

```bash
docker --version
docker compose version
```

---

## Clone Repository

```bash
git clone <repository-url>
cd TRUST_RUST
git checkout main
```

---

## Configure Environment

Create:

```bash
.env.docker
```

Example:

```env
RPC_URL=http://host.docker.internal:8545

FACTORY_ADDRESS=0x...

SELLER_ADDRESS=0x...
ADMIN_ADDRESS=0x...
BIDDER_ADDRESS=0x...
BUYER_ADDRESS=0x...

BIDDING_TIME_SECONDS=3600
STARTING_BID_WEI=1000000000000000000
CONFIRMATION_WINDOW=259200
REGISTRY_PAGE_LIMIT=20
```

---

## Build Docker Image

Linux users may need host networking during Cargo dependency resolution:

```bash
sudo docker build --network=host -t trust-rust-web .
```

---

## Start Application

```bash
sudo docker compose up
```

Application URL:

```text
http://localhost:3000
```

---

## Stop Application

```bash
sudo docker compose down
```

---

## Rebuild Application

```bash
sudo docker compose down

sudo docker build \
  --network=host \
  -t trust-rust-web .

sudo docker compose up
```

---

# Development Branch (Manual Development)

Recommended for contributors working on contracts, backend code, and frontend functionality.

## Prerequisites

Install:

* Rust
* Cargo
* Node.js
* npm
* Hardhat

---

## Clone Repository

```bash
git clone <repository-url>
cd TRUST_RUST
git checkout dev
```

---

# Terminal 1 — Start Hardhat Node

```bash
cd "HH Blockchain node"

npm install

npx hardhat node
```

Blockchain RPC:

```text
http://127.0.0.1:8545
```

---

# Terminal 2 — Deploy Contracts

```bash
cd "HH Blockchain node"

npx hardhat run \
  scripts/deploy_factory.js \
  --network localhost
```

Copy the deployed factory address.

Update:

```env
FACTORY_ADDRESS=0x...
```

Restart the Rust server after updating.

Optional:

```bash
npx hardhat run \
  scripts/create_auction.js \
  --network localhost
```

---

# Terminal 3 — Start Rust Backend

From the workspace root:

```bash
cargo run -p trust_rust_web
```

Application URL:

```text
http://localhost:3000
```

---

# Frontend

The frontend is served automatically by Axum from:

```text
trust_rust_web/static/
```

No frontend build process is required.

---

# Environment Variables

Example:

```env
RPC_URL=http://127.0.0.1:8545

FACTORY_ADDRESS=0x...

SELLER_ADDRESS=0x70997970c51812dc3a010c7d01b50e0d17dc79c8
ADMIN_ADDRESS=0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266
BIDDER_ADDRESS=0xa0ee7a142d267c1f36714e4a8f75612f20a79720
BUYER_ADDRESS=0xbcd4042de499d14e55001ccbb24a551f3b954096

BIDDING_TIME_SECONDS=3600
STARTING_BID_WEI=1000000000000000000
CONFIRMATION_WINDOW=259200
REGISTRY_PAGE_LIMIT=20
```

---

# Development Commands

## Rust

Build:

```bash
cargo build
```

Check:

```bash
cargo check
```

Format:

```bash
cargo fmt --all
```

Clippy:

```bash
cargo clippy
```

Run Tests:

```bash
cargo test
```

Run Tests With Output:

```bash
cargo test -- --nocapture
```

---

## Solidity

Compile:

```bash
cd "HH Blockchain node"

npx hardhat compile
```

Run Tests:

```bash
npx hardhat test
```

Deploy Factory:

```bash
npx hardhat run scripts/deploy_factory.js --network localhost
```

---

# API Endpoints

## Authentication

| Method | Endpoint       | Description           |
| ------ | -------------- | --------------------- |
| GET    | `/auth/nonce`  | Generate nonce        |
| POST   | `/auth/verify` | Verify SIWE signature |
| POST   | `/auth/logout` | Destroy session       |

---

## Auctions

| Method | Endpoint              | Description               |
| ------ | --------------------- | ------------------------- |
| GET    | `/api/auctions`       | List auctions             |
| POST   | `/api/create-auction` | Create auction            |
| POST   | `/api/bid`            | Place bid                 |
| POST   | `/api/withdraw`       | Withdraw refundable funds |

---

## Escrow

| Method | Endpoint                               | Description          |
| ------ | -------------------------------------- | -------------------- |
| GET    | `/api/escrow/status/{auction_address}` | Escrow status        |
| POST   | `/api/escrow/end`                      | End auction          |
| POST   | `/api/escrow/confirm`                  | Confirm receipt      |
| POST   | `/api/escrow/claim-timeout`            | Seller timeout claim |
| POST   | `/api/escrow/refund`                   | Admin refund         |

---

# User Flow

## Sign In

1. Click **Sign In**
2. Enter a Hardhat wallet address
3. Sign the authentication message
4. Wallet pill appears in the navigation bar

---

## Create Auction

1. Complete the auction form
2. Submit transaction
3. Auction is deployed on-chain
4. Auction list refreshes automatically

---

## Place Bid

1. Select auction
2. Enter bid amount
3. Submit bid
4. Transaction executes on-chain
5. Dashboard refreshes automatically

---

## Withdraw Funds

1. Open withdrawal modal
2. Confirm withdrawal
3. Pending returns are withdrawn from escrow

---

# Security Notes

Current implementation is intended for:

* Local development
* Testing
* Demonstration environments

Production improvements would include:

* Secure HTTPS cookies
* Persistent session storage
* Redis-backed state
* Session expiration
* CSRF protection
* Rate limiting
* Production wallet integrations
* Database persistence
* Smart contract audits

---

# Future Improvements

* Real-time auction updates
* WebSocket subscriptions
* Database persistence
* IPFS metadata storage
* Event indexing
* Multi-network deployment support
* Advanced auction filtering
* Fuzz testing
* Gas snapshot testing
* Enhanced frontend UX

---

# Learning Goals

This project was originally inspired by a Python-based group project and later redesigned in Rust to explore:

* Systems programming
* Async Rust
* Backend architecture
* Ethereum integration
* Blockchain infrastructure
* Smart contract development
* Protocol-level escrow systems
* Full-stack Web3 engineering

---

# License

MIT License

