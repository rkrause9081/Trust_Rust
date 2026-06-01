# T.R.U.S.T Auction Protocol

A full-stack decentralized auction and escrow platform built with:

* Rust
* Axum
* Alloy
* Solidity
* Hardhat
* JavaScript frontend

This project implements an Ethereum-based auction system with:

* on-chain auction creation
* on-chain bidding
* auction registry queries
* escrow settlement flows
* refund/withdrawal handling
* SIWE-style wallet authentication
* session cookie management
* Rust backend APIs
* Solidity smart contract testing
* modular blockchain client architecture
* modular frontend state/render/API separation

---

# Architecture

```text
Frontend HTML / CSS / JS
        ↓
Frontend modules:
  auth_session.js
  auction_api.js
  auction_state.js
  auction_cards.js
  bid_form.js
  create_auction_form.js
  withdraw_modal.js
  escrow_actions.js
  app_init.js
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
│   │   ├── main.rs
│   │   └── routes/
│   │       ├── mod.rs
│   │       ├── auction_query.rs
│   │       ├── auction_create.rs
│   │       ├── auction_bid.rs
│   │       ├── auction_withdraw.rs
│   │       └── escrow_routes.rs
│   │
│   └── static/
│       ├── index.html
│       ├── css/
│       │   └── styles.css
│       └── js/
│           ├── utils.js
│           ├── auction_api.js
│           ├── auction_state.js
│           ├── auction_cards.js
│           ├── auth_session.js
│           ├── bid_form.js
│           ├── create_auction_form.js
│           ├── withdraw_modal.js
│           ├── escrow_actions.js
│           └── app_init.js
│
└── README.md
```

---

# Route Module Responsibilities

## `trust_rust_web/src/routes/auction_query.rs`

Read-only auction discovery route.

Handles:

* `GET /api/auctions`
* loading auction addresses from the factory
* fetching registry metadata for each auction
* returning frontend-ready auction JSON

---

## `trust_rust_web/src/routes/auction_create.rs`

Auction creation route.

Handles:

* `POST /api/create-auction`
* validating the active session
* parsing auction creation payloads
* converting frontend values into blockchain values
* calling the Rust blockchain client create-auction helper

---

## `trust_rust_web/src/routes/auction_bid.rs`

Bid placement route.

Handles:

* `POST /api/bid`
* validating the active session
* parsing auction address and bid amount
* submitting the bid transaction through `trust_rust_client`

---

## `trust_rust_web/src/routes/auction_withdraw.rs`

Withdrawal route.

Handles:

* `POST /api/withdraw`
* validating the active session
* parsing the target auction address
* executing pending-return withdrawals

---

## `trust_rust_web/src/routes/escrow_routes.rs`

Escrow lifecycle route group.

Handles:

* `GET /api/escrow/status/{auction_address}`
* `POST /api/escrow/end`
* `POST /api/escrow/confirm`
* `POST /api/escrow/claim-timeout`
* `POST /api/escrow/refund`

---

# Frontend Module Responsibilities

## `auction_api.js`

Single API boundary for frontend requests.

Contains the only direct `fetch()` calls for:

* auction listing
* bid placement
* auction creation
* withdrawals
* escrow status
* escrow actions

---

## `auction_state.js`

Shared auction state/cache.

Handles:

* `refreshAuctions()`
* normalized auction data
* reload coordination after bid/create/withdraw/escrow actions
* compatibility alias `loadActiveAuctions()`

---

## `auction_cards.js`

Rendering layer for auction UI.

Handles:

* dashboard stats
* active auction cards
* bid dropdown population
* card-level bid/withdraw button behavior

No direct API calls should live here.

---

## `auth_session.js`

Wallet authentication and session UI.

Handles:

* SIWE-style login flow
* wallet pill display
* wallet-pill logout behavior
* local wallet UI state

The green wallet pill in the navbar doubles as the logout control.

---

## `bid_form.js`

Bid form behavior.

Handles:

* ETH-to-wei conversion
* bid submit event
* bid API call
* auction refresh after successful bid

After a successful bid, the frontend calls:

```js
await refreshAuctions({ showLoading: false });
```

This refreshes the auction card, dashboard stats, and bid dropdown from `/api/auctions`.

---

## `create_auction_form.js`

Auction creation form behavior.

Handles:

* form validation
* end-date-to-duration conversion
* starting bid conversion
* auction creation API call
* auction refresh after successful creation

---

## `withdraw_modal.js`

