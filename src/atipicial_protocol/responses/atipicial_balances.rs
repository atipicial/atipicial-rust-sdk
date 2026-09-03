use getset::Getters;
use serde::{Deserialize, Serialize};

use atipicial::prelude::{deserialize_script_hash, serialize_script_hash, ScriptHash};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Aep11Balances {
	pub address: String,
	#[serde(rename = "balance")]
	pub balances: Vec<Aep11Balance>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Aep11Balance {
	pub name: String,
	pub symbol: String,
	pub decimals: String,
	pub tokens: Vec<Aep11Token>,
	#[serde(rename = "assethash")]
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	pub asset_hash: ScriptHash,
}

impl Aep11Balance {
	// Constructor function to create a new Aep11Balance instance
	pub fn new(
		asset_hash: ScriptHash,
		name: String,
		symbol: String,
		decimals: String,
		tokens: Vec<Aep11Token>,
	) -> Self {
		Aep11Balance { name, symbol, decimals, tokens, asset_hash }
	}
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Aep11Token {
	#[serde(rename = "tokenid")]
	pub token_id: String,
	pub amount: String,
	#[serde(rename = "lastupdatedblock")]
	pub last_updated_block: u32,
}

impl Aep11Token {
	// Constructor function to create a new Aep11Token instance
	pub fn new(token_id: String, amount: String, last_updated_block: u32) -> Self {
		Aep11Token { token_id, amount, last_updated_block }
	}
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Aep17Balances {
	pub address: String,
	#[serde(rename = "balance", default)]
	pub balances: Vec<Aep17Balance>,
}

#[derive(Getters, Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Aep17Balance {
	pub name: Option<String>,
	pub symbol: Option<String>,
	pub decimals: Option<String>,
	pub amount: String,
	#[serde(rename = "lastupdatedblock")]
	pub last_updated_block: u32,
	#[serde(rename = "assethash")]
	#[serde(deserialize_with = "deserialize_script_hash")]
	#[serde(serialize_with = "serialize_script_hash")]
	pub asset_hash: ScriptHash,
}

impl Aep17Balance {
	// Constructor equivalent to the Java constructor
	pub fn new(
		asset_hash: ScriptHash,
		name: Option<String>,
		symbol: Option<String>,
		decimals: Option<String>,
		amount: String,
		last_updated_block: u32,
	) -> Self {
		Self { name, symbol, decimals, amount, last_updated_block, asset_hash }
	}
}
