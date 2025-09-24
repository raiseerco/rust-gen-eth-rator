use bip39::{Mnemonic};
use ethers::prelude::*;
use anyhow::Result;
use hex;
use rand::RngCore;

fn main() -> Result<()> {
    println!("Generating Ethereum Wallet...\n");
    // 12bytes for 12 words
    let mut random_entropy = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut random_entropy);
    let mnemonic = Mnemonic::from_entropy(&random_entropy)?;
    let phrase = mnemonic.to_string();
    // seed gen
    let seed = mnemonic.to_seed("");
    let private_key = &seed[..32];
    let wallet = LocalWallet::from_bytes(private_key)?;
    println!("Wallet generated successfully!");
    println!("  Mnemonic: {}", phrase);
    println!("  Address: {}", wallet.address());
    println!("  Private Key: 0x{}", hex::encode(private_key));
    Ok(())
}