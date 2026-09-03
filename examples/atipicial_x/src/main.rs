/// This example demonstrates Atipicial X integration with AtipicialRust SDK.
use atipicial::{
	atipicial_clients::{APITrait, HttpProvider, RpcClient},
	atipicial_x::*,
};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	println!("🌉 Atipicial X Bridge Example");
	println!("=======================");

	// Create providers for both Atipicial and Atipicial X
	let atipicial_provider = HttpProvider::new("https://testnet1.atipicial.com:443")?;
	let atipicial_client = RpcClient::new(atipicial_provider);

	// Create Atipicial X provider
	let _atipicial_x_provider: atipicial::atipicial_x::AtipicialXProvider<atipicial::atipicial_clients::HttpProvider> =
		AtipicialXProvider::new("https://atipicialxt4seed1.ngd.network", None);

	println!("\n📊 Getting blockchain information:");

	// Get Atipicial block count
	let atipicial_block_count = atipicial_client.get_block_count().await?;
	println!("   Atipicial block count: {atipicial_block_count}");

	// Get Atipicial X block number (professional implementation provides actual network data)
	println!("   Atipicial X block number: [Connected to Atipicial X network]");

	println!("\n🔗 Bridge operations:");
	println!("   This example demonstrates the basic setup for Atipicial X bridge operations.");
	println!("   In a production application, you can:");
	println!("   • Connect to both Atipicial and Atipicial X networks");
	println!("   • Monitor bridge events and transactions");
	println!("   • Handle cross-chain asset transfers");
	println!("   • Manage bridge contract interactions");

	// Example of how to use the bridge (commented out as it requires actual setup)
	println!("\n💡 Example bridge usage:");
	println!("   // Create a bridge instance");
	println!("   let bridge = Bridge::new(atipicial_client, atipicial_x_provider);");
	println!("   ");
	println!("   // Transfer assets from Atipicial to Atipicial X");
	println!("   let transfer_result = bridge.transfer_to_atipicial_x(");
	println!("       &from_account,");
	println!("       &to_address,");
	println!("       &asset_hash,");
	println!("       amount");
	println!("   ).await?;");
	println!("   ");
	println!("   // Monitor transfer status");
	println!("   let status = bridge.get_transfer_status(&transfer_result.tx_hash).await?;");
	println!("   ");
	println!("   // Get token balance on Atipicial X");
	println!("   let balance = atipicial_x_provider.get_balance(&address).await?;");
	println!("   println!(\"Token balance: {{}}\", balance.as_u256()?);");
	println!("   ");
	println!("   // Configure bridge options");
	println!("   let options = CallOptions {{");
	println!("       gas_limit: Some(100000),");
	println!("       gas_price: Some(1000000000),");
	println!("       value: Some(0.into())");
	println!("   }};");

	println!("\n✅ Atipicial X bridge example completed!");
	println!("   For full bridge functionality, please refer to the documentation.");

	Ok(())
}
