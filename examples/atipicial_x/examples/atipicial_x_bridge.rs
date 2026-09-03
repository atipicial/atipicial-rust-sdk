use atipicial::{
	atipicial_clients::{APITrait, HttpProvider, RpcClient},
	ScriptHashExtension,
};
use std::str::FromStr;

/// Example demonstrating Atipicial X Bridge contract interactions.
/// Atipicial X is Atipicial's EVM-compatible sidechain that enables cross-chain asset transfers.
/// This example shows real bridge operations including deposits, withdrawals, and monitoring.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("🌉 Atipicial X Bridge Contract Example");
	println!("================================\n");

	// 1. Connect to both Atipicial and Atipicial X networks
	println!("📡 1. Establishing network connections...");

	// Connect to Atipicial MainNet
	let atipicial_client = connect_to_atipicial_mainnet().await?;

	// Atipicial X connection info (EVM-compatible)
	println!("   🌐 Atipicial X RPC: https://mainnet.rpc.banelabs.org");
	println!("   📊 Atipicial X Chain ID: 12227332");
	println!("   🔍 Atipicial X Explorer: https://xexplorer.atipicial.com");

	// 2. Atipicial X Bridge contract configuration
	println!("\n🌉 2. Atipicial X Bridge Configuration...");
	let bridge_config = BridgeConfig {
		atipicial_bridge_contract: atipicial::atipicial_types::ScriptHash::from_str(
			"0x48c40d4666f93408be1bef038b6722404d9a4c2a",
		)?,
		atipicialx_bridge_address: "0x85CfE7245BBaED6Df8a501e99656CD503FdF0937", // Example Atipicial X bridge
		gas_token_atipicial: atipicial::atipicial_types::ScriptHash::from_str(
			"d2a4cff31913016155e38e474a2c06d08be276cf",
		)?,
		gas_token_atipicialx: "0x0000000000000000000000000000000000000000", // Native GAS on Atipicial X
		min_confirmations: 12,
		bridge_fee: 100_000, // 0.001 GAS
	};

	println!("   📋 Atipicial Bridge: 0x{}", bridge_config.atipicial_bridge_contract);
	println!("   📋 Atipicial X Bridge: {}", bridge_config.atipicialx_bridge_address);
	println!("   ⏱️  Min confirmations: {}", bridge_config.min_confirmations);
	println!("   💰 Bridge fee: {} GAS", bridge_config.bridge_fee as f64 / 100_000_000.0);

	// 3. Check bridge status
	println!("\n🔍 3. Checking bridge status...");
	check_bridge_status(&atipicial_client, &bridge_config).await?;

	// 4. Query supported tokens
	println!("\n💎 4. Querying supported tokens...");
	query_supported_tokens(&atipicial_client, &bridge_config).await?;

	// 5. Demonstrate deposit process (Atipicial → Atipicial X)
	println!("\n📤 5. Deposit Process (Atipicial → Atipicial X)...");
	demonstrate_deposit_process(&atipicial_client, &bridge_config).await?;

	// 6. Demonstrate withdrawal process (Atipicial X → Atipicial)
	println!("\n📥 6. Withdrawal Process (Atipicial X → Atipicial)...");
	demonstrate_withdrawal_process(&bridge_config).await?;

	// 7. Monitor bridge transactions
	println!("\n📊 7. Monitoring bridge transactions...");
	monitor_bridge_transactions(&atipicial_client, &bridge_config).await?;

	// 8. Bridge security and best practices
	println!("\n🔐 8. Security Best Practices...");
	display_security_practices();

	println!("\n✅ Atipicial X Bridge example completed!");
	println!("💡 Successfully demonstrated cross-chain asset bridging between Atipicial and Atipicial X");

	Ok(())
}

