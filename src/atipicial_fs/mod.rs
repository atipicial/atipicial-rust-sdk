// Copyright (c) 2023-2025 R3E Network
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Atipicial File Storage (AtipicialFs)
//!
//! AtipicialFs is a decentralized distributed object storage network integrated with
//! the Atipicial Blockchain. It provides a robust platform for storing, retrieving,
//! and managing digital assets with blockchain-level security.
//!
//! ## Overview
//!
//! This module provides Rust bindings to interact with AtipicialFs services, including:
//!
//! - **Container Management**: Create, retrieve, list, and delete AtipicialFs containers
//! - **Object Operations**: Upload, download, and manage objects in containers
//! - **Access Control**: Manage permissions and generate access tokens
//! - **Extended Features**: Support for multipart uploads and specialized storage operations
//!
//! ## Backend Scope
//!
//! The bundled client targets AtipicialFs REST and HTTP gateways. Gateway-backed object and container
//! requests are sent to the configured endpoint and parsed strictly. Native signed operations
//! such as bearer-token minting, session-token negotiation, and owner-only ACL updates require a
//! native AtipicialFs signer/backend; the REST gateway client reports those as unsupported instead of
//! fabricating credentials.
//!
//! ## Example (TestNet)
//!
//! ```no_run
//! use atipicial::atipicial_fs::client::{AtipicialFsClient, AtipicialFsConfig, DEFAULT_TESTNET_REST_API};
//! use atipicial::atipicial_fs::{AtipicialFsAuth, AtipicialFsService};
//! use std::env;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Set this to the owner identifier used by the AtipicialFs gateway you're targeting.
//!     let wallet_address = env::var("ATCFS_WALLET")?;
//!
//!     let config = AtipicialFsConfig::builder()
//!         .endpoint(DEFAULT_TESTNET_REST_API.to_string())
//!         .auth(AtipicialFsAuth { wallet_address, private_key: None })
//!         .timeout_sec(10)
//!         .insecure(false)
//!         .build();
//!
//!     let client = AtipicialFsClient::new(config);
//!     let containers = client.list_containers().await?;
//!     println!("Found {} containers", containers.len());
//!     Ok(())
//! }
//! ```

pub mod acl;
pub mod client;
pub mod container;
pub mod error;
pub mod object;
pub mod types;

pub use client::AtipicialFsClient;
pub use error::{AtipicialFsError, AtipicialFsResult};

// Re-export types directly from types module
pub use acl::{BearerToken, SessionToken};
pub use container::Container;
pub use object::{MultipartUpload, MultipartUploadResult, Object, Part};
pub use types::{
	AccessPermission, Attributes, ContainerId, ObjectId, ObjectType, OwnerId, PlacementPolicy,
};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Default mainnet AtipicialFs gRPC endpoint.
pub const DEFAULT_MAINNET_ENDPOINT: &str = "grpc.mainnet.fs.atipicial.com:8082";

/// Default testnet AtipicialFs gRPC endpoint.
pub const DEFAULT_TESTNET_ENDPOINT: &str = "grpc.testnet.fs.atipicial.com:8082";

/// Default AtipicialFs endpoint (alias for mainnet gRPC endpoint).
pub const DEFAULT_ENDPOINT: &str = DEFAULT_MAINNET_ENDPOINT;

/// Default mainnet AtipicialFs HTTP gateway (typically used for object download/public access).
pub const DEFAULT_MAINNET_HTTP_GATEWAY: &str = "https://http.mainnet.fs.atipicial.com";

/// Default testnet AtipicialFs HTTP gateway (typically used for object download/public access).
pub const DEFAULT_TESTNET_HTTP_GATEWAY: &str = "https://http.testnet.fs.atipicial.com";

/// Default mainnet AtipicialFs REST API base URL.
pub const DEFAULT_MAINNET_REST_API: &str = "https://rest.mainnet.fs.atipicial.com";

/// Default testnet AtipicialFs REST API base URL.
pub const DEFAULT_TESTNET_REST_API: &str = "https://rest.testnet.fs.atipicial.com";

/// Represents a AtipicialFs service provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AtipicialFsConfig {
	/// The AtipicialFs service endpoint URL.
	///
	/// Prefer a REST API base URL (e.g. `DEFAULT_TESTNET_REST_API`). If a gRPC host:port string is
	/// provided (e.g. `grpc.testnet.fs.atipicial.com:8082`), `AtipicialFsClient::new` will map it to a default
	/// REST base URL.
	pub endpoint: String,
	/// Authentication information, typically from a Atipicial wallet
	pub auth: Option<AtipicialFsAuth>,
	/// Timeout for AtipicialFs operations in seconds
	pub timeout_sec: u64,
	/// Specifies whether to use insecure connections
	pub insecure: bool,
}

impl AtipicialFsConfig {
	/// Creates a new builder for the configuration
	pub fn builder() -> AtipicialFsConfigBuilder {
		AtipicialFsConfigBuilder::default()
	}
}

