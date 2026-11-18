use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "trust", version, about = "T.R.U.S.T. auction protocol CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Inspect the configured wallet
    Wallet {
        #[command(subcommand)]
        command: WalletCommand,
    },
    /// Explore and manage auctions
    Auction {
        #[command(subcommand)]
        command: AuctionCommand,
    },
}

#[derive(Debug, Subcommand)]
enum WalletCommand {
    /// Display the configured wallet address
    Address,
    /// Display the wallet's ETH balance
    Balance,
}

#[derive(Debug, Subcommand)]
enum AuctionCommand {
    /// List auctions
    List,
    /// Inspect one auction by ID
    Inspect { id: u64 },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Wallet { command } => match command {
            WalletCommand::Address => println!("Wallet address: not configured yet"),
            WalletCommand::Balance => println!("Wallet balance: blockchain integration pending"),
        },
        Commands::Auction { command } => match command {
            AuctionCommand::List => println!("Auction listing: blockchain integration pending"),
            AuctionCommand::Inspect { id } => {
                println!("Auction #{id}: blockchain integration pending")
            }
        },
    }
}
