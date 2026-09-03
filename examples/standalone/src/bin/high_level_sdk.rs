//! Example demonstrating the new high-level SDK API
//!
//! This shows how the simplified API makes common operations much easier.

use atipicial::sdk::{Atipicial, Network};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("🚀 AtipicialRust High-Level SDK Example\n");

	// Simple connection to TestNet
	println!("📡 Connecting to Atipicial TestNet...");
	let atipicial = Atipicial::testnet().await?;
	println!("✅ Connected successfully!\n");

	// Get the current block height
	let height = atipicial.get_block_height().await?;
	println!("📊 Current block height: {}\n", height);

	// Check balance for a test address
	let address = "NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc";
	println!("💰 Checking balance for: {}", address);

	let balance = atipicial.get_balance(address).await?;
	println!("   ATC: {} tokens", balance.atipicial);
	println!("   GAS: {} tokens", balance.gas);

	if !balance.tokens.is_empty() {
		println!("   Other tokens:");
		for token in &balance.tokens {
			println!("     - {}: {}", token.symbol, token.amount);
		}
	}

	// Example of custom configuration
	println!("\n🔧 Creating custom configured client...");
	let custom_atipicial = Atipicial::builder()
		.network(Network::MainNet)
		.timeout(Duration::from_secs(60))
		.retries(5)
		.cache(true)
		.metrics(false)
		.build()
		.await?;
	println!("✅ Custom client created for MainNet");

	// Get MainNet block height
	let mainnet_height = custom_atipicial.get_block_height().await?;
	println!("📊 MainNet block height: {}", mainnet_height);

	println!("\n✨ Example completed successfully!");

	Ok(())
}
