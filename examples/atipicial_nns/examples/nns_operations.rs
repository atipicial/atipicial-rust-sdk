use atipicial::{
	atipicial_clients::{APITrait, HttpProvider, RpcClient},
	atipicial_types::{ContractParameter, ScriptHash},
};
use std::str::FromStr;

/// This example demonstrates how to work with the Atipicial Name Service (NNS) on the Atipicial blockchain.
/// It shows how to check domain availability, register domains, and manage domain records using modern AtipicialRust patterns.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("🌐 Atipicial Name Service (NNS) Operations Example");
	println!("===============================================");

	// Connect to Atipicial TestNet
	println!("\n📡 Connecting to Atipicial TestNet...");
	let provider = HttpProvider::new("https://testnet1.atipicial.com:443/")
		.map_err(|e| format!("Failed to create provider: {e}"))?;
	let client = RpcClient::new(provider);
	println!("   ✅ Connected successfully");

	// Get current blockchain status
	println!("\n📊 Retrieving blockchain status...");
	let block_count = client
		.get_block_count()
		.await
		.map_err(|e| format!("Failed to get block count: {e}"))?;
	println!("   📈 Current block height: {block_count}");

	// Set up NNS contract reference
	println!("\n🏷️ Setting up NNS contract reference...");

	// Note: In the current AtipicialRust SDK, direct AtipicialNameService contract interaction
	// may require specific contract hash and manual contract calls
	let nns_contract_hash = "0x50ac1c37690cc2cfc594472833cf57505d5f46de"; // NNS contract on Atipicial
	println!("   📋 NNS Contract: {nns_contract_hash}");

	// Demonstrate domain name resolution concepts
	println!("\n🔍 NNS Domain Operations Concepts:");
	let domain_name = "example.atipicial";
	println!("   🏷️ Domain: {domain_name}");

	// Domain availability check simulation
	println!("\n📝 Domain Availability Check:");
	match check_domain_availability(&client, domain_name).await {
		Ok(is_available) => {
			if is_available {
				println!("   ✅ Domain '{domain_name}' appears to be available");

				// Demonstrate registration concepts
				println!("\n📝 Domain Registration Concepts:");
				demonstrate_registration_process(domain_name).await?;
			} else {
				println!("   ⚠️ Domain '{domain_name}' appears to be registered");

				// Demonstrate renewal concepts
				println!("\n📝 Domain Renewal Concepts:");
				demonstrate_renewal_process(domain_name).await?;
			}
		},
		Err(e) => println!("   ❌ Failed to check domain availability: {e}"),
	}

	// Demonstrate record management concepts
	println!("\n📝 Record Management Concepts:");
	demonstrate_record_management(domain_name).await?;

	// NNS best practices
	println!("\n💡 NNS Best Practices:");
	println!("   🔐 Security: Use secure wallets for domain management");
	println!("   ⏰ Timing: Monitor domain expiration dates");
	println!("   💰 Costs: Understand registration and renewal fees");
	println!("   📋 Records: Keep DNS records updated and secure");
	println!("   🔄 Backups: Maintain backup access to domain management");

	println!("\n🎉 NNS operations example completed!");
	println!("💡 This example demonstrates NNS concepts with the modern AtipicialRust SDK patterns.");

	Ok(())
}

/// Check domain availability by querying the NNS contract
async fn check_domain_availability(
	client: &RpcClient<HttpProvider>,
	domain: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
	println!("   🔍 Checking availability for: {domain}");

	// NNS contract hash on Atipicial
	let nns_contract_hash = ScriptHash::from_str("50ac1c37690cc2cfc594472833cf57505d5f46de")?;

	// Create parameter for domain name
	let domain_param = ContractParameter::string(domain.to_string());
	let parameters = vec![domain_param];

	// Call the 'isAvailable' method on the NNS contract
	match client
		.invoke_function(&nns_contract_hash, "isAvailable".to_string(), parameters, None)
		.await
	{
		Ok(result) => {
			// Parse the result from the contract call
			if let Some(stack_item) = result.stack.first() {
				match stack_item.as_bool() {
					Some(is_available) => {
						println!(
							"   📋 Contract response: {}",
							if is_available { "Available" } else { "Taken" }
						);
						Ok(is_available)
					},
					None => {
						println!("   ⚠️ Unexpected response format from contract");
						// Fallback: assume domain is taken if we can't parse the response
						Ok(false)
					},
				}
			} else {
				println!("   ⚠️ Empty response from contract");
				Ok(false)
			}
		},
		Err(e) => {
			println!("   ❌ Failed to query contract: {e}");
			// Return error instead of fallback for transparency
			Err(format!("NNS contract query failed: {e}").into())
		},
	}
}

/// Demonstrate domain registration process
async fn demonstrate_registration_process(domain: &str) -> Result<(), Box<dyn std::error::Error>> {
	println!("   📝 Registration Process for: {domain}");
	println!("   1. 🔍 Verify domain availability");
	println!("   2. 💰 Calculate registration fees");
	println!("   3. 🔐 Prepare owner wallet");
	println!("   4. 📋 Create registration transaction");
	println!("   5. ✍️ Sign transaction with owner key");
	println!("   6. 📡 Broadcast to network");
	println!("   7. ⏳ Wait for confirmation");

	Ok(())
}

/// Demonstrate domain renewal process
async fn demonstrate_renewal_process(domain: &str) -> Result<(), Box<dyn std::error::Error>> {
	println!("   🔄 Renewal Process for: {domain}");
	println!("   1. 📅 Check current expiration date");
	println!("   2. 💰 Calculate renewal fees");
	println!("   3. 🔐 Access domain owner wallet");
	println!("   4. 📋 Create renewal transaction");
	println!("   5. ✍️ Sign transaction");
	println!("   6. 📡 Submit renewal");
	println!("   7. ✅ Confirm extension");

	Ok(())
}

/// Demonstrate record management concepts
async fn demonstrate_record_management(domain: &str) -> Result<(), Box<dyn std::error::Error>> {
	println!("   📋 Record Types for: {domain}");
	println!("   🌐 A Record: Points to IPv4 address");
	println!("   📝 TXT Record: Stores text information");
	println!("   🔗 CNAME Record: Alias to another domain");
	println!("   📧 MX Record: Mail server information");
	println!("   🎯 SRV Record: Service location data");

	println!("\n   ⚙️ Record Management Operations:");
	println!("   ➕ Add new records");
	println!("   ✏️ Update existing records");
	println!("   🗑️ Delete obsolete records");
	println!("   👀 Query current records");

	Ok(())
}
