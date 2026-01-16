use anyhow::{anyhow, Result};
use bip39::{Language, Mnemonic, MnemonicType, Seed};
use bip32::{XPrv, DerivationPath, ExtendedPrivateKey};
use secp256k1::{SecretKey, PublicKey, Secp256k1};
use std::str::FromStr;

/// Wallet core: mnemonic -> seed -> XPrv -> child private key
pub struct Wallet {
    mnemonic: Mnemonic,
}

pub struct WalletConfig {
    pub derivation_path: String,
}

impl Wallet {
    /// Create a new random 12-word mnemonic wallet
    pub fn new_random() -> Self {
        let m = Mnemonic::new(MnemonicType::Words12, Language::English);
        Wallet { mnemonic: m }
    }

    pub fn mnemonic_phrase(&self) -> String {
        self.mnemonic.phrase().to_string()
    }

    pub fn from_mnemonic(phrase: &str) -> Result<Self> {
        let m = Mnemonic::from_phrase(phrase, Language::English)
            .map_err(|e| anyhow!("Invalid mnemonic: {}", e))?;
        Ok(Wallet { mnemonic: m })
    }

    /// Derive a secp256k1 secret key for the given BIP32 derivation path (e.g. m/44'/0'/0'/0/0)
    pub fn derive_private_key(&self, derivation_path: &str) -> Result<SecretKey> {
        // Convert mnemonic -> seed
        let seed = Seed::new(&self.mnemonic, "");
        let seed_bytes = seed.as_bytes();

        // master XPrv from seed
        let xprv = XPrv::new(seed_bytes)
            .map_err(|e| anyhow!("failed to create xprv: {:?}", e))?;

        let dp = DerivationPath::from_str(derivation_path)
            .map_err(|e| anyhow!("invalid derivation path: {:?}", e))?;

        let child_xprv = xprv.derive_priv(&dp)
            .map_err(|e| anyhow!("derive priv failed: {:?}", e))?;

        // Get raw 32-byte secret
        let sk_bytes = child_xprv.to_bytes();
        let secret = SecretKey::from_slice(&sk_bytes)
            .map_err(|e| anyhow!("invalid secret key from derived bytes: {:?}", e))?;
        Ok(secret)
    }

    /// Return compressed public key hex for a given secret key
    pub fn public_key_hex(&self, sk: &SecretKey) -> String {
        let secp = Secp256k1::new();
        let pk = PublicKey::from_secret_key(&secp, sk);
        hex::encode(pk.serialize())
    }
}
