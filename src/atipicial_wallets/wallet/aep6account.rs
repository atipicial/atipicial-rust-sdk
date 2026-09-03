use std::{collections::HashMap, fmt};

use crate::{
	builder::VerificationScript,
	codec::AtipicialSerializable,
	atipicial_protocol::Account,
	atipicial_wallets::{AEP6Contract, AEP6Parameter, WalletError},
	Address, AddressOrScriptHash, Base64Encode, ContractParameterType, ScriptHashExtension,
	StringExt,
};
use getset::{Getters, Setters};
use p256::elliptic_curve::zeroize::Zeroize;
use primitive_types::H160;
use serde::{Deserialize, Serialize};

/// Represents an account in the AEP-6 format.
#[derive(Clone, Serialize, Deserialize, Getters, Setters)]
pub struct AEP6Account {
	/// The address of the account.
	#[getset(get = "pub")]
	#[serde(rename = "address")]
	pub address: Address,

	/// An optional label for the account.
	#[getset(get = "pub")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "label")]
	pub label: Option<String>,

	/// Indicates whether the account is set as default.
	#[getset(get = "pub")]
	#[serde(default)]
	#[serde(rename = "isDefault")]
	pub is_default: bool,

	/// Indicates whether the account is locked.
	#[getset(get = "pub")]
	#[serde(rename = "lock")]
	pub lock: bool,

	/// An optional private key associated with the account.
	#[getset(get = "pub")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "key")]
	pub key: Option<String>,

	/// An optional AEP-6 contract associated with the account.
	#[getset(get = "pub")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "contract")]
	pub contract: Option<AEP6Contract>,

	/// An optional additional data associated with the account.
	#[getset(get = "pub")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(rename = "extra")]
	pub extra: Option<HashMap<String, String>>,
}

impl fmt::Debug for AEP6Account {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("AEP6Account")
			.field("address", &self.address)
			.field("label", &self.label)
			.field("is_default", &self.is_default)
			.field("lock", &self.lock)
			.field("has_key", &self.key.is_some())
			.field("contract", &self.contract)
			.field("extra", &self.extra.as_ref().map(|m| m.len()))
			.finish()
	}
}

impl AEP6Account {
	/// Creates a new AEP-6 account with the given parameters.
	///
	/// # Arguments
	///
	/// * `address` - The address of the account.
	/// * `label` - An optional label for the account.
	/// * `is_default` - Indicates whether the account is set as default.
	/// * `lock` - Indicates whether the account is locked.
	/// * `key` - An optional private key associated with the account.
	/// * `contract` - An optional AEP-6 contract associated with the account.
	/// * `extra` - An optional additional data associated with the account.
	///
	/// # Example
	///
	/// ```
	/// use std::collections::HashMap;
	/// use atipicial::prelude::*;
	///
	/// let address = "example_address".to_string();
	/// let label = Some("My Account".to_string());
	/// let is_default = true;
	/// let lock = false;
	/// let key = Some("example_private_key".to_string());
	/// let contract = None; // AEP6Contract::new() is not directly available
	/// let extra = Some(HashMap::new());
	///
	/// let account = wallets::AEP6Account::new(address, label, is_default, lock, key, contract, extra);
	/// ```
	pub fn new(
		address: Address,
		label: Option<String>,
		is_default: bool,
		lock: bool,
		key: Option<String>,
		contract: Option<AEP6Contract>,
		extra: Option<HashMap<String, String>>,
	) -> Self {
		Self { address, label, is_default, lock, key, contract, extra }
	}

