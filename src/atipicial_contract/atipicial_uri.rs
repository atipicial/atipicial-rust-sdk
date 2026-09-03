use std::str::FromStr;

use crate::{
	atipicial_builder::{AccountSigner, ScriptBuilder, TransactionBuilder},
	atipicial_clients::{JsonRpcProvider, RpcClient},
	atipicial_contract::{
		ContractError, FungibleTokenContract, GasToken, AtipicialCoin, SmartContractTrait, TokenTrait,
	},
	atipicial_protocol::Account,
	atipicial_types::{
		serde_with_utils::{
			deserialize_script_hash_option, deserialize_url_option, serialize_script_hash_option,
			serialize_url_option,
		},
		ContractParameter, ScriptHash, ScriptHashExtension,
	},
};
use getset::{Getters, Setters};
use primitive_types::H160;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)]
pub struct AtipicialURI<'a, P: JsonRpcProvider> {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "deserialize_url_option")]
	#[serde(serialize_with = "serialize_url_option")]
	#[getset(get = "pub", set = "pub")]
	uri: Option<Url>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "deserialize_script_hash_option")]
	#[serde(serialize_with = "serialize_script_hash_option")]
	#[getset(get = "pub", set = "pub")]
	recipient: Option<ScriptHash>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(deserialize_with = "deserialize_script_hash_option")]
	#[serde(serialize_with = "serialize_script_hash_option")]
	#[getset(get = "pub", set = "pub")]
	token: Option<ScriptHash>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[getset(get = "pub", set = "pub")]
	amount: Option<u64>,
	#[serde(skip)]
	provider: Option<&'a RpcClient<P>>,
}

