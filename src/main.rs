use anyhow::Result;
use bip39::Mnemonic;
use bip32::{XPrv, DerivationPath};
use ethers::prelude::*;
use rand_core::{OsRng, RngCore};
use zeroize::Zeroize;
use hex;
use std::str::FromStr;

fn main() -> Result<()> {
    println!("Generating wallet...\n");

    // 16 bytes = 12 words
    let mut random_entropy = [0u8; 16];
    OsRng.fill_bytes(&mut random_entropy);
    let mnemonic = Mnemonic::from_entropy(&random_entropy)?;
    let phrase = mnemonic.to_string();

    // seed gen (64 bytes)
    let mut seed = mnemonic.to_seed("");

    // std derivation BIP44: m/44'/60'/0'/0/0
    let derivation_path = DerivationPath::from_str("m/44'/60'/0'/0/0")?;
    let child_xprv = XPrv::derive_from_path(&seed, &derivation_path)?;
    let private_key = child_xprv.private_key();

    let wallet = LocalWallet::from_bytes(private_key.to_bytes().as_slice())?;

    println!("Wallet generated successfully!");
    println!("  Mnemonic: {}", phrase);
    println!("  Address: {}", wallet.address());
    println!("  Private Key: 0x{}", hex::encode(private_key.to_bytes()));
    seed.zeroize();
    random_entropy.zeroize();

    Ok(())
}
