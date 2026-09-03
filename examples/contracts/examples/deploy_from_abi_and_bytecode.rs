/// Atipicial Contract Deployment from AEF and Manifest
///
/// This example demonstrates how to deploy smart contracts on Atipicial
/// using compiled AEF files and manifest data.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("📦 Atipicial Contract Deployment Example");
	println!("=====================================");

	println!("\n📚 Understanding Atipicial Smart Contract Format:");
	println!("   • AEF (Atipicial Executable Format) - Compiled bytecode");
	println!("   • Manifest - Contract metadata and permissions");
	println!("   • Both files required for deployment");
	println!("   • Contracts are immutable once deployed");
	println!("   • Upgradeable contracts require special design");

	println!("\n🔧 AEF File Structure:");
	println!("   • Magic header (0x3346454E)");
	println!("   • Compiler information");
	println!("   • Source code hash");
	println!("   • Method tokens");
	println!("   • Script bytecode");
	println!("   • Checksum validation");

	println!("\n📋 Manifest Contents:");
	println!("   • Contract name and version");
	println!("   • ABI (methods, events, parameters)");
	println!("   • Permissions and trust settings");
	println!("   • Supported standards (AEP-17, etc.)");
	println!("   • Extra metadata");
	println!("   • Safe methods list");

	println!("\n💡 Deployment Process:");
	println!("   1. Compile contract to AEF + Manifest");
	println!("   2. Calculate deployment costs");
	println!("   3. Build deployment script");
	println!("   4. Create deployment transaction");
	println!("   5. Sign and send transaction");
	println!("   6. Wait for confirmation");
	println!("   7. Verify deployment success");

	println!("\n💰 Deployment Costs:");
	println!("   • Base deployment fee: 10 GAS");
	println!("   • Storage fee: Based on contract size");
	println!("   • Additional fees for contract name");
	println!("   • System fee for execution");
	println!("   • Network fee for transaction");

	println!("\n⚙️ Contract Management Methods:");
	println!("   • deploy - Deploy new contract");
	println!("   • update - Update existing contract");
	println!("   • destroy - Remove contract");
	println!("   • getContract - Query contract info");
	println!("   • getContractById - Query by ID");
	println!("   • getContractHashes - List all contracts");

	println!("\n🔐 Permission System:");
	println!("   • Wildcard (*) - Call any contract/method");
	println!("   • Contract-specific - Call specific contracts");
	println!("   • Method-specific - Call specific methods");
	println!("   • Group permissions - Trust contract groups");
	println!("   • ECDsa verification - Custom signatures");

	println!("\n📦 Example Deployment Script:");
	println!("   // Load AEF and Manifest");
	println!("   let aef = load_aef_file(\"contract.aef\");");
	println!("   let manifest = load_manifest(\"contract.manifest.json\");");
	println!("   ");
	println!("   // Build deployment script");
	println!("   script_builder.contract_call(");
	println!("     MANAGEMENT_CONTRACT,");
	println!("     \"deploy\",");
	println!("     [aef_bytes, manifest_json]");
	println!("   );");

	println!("\n⚠️ Deployment Best Practices:");
	println!("   • Test thoroughly on testnet first");
	println!("   • Verify manifest permissions");
	println!("   • Check contract size limits");
	println!("   • Implement upgrade mechanism if needed");
	println!("   • Document deployment parameters");
	println!("   • Backup deployment transaction ID");

	println!("\n🎯 Post-Deployment Steps:");
	println!("   • Verify contract on explorer");
	println!("   • Test all contract methods");
	println!("   • Set up monitoring");
	println!("   • Initialize contract state");
	println!("   • Transfer ownership if applicable");
	println!("   • Publish contract address");

	println!("\n🚀 For deployment examples, see:");
	println!("   • examples/atipicial_contracts/");
	println!("   • Atipicial smart contract documentation");
	println!("   • Contract development tools");

	Ok(())
}