	/// Converts an `Account` into a `AEP6Account`.
	///
	/// # Arguments
	///
	/// * `account` - The account to convert.
	///
	/// # Errors
	///
	/// Returns a `WalletError` if there is an issue converting the account.
	///
	/// # Example
	///
	/// ```
	/// use atipicial::prelude::*;
	/// use atipicial::atipicial_protocol::AccountTrait;
	///
	/// let account = protocol::Account::create().unwrap();
	/// let aep6_account = wallets::AEP6Account::from_account(&account);
	/// ```
	pub fn from_account(account: &Account) -> Result<AEP6Account, WalletError> {
		if account.key_pair.is_some() && account.encrypted_private_key.is_none() {
			return Err(WalletError::AccountState(
				"Account private key is available but not encrypted.".to_string(),
			));
		}

		let mut parameters = Vec::new();
		if let Some(verification_script) = &account.verification_script {
			if verification_script.is_multi_sig() {
				for i in 0..verification_script.get_nr_of_accounts()? {
					parameters.push(AEP6Parameter {
						param_name: format!("signature{i}"),
						param_type: ContractParameterType::Signature,
					});
				}
			} else if verification_script.is_single_sig() {
				parameters.push(AEP6Parameter {
					param_name: "signature".to_string(),
					param_type: ContractParameterType::Signature,
				});
			}
		}

		let contract = if !parameters.is_empty() {
			Some(AEP6Contract {
				script: account
					.verification_script
					.as_ref()
					.map(|script| script.to_array().to_base64()),
				is_deployed: false,
				aep6_parameters: parameters,
			})
		} else {
			None
		};

		Ok(AEP6Account {
			address: account.address_or_scripthash.address().clone(),
			label: account.label.clone(),
			is_default: account.is_default,
			lock: account.is_locked,
			key: account.encrypted_private_key.clone(),
			contract,
			extra: None,
		})
	}

	/// Converts a `AEP6Account` into an `Account`.
	///
	/// Accounts with an address or contract verification script are converted directly.
	/// Encrypted-key-only accounts are also accepted and use a temporary script hash until
	/// [`crate::atipicial_protocol::AccountTrait::decrypt_private_key`] derives the real single-signature state. Truly
	/// incomplete accounts without address, contract script, or encrypted key return an error.
	///
	/// # Errors
	///
	/// Returns a `WalletError` if there is an issue converting the account.
	///
	/// # Example
	///
	/// ```
	/// use atipicial::prelude::*;
	/// # let aep6_account = wallets::AEP6Account::new(String::new(), None, false, false, None, None, None);
	/// let account = aep6_account.to_account();
	/// ```
	pub fn to_account(&self) -> Result<Account, WalletError> {
		let mut verification_script: Option<VerificationScript> = None;
		let mut signing_threshold: Option<u8> = None;
		let mut nr_of_participants: Option<u8> = None;

		if let Some(contract) = &self.contract {
			if contract.script.is_some() {
				verification_script = Some(VerificationScript::from(
					contract
						.script
						.clone()
						.ok_or_else(|| {
							WalletError::AccountState("Contract script is missing".to_string())
						})?
						.base64_decoded()
						.map_err(|e| {
							WalletError::AccountState(format!(
								"Failed to decode base64 script: {}",
								e
							))
						})?,
				));

				if let Some(script) = verification_script.as_ref() {
					if script.is_multi_sig() {
						signing_threshold = Some(script.get_signing_threshold()? as u8);
						nr_of_participants = Some(script.get_nr_of_accounts()? as u8);
					}
				}
			}
		}

		let script_hash = if self.address.is_empty() {
			if let Some(script) = verification_script.as_ref() {
				H160::from_script(script.script())
			} else if self.key.is_some() {
				H160::zero()
			} else {
				return Err(WalletError::AccountState(
					"AEP6 account is missing both address and verification script".to_string(),
				));
			}
		} else {
			let address = self.address.clone();
			H160::from_address(&address).map_err(|e| {
				WalletError::AccountState(format!("Invalid address '{address}': {e}"))
			})?
		};

		Ok(Account {
			key_pair: None,
			address_or_scripthash: AddressOrScriptHash::ScriptHash(script_hash),
			label: self.label.clone(),
			verification_script,
			is_default: self.is_default,
			is_locked: self.lock,
			encrypted_private_key: self.key.clone(),
			signing_threshold: signing_threshold.map(|s| s as u32),
			nr_of_participants: nr_of_participants.map(|s| s as u32),
			wallet: None,
		})
	}
}