impl<'a, P: JsonRpcProvider + 'static> AtipicialURI<'a, P> {
	const ATC_SCHEME: &'static str = "atipicial";
	const MIN_AEP9_URI_LENGTH: usize = 38;
	const ATC_TOKEN_STRING: &'static str = "atipicial";
	const GAS_TOKEN_STRING: &'static str = "gas";

	pub fn new(provider: Option<&'a RpcClient<P>>) -> Self {
		Self { uri: None, recipient: None, token: None, amount: None, provider }
	}

	fn parse_token(token_str: &str) -> Option<ScriptHash> {
		match token_str {
			Self::ATC_TOKEN_STRING => Some(AtipicialCoin::<P>::new(None).script_hash()),
			Self::GAS_TOKEN_STRING => Some(GasToken::<P>::new(None).script_hash()),
			_ => {
				if let Ok(hash) = H160::from_str(token_str) {
					return Some(hash);
				}

				// Support ScriptHashExtension::to_bs58_string() encoding (20 raw bytes in base58)
				match bs58::decode(token_str).into_vec() {
					Ok(bytes) if bytes.len() == 20 => Some(H160::from_slice(&bytes)),
					_ => None,
				}
			},
		}
	}

	pub fn from_uri(uri_string: &str) -> Result<Self, ContractError> {
		let (base, query) = uri_string.split_once('?').unwrap_or((uri_string, ""));

		let (scheme, address) = base.split_once(':').ok_or_else(|| {
			ContractError::InvalidAtipicialName("Invalid AEP-9 URI: missing scheme separator".to_string())
		})?;

		if scheme != Self::ATC_SCHEME || uri_string.len() < Self::MIN_AEP9_URI_LENGTH {
			return Err(ContractError::InvalidAtipicialName("Invalid AEP-9 URI".to_string()));
		}

		let mut atipicial_uri = Self::new(None);
		atipicial_uri.recipient = Some(ScriptHash::from_address(address).map_err(|_| {
			ContractError::InvalidArgError("Invalid recipient address".to_string())
		})?);
		atipicial_uri.uri = uri_string.parse().ok();

		if !query.is_empty() {
			let query_str = query;
			for part in query_str.split("&") {
				let (key, value) = part
					.split_once('=')
					.ok_or_else(|| ContractError::InvalidAtipicialName("Invalid query".to_string()))?;

				if key.is_empty() || value.is_empty() {
					return Err(ContractError::InvalidAtipicialName("Invalid query".to_string()));
				}

				match key {
					"asset" if atipicial_uri.token().is_none() => {
						let token = Self::parse_token(value).ok_or_else(|| {
							ContractError::InvalidArgError("Invalid asset".to_string())
						})?;
						atipicial_uri.token = Some(token);
					},
					"amount" if atipicial_uri.amount.is_none() => {
						atipicial_uri.amount = Some(value.parse().map_err(|_| {
							ContractError::InvalidArgError("Invalid amount".to_string())
						})?);
					},
					_ => {},
				}
			}
		}

		Ok(atipicial_uri)
	}

	// Getters

	pub fn uri_string(&self) -> Option<String> {
		self.uri.as_ref().map(|uri| uri.to_string())
	}

	pub fn recipient_address(&self) -> Option<String> {
		self.recipient.as_ref().map(H160::to_address)
	}

	pub fn token_string(&self) -> Option<String> {
		self.token.as_ref().map(|token| match token {
			token if *token == AtipicialCoin::<P>::new(None).script_hash() => {
				Self::ATC_TOKEN_STRING.to_owned()
			},
			token if *token == GasToken::<P>::new(None).script_hash() => {
				Self::GAS_TOKEN_STRING.to_owned()
			},
			_ => ScriptHashExtension::to_bs58_string(token),
		})
	}

	// Builders

	pub async fn build_transfer_from(
		&self,
		sender: &Account,
	) -> Result<TransactionBuilder<'_, P>, ContractError> {
		let recipient = self
			.recipient
			.ok_or_else(|| ContractError::InvalidStateError("Recipient not set".to_string()))?;
		let amount = self
			.amount
			.ok_or_else(|| ContractError::InvalidStateError("Amount not set".to_string()))?;
		let token_hash = self
			.token
			.ok_or_else(|| ContractError::InvalidStateError("Token not set".to_string()))?;

		let mut token = FungibleTokenContract::new(&token_hash, self.provider);

		let decimals = token.get_decimals().await?;
		let amt = token.to_fractions(amount, decimals as u32)?;

		// Create a new TransactionBuilder
		let mut tx_builder = TransactionBuilder::new();

		// Build the script for the transfer
		let script = ScriptBuilder::new()
			.contract_call(
				&token_hash,
				"transfer",
				&[
					ContractParameter::h160(&sender.get_script_hash()),
					ContractParameter::h160(&recipient),
					ContractParameter::integer(amt as i64),
					ContractParameter::any(),
				],
				None,
			)
			.map_err(|err| ContractError::RuntimeError(err.to_string()))?
			.to_bytes();

		// Set up the TransactionBuilder
		tx_builder.set_script(Some(script));
		let signer = AccountSigner::called_by_entry(sender)
			.map_err(|err| ContractError::RuntimeError(err.to_string()))?;
		tx_builder
			.set_signers(vec![signer.into()])
			.map_err(|err| ContractError::RuntimeError(err.to_string()))?;

		Ok(tx_builder)
	}

	// Setters

	pub fn token_str(&mut self, token_str: &str) {
		self.token = Self::parse_token(token_str);
		if self.token.is_none() {
			tracing::warn!(token = %token_str, "Invalid asset string; ignoring");
		}
	}

	// URI builder

	fn build_query(&self) -> String {
		let mut parts = Vec::new();

		if let Some(token) = &self.token {
			let token_str = match token {
				token if *token == AtipicialCoin::new(self.provider).script_hash() => {
					Self::ATC_TOKEN_STRING.to_owned()
				},
				token if *token == GasToken::new(self.provider).script_hash() => {
					Self::GAS_TOKEN_STRING.to_owned()
				},
				_ => ScriptHashExtension::to_bs58_string(token),
			};

			parts.push(format!("asset={}", token_str));
		}

		if let Some(amount) = &self.amount {
			parts.push(format!("amount={}", amount));
		}

		parts.join("&")
	}

	pub fn build_uri(&mut self) -> Result<Url, ContractError> {
		let recipient = self
			.recipient
			.ok_or(ContractError::InvalidStateError("No recipient set".to_string()))?;

		let base = format!("{}:{}", Self::ATC_SCHEME, recipient.to_address());
		let query = self.build_query();
		let uri_str = if query.is_empty() { base } else { format!("{}?{}", base, query) };

		let uri: Url = uri_str
			.parse()
			.map_err(|e| ContractError::InvalidArgError(format!("Invalid AEP-9 URI: {e}")))?;
		self.uri = Some(uri.clone());

		Ok(uri)
	}
}
