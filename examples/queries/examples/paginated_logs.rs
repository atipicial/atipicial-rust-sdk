use atipicial::{
	atipicial_clients::{APITrait, HttpProvider, RpcClient},
	atipicial_types::ScriptHash,
};
use std::str::FromStr;

/// Atipicial Paginated Event Logs Example
///
/// This example demonstrates how to query and paginate through Atipicial transaction logs
/// and contract events. Unlike Ethereum's event filtering system, Atipicial uses application
/// logs and notifications to track contract events.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("📋 Atipicial Paginated Event Logs Example");
	println!("======================================");

	// 1. Connect to Atipicial TestNet
	println!("\n1. Connecting to Atipicial TestNet...");
	let provider = HttpProvider::new("https://testnet1.atipicial.coz.io:443/")?;
	let client = RpcClient::new(provider);
	println!("   ✅ Connected to Atipicial TestNet");

	// 2. Get current block information
	println!("\n2. Getting current block information...");
	let latest_block_count = client.get_block_count().await?;
	println!("   📦 Latest block: {latest_block_count}");

	// Start from a recent block range for demonstration
	let start_block = if latest_block_count > 1000 { latest_block_count - 1000 } else { 1 };
	let end_block = latest_block_count;

	println!("   🔍 Scanning blocks {start_block} to {end_block} for events");

	// 3. Query AEP-17 token transfer events
	println!("\n3. Querying AEP-17 token transfer events...");

	let gas_token = ScriptHash::from_str("d2a4cff31913016155e38e474a2c06d08be276cf")?;
	let atipicial_token = ScriptHash::from_str("ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")?;

	// Paginated event scanning
	let page_size = 10; // Process 10 blocks at a time
	let mut total_events = 0;

	for page_start in (start_block..end_block).step_by(page_size) {
		let page_end = std::cmp::min(page_start + page_size as u32, end_block);

		println!("   📄 Processing page: blocks {page_start} to {page_end}");

		let page_events =
			scan_blocks_for_events(&client, page_start, page_end, &[gas_token, atipicial_token]).await?;
		total_events += page_events;

		// Add a small delay to be respectful to the RPC server
		tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
	}

	// 4. Query specific contract events
	println!("\n4. Querying specific contract events...");

	// Example: Query ManagementContract events
	let management_contract = ScriptHash::from_str("fffdc93764dbaddd97c48f252a53ea4643faa3fd")?;
	let management_events =
		query_contract_events(&client, &management_contract, start_block, end_block).await?;

	println!("   🏛️ Management contract events found: {management_events}");

	// 5. Summary
	println!("\n5. Event scanning summary:");
	println!("   📊 Total blocks scanned: {}", end_block - start_block);
	println!("   🎯 Total events found: {total_events}");
	println!("   📄 Pages processed: {}", (end_block - start_block).div_ceil(page_size as u32));

	println!("\n✅ Atipicial paginated event logs example completed!");
	println!("💡 Key features demonstrated:");
	println!("   • Paginated block scanning for performance");
	println!("   • AEP-17 token transfer event detection");
	println!("   • Contract-specific event filtering");
	println!("   • Rate-limited RPC calls for server respect");

	Ok(())
}

/// Scan a range of blocks for AEP-17 transfer events
async fn scan_blocks_for_events(
	client: &RpcClient<HttpProvider>,
	start_block: u32,
	end_block: u32,
	token_contracts: &[ScriptHash],
) -> Result<usize, Box<dyn std::error::Error>> {
	let mut event_count = 0;

	for block_index in start_block..end_block {
		// Get block with transactions
		match client.get_block_by_index(block_index, true).await {
			Ok(block) => {
				if let Some(transactions) = &block.transactions {
					for (tx_index, transaction) in transactions.iter().enumerate() {
						// Check if this transaction involves our tokens of interest
						if transaction_involves_tokens(&transaction.script, token_contracts) {
							println!(
								"      🔍 Found potential token transaction in block {block_index}, tx {tx_index}"
							);
							event_count += 1;

							// In a real implementation, you would:
							// 1. Get the application log for this transaction
							// 2. Parse the notifications for Transfer events
							// 3. Extract sender, receiver, and amount
						}
					}
				}
			},
			Err(_) => {
				// Skip blocks that can't be retrieved
				continue;
			},
		}
	}

	Ok(event_count)
}

/// Check if a transaction script involves specific token contracts
fn transaction_involves_tokens(script: &str, token_contracts: &[ScriptHash]) -> bool {
	// Simplified check - in reality you'd parse the script properly
	// This is a basic heuristic to detect token-related transactions
	for contract in token_contracts {
		let contract_hex = format!("{contract:x}");
		if script.contains(&contract_hex) {
			return true;
		}
	}
	false
}

/// Query events for a specific contract
async fn query_contract_events(
	client: &RpcClient<HttpProvider>,
	contract_hash: &ScriptHash,
	start_block: u32,
	end_block: u32,
) -> Result<usize, Box<dyn std::error::Error>> {
	println!("   🔍 Querying events for contract: {contract_hash:x}");

	let mut event_count = 0;
	let sample_size = std::cmp::min(50, end_block - start_block); // Sample up to 50 blocks

	for block_index in start_block..(start_block + sample_size) {
		match client.get_block_by_index(block_index, true).await {
			Ok(block) => {
				if let Some(transactions) = &block.transactions {
					for transaction in transactions {
						if transaction_involves_contract(&transaction.script, contract_hash) {
							event_count += 1;
							println!("      📝 Contract interaction found in block {block_index}");
						}
					}
				}
			},
			Err(_) => continue,
		}
	}

	Ok(event_count)
}

/// Check if a transaction involves a specific contract
fn transaction_involves_contract(script: &str, contract_hash: &ScriptHash) -> bool {
	let contract_hex = format!("{contract_hash:x}");
	script.contains(&contract_hex)
}
