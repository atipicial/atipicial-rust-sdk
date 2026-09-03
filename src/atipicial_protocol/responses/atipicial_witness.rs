use crate::{builder::Witness, Base64Encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct AtipicialWitness {
	pub invocation: String,
	pub verification: String,
}

impl AtipicialWitness {
	pub fn new(invocation: String, verification: String) -> Self {
		Self { invocation, verification }
	}

	pub fn from_witness(witness: Witness) -> Self {
		Self {
			invocation: Base64Encode::to_base64(witness.invocation.script()),
			verification: Base64Encode::to_base64(witness.verification.script()),
		}
	}
}
