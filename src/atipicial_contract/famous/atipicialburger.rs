use async_trait::async_trait;
use hex_literal::hex;
use primitive_types::H160;
use serde::{Deserialize, Serialize};

use crate::atipicial_types::{
	deserialize_script_hash, serialize_script_hash, ContractParameter, NNSName, ScriptHash,
};
use crate::{
	builder::{AccountSigner, TransactionBuilder},
	atipicial_clients::{JsonRpcProvider, RpcClient},
	atipicial_contract::{ContractError, SmartContractTrait, TokenTrait},
	atipicial_protocol::Account,
};

/// AtipicialburgerAtipicial contract interface for Atipicial
///
/// AtipicialburgerAtipicial (bATC) is a wrapped ATC token that allows users to earn GAS while using their ATC in DeFi.
/// This contract interface provides methods to interact with the AtipicialburgerAtipicial smart contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtipicialburgerContract<'a, P: JsonRpcProvider> {
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	script_hash: ScriptHash,
	#[serde(skip_serializing_if = "Option::is_none")]
	total_supply: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	decimals: Option<u8>,
	#[serde(skip_serializing_if = "Option::is_none")]
	symbol: Option<String>,
	#[serde(skip)]
	provider: Option<&'a RpcClient<P>>,
}

impl<'a, P: JsonRpcProvider + 'static> AtipicialburgerContract<'a, P> {
	/// The script hash of the AtipicialburgerAtipicial contract on Atipicial MainNet
	pub const CONTRACT_HASH: &'static str = "48c40d4666f93408be1bef038b6722404f5c4a5a";
	/// The symbol of the AtipicialburgerAtipicial token
	pub const SYMBOL: &'static str = "bATC";
	/// The number of decimals for the AtipicialburgerAtipicial token
	pub const DECIMALS: u8 = 8;

	// Method constants
	/// Method name for wrapping ATC to bATC
	pub const WRAP: &'static str = "wrap";
	/// Method name for unwrapping bATC to ATC
	pub const UNWRAP: &'static str = "unwrap";
	/// Method name for claiming GAS
	pub const CLAIM_GAS: &'static str = "claimGas";
	/// Method name for getting the exchange rate
	pub const GET_RATE: &'static str = "getRate";

	/// Creates a new AtipicialburgerContract instance with the default contract hash
	///
	/// # Arguments
	///
	/// * `provider` - An optional reference to an RPC client
	///
	/// # Returns
	///
	/// A new AtipicialburgerContract instance
	pub fn new(provider: Option<&'a RpcClient<P>>) -> Self {
		Self {
			script_hash: ScriptHash::from(hex!("48c40d4666f93408be1bef038b6722404f5c4a5a")),
			total_supply: None,
			decimals: Some(Self::DECIMALS),
			symbol: Some(Self::SYMBOL.to_string()),
			provider,
		}
	}

	/// Creates a new AtipicialburgerContract instance with a custom script hash
	///
	/// # Arguments
	///
	/// * `script_hash` - The script hash of the AtipicialburgerAtipicial contract
	/// * `provider` - An optional reference to an RPC client
	///
	/// # Returns
	///
	/// A new AtipicialburgerContract instance
	pub fn with_script_hash(script_hash: ScriptHash, provider: Option<&'a RpcClient<P>>) -> Self {
		Self {
			script_hash,
			total_supply: None,
			decimals: Some(Self::DECIMALS),
			symbol: Some(Self::SYMBOL.to_string()),
			provider,
		}
	}

	/// Wraps ATC to bATC
	///
	/// # Arguments
	///
	/// * `amount` - The amount of ATC to wrap
	/// * `account` - The account that will sign the transaction
	///
	/// # Returns
	///
	/// A transaction builder that can be used to build and sign the transaction
	pub async fn wrap(
		&self,
		amount: i64,
		account: &Account,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		let params = vec![ContractParameter::integer(amount)];

		let mut builder = self.invoke_function(Self::WRAP, params).await?;
		let signer = AccountSigner::called_by_entry(account)
			.map_err(|err| ContractError::RuntimeError(err.to_string()))?;
		builder
			.set_signers(vec![signer.into()])
			.map_err(|err| ContractError::RuntimeError(err.to_string()))?;

		Ok(builder)
	}

	/// Unwraps bATC to ATC
	///
	/// # Arguments
	///
	/// * `amount` - The amount of bATC to unwrap
	/// * `account` - The account that will sign the transaction
	///
	/// # Returns
	///
	/// A transaction builder that can be used to build and sign the transaction
	pub async fn unwrap(
		&self,
		amount: i64,
		account: &Account,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		let params = vec![ContractParameter::integer(amount)];

		let mut builder = self.invoke_function(Self::UNWRAP, params).await?;
		let signer = AccountSigner::called_by_entry(account)
			.map_err(|err| ContractError::RuntimeError(err.to_string()))?;
		builder
			.set_signers(vec![signer.into()])
			.map_err(|err| ContractError::RuntimeError(err.to_string()))?;

		Ok(builder)
	}

	/// Claims GAS rewards from holding bATC
	///
	/// # Arguments
	///
	/// * `account` - The account that will sign the transaction
	///
	/// # Returns
	///
	/// A transaction builder that can be used to build and sign the transaction
	pub async fn claim_gas(
		&self,
		account: &Account,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		let params = vec![];

		let mut builder = self.invoke_function(Self::CLAIM_GAS, params).await?;
		let signer = AccountSigner::called_by_entry(account)
			.map_err(|err| ContractError::RuntimeError(err.to_string()))?;
		builder
			.set_signers(vec![signer.into()])
			.map_err(|err| ContractError::RuntimeError(err.to_string()))?;

		Ok(builder)
	}

	/// Gets the current exchange rate between ATC and bATC
	///
	/// # Returns
	///
	/// The exchange rate as a floating-point number
	pub async fn get_rate(&self) -> Result<f64, ContractError> {
		let result = self.call_function_returning_int(Self::GET_RATE, vec![]).await?;
		// Convert the integer result to a floating-point rate (assuming rate is stored as an integer with a fixed decimal point)
		Ok(result as f64 / 100_000_000.0) // Assuming 8 decimal places
	}
}

#[async_trait]
impl<'a, P: JsonRpcProvider> SmartContractTrait<'a> for AtipicialburgerContract<'a, P> {
	type P = P;

	fn script_hash(&self) -> H160 {
		self.script_hash
	}

	fn set_script_hash(&mut self, script_hash: H160) {
		self.script_hash = script_hash;
	}

	fn provider(&self) -> Option<&RpcClient<P>> {
		self.provider
	}
}

#[async_trait]
impl<'a, P: JsonRpcProvider> TokenTrait<'a, P> for AtipicialburgerContract<'a, P> {
	fn total_supply(&self) -> Option<u64> {
		self.total_supply
	}

	fn set_total_supply(&mut self, total_supply: u64) {
		self.total_supply = Some(total_supply);
	}

	fn decimals(&self) -> Option<u8> {
		self.decimals
	}

	fn set_decimals(&mut self, decimals: u8) {
		self.decimals = Some(decimals);
	}

	fn symbol(&self) -> Option<String> {
		self.symbol.clone()
	}

	fn set_symbol(&mut self, symbol: String) {
		self.symbol = Some(symbol);
	}

	async fn resolve_nns_text_record(&self, _name: &NNSName) -> Result<H160, ContractError> {
		Err(ContractError::InvalidAtipicialName(
			"AtipicialburgerAtipicial does not support NNS resolution".to_string(),
		))
	}
}