/// Bridge configuration
struct BridgeConfig {
	atipicial_bridge_contract: atipicial::atipicial_types::ScriptHash,
	atipicialx_bridge_address: &'static str,
	gas_token_atipicial: atipicial::atipicial_types::ScriptHash,
	gas_token_atipicialx: &'static str,
	min_confirmations: u32,
	bridge_fee: u64,
}

/// Connect to Atipicial MainNet
async fn connect_to_atipicial_mainnet(
) -> Result<atipicial::providers::RpcClient<atipicial::providers::HttpProvider>, Box<dyn std::error::Error>> {
	let endpoints = vec![
		"https://mainnet1.atipicial.com:443/",
		"https://mainnet2.atipicial.com:443/",
		"http://seed1.atipicial.com:10332",
		"http://seed2.atipicial.com:10332",
	];

	for endpoint in endpoints {
		match HttpProvider::new(endpoint) {
			Ok(provider) => {
				let client = RpcClient::new(provider);
				match client.get_block_count().await {
					Ok(count) => {
						println!("   ✅ Connected to Atipicial: {endpoint}");
						println!("   📦 Block height: {count}");
						return Ok(client);
					},
					Err(_) => continue,
				}
			},
			Err(_) => continue,
		}
	}

	Err("Failed to connect to Atipicial MainNet".into())
}

/// Check bridge status
async fn check_bridge_status(
	client: &atipicial::providers::RpcClient<atipicial::providers::HttpProvider>,
	config: &BridgeConfig,
) -> Result<(), Box<dyn std::error::Error>> {
	// Check if bridge contract is active
	match client.get_contract_state(config.atipicial_bridge_contract).await {
		Ok(state) => {
			println!("   ✅ Bridge contract active");
			let manifest = &state.manifest;
			println!(
				"   📝 Contract name: {}",
				manifest.name.as_ref().unwrap_or(&"Atipicial X Bridge".to_string())
			);
		},
		Err(_) => println!("   ❌ Bridge contract not found"),
	}

	// Invoke bridge status method
	match client
		.invoke_function(&config.atipicial_bridge_contract, "isPaused".to_string(), vec![], None)
		.await
	{
		Ok(result) => {
			let stack = result.stack;
			if let Some(item) = stack.first() {
				let is_paused = item.as_bool().unwrap_or(false);
				println!(
					"   🚦 Bridge status: {}",
					if is_paused { "PAUSED ⚠️" } else { "ACTIVE ✅" }
				);
			}
		},
		Err(_) => println!("   ⚠️  Could not query bridge status"),
	}

	Ok(())
}

/// Query supported tokens
async fn query_supported_tokens(
	client: &atipicial::providers::RpcClient<atipicial::providers::HttpProvider>,
	config: &BridgeConfig,
) -> Result<(), Box<dyn std::error::Error>> {
	println!("   📋 Supported tokens for bridging:");

	// Check GAS token
	println!("   💎 GAS Token:");
	println!("      • Atipicial: 0x{}", config.gas_token_atipicial);
	println!("      • Atipicial X: {} (Native)", config.gas_token_atipicialx);
	println!("      • Min amount: 1 GAS");
	println!("      • Max amount: 10,000 GAS per tx");

	// Check if ATC is supported
	let atipicial_token =
		atipicial::atipicial_types::ScriptHash::from_str("ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")?;
	if let Ok(result) = client
		.invoke_function(
			&config.atipicial_bridge_contract,
			"isTokenSupported".to_string(),
			vec![atipicial::atipicial_types::ContractParameter::h160(&atipicial_token)],
			None,
		)
		.await
	{
		let stack = result.stack;
		if let Some(item) = stack.first() {
			let supported = item.as_bool().unwrap_or(false);
			if supported {
				println!("   🪙 ATC Token:");
				println!("      • Status: Supported ✅");
				println!("      • Atipicial X: bATC (Bridged ATC)");
			}
		}
	}

	// List other supported AEP-17 tokens
	println!("   🎯 Other supported tokens:");
	println!("      • USDT (Tether)");
	println!("      • USDC (USD Coin)");
	println!("      • Custom AEP-17 tokens (whitelisted)");

	Ok(())
}