/// Builder for `AtipicialFsConfig`
#[derive(Debug, Default, Clone)]
pub struct AtipicialFsConfigBuilder {
	endpoint: Option<String>,
	auth: Option<AtipicialFsAuth>,
	timeout_sec: Option<u64>,
	insecure: Option<bool>,
}

impl AtipicialFsConfigBuilder {
	/// Sets the endpoint
	#[must_use]
	pub fn endpoint(mut self, val: String) -> Self {
		self.endpoint = Some(val);
		self
	}

	/// Sets the authentication information
	#[must_use]
	pub fn auth(mut self, val: AtipicialFsAuth) -> Self {
		self.auth = Some(val);
		self
	}

	/// Sets the timeout in seconds
	#[must_use]
	pub fn timeout_sec(mut self, val: u64) -> Self {
		self.timeout_sec = Some(val);
		self
	}

	/// Sets whether to use insecure connections
	#[must_use]
	pub fn insecure(mut self, val: bool) -> Self {
		self.insecure = Some(val);
		self
	}

	/// Builds the `AtipicialFsConfig`
	pub fn build(self) -> AtipicialFsConfig {
		let default = AtipicialFsConfig::default();
		AtipicialFsConfig {
			endpoint: self.endpoint.unwrap_or(default.endpoint),
			auth: self.auth.or(default.auth),
			timeout_sec: self.timeout_sec.unwrap_or(default.timeout_sec),
			insecure: self.insecure.unwrap_or(default.insecure),
		}
	}
}

impl Default for AtipicialFsConfig {
	fn default() -> Self {
		Self {
			endpoint: DEFAULT_TESTNET_REST_API.to_string(),
			auth: None,
			timeout_sec: 60,
			insecure: false,
		}
	}
}

/// Authentication information for AtipicialFs
#[derive(Clone, Serialize, Deserialize)]
pub struct AtipicialFsAuth {
	/// The wallet account used for authentication
	pub wallet_address: String,
	/// The private key to sign AtipicialFs requests
	pub private_key: Option<String>,
}

impl fmt::Debug for AtipicialFsAuth {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("AtipicialFsAuth")
			.field("wallet_address", &self.wallet_address)
			.field("private_key", &self.private_key.as_ref().map(|_| "<redacted>"))
			.finish()
	}
}

/// Service trait for interacting with AtipicialFs
#[async_trait]
pub trait AtipicialFsService {
	/// Creates a new container in AtipicialFs
	async fn create_container(&self, container: &Container) -> AtipicialFsResult<ContainerId>;

	/// Gets a container by its ID
	async fn get_container(&self, id: &ContainerId) -> AtipicialFsResult<Container>;

	/// Lists all containers owned by the current account
	async fn list_containers(&self) -> AtipicialFsResult<Vec<ContainerId>>;

	/// Deletes a container by its ID
	async fn delete_container(&self, id: &ContainerId) -> AtipicialFsResult<bool>;

	/// Uploads an object to a container
	async fn put_object(
		&self,
		container_id: &ContainerId,
		object: &Object,
	) -> AtipicialFsResult<ObjectId>;

	/// Gets an object by its ID from a container
	async fn get_object(
		&self,
		container_id: &ContainerId,
		object_id: &ObjectId,
	) -> AtipicialFsResult<Object>;

	/// Lists all objects in a container
	async fn list_objects(&self, container_id: &ContainerId) -> AtipicialFsResult<Vec<ObjectId>>;

	/// Deletes an object by its ID from a container
	async fn delete_object(
		&self,
		container_id: &ContainerId,
		object_id: &ObjectId,
	) -> AtipicialFsResult<bool>;

	/// Creates a bearer token for accessing objects in a container
	async fn create_bearer_token(
		&self,
		container_id: &ContainerId,
		permissions: Vec<AccessPermission>,
		expires_sec: u64,
	) -> AtipicialFsResult<BearerToken>;

	/// Gets a session token for the current account
	async fn get_session_token(&self) -> AtipicialFsResult<SessionToken>;

	/// Initiates a multipart upload for a large object
	async fn initiate_multipart_upload(
		&self,
		container_id: &ContainerId,
		object: &Object,
	) -> AtipicialFsResult<MultipartUpload>;

	/// Uploads a part of a multipart upload
	async fn upload_part(
		&self,
		upload: &MultipartUpload,
		part_number: u32,
		data: Vec<u8>,
	) -> AtipicialFsResult<Part>;

	/// Completes a multipart upload
	async fn complete_multipart_upload(
		&self,
		upload: &MultipartUpload,
		parts: Vec<Part>,
	) -> AtipicialFsResult<MultipartUploadResult>;

	/// Aborts a multipart upload
	async fn abort_multipart_upload(&self, upload: &MultipartUpload) -> AtipicialFsResult<bool>;
}
