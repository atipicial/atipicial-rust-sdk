/// Atipicial AEP-17 Token Transfer Example
///
/// This example demonstrates how to transfer AEP-17 tokens (like GAS) on the Atipicial blockchain.
/// AEP-17 is the standard token protocol on Atipicial, similar to ERC-20 on Ethereum.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("🪙 Atipicial AEP-17 Token Transfer Example");
	println!("=======================================");

	println!("\n📚 Understanding AEP-17 Tokens:");
	println!("   • AEP-17 is Atipicial's fungible token standard");
	println!("   • Native tokens: ATC (governance) and GAS (utility)");
	println!("   • All AEP-17 tokens follow the same interface");
	println!("   • Transfers require transaction signing");

	println!("\n🔧 Key Components for Token Transfer:");
	println!("   1. Connect to Atipicial network");
	println!("   2. Load sender's wallet and account");
	println!("   3. Build transfer script with ScriptBuilder");
	println!("   4. Create and sign transaction");
	println!("   5. Send transaction to network");
	println!("   6. Monitor transaction status");

	println!("\n📋 AEP-17 Token Methods:");
	println!("   • symbol() - Token symbol (e.g., 'GAS')");
	println!("   • decimals() - Token decimal places");
	println!("   • totalSupply() - Total token supply");
	println!("   • balanceOf(account) - Account balance");
	println!("   • transfer(from, to, amount, data) - Transfer tokens");

	println!("\n💡 Transfer Best Practices:");
	println!("   • Always check balance before transfer");
	println!("   • Validate recipient address format");
	println!("   • Consider network fees (system fee + network fee)");
	println!("   • Test with small amounts first");
	println!("   • Monitor transaction confirmation");
	println!("   • Handle transfer failures gracefully");

	println!("\n🔐 Security Considerations:");
	println!("   • Never expose private keys");
	println!("   • Verify contract addresses");
	println!("   • Check token decimals (GAS has 8 decimals)");
	println!("   • Validate transaction before signing");
	println!("   • Use hardware wallets for large amounts");

	println!("\n📝 Example Transfer Flow:");
	println!("   1. Check sender's GAS balance");
	println!("   2. Validate recipient address");
	println!("   3. Calculate amount with decimals");
	println!("   4. Build transfer script");
	println!("   5. Create transaction with expiry");
	println!("   6. Sign with sender's private key");
	println!("   7. Send to network");
	println!("   8. Wait for confirmation");

	println!("\n⚡ GAS Token Details:");
	println!("   • Contract: 0xd2a4cff31913016155e38e474a2c06d08be276cf");
	println!("   • Symbol: GAS");
	println!("   • Decimals: 8");
	println!("   • Divisible: Yes");
	println!("   • Used for: Transaction fees and smart contract execution");

	println!("\n💰 ATC Token Details:");
	println!("   • Contract: 0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5");
	println!("   • Symbol: ATC");
	println!("   • Decimals: 0");
	println!("   • Divisible: No (minimum unit is 1 ATC)");
	println!("   • Used for: Governance and GAS generation");

	println!("\n🚀 For working implementation examples, see:");
	println!("   • examples/atipicial_transactions/");
	println!("   • examples/atipicial_aep17_tokens/");
	println!("   • Atipicial documentation");

	Ok(())
}
