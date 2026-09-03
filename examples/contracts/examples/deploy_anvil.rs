/// Atipicial Local Development Example
///
/// This example demonstrates how to set up a local Atipicial development environment.
/// Unlike Ethereum's Anvil, Atipicial uses atipicial-express for local development.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("🏗️ Atipicial Local Development Setup");
	println!("=================================");

	println!("\n📚 Atipicial Development Tools:");
	println!("   • atipicial-express - Local blockchain for development");
	println!("   • atipicial-devpack - Smart contract development kit");
	println!("   • atipicial-debugger - Contract debugging tools");
	println!("   • atipicial-test - Testing framework");

	println!("\n🔧 Setting Up atipicial-express:");
	println!("   1. Install: npm install -g @atipicial-one/atipicial-express");
	println!("   2. Create chain: atipicialxp create");
	println!("   3. Start node: atipicialxp run");
	println!("   4. Create wallet: atipicialxp wallet create <name>");
	println!("   5. Transfer assets: atipicialxp transfer");

	println!("\n💡 atipicial-express Features:");
	println!("   • Fast block times (1 second)");
	println!("   • Pre-funded wallets");
	println!("   • Checkpoint/restore functionality");
	println!("   • Time manipulation for testing");
	println!("   • Built-in contract deployment");

	println!("\n📋 Common Development Commands:");
	println!("   • atipicialxp show balances - View account balances");
	println!("   • atipicialxp contract deploy - Deploy contracts");
	println!("   • atipicialxp contract invoke - Call methods");
	println!("   • atipicialxp checkpoint create - Save state");
	println!("   • atipicialxp checkpoint restore - Restore state");

	println!("\n🚀 Development Workflow:");
	println!("   1. Start local atipicial-express chain");
	println!("   2. Create development wallets");
	println!("   3. Deploy test contracts");
	println!("   4. Run integration tests");
	println!("   5. Debug with checkpoints");

	println!("\n⚡ Advantages over Public Testnets:");
	println!("   • Instant transaction confirmation");
	println!("   • No rate limits");
	println!("   • Deterministic testing");
	println!("   • State snapshots");
	println!("   • Time control");

	println!("\n🔐 Development Best Practices:");
	println!("   • Use checkpoints before major changes");
	println!("   • Test with realistic gas limits");
	println!("   • Simulate network delays");
	println!("   • Test error scenarios");
	println!("   • Profile gas consumption");

	println!("\n📝 Example atipicial-express Configuration:");
	println!("   {{");
	println!("     \"magic\": 1234567890,");
	println!("     \"consensus-nodes\": 1,");
	println!("     \"block-time\": 1000,");
	println!("     \"node-port\": 50012,");
	println!("     \"rpc-port\": 50013");
	println!("   }}");

	println!("\n🎯 For more information:");
	println!("   • atipicial-express documentation");
	println!("   • Atipicial development guides");
	println!("   • examples/atipicial_contracts/");

	Ok(())
}
