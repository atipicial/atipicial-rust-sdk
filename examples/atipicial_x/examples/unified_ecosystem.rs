use atipicial::atipicial_clients::{HttpProvider, RpcClient};
use atipicial::atipicial_x::{AtipicialXProvider, AtipicialXWallet};
use atipicial::sdk::unified::EcosystemClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// 1. Initialize N3 Provider (Optional if you just want EVM, but useful for bridge/unified)
	let n3_provider = HttpProvider::new("https://mainnet1.atipicial.com:443")?;
	let n3_client = RpcClient::new(n3_provider);

	// 2. Setup Atipicial X EVM components
	let atipicial_x_provider = AtipicialXProvider::new("https://rpc.atipicial-x.org", Some(&n3_client));

	// Create a new randomized EVM Wallet (or load from PK)
	let evm_wallet = AtipicialXWallet::create_random();
	println!("Created new EVM Wallet address: {:?}", evm_wallet.address());

	// 3. Create the Unified Client for Atipicial X
	let ecosystem_client = EcosystemClient::new_atipicialx(evm_wallet, atipicial_x_provider);

	// 4. Get Balance directly via the Unified Interface
	// Returns a human-readable GAS amount (18 decimals on Atipicial X).
	match ecosystem_client.get_balance().await {
		Ok(balance) => println!("Atipicial X Balance: {} GAS", balance),
		Err(e) => println!("Error getting balance: {}", e),
	}

	// 5. Example of Anti-MEV Client setup
	let anti_mev_wallet = AtipicialXWallet::create_random();
	let _anti_mev_client = EcosystemClient::new_atipicialx_anti_mev(anti_mev_wallet);

	// Any transactions sent through anti_mev_client will route through the protected mempool endpoint
	println!("Anti-MEV client initialized successfully.");

	Ok(())
}