Withdrawal modal behavior.

Handles:

* opening and closing the withdraw modal
* submitting withdrawals
* refreshing auctions after successful withdrawal

---

## `escrow_actions.js`

Escrow UI and action behavior.

Handles:

* escrow status loading
* escrow panel rendering
* end auction
* confirm receipt
* claim timeout
* refund action
* auction refresh after successful escrow actions

---

## `app_init.js`

Frontend bootstrap file.

Handles:

* startup initialization
* form binding
* wallet UI restoration
* initial auction refresh

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
* Structured route module separation
* Async blockchain interaction
* SIWE-style authentication
* Session cookie management
* Wallet-pill logout endpoint
* JSON REST API responses
* Rust integration test suite

---

## Frontend Features

* Wallet authentication
* Wallet-pill logout
* Active auction list
* Dashboard stats
* Bid placement UI
* Automatic auction-card refresh after successful bids
* Create auction UI
* Withdraw modal
* Escrow interaction controls
* Shared auction state/cache
* Modular API/state/render separation

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
| Authentication         | SIWE-style signatures   |
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

1. Copy the deployed factory address.
2. Update the factory address used by the Rust web server.
3. Restart the Rust web server if it is already running.

The Rust backend uses the deployed factory contract for:

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

## Check Compilation

```bash
cargo check
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
SELLER_ADDRESS=0x70997970c51812dc3a010c7d01b50e0d17dc79c8
ADMIN_ADDRESS=0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266
BIDDER_ADDRESS=0xa0ee7a142d267c1f36714e4a8f75612f20a79720
BUYER_ADDRESS=0xbcd4042de499d14e55001ccbb24a551f3b954096

BIDDING_TIME_SECONDS=3600
STARTING_BID_WEI=1000000000000000000
CONFIRMATION_WINDOW=259200
REGISTRY_PAGE_LIMIT=20
```

Current local development may still hardcode the factory address in `trust_rust_web/src/main.rs`.
Moving it fully into `.env` is recommended.

---

# Running the Web Server

From the workspace root:

```bash
cargo run -p trust_rust_web
```

Server runs at:

```text
http://localhost:3000
```

---

# Recommended Local Dev Flow

Use three terminals.

## Terminal 1: Hardhat node

```bash
cd "HH Blockchain node"
npx hardhat node
```

## Terminal 2: Deploy contracts

```bash
cd "HH Blockchain node"
npx hardhat run scripts/deploy_factory.js --network localhost
```

Copy the deployed factory address into the Rust web server config.

## Terminal 3: Rust web server

```bash
cargo run -p trust_rust_web
```

Open:

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

| Method | Endpoint       | Description                         |
| ------ | -------------- | ----------------------------------- |
| GET    | `/auth/nonce`  | Generate SIWE nonce                 |
| POST   | `/auth/verify` | Verify SIWE signature/create session |
| POST   | `/auth/logout` | Clear active session                |

---

## Auctions

| Method | Endpoint              | Description     |
| ------ | --------------------- | --------------- |
| GET    | `/api/auctions`       | List auctions   |
| POST   | `/api/create-auction` | Create auction  |
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

---

# User Flow

## Sign in

1. Click `Sign In`.
2. Enter a local Hardhat account address.
3. Confirm the MetaMask signature.
4. The navbar displays the wallet pill.

## Log out

Click the green wallet pill in the top-right navbar.

This calls:

```text
POST /auth/logout
```

Then the frontend clears local wallet state and reloads the page.

## Create auction

1. Complete the create-auction form.
2. Submit the form.
3. The auction is created on-chain.
4. The auction list refreshes automatically.

## Place bid

1. Select an auction from the bid dropdown or click `Place Bid` on a card.
2. Enter the bid amount in ETH.
3. Submit the bid.
4. The bid transaction is sent on-chain.
5. The auction list refreshes automatically.
6. The card updates with the new highest bid.

---

# Rust Concepts Used

This project heavily uses:

* async/await
* Futures
* Result-based error handling
* `Arc` shared ownership
* `Mutex` interior mutability
* trait abstraction
* modular route architecture
* modular frontend architecture
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
* session expiration/cleanup
* CSRF protection for authenticated mutation routes
* stronger authorization rules
* rate limiting
* production Ethereum wallet integration
* hardened contract audits

---

# Future Improvements

* Move factory address and RPC URL fully into `.env`
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
* Shared Rust session helper for route modules

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
* frontend modularization
* route-level separation of concerns

---

# License

MIT License