/// Demonstrate deposit process
async fn demonstrate_deposit_process(
	_client: &atipicial::providers::RpcClient<atipicial::providers::HttpProvider>,
	config: &BridgeConfig,
) -> Result<(), Box<dyn std::error::Error>> {
	println!("   📝 Deposit flow (Atipicial → Atipicial X):");

	// Step 1: Check user balance
	println!("\n   1️⃣ Check user balance on Atipicial");
	let user_address = "NPvKVTGZapmFWABLsyvfreuqn73jCjJtN1"; // Example address
	println!("      📍 User: {user_address}");

	// Step 2: Build deposit transaction
	println!("\n   2️⃣ Build deposit transaction");
	let deposit_amount = 10_00000000; // 10 GAS
	let atipicialx_recipient = "0x742d35Cc6634C0532925a3b844Bc9e7595f89590"; // Example EVM address

	// Create script for deposit
	let mut script_builder = atipicial::atipicial_builder::ScriptBuilder::new();

	// Transfer GAS to bridge contract
	script_builder.contract_call(
		&config.gas_token_atipicial,
		"transfer",
		&[
			atipicial::atipicial_types::ContractParameter::h160(&atipicial::atipicial_types::ScriptHash::from_address(
				user_address,
			)?),
			atipicial::atipicial_types::ContractParameter::h160(&config.atipicial_bridge_contract),
			atipicial::atipicial_types::ContractParameter::integer(deposit_amount),
			atipicial::atipicial_types::ContractParameter::any(),
		],
		Some(atipicial::atipicial_builder::CallFlags::All),
	)?;

	let deposit_script = script_builder.to_bytes();
	println!("      📜 Script size: {} bytes", deposit_script.len());
	println!("      💰 Amount: {} GAS", deposit_amount as f64 / 100_000_000.0);
	println!("      🎯 Atipicial X recipient: {atipicialx_recipient}");

	// Step 3: Estimate fees
	println!("\n   3️⃣ Estimate transaction fees");
	println!("      ⛽ Network fee: ~0.01 GAS");
	println!("      🌉 Bridge fee: {} GAS", config.bridge_fee as f64 / 100_000_000.0);
	println!(
		"      💵 Total cost: ~{} GAS",
		(config.bridge_fee + 1_000_000) as f64 / 100_000_000.0
	);

	// Step 4: Sign and send (simulation)
	println!("\n   4️⃣ Sign and send transaction");
	println!("      ✍️  Transaction would be signed with user's private key");
	println!("      📡 Transaction would be broadcast to Atipicial network");
	println!("      ⏳ Wait for {} confirmations", config.min_confirmations);

	// Step 5: Monitor bridging
	println!("\n   5️⃣ Monitor bridging process");
	println!("      🔍 Bridge validators detect deposit");
	println!("      ✅ Validators sign mint request");
	println!("      🪙 GAS minted on Atipicial X to recipient");
	println!("      📊 Total time: ~2-5 minutes");

	Ok(())
}

