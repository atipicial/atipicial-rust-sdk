use atipicial::{
	atipicial_builder::{CallFlags, ScriptBuilder, Signer},
	atipicial_clients::{APITrait, HttpProvider, RpcClient},
	atipicial_crypto::HashableForVec,
	atipicial_protocol::{Account, AccountTrait},
	atipicial_types::{ContractParameter, AtipicialVMStateType, ScriptHash},
};
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

/// Demonstrates how to build a deploy transaction using a real AEF + manifest,
/// simulate it, and compute the expected contract hash. This example does not
/// broadcast the transaction; it focuses on creating a working deployment script.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("🚀 Atipicial Smart Contract Deployment Example (TestNet)");
	println!("====================================================");

	// 1) Connect to TestNet
	let client = RpcClient::new(HttpProvider::new("https://testnet1.atipicial.com:443")?);
	println!("   ✅ Connected to TestNet");

	// 2) Load deployer key from environment (avoid hardcoding secrets).
	let deployer_wif = env::var("ATC_WIF")?;
	let deployer = Account::from_wif(&deployer_wif)?;
	println!("   📍 Deployer: {}", deployer.get_address());

	// 3) Load AEF + manifest fixtures from repo
	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
	let aef_bytes = std::fs::read(
		manifest_dir
			.join("../..")
			.join("test_resources/responses/contract/contracts/TestContract.aef"),
	)?;
	let manifest_json = std::fs::read_to_string(
		manifest_dir
			.join("../..")
			.join("test_resources/responses/contract/contracts/TestContract.manifest.json"),
	)?;
	println!(
		"   📦 Loaded AEF ({} bytes) and manifest ({} bytes)",
		aef_bytes.len(),
		manifest_json.len()
	);

	// 4) Build deploy script (ContractManagement.deploy)
	let mgmt_hash = ScriptHash::from_str("fffdc93764dbaddd97c48f252a53ea4643faa3fd")?;
	let deploy_params = vec![
		ContractParameter::byte_array(aef_bytes.clone()),
		ContractParameter::string(manifest_json.clone()),
		ContractParameter::any(),
	];
	let deploy_script = ScriptBuilder::new()
		.contract_call(&mgmt_hash, "deploy", &deploy_params, Some(CallFlags::All))?
		.to_bytes();
	println!("   🔧 Deployment script size: {} bytes", deploy_script.len());

	// 5) Simulate deployment to inspect VM state and gas
	match client.invoke_script(hex::encode(&deploy_script), vec![]).await {
		Ok(sim) => {
			println!("   🧪 Simulation state: {:?}", sim.state);
			println!("   ⛽ Gas consumed (simulation): {}", sim.gas_consumed);
			println!("   🧱 Stack items returned: {}", sim.stack.len());
		},
		Err(e) => {
			println!("   ⚠️  Simulation failed: {e}");
			println!("      (Ensure the node is reachable and accepts invokeScript calls)");
		},
	};

	// 6) Compute expected contract hash (simplified demo hash)
	let expected_hash = calculate_contract_hash(&deployer.get_script_hash(), &manifest_json)?;
	println!("   🔑 Expected contract hash: {expected_hash}");

	// 7) (Optional) Build a transaction with signer; not signed/broadcast here
	let mut builder: atipicial::atipicial_builder::TransactionBuilder<HttpProvider> =
		atipicial::atipicial_builder::TransactionBuilder::new();
	builder.set_script(Some(deploy_script.clone()));
	builder.set_signers(vec![Signer::AccountSigner(
		atipicial::atipicial_builder::AccountSigner::called_by_entry_hash160(deployer.get_script_hash())?,
	)])?;
	let block_height = match client.get_block_count().await {
		Ok(h) => h,
		Err(e) => {
			println!("   ⚠️  Could not fetch latest block height: {e}");
			0
		},
	};
	builder.valid_until_block(block_height + 1000)?;
	println!("   📝 Transaction ready for signing (valid until block {})", block_height + 1000);
	println!("   💡 Sign and send with your own key when ready.");

	Ok(())
}

fn calculate_contract_hash(
	sender: &atipicial::atipicial_types::ScriptHash,
	manifest_json: &str,
) -> Result<atipicial::atipicial_types::ScriptHash, Box<dyn std::error::Error>> {
	// Demo-only hash: SHA256(sender || manifest_json) then take first 20 bytes.
	let mut data = Vec::new();
	data.extend_from_slice(sender.as_bytes());
	data.extend_from_slice(manifest_json.as_bytes());

	let hash = data.hash256();
	Ok(atipicial::atipicial_types::ScriptHash::from_slice(&hash[..20]))
}

fn _assert_display_state(state: AtipicialVMStateType) -> String {
	format!("{:?}", state)
}
