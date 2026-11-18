# T.R.U.S.T. CLI starter

This archive contains **only a new CLI crate**, not a replacement for your project.

1. Extract `trust_rust_cli/` into the root of your existing `Trust_Rust` repository, beside `trust_rust_client/` and `trust_rust_web/`.
2. In the root `Cargo.toml`, add `"trust_rust_cli",` to the `[workspace] members` array.
3. From the repository root, run:

   cargo run -p trust_rust_cli -- --help
   cargo run -p trust_rust_cli -- wallet address
   cargo run -p trust_rust_cli -- auction inspect 1

For a standalone executable, run `cargo build -p trust_rust_cli` and use `./target/debug/trust --help`.

All commands are currently placeholders: no wallet keys, RPC requests, or smart contract transactions are performed.
