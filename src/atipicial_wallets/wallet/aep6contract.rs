use getset::Getters;
use serde::{Deserialize, Serialize};

use atipicial::prelude::ContractParameterType;

/// Represents a AEP-6 contract.
#[derive(Clone, Debug, Serialize, Deserialize, Getters)]
pub struct AEP6Contract {
	/// The script associated with the contract.
	#[getset(get = "pub")]
	#[serde(rename = "script")]
	pub script: Option<String>,

	/// Indicates whether the contract is deployed.
	#[getset(get = "pub")]
	#[serde(rename = "deployed")]
	pub is_deployed: bool,

	/// The AEP-6 parameters associated with the contract.
	#[getset(get = "pub")]
	#[serde(rename = "parameters")]
	pub aep6_parameters: Vec<AEP6Parameter>,
}

/// Represents a AEP-6 parameter.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Getters)]
pub struct AEP6Parameter {
	/// The name of the parameter.
	#[getset(get = "pub")]
	#[serde(rename = "name")]
	pub param_name: String,

	/// The type of the parameter.
	#[getset(get = "pub")]
	#[serde(rename = "type")]
	pub param_type: ContractParameterType,
}

impl PartialEq for AEP6Contract {
	/// Checks if two `AEP6Contract` instances are equal.
	///
	/// # Example
	///
	/// ```
	/// use atipicial::prelude::*;
	///
	/// # let contract1 = wallets::AEP6Contract { script: None, is_deployed: false, aep6_parameters: vec![] };
	/// # let contract2 = wallets::AEP6Contract { script: None, is_deployed: false, aep6_parameters: vec![] };
	/// assert_eq!(contract1, contract2);
	/// ```
	fn eq(&self, other: &Self) -> bool {
		self.script == other.script
			&& self.aep6_parameters == other.aep6_parameters
			&& self.is_deployed == other.is_deployed
	}
}