impl PartialEq for AEP6Account {
	/// Checks if two `AEP6Account` instances are equal based on their addresses.
	///
	/// # Example
	///
	/// ```
	///
	/// use atipicial::prelude::*;
	///
	/// let account1 = wallets::AEP6Account::new(String::new(), None, false, false, None, None, None);
	/// let account2 = wallets::AEP6Account::new(String::new(), None, false, false, None, None, None);
	/// assert_eq!(account1, account2);
	/// ```
	fn eq(&self, other: &Self) -> bool {
		self.address == other.address
	}
}

impl Drop for AEP6Account {
	fn drop(&mut self) {
		// Zeroize the encrypted private key to prevent sensitive data
		// from lingering in memory after the account is dropped.
		if let Some(ref mut key) = self.key {
			key.zeroize();
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		config::TestConstants,
		crypto::{PrivateKeyExtension, Secp256r1PrivateKey, Secp256r1PublicKey},
		atipicial_clients::ProviderError,
		atipicial_protocol::{Account, AccountTrait},
		atipicial_types::Base64Encode,
		atipicial_wallets::AEP6Account,
		ContractParameterType,
	};

	#[test]
	fn test_decrypt_with_standard_scrypt_params() {
		use crate::{crypto::KeyPair, atipicial_protocol::AEP2};

		let private_key = Secp256r1PrivateKey::from_bytes(
			&hex::decode(TestConstants::DEFAULT_ACCOUNT_PRIVATE_KEY)
				.expect("Should be able to decode valid hex in test"),
		)
		.expect("Should be able to create private key from valid bytes in test");

		// Create a key pair and encrypt it with current parameters
		let key_pair = KeyPair::from_secret_key(&private_key);
		let encrypted_key = AEP2::encrypt(TestConstants::DEFAULT_ACCOUNT_PASSWORD, &key_pair)
			.expect("Should be able to encrypt key pair");

		let aep6_account =
			AEP6Account::new("".to_string(), None, true, false, Some(encrypted_key), None, None);

		let mut account = aep6_account
			.to_account()
			.expect("Should be able to convert AEP6Account to Account in test");

		account
			.decrypt_private_key(TestConstants::DEFAULT_ACCOUNT_PASSWORD)
			.expect("Should be able to decrypt private key with correct password in test");

		assert_eq!(
			account
				.key_pair
				.clone()
				.expect("Key pair should be present after decryption")
				.private_key_bytes()
				.unwrap()
				.to_vec(),
			private_key.to_vec()
		);

		// Decrypt again
		account
			.decrypt_private_key(TestConstants::DEFAULT_ACCOUNT_PASSWORD)
			.expect("Should be able to decrypt private key with correct password in test");
		assert_eq!(
			account
				.key_pair
				.clone()
				.expect("Key pair should be present after decryption")
				.private_key()
				.unwrap(),
			private_key
		);
	}

	#[test]
	fn test_decrypt_encrypted_only_account_repairs_script_hash() {
		use crate::{crypto::KeyPair, atipicial_protocol::AEP2};

		let private_key = Secp256r1PrivateKey::from_bytes(
			&hex::decode(TestConstants::DEFAULT_ACCOUNT_PRIVATE_KEY)
				.expect("Should be able to decode valid hex in test"),
		)
		.expect("Should be able to create private key from valid bytes in test");

		let key_pair = KeyPair::from_secret_key(&private_key);
		let encrypted_key = AEP2::encrypt(TestConstants::DEFAULT_ACCOUNT_PASSWORD, &key_pair)
			.expect("Should be able to encrypt key pair");

		let aep6_account =
			AEP6Account::new(String::new(), None, true, false, Some(encrypted_key), None, None);

		let mut account = aep6_account
			.to_account()
			.expect("Should be able to convert encrypted-only AEP6Account to Account in test");

		account
			.decrypt_private_key(TestConstants::DEFAULT_ACCOUNT_PASSWORD)
			.expect("Should be able to decrypt private key with correct password in test");

		assert_eq!(account.get_script_hash(), key_pair.get_script_hash());
	}

	#[test]
	fn test_load_account_from_aep6() {
		let data = include_str!("../../../test_resources/wallet/account.json");
		let aep6_account: AEP6Account = serde_json::from_str(data)
			.expect("Should be able to deserialize valid AEP6Account JSON in test");

		let account = aep6_account
			.to_account()
			.expect("Should be able to convert AEP6Account to Account in test");

		assert!(!account.is_default);
		assert!(!account.is_locked);
		assert_eq!(
			account.address_or_scripthash().address(),
			TestConstants::DEFAULT_ACCOUNT_ADDRESS
		);
		assert_eq!(
			account
				.encrypted_private_key()
				.clone()
				.expect("Encrypted private key should be present"),
			TestConstants::DEFAULT_ACCOUNT_ENCRYPTED_PRIVATE_KEY
		);

		assert_eq!(
			account
				.verification_script
				.as_ref()
				.expect("Verification script should be present")
				.script(),
			&hex::decode(TestConstants::DEFAULT_ACCOUNT_VERIFICATION_SCRIPT)
				.expect("Should be able to decode valid verification script hex in test")
		);
	}

	#[test]
	fn test_load_multi_sig_account_from_aep6() {
		let data = include_str!("../../../test_resources/wallet/multiSigAccount.json");
		let aep6_account: AEP6Account = serde_json::from_str(data)
			.expect("Should be able to deserialize valid AEP6Account JSON in test");

		let account = aep6_account
			.to_account()
			.expect("Should be able to convert AEP6Account to Account in test");

		assert!(!account.is_default);
		assert!(!account.is_locked);
		assert_eq!(
			account.address_or_scripthash().address(),
			TestConstants::COMMITTEE_ACCOUNT_ADDRESS
		);
		assert_eq!(
			account
				.verification_script()
				.clone()
				.expect("Verification script should be present")
				.script(),
			&hex::decode(TestConstants::COMMITTEE_ACCOUNT_VERIFICATION_SCRIPT)
				.expect("Should be able to decode valid verification script hex in test")
		);
		assert_eq!(
			account
				.get_nr_of_participants()
				.expect("Should be able to get number of participants"),
			1
		);
		assert_eq!(
			account
				.get_signing_threshold()
				.expect("Should be able to get signing threshold"),
			1
		);
	}

	#[test]
	fn test_to_aep6_account_with_only_an_address() {
		let account = Account::from_address(TestConstants::DEFAULT_ACCOUNT_ADDRESS)
			.expect("Should be able to create account from valid address in test");

		let aep6_account = account
			.to_aep6_account()
			.expect("Should be able to convert Account to AEP6Account in test");

		assert!(aep6_account.contract().is_none());
		assert!(!aep6_account.is_default());
		assert!(!aep6_account.lock());
		assert_eq!(aep6_account.address(), TestConstants::DEFAULT_ACCOUNT_ADDRESS);
		assert_eq!(
			aep6_account.label().clone().expect("Label should be present in test"),
			TestConstants::DEFAULT_ACCOUNT_ADDRESS
		);
		assert!(aep6_account.extra().is_none());
	}

	#[test]
	fn test_to_account_rejects_missing_address_and_contract_script() {
		let aep6_account = AEP6Account::new(String::new(), None, false, false, None, None, None);

		let result = aep6_account.to_account();
		assert!(result.is_err());
	}

	#[test]
	fn test_to_aep6_account_with_unecrypted_private_key() {
		let account = Account::from_wif(TestConstants::DEFAULT_ACCOUNT_WIF)
			.expect("Should be able to create account from valid WIF in test");

		let err = account.to_aep6_account().unwrap_err();

		assert_eq!(
			err,
			ProviderError::IllegalState(
				"Account private key is available but not encrypted.".to_string()
			)
		);
	}

	#[test]
	fn test_to_aep6_account_with_ecrypted_private_key() {
		let mut account = Account::from_wif(TestConstants::DEFAULT_ACCOUNT_WIF)
			.expect("Should be able to create account from valid WIF in test");
		account
			.encrypt_private_key("atipicial")
			.expect("Should be able to encrypt private key with password in test");

		let aep6_account = account
			.to_aep6_account()
			.expect("Should be able to convert Account to AEP6Account in test");

		assert_eq!(
			aep6_account
				.contract()
				.clone()
				.expect("Contract should be present")
				.script()
				.clone()
				.expect("Script should be present"),
			TestConstants::DEFAULT_ACCOUNT_VERIFICATION_SCRIPT.to_string().to_base64()
		);

		// Instead of comparing exact encrypted value, verify that:
		// 1. There is an encrypted key
		// 2. It can be decrypted back to the original private key
		let encrypted_key = aep6_account.key().clone().expect("Key should be present");
		assert!(!encrypted_key.is_empty());
		assert!(encrypted_key.starts_with("6P")); // AEP-2 encrypted keys start with "6P"

		// Verify we can decrypt it back
		let mut account_from_aep6 = aep6_account
			.to_account()
			.expect("Should be able to convert AEP6Account to Account");
		account_from_aep6
			.decrypt_private_key("atipicial")
			.expect("Should be able to decrypt with correct password");

		// Verify the decrypted private key matches the original
		let original_private_key =
			hex::decode(TestConstants::DEFAULT_ACCOUNT_PRIVATE_KEY).expect("Should decode hex");
		assert_eq!(
			account_from_aep6
				.key_pair
				.as_ref()
				.expect("Key pair should be present")
				.private_key_bytes()
				.unwrap()
				.to_vec(),
			original_private_key
		);

		assert!(!aep6_account.is_default());
		assert!(!aep6_account.lock());
		assert_eq!(aep6_account.address(), TestConstants::DEFAULT_ACCOUNT_ADDRESS);
		assert_eq!(
			aep6_account.label().clone().expect("Label should be present in test"),
			TestConstants::DEFAULT_ACCOUNT_ADDRESS
		);
	}

	#[test]
	fn test_to_aep6_account_with_muliti_sig_account() {
		let public_key = Secp256r1PublicKey::from_bytes(
			&hex::decode(TestConstants::DEFAULT_ACCOUNT_PUBLIC_KEY)
				.expect("Should be able to decode valid public key hex in test"),
		)
		.expect("Should be able to create public key from valid bytes in test");
		let account = Account::multi_sig_from_public_keys(&mut [public_key], 1)
			.expect("Should be able to create multi-sig account from valid public key in test");
		let aep6_account = account
			.to_aep6_account()
			.expect("Should be able to convert Account to AEP6Account in test");

		assert_eq!(
			aep6_account
				.contract()
				.clone()
				.expect("Contract should be present")
				.script()
				.clone()
				.expect("Script should be present"),
			TestConstants::COMMITTEE_ACCOUNT_VERIFICATION_SCRIPT.to_string().to_base64()
		);
		assert!(!aep6_account.is_default());
		assert!(!aep6_account.lock());
		assert_eq!(aep6_account.address(), TestConstants::COMMITTEE_ACCOUNT_ADDRESS);
		assert_eq!(
			aep6_account.label().clone().expect("Label should be present"),
			TestConstants::COMMITTEE_ACCOUNT_ADDRESS
		);
		assert!(aep6_account.key().is_none());
		assert_eq!(
			aep6_account
				.contract()
				.clone()
				.expect("Contract should be present")
				.aep6_parameters()[0]
				.param_name(),
			"signature0"
		);
		assert_eq!(
			aep6_account
				.contract()
				.clone()
				.expect("Contract should be present")
				.aep6_parameters()[0]
				.param_type(),
			&ContractParameterType::Signature
		);
	}
}
