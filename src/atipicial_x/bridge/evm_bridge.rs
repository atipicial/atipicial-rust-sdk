use alloy::primitives::{Address, U256};
use alloy::providers::RootProvider;
use alloy::sol;
use std::sync::Arc;

use crate::atipicial_contract::ContractError;

// sol! binding for the Atipicial X EVM Bridge Contract.
// (alloy::sol! is the maintained successor to ethers::contract::abigen!.)
sol! {
	#[sol(rpc)]
	interface AtipicialXBridgeEVM {
		function withdraw(address token, uint256 amount, string memory destination) external payable;
		function getFee(address token) external view returns (uint256);
	}
}

/// A wrapper around the Atipicial X bridge contract on the EVM side.
/// Used to bridge assets from Atipicial X back to Atipicial.
pub struct AtipicialXBridgeContractEVM {
	contract: AtipicialXBridgeEVM::AtipicialXBridgeEVMInstance<Arc<RootProvider>>,
}

impl AtipicialXBridgeContractEVM {
	/// Creates a new AtipicialXBridgeContractEVM instance with an explicit contract address.
	pub fn new(address: Address, provider: Arc<RootProvider>) -> Self {
		let contract = AtipicialXBridgeEVM::new(address, provider);
		Self { contract }
	}

	/// Creates an instance using `ATCX_BRIDGE_EVM_ADDRESS`.
	pub fn default_bridge(provider: Arc<RootProvider>) -> Result<Self, ContractError> {
		let configured = std::env::var("ATCX_BRIDGE_EVM_ADDRESS").map_err(|_| {
			ContractError::InvalidStateError(
				"ATCX_BRIDGE_EVM_ADDRESS must be set to the deployed Atipicial X bridge contract address"
					.to_string(),
			)
		})?;
		let address: Address = configured.parse().map_err(|e| {
			ContractError::InvalidArgError(format!(
				"Invalid ATCX_BRIDGE_EVM_ADDRESS '{}': {}",
				configured, e
			))
		})?;
		if address == Address::ZERO {
			return Err(ContractError::InvalidArgError(
				"ATCX_BRIDGE_EVM_ADDRESS must not be the zero address".to_string(),
			));
		}
		Ok(Self::new(address, provider))
	}

	/// Returns the on-chain address of the bridge contract.
	pub fn address(&self) -> Address {
		*self.contract.address()
	}

	/// Gets the required bridge fee for a given token
	pub async fn get_fee(&self, token: Address) -> Result<U256, alloy::contract::Error> {
		self.contract.getFee(token).call().await
	}
}
