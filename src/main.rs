use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod wallet;
mod storage;
mod rpc_client;

use wallet::{Wallet, WalletConfig};
use storage::{EncryptedSeed};
use rpc_client::RpcClient;

#[derive(Parser)]
#[command(name = "kaspa-wallet")]
#[command(about = "Minimal Kaspa wallet scaffold (HD keys + encrypted seed)", long_about = None)]
struct Cli {
    /// Path to wallet file (encrypted seed)
    #[arg(short, long, default_value = "wallet.dat")]
    file: PathBuf,

    /// Kaspa node RPC endpoint (e.g. http://127.0.0.1:port)
    #[arg(short, long, default_value = "http://127.0.0.1:16110")]
    node: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new wallet (writes encrypted seed to file)
    New {
        /// Passphrase to encrypt seed with (use a strong passphrase)
        #[arg(short, long)]
        passphrase: String,
    },
    /// Restore wallet from mnemonic (writes encrypted seed to file)
    Restore {
        /// Mnemonic words (quoted)
        #[arg(short, long)]
        mnemonic: String,
        /// Passphrase to encrypt seed with
        #[arg(short, long)]
        passphrase: String,
    },
    /// Show derived address (default account 0, index 0)
    Address {
        /// Derivation path e.g. "m/44'/0'/0'/0/0"
        #[arg(short, long, default_value = "m/44'/0'/0'/0/0")]
        path: String,

        /// Passphrase to decrypt wallet
        #[arg(short, long)]
        passphrase: String,
    },
    /// Export compressed public key hex for a derivation path
    ExportPub {
        #[arg(short, long, default_value = "m/44'/0'/0'/0/0")]
        path: String,
        #[arg(short, long)]
        passphrase: String,
    },
    /// Query balance for an address using node RPC
    Balance {
        /// Address to query (depends on Kaspa address format)
        address: String,
    },
    /// Send funds (skeleton — transaction building is left as TODO)
    Send {
        /// Destination address
        to: String,
        /// Amount in atomic units
        amount: u64,
        /// Passphrase to decrypt wallet
        #[arg(short, long)]
        passphrase: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let rpc = RpcClient::new(cli.node.clone());

    match cli.command {
        Commands::New { passphrase } => {
            let w = Wallet::new_random();
            let enc = EncryptedSeed::encrypt(&w.mnemonic_phrase(), &passphrase)?;
            enc.save_to_file(&cli.file)?;
            println!("New wallet created and saved to {}", cli.file.display());
            println!("Mnemonic (store this safely):\n{}", w.mnemonic_phrase());
        }

        Commands::Restore { mnemonic, passphrase } => {
            let w = Wallet::from_mnemonic(&mnemonic)?;
            let enc = EncryptedSeed::encrypt(&w.mnemonic_phrase(), &passphrase)?;
            enc.save_to_file(&cli.file)?;
            println!("Wallet restored and saved to {}", cli.file.display());
        }

        Commands::Address { path, passphrase } => {
            let enc = EncryptedSeed::load_from_file(&cli.file)?;
            let mnemonic = enc.decrypt(&passphrase)?;
            let w = Wallet::from_mnemonic(&mnemonic)?;
            let pk = w.derive_private_key(&path)?;
            let pubkey = w.public_key_hex(&pk);
            // Kaspa address encoding left as TODO (uses Kaspa-specific format)
            println!("Derived public key (compressed hex): {}", pubkey);
            println!("Address encoding for Kaspa is Kaspa-specific and is TODO in this scaffold.");
        }

        Commands::ExportPub { path, passphrase } => {
            let enc = EncryptedSeed::load_from_file(&cli.file)?;
            let mnemonic = enc.decrypt(&passphrase)?;
            let w = Wallet::from_mnemonic(&mnemonic)?;
            let pk = w.derive_private_key(&path)?;
            let pubkey = w.public_key_hex(&pk);
            println!("{}", pubkey);
        }

        Commands::Balance { address } => {
            let balance = rpc.get_balance(&address).await?;
            println!("Balance for {}: {:?}", address, balance);
        }

        Commands::Send { to, amount, passphrase } => {
            // This is a skeleton showing where you'd build and sign a Kaspa transaction.
            // Transaction format, UTXO selection and serialization are Kaspa-specific and must be implemented here.
            let enc = EncryptedSeed::load_from_file(&cli.file)?;
            let mnemonic = enc.decrypt(&passphrase)?;
            let w = Wallet::from_mnemonic(&mnemonic)?;
            // Example: derive key for index 0:
            let pk = w.derive_private_key("m/44'/0'/0'/0/0")?;
            println!("Derived pubkey (hex): {}", w.public_key_hex(&pk));
            println!("TODO: Implement Kaspa transaction construction (select UTXOs, build tx, sign).");
            println!("Once you have a hex-serialized tx, call rpc.broadcast(hex).");
        }
    }

    Ok(())
}
