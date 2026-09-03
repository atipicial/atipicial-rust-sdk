use atipicial::{
	atipicial_builder::ScriptBuilder,
	atipicial_clients::{APITrait, HttpProvider, RpcClient},
	atipicial_types::{ContractParameter, ScriptHash},
	prelude::*,
};
use std::str::FromStr;

/// This example demonstrates comprehensive AEP-17 token operations on the Atipicial blockchain.
/// It shows token information retrieval, balance checking, transfer preparation, and best practices.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("💰 Atipicial AEP-17 Token Operations Example");
	println!("=========================================");

	// 1. Connect to Atipicial TestNet
	println!("\n📡 1. Connecting to Atipicial TestNet...");
	let provider = HttpProvider::new("https://testnet1.atipicial.com:443/")
		.map_err(|e| format!("Failed to create provider: {e}"))?;
	let client = RpcClient::new(provider);
	println!("   ✅ Connected successfully");

	// 2. Native Token Contract Setup
	println!("\n🪙 2. Setting up native token contracts...");

	// GAS Token Contract (Atipicial's utility token)
	let gas_hash = ScriptHash::from_str("0xd2a4cff31913016155e38e474a2c06d08be276cf")?;
	println!("   ⛽ GAS Token: 0x{}", hex::encode(gas_hash.0));

	// ATC Token Contract (Atipicial's governance token)
	let atipicial_hash = ScriptHash::from_str("0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")?;
	println!("   🔷 ATC Token: 0x{}", hex::encode(atipicial_hash.0));

	// 3. Token Information Retrieval
	println!("\n📊 3. Retrieving comprehensive token information...");

	// Query GAS token properties
	match get_token_info(&client, &gas_hash, "GAS").await {
		Ok(info) => {
			println!("   ⛽ GAS Token Properties:");
			println!("     Symbol: {}", info.symbol);
			println!("     Decimals: {}", info.decimals);
			println!("     Total Supply: {} GAS", info.total_supply_formatted);
		},
		Err(e) => println!("   ❌ Failed to get GAS info: {e}"),
	}

	// Query ATC token properties
	match get_token_info(&client, &atipicial_hash, "ATC").await {
		Ok(info) => {
			println!("   🔷 ATC Token Properties:");
			println!("     Symbol: {}", info.symbol);
			println!("     Decimals: {}", info.decimals);
			println!("     Total Supply: {} ATC", info.total_supply_formatted);
		},
		Err(e) => println!("   ❌ Failed to get ATC info: {e}"),
	}

	// 4. Balance Queries for Sample Addresses
	println!("\n💎 4. Querying token balances for sample addresses...");

	let sample_addresses = vec![
		"NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc", // Genesis block address
		"NiNmXL8FjEUEs1nfX9uHFBNaenxDHJtmuB", // Common TestNet address
	];

	for address in &sample_addresses {
		println!("   📍 Address: {address}");

		if let Ok(script_hash) = ScriptHash::from_address(address) {
			// Check GAS balance
			match get_token_balance(&client, &gas_hash, &script_hash, 8).await {
				Ok(balance) => println!("     ⛽ GAS Balance: {balance} GAS"),
				Err(_) => println!("     ⛽ GAS Balance: Unable to query"),
			}

			// Check ATC balance
			match get_token_balance(&client, &atipicial_hash, &script_hash, 0).await {
				Ok(balance) => println!("     🔷 ATC Balance: {balance} ATC"),
				Err(_) => println!("     🔷 ATC Balance: Unable to query"),
			}
		}
		println!();
	}

	// 5. Transfer Script Building
	println!("\n🔄 5. Building token transfer scripts...");

	let sender_address = "NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc";
	let recipient_address = "NiNmXL8FjEUEs1nfX9uHFBNaenxDHJtmuB";

	let sender_hash = ScriptHash::from_address(sender_address)?;
	let recipient_hash = ScriptHash::from_address(recipient_address)?;

	// GAS transfer script (1 GAS)
	let gas_transfer_amount = 100_000_000u64; // 1 GAS (8 decimals)

	let mut gas_script_builder = ScriptBuilder::new();
	gas_script_builder.contract_call(
		&gas_hash,
		"transfer",
		&[
			ContractParameter::h160(&sender_hash),
			ContractParameter::h160(&recipient_hash),
			ContractParameter::integer(gas_transfer_amount as i64),
			ContractParameter::any(),
		],
		None,
	)?;

	let gas_script = gas_script_builder.to_bytes();
	println!("   ✅ GAS transfer script built ({} bytes)", gas_script.len());
	println!(
		"   📝 Transfer: {} GAS from {} to {}",
		gas_transfer_amount as f64 / 100_000_000.0,
		sender_address,
		recipient_address
	);

	// ATC transfer script (1 ATC)
	let atipicial_transfer_amount = 1u64; // 1 ATC (indivisible)

	let mut atipicial_script_builder = ScriptBuilder::new();
	atipicial_script_builder.contract_call(
		&atipicial_hash,
		"transfer",
		&[
			ContractParameter::h160(&sender_hash),
			ContractParameter::h160(&recipient_hash),
			ContractParameter::integer(atipicial_transfer_amount as i64),
			ContractParameter::any(),
		],
		None,
	)?;

	let atipicial_script = atipicial_script_builder.to_bytes();
	println!("   ✅ ATC transfer script built ({} bytes)", atipicial_script.len());
	println!(
		"   📝 Transfer: {atipicial_transfer_amount} ATC from {sender_address} to {recipient_address}"
	);

	// 6. Multi-Token Transfer Example
	println!("\n🔀 6. Multi-token transfer transaction...");

	let mut multi_script_builder = ScriptBuilder::new();

	// Transfer GAS
	multi_script_builder.contract_call(
		&gas_hash,
		"transfer",
		&[
			ContractParameter::h160(&sender_hash),
			ContractParameter::h160(&recipient_hash),
			ContractParameter::integer(50_000_000), // 0.5 GAS
			ContractParameter::any(),
		],
		None,
	)?;

	// Transfer ATC
	multi_script_builder.contract_call(
		&atipicial_hash,
		"transfer",
		&[
			ContractParameter::h160(&sender_hash),
			ContractParameter::h160(&recipient_hash),
			ContractParameter::integer(1), // 1 ATC
			ContractParameter::any(),
		],
		None,
	)?;

	let multi_script = multi_script_builder.to_bytes();
	println!("   ✅ Multi-token transfer script built ({} bytes)", multi_script.len());
	println!("   📝 Combined transfer: 0.5 GAS + 1 ATC");

	// 7. Token Allowance and Advanced Operations
	println!("\n🔧 7. Advanced AEP-17 operations...");

	println!("   📋 Available AEP-17 standard methods:");
	println!("     • symbol() - Get token symbol");
	println!("     • decimals() - Get decimal places");
	println!("     • totalSupply() - Get total token supply");
	println!("     • balanceOf(account) - Get account balance");
	println!("     • transfer(from, to, amount, data) - Transfer tokens");

	println!("   🔍 Optional methods (if supported):");
	println!("     • approve(spender, amount) - Approve spending allowance");
	println!("     • allowance(owner, spender) - Check spending allowance");
	println!("     • transferFrom(spender, from, to, amount, data) - Third-party transfer");

	// 8. Best Practices for AEP-17 Tokens
	println!("\n💡 8. AEP-17 Token Best Practices:");

	println!("   🔐 Security:");
	println!("     • Always verify token contract authenticity");
	println!("     • Test transfers with small amounts first");
	println!("     • Validate recipient addresses before sending");
	println!("     • Use proper witness scopes for transfers");

	println!("   ⚡ Performance:");
	println!("     • Batch multiple token queries in single invoke_function");
	println!("     • Cache token metadata (symbol, decimals) to reduce RPC calls");
	println!("     • Use appropriate gas fees for timely execution");

	println!("   🧪 Testing:");
	println!("     • Always test on TestNet before MainNet");
	println!("     • Simulate all transactions before broadcasting");
	println!("     • Verify token contract implementation");

	println!("   📊 Monitoring:");
	println!("     • Track transaction confirmations");
	println!("     • Monitor for Transfer events");
	println!("     • Handle failed transactions gracefully");

	// 9. Common AEP-17 Token Standards
	println!("\n📜 9. Common AEP-17 tokens on Atipicial:");
	println!("   • GAS (d2a4cff31913016155e38e474a2c06d08be276cf) - Network utility token");
	println!("   • ATC (ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5) - Governance token");
	println!("   • bATC - Liquid staking ATC wrapper");
	println!("   • USDT - USD Tether stablecoin");
	println!("   • USDC - USD Coin stablecoin");

	println!("\n🎉 AEP-17 token operations example completed!");
	println!("💡 Remember: This example demonstrates concepts and patterns.");
	println!("💡 For live transactions, ensure proper key management and testing.");

	Ok(())
}