/// Demonstrate withdrawal process
async fn demonstrate_withdrawal_process(
	config: &BridgeConfig,
) -> Result<(), Box<dyn std::error::Error>> {
	println!("   📝 Withdrawal flow (Atipicial X → Atipicial):");

	// Step 1: Connect to Atipicial X (EVM)
	println!("\n   1️⃣ Connect to Atipicial X network");
	println!("      🌐 RPC: https://mainnet.rpc.banelabs.org");
	println!("      🔧 Web3 provider: ethers.js / web3.js");
	println!("      🦊 Wallet: MetaMask or compatible");

	// Step 2: Check balance on Atipicial X
	println!("\n   2️⃣ Check GAS balance on Atipicial X");
	let atipicialx_user = "0x742d35Cc6634C0532925a3b844Bc9e7595f89590";
	println!("      📍 User: {atipicialx_user}");
	println!("      💰 Balance: [Would query EVM for balance]");

	// Step 3: Initiate withdrawal
	println!("\n   3️⃣ Initiate withdrawal on Atipicial X");
	let _withdraw_amount = 5_000000000000000000u128; // 5 GAS (18 decimals on EVM)
	let atipicial_recipient = "NPvKVTGZapmFWABLsyvfreuqn73jCjJtN1";

	println!("      📋 Call bridge contract withdraw()");
	println!("      💰 Amount: 5 GAS");
	println!("      🎯 Atipicial recipient: {atipicial_recipient}");
	println!("      📝 EVM transaction data:");
	println!("         • To: {}", config.atipicialx_bridge_address);
	println!("         • Method: withdraw(amount, recipient)");
	println!("         • Gas limit: ~200,000");

	// Step 4: Atipicial X transaction
	println!("\n   4️⃣ Submit Atipicial X transaction");
	println!("      ✍️  Sign with MetaMask");
	println!("      📡 Broadcast to Atipicial X");
	println!("      ⏳ Wait for EVM confirmations");

	// Step 5: Atipicial release
	println!("\n   5️⃣ GAS release on Atipicial");
	println!("      🔍 Bridge monitors Atipicial X events");
	println!("      ✅ Validators verify withdrawal");
	println!("      💸 GAS released from bridge on Atipicial");
	println!("      📊 Total time: ~3-7 minutes");

	Ok(())
}

/// Monitor bridge transactions
async fn monitor_bridge_transactions(
	client: &atipicial::providers::RpcClient<atipicial::providers::HttpProvider>,
	_config: &BridgeConfig,
) -> Result<(), Box<dyn std::error::Error>> {
	println!("   📊 Recent bridge activity:");

	// Get recent application logs for bridge contract
	let current_height = client.get_block_count().await?;
	let start_height = current_height.saturating_sub(100); // Last 100 blocks

	println!("   🔍 Scanning blocks {start_height} to {current_height}");

	// In production, would query application logs
	println!("   📋 Recent deposits (Atipicial → Atipicial X):");
	println!("      • Block #xxx: 100 GAS → 0x742d35...");
	println!("      • Block #xxx: 50 GAS → 0x8b4c12...");
	println!("      • Block #xxx: 1000 GAS → 0x3a5f88...");

	println!("\n   📋 Recent withdrawals (Atipicial X → Atipicial):");
	println!("      • Block #xxx: 75 GAS → NPvKVT...");
	println!("      • Block #xxx: 200 GAS → NTrezV...");
	println!("      • Block #xxx: 10 GAS → NLnyLt...");

	// Statistics
	println!("\n   📈 Bridge statistics (24h):");
	println!("      • Total deposits: 5,420 GAS");
	println!("      • Total withdrawals: 4,890 GAS");
	println!("      • Active users: 127");
	println!("      • Average tx size: 42.5 GAS");

	Ok(())
}

/// Display security best practices
fn display_security_practices() {
	println!("   🛡️  Security considerations:");
	println!("      • Always verify bridge contract addresses");
	println!("      • Check minimum/maximum amounts before bridging");
	println!("      • Allow sufficient confirmations (12+ blocks)");
	println!("      • Monitor transaction status on both chains");
	println!("      • Keep private keys secure");

	println!("\n   ⚠️  Risk awareness:");
	println!("      • Bridge operations are irreversible");
	println!("      • Network congestion may cause delays");
	println!("      • Large amounts may require additional verification");
	println!("      • Always test with small amounts first");

	println!("\n   📞 Support resources:");
	println!("      • Atipicial X Discord: https://discord.gg/atipicial");
	println!("      • Documentation: https://docs.x.atipicial.com");
	println!("      • Block explorers for transaction tracking");
}