/// Token information structure
#[derive(Debug)]
#[allow(dead_code)]
struct TokenInfo {
	symbol: String,
	decimals: u32,
	total_supply: u64,
	total_supply_formatted: f64,
}

/// Get comprehensive token information
async fn get_token_info(
	client: &RpcClient<HttpProvider>,
	token_hash: &ScriptHash,
	token_name: &str,
) -> Result<TokenInfo, Box<dyn std::error::Error>> {
	// Query symbol
	let symbol_result =
		client.invoke_function(token_hash, "symbol".to_string(), vec![], None).await?;
	let symbol = symbol_result
		.stack
		.first()
		.and_then(|s| s.as_string())
		.unwrap_or_else(|| token_name.to_string());

	// Query decimals
	let decimals_result =
		client.invoke_function(token_hash, "decimals".to_string(), vec![], None).await?;
	let decimals = decimals_result.stack.first().and_then(|s| s.as_int()).unwrap_or(0) as u32;

	// Query total supply
	let supply_result = client
		.invoke_function(token_hash, "totalSupply".to_string(), vec![], None)
		.await?;
	let total_supply = supply_result.stack.first().and_then(|s| s.as_int()).unwrap_or(0) as u64;
	let total_supply_formatted = total_supply as f64 / 10f64.powi(decimals as i32);

	Ok(TokenInfo { symbol, decimals, total_supply, total_supply_formatted })
}

/// Get token balance for an account
async fn get_token_balance(
	client: &RpcClient<HttpProvider>,
	token_hash: &ScriptHash,
	account_hash: &ScriptHash,
	decimals: u32,
) -> Result<f64, Box<dyn std::error::Error>> {
	// Use the AEP-17 balances query method
	match client.get_aep17_balances(*account_hash).await {
		Ok(balances) => {
			// Find the specific token balance
			if let Some(balance_info) =
				balances.balances.iter().find(|b| b.asset_hash == *token_hash)
			{
				let balance_value = balance_info.amount.parse::<u64>().unwrap_or(0);
				let formatted_balance = balance_value as f64 / 10f64.powi(decimals as i32);
				Ok(formatted_balance)
			} else {
				Ok(0.0)
			}
		},
		Err(_) => Ok(0.0),
	}
}
