<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

<p align="center">

# ATIPICIAL RUST SDK

### Speak fluent Atipicial from Rust

**Production SDK: wallets · transactions · JSON-RPC · contract invocation · AEP-6 wallets · unified error model**

</p>

<p align="center">
  <img alt="Founder" src="https://img.shields.io/badge/%F0%9F%91%91_Founder-xmoohad-ff006e?style=for-the-badge">
  <img alt="Chain" src="https://img.shields.io/badge/Chain-Atipicial_L1-9d4edd?style=for-the-badge">
  <img alt="ATC" src="https://img.shields.io/badge/%F0%9F%AA%99_ATC-Atipicial_Coin-ffd60a?style=for-the-badge">
  <img alt="ATD" src="https://img.shields.io/badge/%F0%9F%92%B5_ATD-AtipicialDollar-06d6a0?style=for-the-badge">
  <img alt="License" src="https://img.shields.io/badge/License-MIT-3a86ff?style=for-the-badge">
</p>

---


Atipicial is a sovereign Layer-1 blockchain for smart contracts, digital assets,
and decentralized applications. **This SDK is
how Rust applications speak Atipicial**: build and sign transactions, manage
AEP-6 wallets, query JSON-RPC, invoke contracts, track AEP-17/AEP-11 balances,
and estimate ATD fees — through one typed, unified API with a single coherent
error model.

The SDK covers the full surface: key management, address derivation (every
address starts with capital `A`), transaction construction and signing,
RPC transport with retry/rate-limiting, Atipicial-X (EVM sidechain) providers,
and a circuit-breaker-guarded production client.


---

## ⚖️ The Design Laws of ATIPICIAL RUST SDK

1. **Real, not fake.** Every function that claims to do something, does it.
   No stubs, no mocks wearing production clothes, no vaporware APIs.
2. **Typed or it doesn't exist.** Strings lying about being integers are a bug.
   Domain types own their invariants at construction.
3. **Determinism above all.** No nondeterministic iteration, floating point,
   or wall-clock time anywhere near consensus or state.
4. **Boundaries are law.** Layers depend downward. A service never reopens a
   database another service owns. Capabilities cross boundaries as narrow traits.
5. **Fail loud, fail early.** Invalid configuration is a startup error, not a
   runtime surprise three weeks later.
6. **Performance is earned.** Measured, benchmarked, and never traded against
   correctness.


---

## ⛓️ Chain Identity — What Every Atipicial Component Shares

| Attribute | Value |
|---|---|
| **Chain** | Atipicial Chain — sovereign Layer-1 for smart contracts and digital assets |
| **ATC** | Atipicial Coin — governance & staking, 1,000,000,000 total |
| **ATD** | AtipicialDollar — settlement & fees, 500,000,000 genesis |
| **Addresses** | Begin with capital **`A`** (version byte `0x09`) |
| **Genesis** | 2026-07-20 00:00:00 UTC |
| **Standards** | AEP-17 (fungible) · AEP-11 (NFT) · AEP-6 (wallets) · AEP-2 (keys) |
| **Format** | AEF — Atipicial Executable Format |
| **Consensus** | dBFT 2.0 — single-block finality |
| **Seeds** | `seed1-5.atipicial.com:10333` (P2P) · `seed1-5.atipicial.com:10332` (RPC) |


---

## ⚡ Quick Start — atipicial-rust-sdk

```bash
```
# 1 — get it
cargo add atipicial

# 2 — run it
see examples/ — wallet create, transfer, invoke

# 3 — prove it works
let account = Account::create()?; // address starts with 'A'
```


```
## 🌳 Repository Tree

```text
atipicial-rust-sdk/
├── .cargo/
│   ├── audit.toml
│   └── config.toml
├── .env.example
├── .gitattributes
├── .github/
│   ├── dependabot.yml
│   └── workflows/
├── .gitignore
├── .serena/
│   ├── .gitignore
│   ├── memories/
│   └── project.yml
├── .vscode/
│   ├── launch.json
│   └── tasks.json
├── API_GUIDELINES.md
├── API_SPECIFICATION.md
├── CHANGELOG.md
├── COMPONENT_INTERFACES.md
├── CONTRIBUTING.md
├── COPYRIGHT
├── Cargo.lock
├── Cargo.toml
├── Dockerfile
├── KNOWN_ISSUES.md
├── LICENSE
├── LICENSE-APACHE
├── LICENSE-MIT
├── PROJECT_STRUCTURE.md
├── README.md
├── SYSTEM_ARCHITECTURE_DESIGN.md
├── assets/
│   ├── atipicial-rust-sdk-logo.svg
│   ├── atipicial_rust_banner.png
│   ├── atipicial_rust_logo.png
│   ├── images/
│   └── logo-atipicialrust.svg
├── atipicial-cli/
│   ├── .cargo/
│   ├── .gitignore
│   ├── Cargo.toml
│   ├── DOCUMENTATION.md
│   ├── README.md
│   ├── USAGE_EXAMPLES.md
│   ├── docs/
│   ├── src/
│   ├── templates/
│   └── tests/
├── benches/
│   ├── crypto_benchmarks.rs
│   ├── gas_estimator_benchmarks.rs
│   ├── script_builder_benchmarks.rs
│   └── wallet_benchmarks.rs
├── build.rs
├── cliff.toml
├── compose-dev.yaml
├── config/
│   ├── development.toml
│   └── production.toml
├── deny.toml
├── docs/
│   ├── API_DOCUMENTATION.md
│   ├── GAS_ESTIMATION_GUIDE.md
│   ├── PRODUCTION_DEPLOYMENT_GUIDE.md
│   ├── RATE_LIMITING_GUIDE.md
│   ├── README.md
│   ├── RELEASE_PROCESS.md
│   ├── SECURITY_AUDIT.md
│   ├── SGX_GUIDE.md
│   ├── assets/
│   ├── atipicial-x/
│   ├── book.toml
│   ├── contracts/
│   ├── crypto/
│   ├── getting-started.mdx
│   ├── guides/
│   ├── mdbook-admonish.css
│   ├── plans/
│   ├── src/
│   ├── superpowers/
│   ├── theme/
│   └── wallets/
├── examples/
│   ├── README.md
│   ├── advanced/
│   ├── atipicial_aep17_tokens/
│   ├── atipicial_contracts/
│   ├── atipicial_crypto/
│   ├── atipicial_famous_contracts/
│   ├── atipicial_fs/
│   ├── atipicial_nns/
│   ├── atipicial_nodes/
│   ├── atipicial_smart_contracts/
│   ├── atipicial_transactions/
│   ├── atipicial_wallets/
│   ├── atipicial_x/
│   ├── basic/
│   ├── big-numbers/
│   ├── contract_interaction/
│   ├── contracts/
│   ├── events/
│   ├── intermediate/
│   ├── message_signing/
│   ├── middleware/
│   ├── providers/
│   ├── queries/
│   ├── sgx_enclave/
│   ├── standalone/
│   ├── subscriptions/
│   ├── transactions/
│   └── wallets/
├── monitoring/
│   ├── README.md
│   ├── alertmanager/
│   ├── docker-compose.yml
│   ├── docs/
│   ├── grafana/
│   ├── loki/
│   ├── otel/
│   ├── prometheus/
│   ├── promtail/
│   └── test/
├── release.toml
├── rust-toolchain.toml
├── rustfmt.toml
├── src/
│   ├── atipicial_builder/
│   ├── atipicial_clients/
│   ├── atipicial_codec/
│   ├── atipicial_config/
│   ├── atipicial_contract/
│   ├── atipicial_crypto/
│   ├── atipicial_error/
│   ├── atipicial_fs/
│   ├── atipicial_protocol/
│   ├── atipicial_sgx/
│   ├── atipicial_types/
│   ├── atipicial_utils/
│   ├── atipicial_wallets/
│   ├── atipicial_x/
│   ├── constants/
│   ├── lib.rs
│   ├── monitoring/
│   ├── prelude.rs
│   └── sdk/
├── test_resources/
│   ├── responses/
│   └── wallet/
├── tests/
│   ├── README.md
│   ├── common/
│   ├── gas_estimator_integration_tests.rs
│   ├── integration_wallet_security.rs
│   ├── ledger_tests.rs
│   └── sdk_integration_tests.rs
└── website/
│   ├── .gitignore
│   ├── CONTRIBUTING.md
│   ├── README.md
│   ├── TROUBLESHOOTING.md
│   ├── __mocks__/
│   ├── blog/
│   ├── cli/
│   ├── coverage/
│   ├── docs/
│   ├── docusaurus.config.js
│   ├── netlify.toml
│   ├── package-lock.json
│   ├── package.json
│   ├── public/
│   ├── scripts/
│   ├── sdk/
│   ├── sidebars-cli.js
│   ├── sidebars-sdk.js
│   ├── sidebars.js
│   ├── src/
│   ├── static/
│   └── tsconfig.json


```
---

## 🗂️ Complete Source Registry — 419 files · 97,804 lines

Every source file in this repository, inventoried. No file hidden,
no module forgotten. This is the real map of ATIPICIAL RUST SDK.


### `(root)/` — 1 files · 9 lines

| File | Lang | Lines | Lead doc |
|---|---|---|---|
| `build.rs` | Rust | 9 | — |

### `atipicial-cli/` — 43 files · 14,517 lines

| File | Lang | Lines | Lead doc |
|---|---|---|---|
| `atipicial-cli/src/commands/atipicialfs.rs` | Rust | 779 | ![allow(dead_code)] |
| `atipicial-cli/src/commands/blockchain.rs` | Rust | 681 | ![allow(dead_code)] |
| `atipicial-cli/src/commands/contract.rs` | Rust | 905 | [derive(Args, Debug)] |
| `atipicial-cli/src/commands/defi/mod.rs` | Rust | 153 | DeFi operations on Atipicial blockchain |
| `atipicial-cli/src/commands/defi/tokens.rs` | Rust | 639 | ![allow(dead_code)] |
| `atipicial-cli/src/commands/defi/types.rs` | Rust | 213 | Arguments for token operations |
| `atipicial-cli/src/commands/defi/utils.rs` | Rust | 356 | ![allow(dead_code)] |
| `atipicial-cli/src/commands/fs.rs` | Rust | 1163 | ![allow(dead_code)] |
| `atipicial-cli/src/commands/mod.rs` | Rust | 9 | — |
| `atipicial-cli/src/commands/network.rs` | Rust | 706 | ![allow(dead_code)] |
| `atipicial-cli/src/commands/nft.rs` | Rust | 576 | [derive(Args, Debug)] |
| `atipicial-cli/src/commands/tools.rs` | Rust | 751 | digit = (value % 58) as u8; |
| `atipicial-cli/src/commands/wallet.rs` | Rust | 1750 | ![allow(dead_code)] |
| `atipicial-cli/src/config/mod.rs` | Rust | 92 | ![allow(dead_code)] |
| `atipicial-cli/src/errors.rs` | Rust | 148 | Unified error type for the Atipicial CLI. |
| `atipicial-cli/src/generator.rs` | Rust | 283 | Code generation module for Atipicial dApp templates |
| `atipicial-cli/src/main.rs` | Rust | 530 | ![allow(clippy::redundant_closure)] |
| `atipicial-cli/src/monitoring/logger.rs` | Rust | 370 | ![allow(dead_code, unused_imports)] |
| `atipicial-cli/src/monitoring/metrics.rs` | Rust | 399 | ![allow(dead_code, unused_imports)] |
| `atipicial-cli/src/monitoring/mod.rs` | Rust | 57 | ![allow(dead_code, unused_imports)] |
| `atipicial-cli/src/security/error_handler.rs` | Rust | 410 | ![allow(unexpected_cfgs, dead_code)] |
| `atipicial-cli/src/security/keychain.rs` | Rust | 442 | ![allow(dead_code, unused_imports, unused_variables)] |
| `atipicial-cli/src/security/mod.rs` | Rust | 48 | ![allow(unused_imports, dead_code)] |
| `atipicial-cli/src/security/network_failover.rs` | Rust | 405 | ![allow(dead_code)] |
| `atipicial-cli/src/security/session.rs` | Rust | 424 | ![allow(dead_code)] |
| `atipicial-cli/src/utils/atipicialfs.rs` | Rust | 137 | ![allow(dead_code)] |
| `atipicial-cli/src/utils/config.rs` | Rust | 121 | [derive(Debug, Serialize, Deserialize, Clone)] |
| `atipicial-cli/src/utils/error.rs` | Rust | 58 | ![allow(dead_code)] |
| `atipicial-cli/src/utils/extensions.rs` | Rust | 31 | Extension trait for Transaction to get human-readable transaction type name |
| `atipicial-cli/src/utils/mod.rs` | Rust | 50 | ![allow(dead_code)] |
| `atipicial-cli/src/utils_core.rs` | Rust | 356 | Print an informational message with icon |
| `atipicial-cli/src/wizard.rs` | Rust | 435 | Interactive wizard for Atipicial CLI |
| `atipicial-cli/tests/integration/blockchain_tests.rs` | Rust | 57 | [test] |
| `atipicial-cli/tests/integration/contract_tests.rs` | Rust | 136 | [cfg(test)] |
| `atipicial-cli/tests/integration/defi_tests.rs` | Rust | 298 | [cfg(test)] |
| `atipicial-cli/tests/integration/fs_tests.rs` | Rust | 212 | Test the AtipicialFs endpoints list command |
| `atipicial-cli/tests/integration/generate_tests.rs` | Rust | 16 | [cfg(test)] |
| `atipicial-cli/tests/integration/init_tests.rs` | Rust | 42 | [cfg(test)] |
| `atipicial-cli/tests/integration/mod.rs` | Rust | 9 | — |
| `atipicial-cli/tests/integration/network_tests.rs` | Rust | 93 | [test] |
| `atipicial-cli/tests/integration/utils.rs` | Rust | 72 | Helper function to assert that command was successful |
| `atipicial-cli/tests/integration/wallet_tests.rs` | Rust | 101 | [test] |
| `atipicial-cli/tests/integration_tests.rs` | Rust | 4 | — |

### `benches/` — 4 files · 459 lines

| File | Lang | Lines | Lead doc |
|---|---|---|---|
| `benches/crypto_benchmarks.rs` | Rust | 92 | — |
| `benches/gas_estimator_benchmarks.rs` | Rust | 146 | — |
| `benches/script_builder_benchmarks.rs` | Rust | 75 | — |
| `benches/wallet_benchmarks.rs` | Rust | 146 | — |

### `examples/` — 105 files · 15,746 lines

| File | Lang | Lines | Lead doc |
|---|---|---|---|
| `examples/advanced/cross_chain_bridge.rs` | Rust | 11 | Cross-Chain Bridge Example |
| `examples/advanced/defi_swap.rs` | Rust | 11 | DeFi Swap Example |
| `examples/advanced/nft_marketplace.rs` | Rust | 11 | NFT Marketplace Example |
| `examples/advanced/oracle_integration.rs` | Rust | 11 | Oracle Integration Example |
| `examples/atipicial_aep17_tokens/examples/aep17_token_operations.rs` | Rust | 294 | This example demonstrates comprehensive AEP-17 token operations on the Atipicial blockchai |
| `examples/atipicial_contracts/examples/deploy_atipicial_contract.rs` | Rust | 112 | Demonstrates how to build a deploy transaction using a real AEF + manifest, |
| `examples/atipicial_crypto/examples/aep2_example.rs` | Rust | 264 | Production-ready comprehensive AEP-2 encryption concept demonstration |
| `examples/atipicial_crypto/examples/key_pair.rs` | Rust | 195 | Production-ready comprehensive example demonstrating Atipicial key pair operations |
| `examples/atipicial_crypto/src/lib.rs` | Rust | 2 | — |
| `examples/atipicial_famous_contracts/examples/atipicialburger_atipicial.rs` | Rust | 7 | [tokio::main] |
| `examples/atipicial_famous_contracts/examples/atipicialcompound.rs` | Rust | 7 | [tokio::main] |
| `examples/atipicial_famous_contracts/examples/flamingo_finance.rs` | Rust | 173 | This example demonstrates interaction with Flamingo Finance, one of Atipicial's leading De |
| `examples/atipicial_famous_contracts/examples/grandshare.rs` | Rust | 5 | [tokio::main] |
| `examples/atipicial_famous_contracts/examples/query_atipicial.rs` | Rust | 7 | [tokio::main] |
| `examples/atipicial_famous_contracts/examples/query_gas.rs` | Rust | 195 | This example demonstrates how to query GAS token information and balances on the Atipicial |
| `examples/atipicial_fs/basic_usage.rs` | Rust | 95 | Minimal, runnable AtipicialFs example that builds real request payloads and probes the pub |
| `examples/atipicial_fs/mod.rs` | Rust | 3 | — |
| `examples/atipicial_fs/multipart_upload.rs` | Rust | 92 | Demonstrates building a multipart upload plan for AtipicialFs and simulating part assembly |
| `examples/atipicial_nns/examples/nns_operations.rs` | Rust | 174 | This example demonstrates how to work with the Atipicial Name Service (NNS) on the Atipici |
| `examples/atipicial_nodes/examples/block_explorer.rs` | Rust | 68 | This example demonstrates how to build a simple block explorer to view block and transacti |
| `examples/atipicial_nodes/examples/connect_to_node.rs` | Rust | 268 | This example demonstrates comprehensive Atipicial node connectivity and blockchain queryin |
| `examples/atipicial_nodes/examples/modern_node_interaction.rs` | Rust | 223 | Modern Atipicial Node Interaction Example |
| `examples/atipicial_smart_contracts/examples/interact_with_contract.rs` | Rust | 265 | This example demonstrates comprehensive smart contract interaction on the Atipicial blockc |
| `examples/atipicial_transactions/examples/create_and_send_transaction.rs` | Rust | 209 | This example demonstrates comprehensive transaction creation, signing, and sending on the  |
| `examples/atipicial_wallets/examples/wallet_management.rs` | Rust | 171 | This example demonstrates real wallet management functionality in Atipicial. |
| `examples/atipicial_wallets/examples/wallet_security.rs` | Rust | 146 | This example demonstrates comprehensive wallet security features in Atipicial. |
| `examples/atipicial_x/examples/atipicial_x_bridge.rs` | Rust | 366 | Example demonstrating Atipicial X Bridge contract interactions. |
| `examples/atipicial_x/examples/atipicial_x_evm.rs` | Rust | 366 | Example demonstrating Atipicial X EVM compatibility layer with real interactions. |
| `examples/atipicial_x/examples/unified_ecosystem.rs` | Rust | 36 | [tokio::main] |
| `examples/atipicial_x/src/main.rs` | Rust | 69 | This example demonstrates Atipicial X integration with AtipicialRust SDK. |
| `examples/basic/01_connect_to_network.rs` | Rust | 141 | Basic Network Connection Example |
| `examples/basic/02_create_wallet.rs` | Rust | 145 | Basic Wallet Creation Example |
| `examples/basic/03_check_balance.rs` | Rust | 203 | Basic Balance Checking Example |
| `examples/basic/04_send_transaction.rs` | Rust | 234 | Basic Transaction Sending Example |
| `examples/big-numbers/examples/comparison_equivalence.rs` | Rust | 215 | Atipicial Big Number Comparison and Equivalence Example |
| `examples/big-numbers/examples/conversion.rs` | Rust | 273 | Atipicial Big Number Conversion Example |
| `examples/big-numbers/examples/create_instances.rs` | Rust | 217 | Atipicial Big Numbers Example |
| `examples/big-numbers/examples/math_operations.rs` | Rust | 44 | `U256` implements traits in `std::ops`, that means it supports arithmetic operations |
| `examples/contract_interaction/src/main.rs` | Rust | 133 | Atipicial Smart Contract Interaction Example |
| `examples/contracts/examples/abigen.rs` | Rust | 95 | This example demonstrates the concept of working with smart contract ABIs in Atipicial. |
| `examples/contracts/examples/atipicial_contract_interaction.rs` | Rust | 456 | Atipicial Smart Contract Interaction Example |
| `examples/contracts/examples/compile.rs` | Rust | 29 | Atipicial Contract Compilation Example |
| `examples/contracts/examples/contracts/contract.sol` | Solidity | 23 | — |
| `examples/contracts/examples/deploy_anvil.rs` | Rust | 74 | Atipicial Local Development Example |
| `examples/contracts/examples/deploy_from_abi_and_bytecode.rs` | Rust | 99 | Atipicial Contract Deployment from AEF and Manifest |
| `examples/contracts/examples/deploy_from_solidity.rs` | Rust | 107 | Atipicial Smart Contract Development Example |
| `examples/contracts/examples/deploy_moonbeam.rs` | Rust | 99 | Atipicial Cross-Chain Development Example |
| `examples/contracts/examples/events.rs` | Rust | 114 | Atipicial Event Streaming Example |
| `examples/contracts/examples/events_with_meta.rs` | Rust | 90 | Atipicial Contract Events and Notifications Example |
| `examples/contracts/examples/instances.rs` | Rust | 2 | [tokio::main] |
| `examples/contracts/examples/methods.rs` | Rust | 262 | Atipicial Contract Methods Example |
| `examples/contracts/examples/methods_abi_and_structs.rs` | Rust | 396 | Atipicial Contract Methods, ABI and Structured Data Example |
| `examples/events/examples/filtering.rs` | Rust | 162 | Atipicial Event Filtering Example |
| `examples/events/examples/placeholder.rs` | Rust | 287 | Atipicial Event Monitoring Example |
| `examples/events/examples/subscribe.rs` | Rust | 202 | Atipicial Event Subscription Example |
| `examples/intermediate/aep17_operations.rs` | Rust | 11 | AEP-17 Token Operations Example |
| `examples/intermediate/deploy_contract.rs` | Rust | 11 | Contract Deployment Example |
| `examples/intermediate/invoke_contract.rs` | Rust | 11 | Contract Invocation Example |
| `examples/intermediate/multi_sig_wallet.rs` | Rust | 11 | Multi-Signature Wallet Example |
| `examples/message_signing/src/main.rs` | Rust | 99 | This example demonstrates message signing in Atipicial. |
| `examples/middleware/examples/builder.rs` | Rust | 201 | Atipicial Middleware Builder Example |
| `examples/middleware/examples/create_custom_middleware.rs` | Rust | 321 | This example demonstrates how to create custom middleware for Atipicial transactions. |
| `examples/middleware/examples/gas_escalator.rs` | Rust | 286 | In Atipicial, gas consumption is more predictable than Ethereum, but network congestion |
| `examples/middleware/examples/gas_oracle.rs` | Rust | 159 | Atipicial GAS Fee Estimation Example |
| `examples/middleware/examples/nonce_manager.rs` | Rust | 28 | Atipicial Transaction Management Example |
| `examples/middleware/examples/policy.rs` | Rust | 317 | Policy middleware for Atipicial provides a way to inject custom logic into transaction pro |
| `examples/middleware/examples/signer.rs` | Rust | 26 | Atipicial Signer Middleware Example |
| `examples/middleware/examples/timelag.rs` | Rust | 2 | [tokio::main] |
| `examples/middleware/examples/transformer.rs` | Rust | 2 | [tokio::main] |
| `examples/providers/examples/custom.rs` | Rust | 463 | Atipicial Custom Provider Example |
| `examples/providers/examples/http.rs` | Rust | 51 | The Http transport is used to send JSON-RPC requests over HTTP to an Atipicial node. |
| `examples/providers/examples/http_jwt.rs` | Rust | 26 | HTTP JWT authentication example for Atipicial blockchain |
| `examples/providers/examples/ipc.rs` | Rust | 58 | Atipicial IPC (Inter-Process Communication) Example |
| `examples/providers/examples/mock.rs` | Rust | 335 | Mock provider example for Atipicial blockchain |
| `examples/providers/examples/retry.rs` | Rust | 324 | Atipicial Retry Strategy Example |
| `examples/providers/examples/rw.rs` | Rust | 26 | Read/Write client example for Atipicial blockchain |
| `examples/queries/examples/paginated_logs.rs` | Rust | 171 | Atipicial Paginated Event Logs Example |
| `examples/queries/examples/uniswapv2_pair.rs` | Rust | 233 | Atipicial DeFi Pair Query Example |
| `examples/sgx_enclave/src/lib.rs` | Rust | 293 | ![no_std] |
| `examples/sgx_enclave/src/main.rs` | Rust | 193 | — |
| `examples/standalone/src/bin/gas_estimation.rs` | Rust | 225 | Gas Estimation Example |
| `examples/standalone/src/bin/high_level_sdk.rs` | Rust | 55 | Example demonstrating the new high-level SDK API |
| `examples/standalone/src/bin/production_ready_client.rs` | Rust | 258 | Example demonstrating production-ready Atipicial RPC client with all advanced features |
| `examples/standalone/src/bin/v1_0_features.rs` | Rust | 233 | Example demonstrating new features in AtipicialRust v1.0.6 |
| `examples/subscriptions/examples/subscribe_blocks.rs` | Rust | 249 | Atipicial Block Subscription Example |
| `examples/subscriptions/examples/subscribe_events_by_type.rs` | Rust | 322 | Atipicial Event Subscription by Type Example |
| `examples/subscriptions/examples/subscribe_logs.rs` | Rust | 272 | Professional Atipicial blockchain subscription example |
| `examples/transactions/examples/call_override.rs` | Rust | 91 | Atipicial Test Invoke Example |
| `examples/transactions/examples/contracts/erc20_example/@openzeppelin/contracts/token/ERC20/ERC20.sol` | Solidity | 383 | @dev Implementation of the {IERC20} interface. |
| `examples/transactions/examples/contracts/erc20_example/@openzeppelin/contracts/token/ERC20/IERC20.sol` | Solidity | 82 | @dev Interface of the ERC20 standard as defined in the EIP. |
| `examples/transactions/examples/contracts/erc20_example/@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol` | Solidity | 28 | @dev Interface for the optional metadata functions from the ERC20 standard. |
| `examples/transactions/examples/contracts/erc20_example/@openzeppelin/contracts/utils/Context.sol` | Solidity | 24 | @dev Provides information about the current execution context, including the |
| `examples/transactions/examples/contracts/erc20_example/ERC20Example.sol` | Solidity | 10 | — |
| `examples/transactions/examples/decode_input.rs` | Rust | 93 | Atipicial Transaction Decoding Example |
| `examples/transactions/examples/ens.rs` | Rust | 92 | Atipicial Name Service (NNS) Example |
| `examples/transactions/examples/remove_liquidity.rs` | Rust | 95 | Atipicial DeFi Liquidity Example |
| `examples/transactions/examples/trace_call.rs` | Rust | 78 | Atipicial Transaction Debugging Example |
| `examples/transactions/examples/trace_transaction.rs` | Rust | 47 | Fetch a transaction and its execution log from TestNet. |
| `examples/transactions/examples/transfer_erc20.rs` | Rust | 77 | Atipicial AEP-17 Token Transfer Example |
| `examples/transactions/examples/transfer_eth.rs` | Rust | 331 | Atipicial GAS Transfer Example |
| `examples/wallets/examples/ledger.rs` | Rust | 264 | Atipicial Ledger Hardware Wallet Integration Example |
| `examples/wallets/examples/local_signer.rs` | Rust | 167 | This example demonstrates how to use a local signer to sign messages in Atipicial. |
| `examples/wallets/examples/mnemonic.rs` | Rust | 85 | This example demonstrates how to manage wallets in the Atipicial blockchain. |
| `examples/wallets/examples/sign_message.rs` | Rust | 191 | Atipicial Message Signing Example |
| `examples/wallets/examples/yubi.rs` | Rust | 378 | Atipicial YubiHSM Hardware Security Module Integration Example |

### `src/` — 246 files · 63,724 lines

| File | Lang | Lines | Lead doc |
|---|---|---|---|
| `src/atipicial_builder/error.rs` | Rust | 57 | [derive(Debug, Error, PartialEq, Clone)] |
| `src/atipicial_builder/mod.rs` | Rust | 81 | # Atipicial Builder Module |
| `src/atipicial_builder/script/interop_service.rs` | Rust | 145 | [derive(EnumString, EnumIter, Display, Copy, Clone, PartialEq, Eq)] |
| `src/atipicial_builder/script/mod.rs` | Rust | 39 | ![allow(warnings)] |
| `src/atipicial_builder/script/script_builder.rs` | Rust | 989 | A builder for constructing Atipicial smart contract scripts. |
| `src/atipicial_builder/script/script_reader.rs` | Rust | 180 | A utility struct for reading and interpreting Atipicial smart contract scripts. |
| `src/atipicial_builder/transaction/call_flags.rs` | Rust | 40 | [derive(Debug, Hash, Eq, PartialEq, Clone, Copy)] |
| `src/atipicial_builder/transaction/contract_parameters_context.rs` | Rust | 47 | [derive(Serialize, Deserialize, Debug)] |
| `src/atipicial_builder/transaction/gas_estimator.rs` | Rust | 173 | Enhanced gas estimation utilities for Atipicial transactions |
| `src/atipicial_builder/transaction/invocation_script.rs` | Rust | 317 | An invocation script is part of a witness and is simply a sequence of atipicial-vm instruc |
| `src/atipicial_builder/transaction/mod.rs` | Rust | 53 | ![allow(clippy::module_inception)] |
| `src/atipicial_builder/transaction/oracle_response_code.rs` | Rust | 41 | [derive( |
| `src/atipicial_builder/transaction/production_transaction_builder.rs` | Rust | 382 | Production-ready Transaction Builder utilities |
| `src/atipicial_builder/transaction/proptest_tests.rs` | Rust | 169 | [cfg(test)] |
| `src/atipicial_builder/transaction/signers/account_signer.rs` | Rust | 313 | Represents an account signer in the ATC blockchain. |
| `src/atipicial_builder/transaction/signers/contract_signer.rs` | Rust | 279 | Represents a contract signer in the ATC blockchain. |
| `src/atipicial_builder/transaction/signers/mod.rs` | Rust | 74 | This module contains implementations for different types of signers in the ATC blockchain. |
| `src/atipicial_builder/transaction/signers/signer.rs` | Rust | 1364 | ![allow( |
| `src/atipicial_builder/transaction/signers/transaction_signer.rs` | Rust | 281 | Represents a transaction signer in the ATC blockchain. |
| `src/atipicial_builder/transaction/transaction.rs` | Rust | 1044 | A Atipicial blockchain transaction. |
| `src/atipicial_builder/transaction/transaction_attribute.rs` | Rust | 391 | [derive(Serialize, Deserialize, PartialEq, Hash, Debug, Clone)] |
| `src/atipicial_builder/transaction/transaction_builder.rs` | Rust | 1348 | A builder for constructing and configuring ATC blockchain transactions. |
| `src/atipicial_builder/transaction/transaction_builder_tests.rs` | Rust | 1827 | [cfg(test)] |
| `src/atipicial_builder/transaction/transaction_error.rs` | Rust | 53 | [derive(Error, Debug, PartialEq, Clone)] |
| `src/atipicial_builder/transaction/transaction_send_token.rs` | Rust | 23 | [derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Getters)] |
| `src/atipicial_builder/transaction/verification_script.rs` | Rust | 748 | [derive(Debug, Clone, PartialEq, Eq, Hash, Getters, Setters, Serialize, Deserialize)] |
| `src/atipicial_builder/transaction/witness.rs` | Rust | 151 | [derive(Hash, Serialize, Deserialize, PartialEq, Eq, Debug, Clone)] |
| `src/atipicial_builder/transaction/witness_rule/mod.rs` | Rust | 62 | This module contains implementations for witness rules in the ATC blockchain. |
| `src/atipicial_builder/transaction/witness_rule/witness_action.rs` | Rust | 24 | [derive( |
| `src/atipicial_builder/transaction/witness_rule/witness_condition.rs` | Rust | 519 | Enum representing the different types of witness conditions that can be used in a smart co |
| `src/atipicial_builder/transaction/witness_rule/witness_rule.rs` | Rust | 429 | [derive(Serialize, Deserialize, Debug, Hash, PartialEq, Clone)] |
| `src/atipicial_builder/transaction/witness_scope.rs` | Rust | 89 | [derive( |
| `src/atipicial_builder/utils.rs` | Rust | 206 | Converts a list of public keys to a script hash using a given threshold. |
| `src/atipicial_clients/api_trait.rs` | Rust | 384 | [async_trait] |
| `src/atipicial_clients/cache.rs` | Rust | 494 | Cache configuration |
| `src/atipicial_clients/circuit_breaker.rs` | Rust | 455 | Circuit breaker states |
| `src/atipicial_clients/connection_pool.rs` | Rust | 497 | ![allow(dead_code)] |
| `src/atipicial_clients/errors.rs` | Rust | 217 | [derive(Debug, Error)] |
| `src/atipicial_clients/ext/dev_rpc.rs` | Rust | 194 | A middleware supporting development-specific JSON RPC methods |
| `src/atipicial_clients/ext/mod.rs` | Rust | 2 | Types for the admin api |
| `src/atipicial_clients/ext/nns.rs` | Rust | 1 | — |
| `src/atipicial_clients/mock_blocks.rs` | Rust | 74 | ![allow(dead_code)] |
| `src/atipicial_clients/mock_client.rs` | Rust | 277 | — |
| `src/atipicial_clients/mod.rs` | Rust | 286 | ![allow(clippy::type_complexity)] |
| `src/atipicial_clients/production_client.rs` | Rust | 454 | Production-ready RPC client with connection pooling, caching, and circuit breaker |
| `src/atipicial_clients/rate_limiter.rs` | Rust | 301 | ![allow(dead_code)] |
| `src/atipicial_clients/rpc/connections.rs` | Rust | 25 | [cfg_attr(target_arch = "wasm32", async_trait(? Send))] |
| `src/atipicial_clients/rpc/mod.rs` | Rust | 69 | # Atipicial RPC Client Module |
| `src/atipicial_clients/rpc/pubsub.rs` | Rust | 121 | A transport implementation supporting pub sub subscriptions. |
| `src/atipicial_clients/rpc/rpc_client.rs` | Rust | 1424 | Node Clients |
| `src/atipicial_clients/rpc/transports/common.rs` | Rust | 658 | A JSON-RPC 2.0 error |
| `src/atipicial_clients/rpc/transports/http_provider.rs` | Rust | 567 | A low-level JSON-RPC Client over HTTP. |
| `src/atipicial_clients/rpc/transports/ipc.rs` | Rust | 648 | [derive(Debug)] |
| `src/atipicial_clients/rpc/transports/legacy_ws.rs` | Rust | 761 | [cfg(not(target_arch = "wasm32"))] |
| `src/atipicial_clients/rpc/transports/mock.rs` | Rust | 273 | [derive(Clone, Copy, Debug, PartialEq, Eq)] |
| `src/atipicial_clients/rpc/transports/mod.rs` | Rust | 31 | [cfg(all(feature = "ipc", any(unix, windows)))] |
| `src/atipicial_clients/rpc/transports/retry.rs` | Rust | 658 | A [JsonRpcProvider] implementation that retries requests filtered by [RetryPolicy] |
| `src/atipicial_clients/rpc/transports/rw.rs` | Rust | 126 | A [JsonRpcProvider] implementation that serves as a wrapper around two different [JsonRpcP |
| `src/atipicial_clients/rpc/transports/ws/backend.rs` | Rust | 283 | `BackendDriver` drives a specific `WsBackend`. It can be used to issue |
| `src/atipicial_clients/rpc/transports/ws/error.rs` | Rust | 83 | [derive(Debug, thiserror::Error)] |
| `src/atipicial_clients/rpc/transports/ws/manager.rs` | Rust | 702 | [cfg(not(target_arch = "wasm32"))] |
| `src/atipicial_clients/rpc/transports/ws/mod.rs` | Rust | 215 | ![allow(missing_docs)] |
| `src/atipicial_clients/rpc/transports/ws/types.rs` | Rust | 308 | [cfg(not(target_arch = "wasm32"))] |
| `src/atipicial_clients/rx/atipicial_rust_rx_trait.rs` | Rust | 108 | ![allow(dead_code)] |
| `src/atipicial_clients/rx/mod.rs` | Rust | 1 | — |
| `src/atipicial_clients/utils.rs` | Rust | 226 | A simple gas escalation policy |
| `src/atipicial_codec/binary_decoder.rs` | Rust | 692 | This module provides a binary decoder that can read various types of data from a byte slic |
| `src/atipicial_codec/binary_encoder.rs` | Rust | 314 | A binary encoder that can write various primitive types and serializable objects to a byte |
| `src/atipicial_codec/encode.rs` | Rust | 125 | — |
| `src/atipicial_codec/error.rs` | Rust | 45 | [derive(Error, Debug, PartialEq, Eq, Clone)] |
| `src/atipicial_codec/mod.rs` | Rust | 33 | Binary encoding and decoding for the Atipicial wire format. |
| `src/atipicial_config/config.rs` | Rust | 363 | [derive(Clone, Debug, PartialEq, Eq, Hash, Deserialize, Serialize)] |
| `src/atipicial_config/constant.rs` | Rust | 143 | [cfg(not(all(feature = "sgx", target_env = "sgx")))] |
| `src/atipicial_config/mod.rs` | Rust | 28 | Network and runtime configuration for the Atipicial SDK. |
| `src/atipicial_config/test_properties.rs` | Rust | 45 | — |
| `src/atipicial_contract/atipicial_token.rs` | Rust | 409 | [derive(Debug, Clone, Serialize, Deserialize)] |
| `src/atipicial_contract/atipicial_uri.rs` | Rust | 247 | [derive(Debug, Clone, Serialize, Deserialize, Getters, Setters)] |
| `src/atipicial_contract/contract_error.rs` | Rust | 56 | Custom error type for contract-related errors |
| `src/atipicial_contract/contract_management.rs` | Rust | 163 | A struct representing contract management functionalities |
| `src/atipicial_contract/famous/atipicialburger.rs` | Rust | 231 | AtipicialburgerAtipicial contract interface for Atipicial |
| `src/atipicial_contract/famous/atipicialcompound.rs` | Rust | 189 | AtipicialCompound contract interface for Atipicial |
| `src/atipicial_contract/famous/contracts.rs` | Rust | 237 | Famous contract addresses and information for Atipicial mainnet and testnet. |
| `src/atipicial_contract/famous/flamingo.rs` | Rust | 252 | Flamingo Finance contract interface for Atipicial |
| `src/atipicial_contract/famous/grandshare.rs` | Rust | 210 | GrandShare contract interface for Atipicial |
| `src/atipicial_contract/famous/mod.rs` | Rust | 13 | Module containing implementations of famous Atipicial smart contracts |
| `src/atipicial_contract/fungible_token_contract.rs` | Rust | 86 | [derive(Debug)] |
| `src/atipicial_contract/gas_token.rs` | Rust | 99 | [derive(Debug, Clone, Serialize, Deserialize)] |
| `src/atipicial_contract/iterator.rs` | Rust | 68 | — |
| `src/atipicial_contract/mod.rs` | Rust | 181 | # Atipicial Contract Module |
| `src/atipicial_contract/name_service.rs` | Rust | 376 | [derive(Debug, Clone, Copy, PartialEq, Eq)] |
| `src/atipicial_contract/nft_contract.rs` | Rust | 87 | [derive(Debug)] |
| `src/atipicial_contract/notary.rs` | Rust | 277 | Notary native contract for Atipicial. |
| `src/atipicial_contract/policy_contract.rs` | Rust | 657 | Policy native contract for Atipicial. |
| `src/atipicial_contract/role_management.rs` | Rust | 156 | [derive(Debug, Clone, Serialize, Deserialize)] |
| `src/atipicial_contract/tests.rs` | Rust | 359 | [cfg(test)] |
| `src/atipicial_contract/traits/fungible_token.rs` | Rust | 122 | [async_trait] |
| `src/atipicial_contract/traits/mod.rs` | Rust | 9 | — |
| `src/atipicial_contract/traits/nft.rs` | Rust | 393 | [async_trait] |
| `src/atipicial_contract/traits/smart_contract.rs` | Rust | 486 | [async_trait] |
| `src/atipicial_contract/traits/token.rs` | Rust | 100 | [async_trait] |
| `src/atipicial_contract/treasury.rs` | Rust | 112 | Treasury native contract for Atipicial. |
| `src/atipicial_crypto/base58_helper.rs` | Rust | 201 | [derive(Debug, Error, Clone, PartialEq, Eq)] |
| `src/atipicial_crypto/crypto_lib.rs` | Rust | 412 | CryptoLib functions for Atipicial. |
| `src/atipicial_crypto/error.rs` | Rust | 60 | [derive(Debug, Error, PartialEq, Clone)] |
| `src/atipicial_crypto/hash.rs` | Rust | 226 | [cfg(test)] |
| `src/atipicial_crypto/key_pair.rs` | Rust | 305 | # KeyPair |
| `src/atipicial_crypto/keys.rs` | Rust | 832 | # Secp256r1 Cryptographic Module |
| `src/atipicial_crypto/mod.rs` | Rust | 89 | # Atipicial Crypto |
| `src/atipicial_crypto/proptest_tests.rs` | Rust | 109 | [cfg(test)] |
| `src/atipicial_crypto/utils.rs` | Rust | 152 | Convert a private key to a public key. |
| `src/atipicial_crypto/wif.rs` | Rust | 198 | Converts a WIF (Wallet Import Format) string into a `Secp256r1PrivateKey`. |
| `src/atipicial_error/mod.rs` | Rust | 344 | ![deny(missing_docs)] |
| `src/atipicial_error/unified.rs` | Rust | 1386 | ![deny(missing_docs)] |
| `src/atipicial_fs/acl.rs` | Rust | 215 | # AtipicialFs Access Control |
| `src/atipicial_fs/client.rs` | Rust | 577 | Client for interacting with AtipicialFs. |
| `src/atipicial_fs/container.rs` | Rust | 190 | # AtipicialFs Container |
| `src/atipicial_fs/error.rs` | Rust | 86 | # AtipicialFs Error Handling |
| `src/atipicial_fs/mod.rs` | Rust | 284 | # Atipicial File Storage (AtipicialFs) |
| `src/atipicial_fs/object.rs` | Rust | 141 | # AtipicialFs Object Operations |
| `src/atipicial_fs/types.rs` | Rust | 176 | # AtipicialFs Core Types |
| `src/atipicial_protocol/account.rs` | Rust | 855 | # Atipicial Account Module |
| `src/atipicial_protocol/aep2.rs` | Rust | 977 | # ATC AEP2 (Atipicial Extended Protocol 2) Module |
| `src/atipicial_protocol/mod.rs` | Rust | 60 | # Atipicial Protocol |
| `src/atipicial_protocol/protocol_error.rs` | Rust | 18 | [derive(Error, Debug)] |
| `src/atipicial_protocol/responses/atipicial_account_state.rs` | Rust | 38 | [derive(Serialize, Deserialize, Clone, PartialEq, Debug)] |
| `src/atipicial_protocol/responses/atipicial_address.rs` | Rust | 12 | [derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)] |
| `src/atipicial_protocol/responses/atipicial_application_log.rs` | Rust | 110 | [derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)] |
| `src/atipicial_protocol/responses/atipicial_balances.rs` | Rust | 87 | [derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)] |
| `src/atipicial_protocol/responses/atipicial_block.rs` | Rust | 70 | [derive(Serialize, Deserialize, Clone, Debug)] |
| `src/atipicial_protocol/responses/atipicial_find_states.rs` | Rust | 18 | [derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)] |
| `src/atipicial_protocol/responses/atipicial_get_claimable.rs` | Rust | 30 | [derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)] |
| `src/atipicial_protocol/responses/atipicial_get_mem_pool.rs` | Rust | 15 | [derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)] |
| `src/atipicial_protocol/responses/atipicial_get_next_block_validators.rs` | Rust | 15 | [derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)] |
| `src/atipicial_protocol/responses/atipicial_get_peers.rs` | Rust | 20 | [derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)] |
| `src/atipicial_protocol/responses/atipicial_get_state_height.rs` | Rust | 9 | [derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)] |
| `src/atipicial_protocol/responses/atipicial_get_state_root.rs` | Rust | 18 | [derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)] |
| `src/atipicial_protocol/responses/atipicial_get_token_balances.rs` | Rust | 14 | — |
| `src/atipicial_protocol/responses/atipicial_get_token_transfers.rs` | Rust | 22 | — |
| `src/atipicial_protocol/responses/atipicial_get_unclaimed_gas.rs` | Rust | 29 | [derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, Default)] |
| `src/atipicial_protocol/responses/atipicial_get_unspents.rs` | Rust | 65 | [derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)] |
| `src/atipicial_protocol/responses/atipicial_get_version.rs` | Rust | 166 | [derive(Serialize, Deserialize, Debug, Clone)] |
| `src/atipicial_protocol/responses/atipicial_get_wallet_balance.rs` | Rust | 7 | [derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)] |
| `src/atipicial_protocol/responses/atipicial_list_plugins.rs` | Rust | 8 | [derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)] |
| `src/atipicial_protocol/responses/atipicial_network_fee.rs` | Rust | 7 | [derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)] |
| `src/atipicial_protocol/responses/atipicial_send_raw_transaction.rs` | Rust | 16 | [derive(Debug, Hash, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone)] |
| `src/atipicial_protocol/responses/atipicial_submit_block.rs` | Rust | 9 | [derive(Serialize, Deserialize, Debug, Clone, Hash)] |
| `src/atipicial_protocol/responses/atipicial_transaction_result.rs` | Rust | 51 | [derive(Serialize, Deserialize, Hash, Clone, Debug)] |
| `src/atipicial_protocol/responses/atipicial_transfers.rs` | Rust | 133 | [derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug)] |
| `src/atipicial_protocol/responses/atipicial_validate_address.rs` | Rust | 14 | [derive(Serialize, Deserialize, Debug, Clone)] |
| `src/atipicial_protocol/responses/atipicial_witness.rs` | Rust | 21 | [derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)] |
| `src/atipicial_protocol/responses/diagnostics.rs` | Rust | 32 | [derive(Serialize, Deserialize, Hash)] |
| `src/atipicial_protocol/responses/express_contract_state.rs` | Rust | 20 | [derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash)] |
| `src/atipicial_protocol/responses/express_shutdown.rs` | Rust | 13 | [derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)] |
| `src/atipicial_protocol/responses/mod.rs` | Rust | 72 | — |
| `src/atipicial_protocol/responses/notification.rs` | Rust | 20 | [derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)] |
| `src/atipicial_protocol/responses/oracle_request.rs` | Rust | 59 | [derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone)] |
| `src/atipicial_protocol/responses/populated_blocks.rs` | Rust | 13 | [derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone)] |
| `src/atipicial_protocol/responses/response_transaction.rs` | Rust | 174 | [derive(Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters, Debug, Clone)] |
| `src/atipicial_protocol/responses/response_transaction_attribute.rs` | Rust | 72 | [derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)] |
| `src/atipicial_protocol/responses/response_transaction_signer.rs` | Rust | 256 | ![allow(missing_docs)] |
| `src/atipicial_protocol/role.rs` | Rust | 21 | ![allow(dead_code)] |
| `src/atipicial_sgx/allocator.rs` | Rust | 130 | ![cfg_attr(feature = "sgx", no_std)] |
| `src/atipicial_sgx/attestation.rs` | Rust | 280 | ![cfg_attr(feature = "sgx", no_std)] |
| `src/atipicial_sgx/crypto.rs` | Rust | 422 | ![cfg_attr(feature = "sgx", no_std)] |
| `src/atipicial_sgx/enclave.rs` | Rust | 367 | ![cfg_attr(feature = "sgx", no_std)] |
| `src/atipicial_sgx/mod.rs` | Rust | 184 | # Atipicial SGX Support |
| `src/atipicial_sgx/networking.rs` | Rust | 311 | ![cfg_attr(feature = "sgx", no_std)] |
| `src/atipicial_sgx/storage.rs` | Rust | 230 | ![cfg_attr(feature = "sgx", no_std)] |
| `src/atipicial_types/address.rs` | Rust | 172 | ![allow(clippy::items_after_test_module)] |
| `src/atipicial_types/address_or_scripthash.rs` | Rust | 227 | [derive(Clone, Debug, PartialEq, Eq, Serialize)] |
| `src/atipicial_types/block.rs` | Rust | 90 | Lightweight transaction hash provider used for block equality checks. |
| `src/atipicial_types/bytes.rs` | Rust | 113 | ![allow(dead_code)] |
| `src/atipicial_types/contract/aef_file.rs` | Rust | 667 | / |
| `src/atipicial_types/contract/aep17contract.rs` | Rust | 19 | [derive(Serialize, Deserialize, PartialEq, Eq, Hash, Debug, Clone)] |
| `src/atipicial_types/contract/contract_aef.rs` | Rust | 52 | [derive(Serialize, Deserialize, Default, Hash, Clone, Debug, PartialEq)] |
| `src/atipicial_types/contract/contract_manifest.rs` | Rust | 551 | Contract Manifest types for Atipicial smart contracts. |
| `src/atipicial_types/contract/contract_method_token.rs` | Rust | 30 | [derive(Serialize, Deserialize, Debug, Hash, Clone, PartialEq)] |
| `src/atipicial_types/contract/contract_parameter.rs` | Rust | 1326 | [derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)] |
| `src/atipicial_types/contract/contract_parameter_type.rs` | Rust | 92 | [derive( |
| `src/atipicial_types/contract/contract_state.rs` | Rust | 78 | , |
| `src/atipicial_types/contract/contract_storage_entry.rs` | Rust | 13 | [derive(Serialize, Deserialize, Hash, Clone)] |
| `src/atipicial_types/contract/invocation_result.rs` | Rust | 319 | [derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)] |
| `src/atipicial_types/contract/mod.rs` | Rust | 23 | — |
| `src/atipicial_types/contract/native_contract_state.rs` | Rust | 32 | [derive(Serialize, Deserialize, Getters, Setters, Default, Debug, Clone)] |
| `src/atipicial_types/error.rs` | Rust | 58 | [derive(Error, Debug, PartialEq, Eq, Hash, Clone)] |
| `src/atipicial_types/hardfork.rs` | Rust | 174 | Hardfork definitions for Atipicial blockchain. |
| `src/atipicial_types/mod.rs` | Rust | 353 | ![allow(clippy::items_after_test_module)] |
| `src/atipicial_types/nns/mod.rs` | Rust | 8 | — |
| `src/atipicial_types/nns/name_state.rs` | Rust | 16 | ![allow(dead_code)] |
| `src/atipicial_types/nns/nns_name.rs` | Rust | 103 | [derive(Debug, Clone, Display, PartialEq, Eq, Serialize, Deserialize, Getters, Setters)] |
| `src/atipicial_types/nns/record_state.rs` | Rust | 38 | [derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)] |
| `src/atipicial_types/nns/record_type.rs` | Rust | 39 | [derive( |
| `src/atipicial_types/numeric.rs` | Rust | 157 | ![allow(dead_code)] |
| `src/atipicial_types/op_code.rs` | Rust | 620 | [derive( |
| `src/atipicial_types/path_or_string.rs` | Rust | 37 | A type that can either be a `Path` or a `String` |
| `src/atipicial_types/plugin_type.rs` | Rust | 39 | [derive(Display, EnumString, Debug, Copy, Clone, PartialEq, Eq)] |
| `src/atipicial_types/proptest_tests.rs` | Rust | 149 | [cfg(test)] |
| `src/atipicial_types/script_hash.rs` | Rust | 517 | Trait that provides additional methods for types related to `ScriptHash`. |
| `src/atipicial_types/serde_value.rs` | Rust | 74 | — |
| `src/atipicial_types/serde_with_utils.rs` | Rust | 1622 | Serde serialization/deserialization helpers for Atipicial types. |
| `src/atipicial_types/stack_item.rs` | Rust | 620 | This module defines the `StackItem` enum and `MapEntry` struct, which are used to represen |
| `src/atipicial_types/string.rs` | Rust | 217 | [cfg(test)] |
| `src/atipicial_types/syncing.rs` | Rust | 211 | Types for `atipicial_syncing` RPC call |
| `src/atipicial_types/tx_pool.rs` | Rust | 145 | Transaction summary as found in the Txpool Inspection property. |
| `src/atipicial_types/url_session.rs` | Rust | 21 | — |
| `src/atipicial_types/util.rs` | Rust | 449 | Parses a string into a `u64`, supporting both decimal and hexadecimal (prefixed with "0x") |
| `src/atipicial_types/vm_state.rs` | Rust | 36 | Represents the state of a virtual machine. |
| `src/atipicial_types/whitelisted_contract.rs` | Rust | 112 | Whitelisted contract type for fee exemptions. |
| `src/atipicial_utils/error.rs` | Rust | 172 | Error handling utilities for the Atipicial SDK. |
| `src/atipicial_utils/mod.rs` | Rust | 34 | # Atipicial Utilities |
| `src/atipicial_wallets/bip39_account.rs` | Rust | 231 | A BIP-39 compatible atipicial account that uses mnemonic phrases for key generation and re |
| `src/atipicial_wallets/error.rs` | Rust | 93 | Represents errors that can occur within the signing process. |
| `src/atipicial_wallets/ledger.rs` | Rust | 277 | Atipicial APDU commands for Ledger devices. |
| `src/atipicial_wallets/mod.rs` | Rust | 104 | ![deny(unsafe_code)] |
| `src/atipicial_wallets/wallet/aep6account.rs` | Rust | 612 | Represents an account in the AEP-6 format. |
| `src/atipicial_wallets/wallet/aep6contract.rs` | Rust | 56 | Represents a AEP-6 contract. |
| `src/atipicial_wallets/wallet/aep6wallet.rs` | Rust | 134 | ![allow(missing_docs)] |
| `src/atipicial_wallets/wallet/backup.rs` | Rust | 140 | [cfg(not(unix))] |
| `src/atipicial_wallets/wallet/mod.rs` | Rust | 13 | — |
| `src/atipicial_wallets/wallet/wallet.rs` | Rust | 1618 | ![allow(missing_docs)] |
| `src/atipicial_wallets/wallet/wallet_error.rs` | Rust | 161 | Errors that may occur within wallet operations. |
| `src/atipicial_wallets/wallet_signer.rs` | Rust | 142 | A Atipicial private-public key pair which can be used for signing messages. |
| `src/atipicial_wallets/wallet_trait.rs` | Rust | 107 | Represents the core functionalities of a cryptocurrency wallet. |
| `src/atipicial_wallets/yubi.rs` | Rust | 237 | Helpers for creating wallets for YubiHSM2 |
| `src/atipicial_x/bridge/bridge_contract.rs` | Rust | 219 | Atipicial X Bridge contract interface for token transfers between Atipicial and Atipicial  |
| `src/atipicial_x/bridge/evm_bridge.rs` | Rust | 62 | [sol(rpc)] |
| `src/atipicial_x/bridge/mod.rs` | Rust | 7 | Atipicial X bridge helpers |
| `src/atipicial_x/evm/mod.rs` | Rust | 9 | Atipicial X EVM compatibility layer |
| `src/atipicial_x/evm/provider.rs` | Rust | 185 | Atipicial X MainNet Anti-MEV RPC endpoint. |
| `src/atipicial_x/evm/transaction.rs` | Rust | 132 | Atipicial X EVM transaction for interacting with the Atipicial X EVM-compatible chain |
| `src/atipicial_x/evm/wallet.rs` | Rust | 102 | Atipicial X EVM Wallet for managing accounts and signing transactions on Atipicial X. |
| `src/atipicial_x/mod.rs` | Rust | 103 | # Atipicial X |
| `src/constants/mod.rs` | Rust | 12 | Constants used throughout the Atipicial SDK. |
| `src/constants/native_contracts.rs` | Rust | 78 | Definitions of Native Contract script hashes for Atipicial. |
| `src/lib.rs` | Rust | 700 | ![allow(clippy::result_large_err)] |
| `src/monitoring/health.rs` | Rust | 233 | # Health Checks |
| `src/monitoring/metrics.rs` | Rust | 142 | # Metrics |
| `src/monitoring/mod.rs` | Rust | 164 | # Monitoring |
| `src/monitoring/tracing.rs` | Rust | 91 | # Distributed Tracing |
| `src/prelude.rs` | Rust | 135 | # AtipicialRust prelude |
| `src/sdk/hd_wallet.rs` | Rust | 1135 | HD Wallet support with BIP-39/44 implementation |
| `src/sdk/mod.rs` | Rust | 1936 | ![deny(missing_docs)] |
| `src/sdk/retry.rs` | Rust | 204 | Lightweight retry helper used by the high-level [`crate::sdk::Atipicial`] API. |
| `src/sdk/transaction_simulator.rs` | Rust | 1103 | Transaction simulation for gas estimation and state preview |
| `src/sdk/unified.rs` | Rust | 294 | Build a [`AtipicialError::Validation`] for a malformed amount field. |
| `src/sdk/websocket.rs` | Rust | 1131 | WebSocket support for real-time blockchain updates |

### `tests/` — 6 files · 912 lines

| File | Lang | Lines | Lead doc |
|---|---|---|---|
| `tests/common/mod.rs` | Rust | 17 | Common Test Utilities |
| `tests/common/test_utils.rs` | Rust | 154 | Test Utilities |
| `tests/gas_estimator_integration_tests.rs` | Rust | 184 | [cfg(all(test, feature = "mock"))] |
| `tests/integration_wallet_security.rs` | Rust | 224 | [tokio::test] |
| `tests/ledger_tests.rs` | Rust | 19 | [cfg(feature = "ledger")] |
| `tests/sdk_integration_tests.rs` | Rust | 314 | Integration tests for the high-level SDK API |

### `website/` — 14 files · 2,437 lines

| File | Lang | Lines | Lead doc |
|---|---|---|---|
| `website/src/__tests__/BlockchainInfo.test.tsx` | TSX | 372 | — |
| `website/src/__tests__/Footer.test.tsx` | TSX | 99 | — |
| `website/src/__tests__/Header.test.tsx` | TSX | 122 | — |
| `website/src/__tests__/Search.test.tsx` | TSX | 127 | — |
| `website/src/__tests__/ThemeContext.test.tsx` | TSX | 69 | — |
| `website/src/__tests__/ThemeToggle.test.tsx` | TSX | 75 | — |
| `website/src/__tests__/test-utils.test.tsx` | TSX | 15 | — |
| `website/src/components/AnimatedBackground.tsx` | TSX | 165 | — |
| `website/src/components/ThemeToggle.tsx` | TSX | 66 | — |
| `website/src/context/ThemeContext.tsx` | TSX | 65 | — |
| `website/src/pages/examples.tsx` | TSX | 600 | [tokio::main] |
| `website/src/pages/index.tsx` | TSX | 558 | [tokio::main] |
| `website/src/templates/DocTemplate.tsx` | TSX | 103 | — |
| `website/src/types.d.ts` | TypeScript | 1 | — |

---

## 🧬 Public API Surface — Every Exported Symbol

The complete inventory of public symbols this project exposes. 
**2,048 exported symbols across 259 files.**

This is the contract surface other Atipicial components build against.

**`atipicial-cli/src/errors.rs`** (1 symbols): `CliError`

**`atipicial-cli/src/generator.rs`** (6 symbols): `Template`, `TemplateMetadata`, `ProjectTemplate`, `display_name`, `generate_project`, `list_templates`

**`atipicial-cli/src/main.rs`** (1 symbols): `Cli`

**`atipicial-cli/src/utils_core.rs`** (30 symbols): `print_info`, `print_success`, `print_error`, `print_warning`, `print_debug`, `create_table`, `create_progress_bar`, `create_spinner`, `prompt_yes_no`, `prompt_password`, `prompt_input`, `prompt_input_with_default`, `prompt_select`, `LightweightProgress`, `new`, `enable_spinner`, `inc`, `finish_with_message`, `display_key_value`, `display_key_value_colored`, `format_number`, `format_bytes`, `format_duration`, `print_section_header`, `print_subsection_header`, `clear_screen`, `wait_for_enter`, `ensure_account_loaded`, `display_error_details`, `status_indicator`

**`atipicial-cli/src/commands/atipicialfs.rs`** (6 symbols): `AtipicialFsArgs`, `AtipicialFsCommands`, `ContainerCommands`, `ObjectCommands`, `AclCommands`, `ConfigCommands`

**`atipicial-cli/src/commands/blockchain.rs`** (2 symbols): `BlockchainArgs`, `BlockchainCommands`

**`atipicial-cli/src/commands/contract.rs`** (2 symbols): `ContractArgs`, `ContractCommands`

**`atipicial-cli/src/commands/fs.rs`** (8 symbols): `ContainerInfo`, `ObjectInfo`, `NetworkStatus`, `FSArgs`, `FSCommands`, `EndpointCommands`, `ContainerCommands`, `ObjectCommands`

**`atipicial-cli/src/commands/mod.rs`** (9 symbols): `blockchain`, `contract`, `defi`, `fs`, `atipicialfs`, `network`, `nft`, `tools`, `wallet`

**`atipicial-cli/src/commands/network.rs`** (3 symbols): `NetworkConfig`, `NetworkArgs`, `NetworkCommands`

**`atipicial-cli/src/commands/nft.rs`** (2 symbols): `NftArgs`, `NftCommands`

**`atipicial-cli/src/commands/tools.rs`** (2 symbols): `ToolsArgs`, `ToolsCommands`

**`atipicial-cli/src/commands/wallet.rs`** (7 symbols): `CliState`, `get_network_type_string`, `set_network_type`, `get_rpc_client`, `get_account`, `WalletArgs`, `WalletCommands`

**`atipicial-cli/src/commands/defi/mod.rs`** (5 symbols): `tokens`, `utils`, `DefiArgs`, `DefiCommands`, `create_h160_param`

**`atipicial-cli/src/commands/defi/types.rs`** (8 symbols): `TokenArgs`, `TokenCommands`, `SwapArgs`, `SwapCommands`, `LiquidityArgs`, `LiquidityCommands`, `StakingArgs`, `StakingCommands`

**`atipicial-cli/src/commands/defi/utils.rs`** (10 symbols): `NetworkType`, `from_network`, `NetworkTypeCli`, `from_magic`, `from_network_string`, `to_network_string`, `prepare_state_from_existing`, `get_token_address_for_network`, `format_token_amount`, `load_wallet_from_state`

**`atipicial-cli/src/config/mod.rs`** (7 symbols): `CliConfig`, `NetworkConfig`, `WalletConfig`, `StorageConfig`, `LoggingConfig`, `load`, `save`

**`atipicial-cli/src/monitoring/logger.rs`** (20 symbols): `LogFormat`, `LoggerConfig`, `init_logger`, `StructuredLogger`, `new`, `with_context`, `log`, `error`, `warn`, `info`, `debug`, `trace`, `PerformanceLogger`, `new`, `with_threshold`, `complete`, `complete_with_result`, `AuditLogger`, `new`, `log_operation`

**`atipicial-cli/src/monitoring/metrics.rs`** (25 symbols): `MetricsConfig`, `MetricType`, `MetricValue`, `SummaryData`, `Metric`, `MetricsRegistry`, `new`, `register`, `get`, `update`, `all`, `export_prometheus`, `MetricsCollector`, `new`, `start_server`, `stop_server`, `record`, `increment`, `gauge`, `histogram`, `export_prometheus`, `Timer`, `new`, `with_label`, `observe`

**`atipicial-cli/src/monitoring/mod.rs`** (8 symbols): `logger`, `metrics`, `initialize_monitoring`, `MonitoringContext`, `record_metric`, `increment_counter`, `update_gauge`, `observe_histogram`

**`atipicial-cli/src/security/error_handler.rs`** (16 symbols): `RetryConfig`, `critical`, `fast`, `RetryHandler`, `new`, `RecoveryStrategy`, `ErrorHandler`, `new`, `ErrorContext`, `new`, `with_context`, `with_stack_trace`, `ErrorReporter`, `new`, `report`, `get_recent_errors`

**`atipicial-cli/src/security/keychain.rs`** (15 symbols): `KeychainManager`, `SecureCredential`, `new`, `store_credential`, `get_credential`, `delete_credential`, `list_credentials`, `SecureWalletStorage`, `new`, `store_private_key`, `get_private_key`, `store_mnemonic`, `get_mnemonic`, `delete_wallet`, `list_wallets`

**`atipicial-cli/src/security/mod.rs`** (8 symbols): `error_handler`, `keychain`, `network_failover`, `session`, `initialize_security`, `SecurityContext`, `new`, `with_config`

**`atipicial-cli/src/security/network_failover.rs`** (23 symbols): `EndpointHealth`, `new`, `success`, `failure`, `score`, `FailoverConfig`, `NetworkFailover`, `new`, `start_health_monitoring`, `stop_health_monitoring`, `get_best_endpoint`, `get_health_stats`, `add_endpoint`, `remove_endpoint`, `reset_health_stats`, `NetworkFailoverBuilder`, `new`, `add_endpoint`, `add_endpoints`, `with_config`, `health_check_interval`, `request_timeout`, `build`

**`atipicial-cli/src/security/session.rs`** (24 symbols): `SessionConfig`, `Session`, `new`, `is_expired`, `is_idle`, `touch`, `invalidate`, `SessionManager`, `new`, `create_session`, `get_session`, `validate_session`, `refresh_session`, `invalidate_session`, `invalidate_user_sessions`, `cleanup_expired`, `add_auth_callback`, `active_sessions_count`, `get_user_sessions`, `load_persisted_sessions`, `SessionGuard`, `new`, `session`, `is_valid`

**`atipicial-cli/src/utils/atipicialfs.rs`** (8 symbols): `validate_container_id`, `validate_file_path`, `validate_directory_path`, `format_size`, `validate_endpoint`, `get_node_info`, `check_endpoint_availability`, `format_permissions`

**`atipicial-cli/src/utils/config.rs`** (8 symbols): `Config`, `NetworkConfig`, `AtipicialFsConfig`, `AtipicialFsEndpoint`, `get_config_dir`, `get_config_path`, `load_config`, `save_config`

**`atipicial-cli/src/utils/error.rs`** (2 symbols): `CliError`, `CliResult`

**`atipicial-cli/src/utils/extensions.rs`** (1 symbols): `TransactionExtensions`

**`atipicial-cli/src/utils/mod.rs`** (11 symbols): `config`, `error`, `extensions`, `atipicialfs`, `print_success`, `print_info`, `print_warning`, `print_error`, `prompt_input`, `prompt_password`, `prompt_yes_no`

**`atipicial-cli/tests/integration/mod.rs`** (9 symbols): `blockchain_tests`, `contract_tests`, `defi_tests`, `fs_tests`, `generate_tests`, `init_tests`, `network_tests`, `utils`, `wallet_tests`

**`atipicial-cli/tests/integration/utils.rs`** (7 symbols): `CliTest`, `new`, `run`, `run_command`, `create_temp_file`, `assert_success`, `assert_output_contains`

**`examples/atipicial_fs/mod.rs`** (2 symbols): `basic_usage`, `multipart_upload`

**`examples/contracts/examples/contracts/contract.sol`** (4 symbols): `SimpleStorage`, `ValueChanged`, `getValue`, `setValue`

**`examples/providers/examples/retry.rs`** (4 symbols): `RetryPolicy`, `RpcError`, `RetryClient`, `new`

**`examples/sgx_enclave/src/main.rs`** (1 symbols): `encode`

**`examples/transactions/examples/contracts/erc20_example/ERC20Example.sol`** (1 symbols): `ERC20Example`

**`examples/transactions/examples/contracts/erc20_example/@openzeppelin/contracts/token/ERC20/ERC20.sol`** (19 symbols): `ERC20`, `name`, `symbol`, `decimals`, `totalSupply`, `balanceOf`, `transfer`, `allowance`, `approve`, `transferFrom`, `increaseAllowance`, `decreaseAllowance`, `_transfer`, `_mint`, `_burn`, `_approve`, `_spendAllowance`, `_beforeTokenTransfer`, `_afterTokenTransfer`

**`examples/transactions/examples/contracts/erc20_example/@openzeppelin/contracts/token/ERC20/IERC20.sol`** (9 symbols): `IERC20`, `Transfer`, `Approval`, `totalSupply`, `balanceOf`, `transfer`, `allowance`, `approve`, `transferFrom`

**`examples/transactions/examples/contracts/erc20_example/@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol`** (4 symbols): `IERC20Metadata`, `name`, `symbol`, `decimals`

**`examples/transactions/examples/contracts/erc20_example/@openzeppelin/contracts/utils/Context.sol`** (3 symbols): `Context`, `_msgSender`, `_msgData`

**`src/lib.rs`** (19 symbols): `VERSION`, `constants`, `atipicial_error`, `atipicial_types`, `atipicial_utils`, `atipicial_builder`, `atipicial_clients`, `atipicial_codec`, `atipicial_config`, `atipicial_contract`, `atipicial_crypto`, `atipicial_fs`, `atipicial_protocol`, `atipicial_sgx`, `atipicial_wallets`, `atipicial_x`, `monitoring`, `sdk`, `prelude`

**`src/atipicial_builder/error.rs`** (1 symbols): `BuilderError`

**`src/atipicial_builder/utils.rs`** (4 symbols): `public_keys_to_scripthash`, `try_public_keys_to_scripthash`, `pubkey_to_scripthash`, `VecValueExtension`

**`src/atipicial_builder/script/interop_service.rs`** (4 symbols): `InteropService`, `hash`, `from_hash`, `price`

**`src/atipicial_builder/script/script_builder.rs`** (22 symbols): `ScriptBuilder`, `new`, `op_code`, `op_code_with_arg`, `contract_call`, `sys_call`, `push_params`, `push_param`, `push_integer`, `try_push_integer`, `push_opcode_bytes`, `push_data`, `push_bool`, `push_array`, `push_map`, `pack`, `to_bytes`, `build_verification_script`, `build_multi_sig_script`, `build_contract_script`, `build_contract_call_and_unwrap_iterator`, `len`

**`src/atipicial_builder/script/script_reader.rs`** (3 symbols): `ScriptReader`, `get_interop_service_code`, `convert_to_op_code_string`

**`src/atipicial_builder/transaction/call_flags.rs`** (3 symbols): `CallFlags`, `value`, `from_value`

**`src/atipicial_builder/transaction/contract_parameters_context.rs`** (4 symbols): `ContractParametersContext`, `new`, `ContextItem`, `new`

**`src/atipicial_builder/transaction/gas_estimator.rs`** (3 symbols): `GasEstimator`, `calculate_estimation_accuracy`, `TransactionBuilderGasExt`

**`src/atipicial_builder/transaction/invocation_script.rs`** (10 symbols): `InvocationScript`, `new`, `new_with_script`, `from_serialized_script`, `from_signature`, `from_message_and_key_pair`, `from_signatures`, `try_encode`, `try_to_array`, `get_signatures`

**`src/atipicial_builder/transaction/mod.rs`** (1 symbols): `init_logger`

**`src/atipicial_builder/transaction/oracle_response_code.rs`** (1 symbols): `OracleResponseCode`

**`src/atipicial_builder/transaction/production_transaction_builder.rs`** (21 symbols): `FeeCalculator`, `new`, `with_params`, `calculate_network_fee`, `calculate_system_fee`, `WitnessGenerator`, `new`, `add_verification_script`, `generate_witness`, `ProductionWitness`, `ProductionSigner`, `ProductionTransactionAttribute`, `ProductionWitnessRule`, `ProductionTransaction`, `ProductionTransactionBuilder`, `new`, `calculate_network_fee`, `calculate_system_fee`, `add_verification_script`, `generate_witness`, `build_transaction`

**`src/atipicial_builder/transaction/transaction.rs`** (7 symbols): `Transaction`, `new`, `pay`, `add_witness`, `tx_id`, `try_encode`, `try_to_array`

**`src/atipicial_builder/transaction/transaction_attribute.rs`** (12 symbols): `TransactionAttribute`, `OracleResponse`, `MAX_RESULT_SIZE`, `try_size`, `try_encode`, `try_to_bytes`, `to_bytes`, `from_bytes`, `try_to_json`, `to_json`, `get_height`, `get_hash`

**`src/atipicial_builder/transaction/transaction_builder.rs`** (19 symbols): `TransactionBuilder`, `BALANCE_OF_FUNCTION`, `DUMMY_PUB_KEY`, `new`, `with_client`, `allow_transmission_on_fault`, `disallow_transmission_on_fault`, `version`, `nonce`, `valid_until_block`, `first_signer`, `first_signer_by_hash`, `extend_script`, `validate`, `is_ready`, `set_signers`, `add_attributes`, `do_if_sender_cannot_cover_fees`, `throw_if_sender_cannot_cover_fees`

**`src/atipicial_builder/transaction/transaction_error.rs`** (1 symbols): `TransactionError`

**`src/atipicial_builder/transaction/transaction_send_token.rs`** (2 symbols): `TransactionSendToken`, `new`

**`src/atipicial_builder/transaction/verification_script.rs`** (14 symbols): `VerificationScript`, `new`, `from`, `from_public_key`, `from_multi_sig`, `is_single_sig`, `is_multi_sig`, `hash`, `get_signatures`, `get_public_keys`, `get_signing_threshold`, `get_nr_of_accounts`, `try_encode`, `try_to_array`

**`src/atipicial_builder/transaction/witness.rs`** (10 symbols): `Witness`, `new`, `from_scripts`, `from_scripts_obj`, `create`, `create_multi_sig_witness`, `create_multi_sig_witness_script`, `create_contract_witness`, `try_encode`, `try_to_array`

**`src/atipicial_builder/transaction/witness_scope.rs`** (5 symbols): `WitnessScope`, `byte_repr`, `validate`, `combine`, `split`

**`src/atipicial_builder/transaction/signers/account_signer.rs`** (12 symbols): `AccountSigner`, `none`, `called_by_entry`, `global`, `is_multi_sig`, `get_script_hash`, `try_encode`, `try_to_array`, `new`, `none_hash160`, `called_by_entry_hash160`, `global_hash160`

**`src/atipicial_builder/transaction/signers/contract_signer.rs`** (5 symbols): `ContractSigner`, `called_by_entry`, `global`, `try_encode`, `try_to_array`

**`src/atipicial_builder/transaction/signers/signer.rs`** (14 symbols): `SignerType`, `SignerTrait`, `Signer`, `from_bytes`, `get_type`, `get_signer_hash`, `as_account_signer`, `as_contract_signer`, `as_transaction_signer`, `to_account_signer`, `to_contract_signer`, `try_to_transaction_signer`, `try_encode`, `try_to_array`

**`src/atipicial_builder/transaction/signers/transaction_signer.rs`** (5 symbols): `TransactionSigner`, `new`, `new_full`, `try_encode`, `try_to_array`

**`src/atipicial_builder/transaction/witness_rule/witness_action.rs`** (1 symbols): `WitnessAction`

**`src/atipicial_builder/transaction/witness_rule/witness_condition.rs`** (11 symbols): `WitnessCondition`, `json_value`, `byte`, `boolean_expression`, `expression`, `expression_list`, `script_hash`, `group`, `from_bytes`, `try_encode`, `try_to_array`

**`src/atipicial_builder/transaction/witness_rule/witness_rule.rs`** (4 symbols): `WitnessRule`, `new`, `try_encode`, `try_to_array`

**`src/atipicial_clients/api_trait.rs`** (1 symbols): `APITrait`

**`src/atipicial_clients/cache.rs`** (15 symbols): `CacheConfig`, `builder`, `CacheConfigBuilder`, `max_entries`, `default_ttl`, `cleanup_interval`, `enable_lru`, `build`, `Cache`, `CacheStats`, `hit_rate`, `new`, `start_cleanup_task`, `RpcCache`, `new_rpc_cache`

**`src/atipicial_clients/circuit_breaker.rs`** (13 symbols): `CircuitState`, `CircuitBreakerConfig`, `builder`, `CircuitBreakerConfigBuilder`, `failure_threshold`, `timeout`, `success_threshold`, `failure_window`, `half_open_max_requests`, `build`, `CircuitBreakerStats`, `CircuitBreaker`, `new`

**`src/atipicial_clients/connection_pool.rs`** (15 symbols): `PoolConfig`, `builder`, `PoolConfigBuilder`, `max_connections`, `min_idle`, `max_idle_time`, `connection_timeout`, `request_timeout`, `max_retries`, `retry_delay`, `build`, `ConnectionPool`, `PoolStats`, `new`, `start_maintenance_task`

**`src/atipicial_clients/errors.rs`** (8 symbols): `ProviderError`, `is_retryable`, `is_rate_limited`, `retry_after`, `http_status`, `is_unknown_transaction`, `is_already_known_transaction`, `is_transaction_rejection`

**`src/atipicial_clients/mock_client.rs`** (2 symbols): `MockClient`, `into_client`

**`src/atipicial_clients/mod.rs`** (7 symbols): `RpcError`, `try_http_provider_from_endpoint`, `try_http_provider_from_env`, `TestProvider`, `new`, `url`, `provider`

**`src/atipicial_clients/production_client.rs`** (12 symbols): `ProductionRpcClient`, `ProductionClientConfig`, `builder`, `ProductionClientConfigBuilder`, `pool_config`, `cache_config`, `circuit_breaker_config`, `enable_logging`, `enable_metrics`, `build`, `ProductionClientStats`, `new`

**`src/atipicial_clients/rate_limiter.rs`** (14 symbols): `RateLimiter`, `new`, `RateLimitPermit`, `RateLimiterBuilder`, `new`, `max_requests`, `window`, `max_concurrent`, `build`, `RateLimiterPresets`, `conservative`, `standard`, `aggressive`, `custom`

**`src/atipicial_clients/utils.rs`** (19 symbols): `EscalationPolicy`, `AtipicialHttpClient`, `ProviderResult`, `AsyncProviderResult`, `AsyncProviderResult`, `interval`, `try_serialize`, `serialize`, `script_hash_from_script`, `public_key_to_address`, `public_key_to_script_hash`, `private_key_to_script_hash`, `private_key_to_address`, `script_hash_to_address`, `address_to_script_hash`, `script_hash_to_hex`, `script_hash_from_hex`, `address_to_hex`, `hex_to_address`

**`src/atipicial_clients/ext/dev_rpc.rs`** (3 symbols): `DevRpcMiddleware`, `DevRpcMiddlewareError`, `new`

**`src/atipicial_clients/ext/mod.rs`** (1 symbols): `nns`

**`src/atipicial_clients/rpc/connections.rs`** (1 symbols): `JsonRpcProvider`

**`src/atipicial_clients/rpc/pubsub.rs`** (4 symbols): `PubsubClient`, `SubscriptionStream`, `new`, `set_loaded_elements`

**`src/atipicial_clients/rpc/rpc_client.rs`** (9 symbols): `AtipicialClient`, `RpcClient`, `new`, `with_sender`, `set_interval`, `interval`, `url`, `url_mut`, `rw`

**`src/atipicial_clients/rpc/transports/common.rs`** (28 symbols): `JsonRpcError`, `is_retryable`, `is_rate_limited`, `retry_after`, `is_unknown_transaction`, `is_already_known_transaction`, `is_transaction_rejection`, `is_revert`, `as_revert_data`, `Request`, `new`, `Response`, `Params`, `Authorization`, `basic`, `bearer`, `raw`, `JWT_SECRET_LENGTH`, `JwtKey`, `from_slice`, `from_hex`, `as_bytes`, `into_bytes`, `JwtAuth`, `new`, `generate_token`, `validate_token`, `Claims`

**`src/atipicial_clients/rpc/transports/http_provider.rs`** (8 symbols): `HttpProvider`, `ClientError`, `new`, `url`, `url_mut`, `new_with_auth`, `new_with_client`, `HttpClientError`

**`src/atipicial_clients/rpc/transports/ipc.rs`** (2 symbols): `Ipc`, `IpcError`

**`src/atipicial_clients/rpc/transports/legacy_ws.rs`** (4 symbols): `Ws`, `new`, `ready`, `ClientError`

**`src/atipicial_clients/rpc/transports/mock.rs`** (10 symbols): `MockResponse`, `MockProvider`, `new`, `push_result`, `push_result_with_params`, `push_result_with_partial_params`, `push_error_any`, `push_error`, `take_requests`, `assert_request`

**`src/atipicial_clients/rpc/transports/mod.rs`** (1 symbols): `legacy_ws`

**`src/atipicial_clients/rpc/transports/retry.rs`** (12 symbols): `RetryPolicy`, `RetryClient`, `new`, `set_compute_units`, `RetryClientBuilder`, `timeout_retries`, `rate_limit_retries`, `compute_units_per_second`, `initial_backoff`, `build`, `RetryClientError`, `HttpRateLimitRetryPolicy`

**`src/atipicial_clients/rpc/transports/rw.rs`** (7 symbols): `RwClient`, `new`, `read_client`, `write_client`, `transpose`, `split`, `RwClientError`

**`src/atipicial_clients/rpc/transports/ws/error.rs`** (1 symbols): `WsClientError`

**`src/atipicial_clients/rpc/transports/ws/mod.rs`** (1 symbols): `WsClient`

**`src/atipicial_clients/rpc/transports/ws/types.rs`** (3 symbols): `ConnectionDetails`, `new`, `new`

**`src/atipicial_codec/binary_decoder.rs`** (30 symbols): `Decoder`, `new`, `read_u8_safe`, `read_bool_safe`, `read_bool`, `read_u8`, `read_u16`, `read_i16`, `read_u32`, `read_i32`, `read_u64`, `read_i64`, `read_bigint`, `read_encoded_ec_point`, `read_bytes`, `read_var_bytes`, `read_var_bytes_bounded`, `read_var_int`, `read_var_string`, `read_var_string_bounded`, `read_push_bytes`, `read_push_int`, `read_push_string`, `read_serializable`, `read_serializable_list_bounded`, `read_serializable_list`, `read_serializable_list_var_bytes`, `mark`, `reset`, `available`

**`src/atipicial_codec/binary_encoder.rs`** (23 symbols): `Encoder`, `new`, `size`, `write_bool`, `write_u8`, `write_i16`, `write_i32`, `write_i64`, `write_u16`, `write_u32`, `write_u64`, `write_bytes`, `write_var_int`, `write_var_string`, `write_fixed_string`, `write_var_bytes`, `write_serializable_fixed`, `write_serializable_list_fixed`, `write_serializable_variable_bytes`, `write_serializable_variable_list`, `write_serializable_variable_list_bytes`, `reset`, `to_bytes`

**`src/atipicial_codec/encode.rs`** (2 symbols): `AtipicialSerializable`, `VarSizeTrait`

**`src/atipicial_codec/error.rs`** (1 symbols): `CodecError`

**`src/atipicial_config/config.rs`** (19 symbols): `AtipicialNetwork`, `to_magic`, `from_magic`, `DEFAULT_BLOCK_TIME`, `DEFAULT_ADDRESS_VERSION`, `MAX_VALID_UNTIL_BLOCK_INCREMENT_BASE`, `AtipicialConfig`, `atipicial_config_lock`, `new`, `set_network`, `get_max_valid_until_block_increment`, `is_hardfork_enabled`, `get_hardfork_height`, `set_hardfork_height`, `mainnet`, `testnet`, `Counter`, `new`, `get_and_increment`

**`src/atipicial_config/constant.rs`** (37 symbols): `AtipicialConstants`, `MAGIC_NUMBER_MAINNET`, `MAGIC_NUMBER_TESTNET`, `MAX_PUBLIC_KEYS_PER_MULTI_SIG`, `HASH160_SIZE`, `HASH256_SIZE`, `PRIVATE_KEY_SIZE`, `PUBLIC_KEY_SIZE_COMPRESSED`, `SIGNATURE_SIZE`, `VERIFICATION_SCRIPT_SIZE`, `MAX_ITERATOR_ITEMS_DEFAULT`, `MAX_SUBITEMS`, `MAX_NESTING_DEPTH`, `CURRENT_TX_VERSION`, `MAX_TRANSACTION_SIZE`, `MAX_TRANSACTION_ATTRIBUTES`, `MAX_SIGNER_SUBITEMS`, `MAX_MANIFEST_SIZE`, `MAX_RPC_MESSAGE_SIZE`, `max_rpc_message_size`, `max_rpc_message_size`, `rpc_request_timeout`, `rpc_request_timeout`, `SEED_1`, `SEED_2`, `SEED_3`, `SEED_4`, `SEED_5`, `SCRYPT_N`, `SCRYPT_R`, `SCRYPT_P`, `SCRYPT_LOG_N`, `SCRYPT_DK_LEN`, `AEP_HEADER_1`, `AEP_HEADER_2`, `AEP_FLAG`, `new`

**`src/atipicial_config/test_properties.rs`** (25 symbols): `TestConstants`, `TEST_RESOURCE_PATH`, `DEFAULT_ACCOUNT_ADDRESS`, `DEFAULT_ACCOUNT_SCRIPT_HASH`, `DEFAULT_ACCOUNT_VERIFICATION_SCRIPT`, `DEFAULT_ACCOUNT_PUBLIC_KEY`, `DEFAULT_ACCOUNT_PRIVATE_KEY`, `DEFAULT_ACCOUNT_ENCRYPTED_PRIVATE_KEY`, `DEFAULT_ACCOUNT_WIF`, `DEFAULT_ACCOUNT_PASSWORD`, `COMMITTEE_ACCOUNT_ADDRESS`, `COMMITTEE_ACCOUNT_SCRIPT_HASH`, `COMMITTEE_ACCOUNT_VERIFICATION_SCRIPT`, `CONTRACT_MANAGEMENT_HASH`, `STD_LIB_HASH`, `CRYPTO_LIB_HASH`, `LEDGER_CONTRACT_HASH`, `ATC_TOKEN_HASH`, `GAS_TOKEN_HASH`, `GAS_TOKEN_NAME`, `POLICY_CONTRACT_HASH`, `ROLE_MANAGEMENT_HASH`, `ORACLE_CONTRACT_HASH`, `NAME_SERVICE_HASH`, `CLIENT1_ACCOUNT_WIF`

**`src/atipicial_contract/atipicial_token.rs`** (8 symbols): `AtipicialCoin`, `NAME`, `DECIMALS`, `SYMBOL`, `TOTAL_SUPPLY`, `Candidate`, `AccountState`, `with_no_balance`

**`src/atipicial_contract/atipicial_uri.rs`** (8 symbols): `AtipicialURI`, `new`, `from_uri`, `uri_string`, `recipient_address`, `token_string`, `token_str`, `build_uri`

**`src/atipicial_contract/contract_error.rs`** (1 symbols): `ContractError`

**`src/atipicial_contract/contract_management.rs`** (4 symbols): `ContractManagement`, `NAME`, `new`, `with_script_hash`

**`src/atipicial_contract/fungible_token_contract.rs`** (2 symbols): `FungibleTokenContract`, `new`

**`src/atipicial_contract/gas_token.rs`** (5 symbols): `GasToken`, `NAME`, `DECIMALS`, `SYMBOL`, `new`

**`src/atipicial_contract/iterator.rs`** (2 symbols): `AtipicialIterator`, `new`

**`src/atipicial_contract/name_service.rs`** (6 symbols): `RecordType`, `NameState`, `AtipicialNameService`, `NAME`, `new`, `with_script_hash`

**`src/atipicial_contract/nft_contract.rs`** (2 symbols): `NftContract`, `new`

**`src/atipicial_contract/notary.rs`** (8 symbols): `NotaryDeposit`, `from_stack_item`, `NotaryContract`, `NAME`, `DEFAULT_MAX_NOT_VALID_BEFORE_DELTA`, `DEFAULT_DEPOSIT_DELTA_TILL`, `new`, `supported_standards`

**`src/atipicial_contract/policy_contract.rs`** (14 symbols): `PolicyContract`, `NAME`, `DEFAULT_EXEC_FEE_FACTOR`, `DEFAULT_STORAGE_PRICE`, `DEFAULT_FEE_PER_BYTE`, `DEFAULT_ATTRIBUTE_FEE`, `DEFAULT_NOTARY_ASSISTED_ATTRIBUTE_FEE`, `MAX_EXEC_FEE_FACTOR`, `MAX_ATTRIBUTE_FEE`, `MAX_STORAGE_PRICE`, `MAX_MILLISECONDS_PER_BLOCK`, `MAX_MAX_VALID_UNTIL_BLOCK_INCREMENT`, `MAX_MAX_TRACEABLE_BLOCKS`, `new`

**`src/atipicial_contract/role_management.rs`** (5 symbols): `RoleManagement`, `NAME`, `new`, `Role`, `fn`

**`src/atipicial_contract/treasury.rs`** (4 symbols): `TreasuryContract`, `NAME`, `new`, `supported_standards`

**`src/atipicial_contract/famous/atipicialburger.rs`** (10 symbols): `AtipicialburgerContract`, `CONTRACT_HASH`, `SYMBOL`, `DECIMALS`, `WRAP`, `UNWRAP`, `CLAIM_GAS`, `GET_RATE`, `new`, `with_script_hash`

**`src/atipicial_contract/famous/atipicialcompound.rs`** (8 symbols): `AtipicialCompoundContract`, `CONTRACT_HASH`, `DEPOSIT`, `WITHDRAW`, `COMPOUND`, `GET_APY`, `new`, `with_script_hash`

**`src/atipicial_contract/famous/contracts.rs`** (15 symbols): `Network`, `FamousContract`, `new_unchecked`, `new`, `flamingo_flm_token`, `flamingo_flamingo_finance`, `ghostmarket`, `atipicialburger_dao`, `atipicialcompound`, `atipicial_name_service`, `bridge_atipicial_to_eth`, `testnet_nns`, `testnet_faucet`, `get_famous_contracts`, `get_all_famous_contracts`

**`src/atipicial_contract/famous/flamingo.rs`** (9 symbols): `FlamingoContract`, `CONTRACT_HASH`, `SWAP`, `ADD_LIQUIDITY`, `REMOVE_LIQUIDITY`, `STAKE`, `CLAIM_REWARDS`, `new`, `with_script_hash`

**`src/atipicial_contract/famous/grandshare.rs`** (8 symbols): `GrandShareContract`, `CONTRACT_HASH`, `SUBMIT_PROPOSAL`, `VOTE`, `FUND_PROJECT`, `CLAIM_FUNDS`, `new`, `with_script_hash`

**`src/atipicial_contract/famous/mod.rs`** (5 symbols): `contracts`, `flamingo`, `grandshare`, `atipicialburger`, `atipicialcompound`

**`src/atipicial_contract/traits/fungible_token.rs`** (1 symbols): `FungibleTokenTrait`

**`src/atipicial_contract/traits/nft.rs`** (1 symbols): `NonFungibleTokenTrait`

**`src/atipicial_contract/traits/smart_contract.rs`** (1 symbols): `SmartContractTrait`

**`src/atipicial_contract/traits/token.rs`** (1 symbols): `TokenTrait`

**`src/atipicial_crypto/base58_helper.rs`** (5 symbols): `Base58CheckError`, `base58check_encode`, `try_base58check_decode`, `base58check_decode`, `calculate_checksum`

**`src/atipicial_crypto/crypto_lib.rs`** (5 symbols): `sha3_512`, `blake2b_512`, `verify_with_ed25519`, `recover_secp256k1`, `CryptoLibHashable`

**`src/atipicial_crypto/error.rs`** (3 symbols): `CryptoError`, `Aep2Error`, `SignError`

**`src/atipicial_crypto/hash.rs`** (2 symbols): `HashableForVec`, `HashableForString`

**`src/atipicial_crypto/keys.rs`** (26 symbols): `Secp256r1PublicKey`, `Secp256r1PrivateKey`, `Secp256r1Signature`, `Secp256r1SignedMsg`, `new`, `from_public_key`, `from_bytes`, `verify`, `get_encoded`, `get_encoded_point`, `get_encoded_compressed_hex`, `from_encoded`, `new_random`, `random`, `from_bytes`, `to_raw_bytes`, `to_public_key`, `erase`, `sign_tx`, `sign_prehash`, `from_scalars`, `from_u256`, `from_bytes`, `to_bytes`, `PrivateKeyExtension`, `PublicKeyExtension`

**`src/atipicial_crypto/key_pair.rs`** (19 symbols): `KeyPair`, `new`, `private_key`, `private_key_ref`, `public_key`, `public_key_ref`, `has_private_key`, `from_secret_key`, `private_key_bytes`, `public_key_bytes`, `sign`, `verify`, `new_random`, `from_private_key`, `from_wif`, `from_public_key`, `export_as_wif`, `get_script_hash`, `get_address`

**`src/atipicial_crypto/mod.rs`** (3 symbols): `crypto_lib`, `hash`, `utils`

**`src/atipicial_crypto/utils.rs`** (10 symbols): `private_key_to_public_key`, `private_key_to_hex_string`, `private_key_from_hex`, `public_key_to_hex_string`, `public_key_from_hex`, `ToArray32`, `ToHexString`, `FromHexString`, `FromBase64String`, `ToBase64String`

**`src/atipicial_crypto/wif.rs`** (2 symbols): `private_key_from_wif`, `wif_from_private_key`

**`src/atipicial_error/mod.rs`** (11 symbols): `unified`, `LegacyError`, `CryptoError`, `WalletError`, `NetworkError`, `TransactionError`, `ContractError`, `SerializationError`, `AtipicialResult`, `ErrorContext`, `legacy_error`

**`src/atipicial_error/unified.rs`** (32 symbols): `AtipicialError`, `ErrorRecovery`, `new`, `suggest`, `retryable`, `retry_after`, `doc`, `Result`, `ProvideErrorMetadata`, `AtipicialErrorKind`, `provider`, `kind`, `recovery`, `is_retryable`, `retry_after`, `message`, `network`, `transaction`, `contract`, `validation`, `wallet`, `ErrorBuilder`, `network`, `wallet`, `contract`, `source`, `with_contract`, `with_method`, `suggest`, `retryable`, `build`, `ErrorContextExt`

**`src/atipicial_fs/acl.rs`** (14 symbols): `Operation`, `ContainerOperation`, `ObjectOperation`, `Target`, `TargetRole`, `Action`, `Filter`, `FilterOperation`, `EACLRecord`, `EACL`, `new`, `add_record`, `BearerToken`, `SessionToken`

**`src/atipicial_fs/client.rs`** (4 symbols): `AtipicialFsClient`, `new`, `with_account`, `get_owner_id`

**`src/atipicial_fs/container.rs`** (13 symbols): `Version`, `Container`, `new`, `with_basic_acl`, `with_name`, `with_creation`, `with_version`, `with_attribute`, `BasicACL`, `full_access`, `read_only`, `to_bitmask`, `from_bitmask`

**`src/atipicial_fs/error.rs`** (2 symbols): `AtipicialFsError`, `AtipicialFsResult`

**`src/atipicial_fs/mod.rs`** (23 symbols): `acl`, `client`, `container`, `error`, `object`, `types`, `DEFAULT_MAINNET_ENDPOINT`, `DEFAULT_TESTNET_ENDPOINT`, `DEFAULT_ENDPOINT`, `DEFAULT_MAINNET_HTTP_GATEWAY`, `DEFAULT_TESTNET_HTTP_GATEWAY`, `DEFAULT_MAINNET_REST_API`, `DEFAULT_TESTNET_REST_API`, `AtipicialFsConfig`, `builder`, `AtipicialFsConfigBuilder`, `endpoint`, `auth`, `timeout_sec`, `insecure`, `build`, `AtipicialFsAuth`, `AtipicialFsService`

**`src/atipicial_fs/object.rs`** (12 symbols): `Object`, `new`, `with_payload`, `with_type`, `with_attribute`, `with_filename`, `with_content_type`, `size`, `MultipartUpload`, `Part`, `new`, `MultipartUploadResult`

**`src/atipicial_fs/types.rs`** (15 symbols): `ContainerId`, `ObjectId`, `OwnerId`, `PlacementPolicy`, `Selector`, `Filter`, `ClauseOperator`, `MatchOperator`, `ObjectType`, `Attributes`, `new`, `add`, `get`, `AccessPermission`, `SessionToken`

**`src/atipicial_protocol/account.rs`** (9 symbols): `AccountTrait`, `Account`, `get_address`, `get_script_hash`, `get_verification_script`, `get_public_key`, `decrypt_private_key_with_params`, `encrypt_private_key_with_params`, `to_aep6_account`

**`src/atipicial_protocol/aep2.rs`** (11 symbols): `AEP2`, `encrypt`, `encrypt_with_params`, `decrypt`, `decrypt_with_params`, `encrypt_for_test_vector`, `decrypt_for_test_vector`, `encrypt_test_vector`, `decrypt_test_vector`, `get_aep2_from_private_key`, `get_private_key_from_aep2`

**`src/atipicial_protocol/protocol_error.rs`** (1 symbols): `ProtocolError`

**`src/atipicial_protocol/responses/atipicial_account_state.rs`** (3 symbols): `AccountState`, `with_no_vote`, `with_no_balance`

**`src/atipicial_protocol/responses/atipicial_address.rs`** (1 symbols): `AtipicialAddress`

**`src/atipicial_protocol/responses/atipicial_application_log.rs`** (8 symbols): `ApplicationLog`, `get_first_execution`, `get_execution`, `Execution`, `get_first_stack_item`, `get_stack_item`, `get_first_notification`, `get_notification`

**`src/atipicial_protocol/responses/atipicial_balances.rs`** (8 symbols): `Aep11Balances`, `Aep11Balance`, `new`, `Aep11Token`, `new`, `Aep17Balances`, `Aep17Balance`, `new`

**`src/atipicial_protocol/responses/atipicial_block.rs`** (2 symbols): `AtipicialBlock`, `get_nonce_as_u64`

**`src/atipicial_protocol/responses/atipicial_find_states.rs`** (2 symbols): `States`, `StateResult`

**`src/atipicial_protocol/responses/atipicial_get_claimable.rs`** (2 symbols): `Claimables`, `Claim`

**`src/atipicial_protocol/responses/atipicial_get_mem_pool.rs`** (1 symbols): `MemPoolDetails`

**`src/atipicial_protocol/responses/atipicial_get_next_block_validators.rs`** (2 symbols): `Validator`, `new`

**`src/atipicial_protocol/responses/atipicial_get_peers.rs`** (3 symbols): `Peers`, `AddressEntry`, `new`

**`src/atipicial_protocol/responses/atipicial_get_state_height.rs`** (1 symbols): `StateHeight`

**`src/atipicial_protocol/responses/atipicial_get_state_root.rs`** (1 symbols): `StateRoot`

**`src/atipicial_protocol/responses/atipicial_get_token_balances.rs`** (2 symbols): `TokenBalances`, `TokenBalance`

**`src/atipicial_protocol/responses/atipicial_get_token_transfers.rs`** (2 symbols): `TokenTransfers`, `TokenTransfer`

**`src/atipicial_protocol/responses/atipicial_get_unclaimed_gas.rs`** (1 symbols): `UnclaimedGas`

**`src/atipicial_protocol/responses/atipicial_get_unspents.rs`** (3 symbols): `Unspents`, `Balance`, `UnspentTransaction`

**`src/atipicial_protocol/responses/atipicial_get_version.rs`** (4 symbols): `AtipicialVersion`, `AtipicialRpcSettings`, `AtipicialProtocol`, `HardForks`

**`src/atipicial_protocol/responses/atipicial_get_wallet_balance.rs`** (1 symbols): `Balance`

**`src/atipicial_protocol/responses/atipicial_list_plugins.rs`** (1 symbols): `Plugin`

**`src/atipicial_protocol/responses/atipicial_network_fee.rs`** (1 symbols): `AtipicialNetworkFee`

**`src/atipicial_protocol/responses/atipicial_send_raw_transaction.rs`** (2 symbols): `RawTransaction`, `new`

**`src/atipicial_protocol/responses/atipicial_submit_block.rs`** (2 symbols): `SubmitBlock`, `get_submit_block`

**`src/atipicial_protocol/responses/atipicial_transaction_result.rs`** (2 symbols): `TransactionResult`, `AtipicialTransactionSigner`

**`src/atipicial_protocol/responses/atipicial_transfers.rs`** (6 symbols): `Aep11Transfers`, `Aep11Transfer`, `new`, `Aep17Transfers`, `Aep17Transfer`, `new`

**`src/atipicial_protocol/responses/atipicial_validate_address.rs`** (2 symbols): `ValidateAddress`, `new`

**`src/atipicial_protocol/responses/atipicial_witness.rs`** (3 symbols): `AtipicialWitness`, `new`, `from_witness`

**`src/atipicial_protocol/responses/diagnostics.rs`** (4 symbols): `Diagnostics`, `new`, `InvokedContract`, `StorageChange`

**`src/atipicial_protocol/responses/express_contract_state.rs`** (2 symbols): `ExpressContractState`, `new`

**`src/atipicial_protocol/responses/express_shutdown.rs`** (2 symbols): `ExpressShutdown`, `new`

**`src/atipicial_protocol/responses/notification.rs`** (2 symbols): `LogNotification`, `new`

**`src/atipicial_protocol/responses/oracle_request.rs`** (2 symbols): `OracleRequest`, `new`

**`src/atipicial_protocol/responses/populated_blocks.rs`** (2 symbols): `PopulatedBlocks`, `new`

**`src/atipicial_protocol/responses/response_transaction.rs`** (6 symbols): `RTransaction`, `new`, `get_first_signer`, `get_signer`, `get_first_attribute`, `get_attribute`

**`src/atipicial_protocol/responses/response_transaction_attribute.rs`** (7 symbols): `TransactionAttributeType`, `HighPriorityAttribute`, `OracleResponseAttribute`, `NotValidBeforeAttribute`, `ConflictsAttribute`, `TransactionAttributeEnum`, `OracleResponse`

**`src/atipicial_protocol/responses/response_transaction_signer.rs`** (11 symbols): `RTransactionSigner`, `new`, `new_full`, `get_first_scope`, `get_scope`, `get_first_allowed_contract`, `get_allowed_contract`, `get_first_allowed_group`, `get_allowed_group`, `get_first_rule`, `get_rule`

**`src/atipicial_sgx/allocator.rs`** (10 symbols): `SgxAllocator`, `fn`, `memory_usage`, `available_memory`, `init_allocator`, `SgxAllocator`, `new`, `memory_usage`, `available_memory`, `init_allocator`

**`src/atipicial_sgx/attestation.rs`** (16 symbols): `RemoteAttestation`, `new`, `configure_spid`, `init_attestation`, `init_attestation`, `generate_quote`, `generate_quote`, `get_quote`, `close`, `close`, `QuoteVerifier`, `new`, `configure_ias`, `verify_quote`, `QuoteVerificationResult`, `TcbStatus`

**`src/atipicial_sgx/crypto.rs`** (15 symbols): `SgxCrypto`, `compute_shared_secret`, `new`, `sha256`, `sign_ecdsa`, `verify_ecdsa`, `random_bytes`, `generate_keypair`, `SgxKeyManager`, `new`, `seal_key`, `unseal_key`, `seal_key`, `unseal_key`, `init_crypto`

**`src/atipicial_sgx/enclave.rs`** (16 symbols): `EnclaveConfig`, `EnclaveAttributes`, `SgxEnclave`, `new`, `initialize`, `initialize`, `is_initialized`, `config`, `ecall`, `ecall`, `destroy`, `destroy`, `EnclaveSerializable`, `EnclaveDeserializable`, `generate_enclave_config`, `generate_edl`

**`src/atipicial_sgx/mod.rs`** (9 symbols): `allocator`, `attestation`, `crypto`, `enclave`, `networking`, `storage`, `init_sgx`, `SgxError`, `prelude`

**`src/atipicial_sgx/networking.rs`** (11 symbols): `SgxNetworking`, `SecureChannel`, `new`, `establish_channel`, `establish_channel`, `send_secure`, `receive_secure`, `complete_handshake`, `is_established`, `ocall_network_request`, `ocall_network_request`

**`src/atipicial_sgx/storage.rs`** (10 symbols): `SecureStorage`, `new`, `store`, `store`, `retrieve`, `retrieve`, `delete`, `delete`, `list_keys`, `list_keys`

**`src/atipicial_types/address.rs`** (4 symbols): `Address`, `NameOrAddress`, `AddressExtension`, `from_script_hash`

**`src/atipicial_types/address_or_scripthash.rs`** (5 symbols): `AddressOrScriptHash`, `try_from_script_hash_bytes`, `address`, `try_script_hash`, `script_hash`

**`src/atipicial_types/bytes.rs`** (1 symbols): `ReverseTrait`

**`src/atipicial_types/error.rs`** (1 symbols): `TypeError`

**`src/atipicial_types/hardfork.rs`** (4 symbols): `Hardfork`, `all`, `name`, `description`

**`src/atipicial_types/mod.rs`** (17 symbols): `script_hash`, `contract`, `error`, `hardfork`, `nns`, `serde_value`, `serde_with_utils`, `whitelisted_contract`, `block`, `Byte`, `Bytes`, `TxHash`, `ExternBase64`, `ScryptParamsDef`, `Base64Encode`, `TryBase64Encode`, `to_checksum`

**`src/atipicial_types/numeric.rs`** (2 symbols): `ToBytesPadded`, `ToBytes`

**`src/atipicial_types/op_code.rs`** (8 symbols): `OpCode`, `price`, `opcode`, `to_hex_string`, `operand_size`, `OperandSize`, `with_size`, `with_prefix_size`

**`src/atipicial_types/path_or_string.rs`** (2 symbols): `PathOrString`, `read`

**`src/atipicial_types/plugin_type.rs`** (2 symbols): `NodePluginType`, `value_of_name`

**`src/atipicial_types/script_hash.rs`** (7 symbols): `ScriptHash`, `ScriptHashExtension`, `Address`, `new`, `as_str`, `to_script_hash`, `IntoScriptHash`

**`src/atipicial_types/serde_value.rs`** (1 symbols): `ValueExtension`

**`src/atipicial_types/serde_with_utils.rs`** (70 symbols): `serialize_h160_without_0x`, `serialize_h160`, `deserialize_h160`, `serialize_scopes`, `deserialize_scopes`, `serialize_boolean_expression`, `deserialize_boolean_expression`, `serialize_bytes`, `deserialize_bytes`, `serialize_url`, `deserialize_pubkey`, `serialize_pubkey`, `deserialize_url`, `serialize_url_option`, `deserialize_url_option`, `serialize_u256`, `deserialize_u256`, `serialize_u256_option`, `deserialize_u256_option`, `serialize_u32`, `deserialize_u32`, `serialize_u64`, `deserialize_u64`, `serialize_u64_option`, `deserialize_u64_option`, `deserialize_script_hash`, `serialize_script_hash`, `deserialize_address_or_script_hash`, `serialize_address_or_script_hash`, `deserialize_vec_script_hash`, `serialize_vec_script_hash`, `deserialize_vec_script_hash_option`, `serialize_vec_script_hash_option`, `serialize_script_hash_option`, `deserialize_script_hash_option`, `serialize_hash_map_h160_account`, `deserialize_hash_map_h160_account`, `deserialize_private_key`, `serialize_private_key`, `deserialize_public_key`, `serialize_public_key`, `deserialize_vec_public_key`, `deserialize_vec_public_key_option`, `serialize_vec_public_key`, `serialize_vec_public_key_option`, `serialize_public_key_option`, `deserialize_public_key_option`, `serialize_h256`, `deserialize_h256`, `serialize_hashset_u256`, `deserialize_hashset_u256`, `serialize_vec_h256`, `deserialize_vec_h256`, `serialize_vec_u256`, `deserialize_vec_u256`, `serialize_h256_option`, `deserialize_h256_option`, `serialize_hashmap_u256_hashset_u256`, `deserialize_hashmap_u256_hashset_u256`, `serialize_hashmap_address_u256`, `deserialize_hashmap_address_u256`, `serialize_hashmap_u256_hashset_h256`, `deserialize_hashmap_u256_hashset_h256`, `serialize_hashmap_u256_vec_u256`, `deserialize_hashmap_u256_vec_u256`, `serialize_map`, `deserialize_map`, `serialize_wildcard`, `deserialize_wildcard`, `deserialize_hardforks`

**`src/atipicial_types/stack_item.rs`** (42 symbols): `StackItem`, `MapEntry`, `ANY_VALUE`, `POINTER_VALUE`, `BOOLEAN_VALUE`, `INTEGER_VALUE`, `BYTE_STRING_VALUE`, `BUFFER_VALUE`, `ARRAY_VALUE`, `STRUCT_VALUE`, `MAP_VALUE`, `INTEROP_INTERFACE_VALUE`, `ANY_BYTE`, `POINTER_BYTE`, `BOOLEAN_BYTE`, `INTEGER_BYTE`, `BYTE_STRING_BYTE`, `BUFFER_BYTE`, `ARRAY_BYTE`, `STRUCT_BYTE`, `MAP_BYTE`, `INTEROP_INTERFACE_BYTE`, `new_byte_string`, `as_bool`, `as_string`, `to_string`, `as_bytes`, `as_array`, `as_array_ref`, `as_int`, `as_map`, `as_map_entries`, `as_address`, `as_public_key`, `as_hash160`, `as_hash256`, `as_interop`, `len`, `is_empty`, `get`, `get_iterator_id`, `get_interface_name`

**`src/atipicial_types/string.rs`** (2 symbols): `TryStringExt`, `StringExt`

**`src/atipicial_types/syncing.rs`** (2 symbols): `SyncingStatus`, `SyncProgress`

**`src/atipicial_types/tx_pool.rs`** (4 symbols): `TxPoolInspectSummary`, `TxpoolContent`, `TxpoolInspect`, `TxpoolStatus`

**`src/atipicial_types/url_session.rs`** (1 symbols): `URLSession`

**`src/atipicial_types/util.rs`** (19 symbols): `parse_string_u64`, `parse_string_u256`, `parse_address`, `encode_string_h160`, `parse_string_h160`, `parse_string_h256`, `encode_string_h256`, `encode_string_u256`, `encode_vec_string_vec_u256`, `parse_vec_string_vec_u256`, `h256_to_u256`, `bytes_to_string`, `string_to_bytes`, `u256_sqrt`, `u256_min`, `vec_to_array32`, `var_size`, `ToBase58`, `ToBase64`

**`src/atipicial_types/vm_state.rs`** (1 symbols): `VMState`

**`src/atipicial_types/whitelisted_contract.rs`** (3 symbols): `WhitelistedContract`, `new`, `from_stack_item`

**`src/atipicial_types/contract/aef_file.rs`** (9 symbols): `AefFile`, `HEADER_SIZE`, `new`, `deserialize`, `try_encode`, `try_to_array`, `MethodToken`, `try_encode`, `try_to_array`

**`src/atipicial_types/contract/aep17contract.rs`** (2 symbols): `Aep17Contract`, `new`

**`src/atipicial_types/contract/contract_aef.rs`** (4 symbols): `ContractAef`, `new`, `get_first_token`, `get_token`

**`src/atipicial_types/contract/contract_manifest.rs`** (20 symbols): `ContractManifest`, `new`, `get_supported_standard`, `get_first_supported_standard`, `get_permission`, `get_first_permission`, `get_first_trust`, `get_trust`, `ContractGroup`, `ContractABI`, `new`, `get_first_method`, `get_method`, `get_first_event`, `get_event`, `ContractMethod`, `new`, `ContractEvent`, `ContractPermission`, `new`

**`src/atipicial_types/contract/contract_method_token.rs`** (2 symbols): `ContractMethodToken`, `new`

**`src/atipicial_types/contract/contract_parameter.rs`** (50 symbols): `ContractParameter2`, `new`, `ContractParameter`, `try_from_aef_file`, `from_json`, `as_public_key`, `as_signature`, `as_bytes`, `as_string`, `ParameterValue`, `new`, `get_type`, `with_value`, `bool`, `to_bool`, `integer`, `to_integer`, `byte_array`, `to_byte_array`, `string`, `to_string`, `h160`, `to_h160`, `h256`, `to_h256`, `public_key`, `to_public_key`, `signature`, `to_signature`, `array`, `to_array`, `map`, `to_map`, `any`, `hash`, `ContractParameterMap`, `new`, `from_map`, `to_map`, `to_bool`, `to_integer`, `to_byte_array`, `to_string`, `to_h160`, `to_h256`, `to_public_key`, `to_signature`, `to_array`, `to_map`, `hash`

**`src/atipicial_types/contract/contract_parameter_type.rs`** (1 symbols): `ContractParameterType`

**`src/atipicial_types/contract/contract_state.rs`** (5 symbols): `ContractState`, `new`, `contract_identifiers`, `ContractIdentifiers`, `from_invocation_result`

**`src/atipicial_types/contract/contract_storage_entry.rs`** (2 symbols): `ContractStorageEntry`, `new`

**`src/atipicial_types/contract/invocation_result.rs`** (19 symbols): `InvocationResult`, `AtipicialVMStateType`, `new`, `has_state_fault`, `get_first_stack_item`, `get_stack_item`, `get_first_notification`, `get_notification`, `PendingSignature`, `Item`, `Diagnostics`, `new`, `InvokedContract`, `new`, `new_hash`, `StorageChange`, `new`, `Notification`, `NotificationState`

**`src/atipicial_types/contract/native_contract_state.rs`** (2 symbols): `NativeContractState`, `new`

**`src/atipicial_types/nns/nns_name.rs`** (8 symbols): `NNSName`, `new`, `is_valid`, `validate`, `bytes`, `is_second_level_domain`, `NNSRoot`, `new`

**`src/atipicial_types/nns/record_state.rs`** (3 symbols): `RecordState`, `new`, `from_stack_item`

**`src/atipicial_types/nns/record_type.rs`** (2 symbols): `RecordType`, `byte_repr`

**`src/atipicial_utils/error.rs`** (3 symbols): `option_to_result`, `with_context`, `result_to_option`

**`src/atipicial_utils/mod.rs`** (1 symbols): `error`

**`src/atipicial_wallets/bip39_account.rs`** (5 symbols): `Bip39Account`, `mnemonic`, `account`, `create`, `from_bip39_mnemonic`

**`src/atipicial_wallets/error.rs`** (1 symbols): `SignerError`

**`src/atipicial_wallets/ledger.rs`** (3 symbols): `HDPath`, `to_vec`, `LedgerWallet`

**`src/atipicial_wallets/mod.rs`** (3 symbols): `wallet`, `LocalWallet`, `YubiWallet`

**`src/atipicial_wallets/wallet_signer.rs`** (7 symbols): `WalletSigner`, `new_with_signer`, `sign_hash`, `signer`, `address`, `network`, `with_network`

**`src/atipicial_wallets/wallet_trait.rs`** (1 symbols): `WalletTrait`

**`src/atipicial_wallets/yubi.rs`** (3 symbols): `connect`, `new`, `from_key`

**`src/atipicial_wallets/wallet/aep6account.rs`** (4 symbols): `AEP6Account`, `new`, `from_account`, `to_account`

**`src/atipicial_wallets/wallet/aep6contract.rs`** (2 symbols): `AEP6Contract`, `AEP6Parameter`

**`src/atipicial_wallets/wallet/aep6wallet.rs`** (2 symbols): `Aep6Wallet`, `new`

**`src/atipicial_wallets/wallet/backup.rs`** (3 symbols): `WalletBackup`, `backup`, `recover`

**`src/atipicial_wallets/wallet/wallet.rs`** (30 symbols): `Wallet`, `DEFAULT_WALLET_NAME`, `CURRENT_VERSION`, `new`, `try_new`, `to_aep6`, `from_aep6`, `from_account`, `from_accounts`, `save_to_file`, `get_account`, `remove_account`, `encrypt_accounts`, `encrypt_accounts_parallel`, `encrypt_accounts_parallel_with_threads`, `encrypt_accounts_batch_parallel`, `create`, `open`, `get_accounts`, `create_account`, `import_private_key`, `verify_password`, `change_password`, `change_password_parallel`, `create_wallet`, `open_wallet`, `get_all_accounts`, `create_new_account`, `import_from_wif`, `with_network`

**`src/atipicial_wallets/wallet/wallet_error.rs`** (1 symbols): `WalletError`

**`src/atipicial_x/mod.rs`** (2 symbols): `bridge`, `evm`

**`src/atipicial_x/bridge/bridge_contract.rs`** (8 symbols): `AtipicialXBridgeContract`, `CONTRACT_HASH`, `DEPOSIT`, `WITHDRAW`, `GET_FEE`, `GET_CAP`, `new`, `with_script_hash`

**`src/atipicial_x/bridge/evm_bridge.rs`** (4 symbols): `AtipicialXBridgeContractEVM`, `new`, `default_bridge`, `address`

**`src/atipicial_x/bridge/mod.rs`** (2 symbols): `bridge_contract`, `evm_bridge`

**`src/atipicial_x/evm/mod.rs`** (3 symbols): `provider`, `transaction`, `wallet`

**`src/atipicial_x/evm/provider.rs`** (7 symbols): `ATC_X_MAINNET_MEV_RPC`, `AtipicialXProvider`, `new`, `new_anti_mev`, `rpc_url`, `set_rpc_url`, `evm_provider`

**`src/atipicial_x/evm/transaction.rs`** (9 symbols): `AtipicialXTransaction`, `new`, `to`, `data`, `value`, `gas_limit`, `gas_price`, `into_alloy_request`, `build_alloy_request`

**`src/atipicial_x/evm/wallet.rs`** (7 symbols): `AtipicialXWallet`, `from_private_key`, `create_random`, `address`, `inner_wallet`, `AtipicialXClient`, `new`

**`src/constants/mod.rs`** (1 symbols): `native_contracts`

**`src/constants/native_contracts.rs`** (10 symbols): `CONTRACT_MANAGEMENT`, `STD_LIB`, `CRYPTO_LIB`, `LEDGER`, `ATC_TOKEN`, `GAS_TOKEN`, `POLICY`, `ROLE_MANAGEMENT`, `ORACLE`, `NAME_SERVICE`

**`src/monitoring/health.rs`** (12 symbols): `HealthStatus`, `HealthCheck`, `HealthRegistry`, `register`, `update`, `overall_status`, `get_all`, `HealthResponse`, `init`, `update_health`, `register_health_check`, `shutdown`

**`src/monitoring/metrics.rs`** (11 symbols): `MetricsSnapshot`, `init`, `increment_counter`, `set_gauge`, `observe_histogram`, `snapshot`, `record_transaction`, `record_rpc_request`, `update_blockchain_metrics`, `record_contract_invocation`, `shutdown`

**`src/monitoring/mod.rs`** (17 symbols): `health`, `metrics`, `tracing`, `MonitoringConfig`, `builder`, `MonitoringConfigBuilder`, `metrics_enabled`, `metrics_port`, `tracing_enabled`, `tracing_endpoint`, `log_level`, `health_check_enabled`, `health_check_port`, `build`, `from_env`, `init`, `shutdown`

**`src/monitoring/tracing.rs`** (5 symbols): `init`, `add_event`, `set_status`, `record_error`, `shutdown`

**`src/sdk/hd_wallet.rs`** (20 symbols): `DerivationPath`, `new_atipicial`, `from_string`, `HDWallet`, `generate`, `from_mnemonic`, `from_phrase`, `mnemonic_phrase`, `derive_account`, `derive_accounts`, `get_default_account`, `export_encrypted`, `import_encrypted`, `HDWalletBuilder`, `new`, `word_count`, `passphrase`, `language`, `mnemonic`, `build`

**`src/sdk/mod.rs`** (45 symbols): `hd_wallet`, `transaction_simulator`, `unified`, `websocket`, `Atipicial`, `Network`, `SdkConfig`, `builder`, `SdkConfigBuilder`, `timeout`, `retries`, `cache_enabled`, `metrics_enabled`, `build`, `DecimalAmount`, `DecimalAmountParseError`, `try_from_raw`, `from_raw`, `parse`, `raw`, `decimals`, `to_fixed_string`, `raw_i64`, `Balance`, `TokenBalance`, `TxHash`, `Token`, `ATC_HASH`, `GAS_HASH`, `contract_hash`, `builder`, `client`, `endpoint`, `network`, `AtipicialBuilder`, `network`, `endpoint`, `config`, `timeout`, `retries`, `cache`, `metrics`, `Transfer`, `new`, `with_memo`

**`src/sdk/transaction_simulator.rs`** (22 symbols): `SimulationResult`, `StateChanges`, `StorageChange`, `BalanceChange`, `TokenTransfer`, `ContractDeployment`, `ContractUpdate`, `Notification`, `SimulationWarning`, `WarningLevel`, `OptimizationSuggestion`, `OptimizationType`, `TransactionSimulator`, `new`, `OptimizationRule`, `GasEstimate`, `TransactionSimulatorBuilder`, `new`, `client`, `cache_duration`, `add_optimization_rule`, `build`

**`src/sdk/unified.rs`** (4 symbols): `EcosystemClient`, `new_n3`, `new_atipicialx`, `new_atipicialx_anti_mev`

**`src/sdk/websocket.rs`** (13 symbols): `SubscriptionType`, `EventData`, `SubscriptionHandle`, `id`, `subscription_type`, `cancel`, `WebSocketClient`, `take_event_receiver`, `set_reconnect_params`, `WebSocketClientBuilder`, `new`, `reconnect_interval`, `max_reconnect_attempts`

**`tests/common/mod.rs`** (4 symbols): `test_utils`, `mock_provider`, `test_accounts`, `assertions`

**`tests/common/test_utils.rs`** (8 symbols): `TEST_TIMEOUT_SECS`, `TESTNET_ENDPOINT`, `MAINNET_ENDPOINT`, `create_test_client`, `create_test_client_with_timeout`, `generate_test_key`, `create_test_account`, `is_ci`


---

## 🔢 Protocol Constants — The Numbers That Govern

**163 public constants define this project's behavior.**

| Constant | Value | Defined in |
|---|---|---|
| `VERSION` | `env!("CARGO_PKG_VERSION")` | `src/lib.rs` |
| `MAX_RESULT_SIZE` | `0xffff` | `src/atipicial_builder/transaction/transaction_attribute.rs` |
| `BALANCE_OF_FUNCTION` | `"balanceOf"` | `src/atipicial_builder/transaction/transaction_builder.rs` |
| `DUMMY_PUB_KEY` | `"02ec143f00b88524caf36a0121c2de09eef0519ddbe1c710a00f0e26632` | `src/atipicial_builder/transaction/transaction_builder.rs` |
| `JWT_SECRET_LENGTH` | `32` | `src/atipicial_clients/rpc/transports/common.rs` |
| `DEFAULT_BLOCK_TIME` | `15_000` | `src/atipicial_config/config.rs` |
| `DEFAULT_ADDRESS_VERSION` | `0x35` | `src/atipicial_config/config.rs` |
| `MAX_VALID_UNTIL_BLOCK_INCREMENT_BASE` | `86_400_000` | `src/atipicial_config/config.rs` |
| `MAGIC_NUMBER_MAINNET` | `860833102` | `src/atipicial_config/constant.rs` |
| `MAGIC_NUMBER_TESTNET` | `894710606` | `src/atipicial_config/constant.rs` |
| `MAX_PUBLIC_KEYS_PER_MULTI_SIG` | `1024` | `src/atipicial_config/constant.rs` |
| `HASH160_SIZE` | `20` | `src/atipicial_config/constant.rs` |
| `HASH256_SIZE` | `32` | `src/atipicial_config/constant.rs` |
| `PRIVATE_KEY_SIZE` | `32` | `src/atipicial_config/constant.rs` |
| `PUBLIC_KEY_SIZE_COMPRESSED` | `33` | `src/atipicial_config/constant.rs` |
| `SIGNATURE_SIZE` | `64` | `src/atipicial_config/constant.rs` |
| `VERIFICATION_SCRIPT_SIZE` | `40` | `src/atipicial_config/constant.rs` |
| `MAX_ITERATOR_ITEMS_DEFAULT` | `100` | `src/atipicial_config/constant.rs` |
| `MAX_SUBITEMS` | `16` | `src/atipicial_config/constant.rs` |
| `MAX_NESTING_DEPTH` | `2` | `src/atipicial_config/constant.rs` |
| `CURRENT_TX_VERSION` | `0` | `src/atipicial_config/constant.rs` |
| `MAX_TRANSACTION_SIZE` | `102400` | `src/atipicial_config/constant.rs` |
| `MAX_TRANSACTION_ATTRIBUTES` | `16` | `src/atipicial_config/constant.rs` |
| `MAX_SIGNER_SUBITEMS` | `16` | `src/atipicial_config/constant.rs` |
| `MAX_MANIFEST_SIZE` | `0xFFFF` | `src/atipicial_config/constant.rs` |
| `MAX_RPC_MESSAGE_SIZE` | `16 * 1024 * 1024` | `src/atipicial_config/constant.rs` |
| `SEED_1` | `"http://seed1.atipicial.com:10332"` | `src/atipicial_config/constant.rs` |
| `SEED_2` | `"http://seed2.atipicial.com:10332"` | `src/atipicial_config/constant.rs` |
| `SEED_3` | `"http://seed3.atipicial.com:10332"` | `src/atipicial_config/constant.rs` |
| `SEED_4` | `"http://seed4.atipicial.com:10332"` | `src/atipicial_config/constant.rs` |
| `SEED_5` | `"http://seed5.atipicial.com:10332"` | `src/atipicial_config/constant.rs` |
| `SCRYPT_N` | `16384` | `src/atipicial_config/constant.rs` |
| `SCRYPT_R` | `8` | `src/atipicial_config/constant.rs` |
| `SCRYPT_P` | `8` | `src/atipicial_config/constant.rs` |
| `SCRYPT_LOG_N` | `14` | `src/atipicial_config/constant.rs` |
| `SCRYPT_DK_LEN` | `64` | `src/atipicial_config/constant.rs` |
| `AEP_HEADER_1` | `0x01` | `src/atipicial_config/constant.rs` |
| `AEP_HEADER_2` | `0x42` | `src/atipicial_config/constant.rs` |
| `AEP_FLAG` | `0xe0` | `src/atipicial_config/constant.rs` |
| `TEST_RESOURCE_PATH` | `"../../../test_resources/"` | `src/atipicial_config/test_properties.rs` |
| `DEFAULT_ACCOUNT_ADDRESS` | `"NM7Aky765FG8NhhwtxjXRx7jEL1cnw7PBP"` | `src/atipicial_config/test_properties.rs` |
| `DEFAULT_ACCOUNT_SCRIPT_HASH` | `"69ecca587293047be4c59159bf8bc399985c160d"` | `src/atipicial_config/test_properties.rs` |
| `DEFAULT_ACCOUNT_VERIFICATION_SCRIPT` | `"0c21033a4d051b04b7fc0230d2b1aaedfd5a84be279a5361a7358db665a` | `src/atipicial_config/test_properties.rs` |
| `DEFAULT_ACCOUNT_PUBLIC_KEY` | `"033a4d051b04b7fc0230d2b1aaedfd5a84be279a5361a7358db665ad785` | `src/atipicial_config/test_properties.rs` |
| `DEFAULT_ACCOUNT_PRIVATE_KEY` | `"84180ac9d6eb6fba207ea4ef9d2200102d1ebeb4b9c07e2c6a738a42742` | `src/atipicial_config/test_properties.rs` |
| `DEFAULT_ACCOUNT_ENCRYPTED_PRIVATE_KEY` | `"6PYM7jHL4GmS8Aw2iEFpuaHTCUKjhT4mwVqdoozGU6sUE25BjV4ePXDdLz"` | `src/atipicial_config/test_properties.rs` |
| `DEFAULT_ACCOUNT_WIF` | `"L1eV34wPoj9weqhGijdDLtVQzUpWGHszXXpdU9dPuh2nRFFzFa7E"` | `src/atipicial_config/test_properties.rs` |
| `DEFAULT_ACCOUNT_PASSWORD` | `"atipicial"` | `src/atipicial_config/test_properties.rs` |
| `COMMITTEE_ACCOUNT_ADDRESS` | `"NXXazKH39yNFWWZF5MJ8tEN98VYHwzn7g3"` | `src/atipicial_config/test_properties.rs` |
| `COMMITTEE_ACCOUNT_SCRIPT_HASH` | `"05859de95ccbbd5668e0f055b208273634d4657f"` | `src/atipicial_config/test_properties.rs` |
| `COMMITTEE_ACCOUNT_VERIFICATION_SCRIPT` | `"110c21033a4d051b04b7fc0230d2b1aaedfd5a84be279a5361a7358db66` | `src/atipicial_config/test_properties.rs` |
| `CONTRACT_MANAGEMENT_HASH` | `"fffdc93764dbaddd97c48f252a53ea4643faa3fd"` | `src/atipicial_config/test_properties.rs` |
| `STD_LIB_HASH` | `"acce6fd80d44e1796aa0c2c625e9e4e0ce39efc0"` | `src/atipicial_config/test_properties.rs` |
| `CRYPTO_LIB_HASH` | `"726cb6e0cd8628a1350a611384688911ab75f51b"` | `src/atipicial_config/test_properties.rs` |
| `LEDGER_CONTRACT_HASH` | `"da65b600f7124ce6c79950c1772a36403104f2be"` | `src/atipicial_config/test_properties.rs` |
| `ATC_TOKEN_HASH` | `"ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5"` | `src/atipicial_config/test_properties.rs` |
| `GAS_TOKEN_HASH` | `"d2a4cff31913016155e38e474a2c06d08be276cf"` | `src/atipicial_config/test_properties.rs` |
| `GAS_TOKEN_NAME` | `"GasToken"` | `src/atipicial_config/test_properties.rs` |
| `POLICY_CONTRACT_HASH` | `"cc5e4edd9f5f8dba8bb65734541df7a1c081c67b"` | `src/atipicial_config/test_properties.rs` |
| `ROLE_MANAGEMENT_HASH` | `"49cf4e5378ffcd4dec034fd98a174c5491e395e2"` | `src/atipicial_config/test_properties.rs` |
| `ORACLE_CONTRACT_HASH` | `"fe924b7cfe89ddd271abaf7210a80a7e11178758"` | `src/atipicial_config/test_properties.rs` |
| `NAME_SERVICE_HASH` | `"7a8fcf0392cd625647907afa8e45cc66872b596b"` | `src/atipicial_config/test_properties.rs` |
| `CLIENT1_ACCOUNT_WIF` | `"L3cNMQUSrvUrHx1MzacwHiUeCWzqK2MLt5fPvJj9mz6L2rzYZpok"` | `src/atipicial_config/test_properties.rs` |
| `NAME` | `"AtipicialCoin"` | `src/atipicial_contract/atipicial_token.rs` |
| `DECIMALS` | `0` | `src/atipicial_contract/atipicial_token.rs` |
| `SYMBOL` | `"ATC"` | `src/atipicial_contract/atipicial_token.rs` |
| `TOTAL_SUPPLY` | `100_000_000` | `src/atipicial_contract/atipicial_token.rs` |
| `NAME` | `"ContractManagement"` | `src/atipicial_contract/contract_management.rs` |
| `NAME` | `"GasToken"` | `src/atipicial_contract/gas_token.rs` |
| `DECIMALS` | `8` | `src/atipicial_contract/gas_token.rs` |
| `SYMBOL` | `"GAS"` | `src/atipicial_contract/gas_token.rs` |
| `NAME` | `"NameService"` | `src/atipicial_contract/name_service.rs` |
| `NAME` | `"Notary"` | `src/atipicial_contract/notary.rs` |
| `DEFAULT_MAX_NOT_VALID_BEFORE_DELTA` | `140` | `src/atipicial_contract/notary.rs` |
| `DEFAULT_DEPOSIT_DELTA_TILL` | `5760` | `src/atipicial_contract/notary.rs` |
| `NAME` | `"PolicyContract"` | `src/atipicial_contract/policy_contract.rs` |
| `DEFAULT_EXEC_FEE_FACTOR` | `30` | `src/atipicial_contract/policy_contract.rs` |
| `DEFAULT_STORAGE_PRICE` | `100000` | `src/atipicial_contract/policy_contract.rs` |
| `DEFAULT_FEE_PER_BYTE` | `1000` | `src/atipicial_contract/policy_contract.rs` |
| `DEFAULT_ATTRIBUTE_FEE` | `0` | `src/atipicial_contract/policy_contract.rs` |
| `DEFAULT_NOTARY_ASSISTED_ATTRIBUTE_FEE` | `10_000_000` | `src/atipicial_contract/policy_contract.rs` |
| `MAX_EXEC_FEE_FACTOR` | `100` | `src/atipicial_contract/policy_contract.rs` |
| `MAX_ATTRIBUTE_FEE` | `10_0000_0000` | `src/atipicial_contract/policy_contract.rs` |
| `MAX_STORAGE_PRICE` | `10_000_000` | `src/atipicial_contract/policy_contract.rs` |
| `MAX_MILLISECONDS_PER_BLOCK` | `30_000` | `src/atipicial_contract/policy_contract.rs` |
| `MAX_MAX_VALID_UNTIL_BLOCK_INCREMENT` | `86400` | `src/atipicial_contract/policy_contract.rs` |
| `MAX_MAX_TRACEABLE_BLOCKS` | `2_102_400` | `src/atipicial_contract/policy_contract.rs` |
| `NAME` | `"RoleManagement"` | `src/atipicial_contract/role_management.rs` |
| `NAME` | `"Treasury"` | `src/atipicial_contract/treasury.rs` |
| `CONTRACT_HASH` | `"48c40d4666f93408be1bef038b6722404f5c4a5a"` | `src/atipicial_contract/famous/atipicialburger.rs` |
| `SYMBOL` | `"bATC"` | `src/atipicial_contract/famous/atipicialburger.rs` |
| `DECIMALS` | `8` | `src/atipicial_contract/famous/atipicialburger.rs` |
| `WRAP` | `"wrap"` | `src/atipicial_contract/famous/atipicialburger.rs` |
| `UNWRAP` | `"unwrap"` | `src/atipicial_contract/famous/atipicialburger.rs` |
| `CLAIM_GAS` | `"claimGas"` | `src/atipicial_contract/famous/atipicialburger.rs` |
| `GET_RATE` | `"getRate"` | `src/atipicial_contract/famous/atipicialburger.rs` |
| `CONTRACT_HASH` | `"f0151f528127558851b39c2cd8aa47da7418ab28"` | `src/atipicial_contract/famous/atipicialcompound.rs` |
| `DEPOSIT` | `"deposit"` | `src/atipicial_contract/famous/atipicialcompound.rs` |
| `WITHDRAW` | `"withdraw"` | `src/atipicial_contract/famous/atipicialcompound.rs` |
| `COMPOUND` | `"compound"` | `src/atipicial_contract/famous/atipicialcompound.rs` |
| `GET_APY` | `"getAPY"` | `src/atipicial_contract/famous/atipicialcompound.rs` |
| `CONTRACT_HASH` | `"f970f4cddcd087ab5d8a5697a32b3cfd32c8b465"` | `src/atipicial_contract/famous/flamingo.rs` |
| `SWAP` | `"swap"` | `src/atipicial_contract/famous/flamingo.rs` |
| `ADD_LIQUIDITY` | `"addLiquidity"` | `src/atipicial_contract/famous/flamingo.rs` |
| `REMOVE_LIQUIDITY` | `"removeLiquidity"` | `src/atipicial_contract/famous/flamingo.rs` |
| `STAKE` | `"stake"` | `src/atipicial_contract/famous/flamingo.rs` |
| `CLAIM_REWARDS` | `"claimRewards"` | `src/atipicial_contract/famous/flamingo.rs` |
| `CONTRACT_HASH` | `"74f2dc36a68fdc4682034178eb2220729231db76"` | `src/atipicial_contract/famous/grandshare.rs` |
| `SUBMIT_PROPOSAL` | `"submitProposal"` | `src/atipicial_contract/famous/grandshare.rs` |
| `VOTE` | `"vote"` | `src/atipicial_contract/famous/grandshare.rs` |
| `FUND_PROJECT` | `"fundProject"` | `src/atipicial_contract/famous/grandshare.rs` |
| `CLAIM_FUNDS` | `"claimFunds"` | `src/atipicial_contract/famous/grandshare.rs` |
| `DEFAULT_MAINNET_ENDPOINT` | `"grpc.mainnet.fs.atipicial.com:8082"` | `src/atipicial_fs/mod.rs` |
| `DEFAULT_TESTNET_ENDPOINT` | `"grpc.testnet.fs.atipicial.com:8082"` | `src/atipicial_fs/mod.rs` |
| `DEFAULT_ENDPOINT` | `DEFAULT_MAINNET_ENDPOINT` | `src/atipicial_fs/mod.rs` |
| `DEFAULT_MAINNET_HTTP_GATEWAY` | `"https://http.mainnet.fs.atipicial.com"` | `src/atipicial_fs/mod.rs` |
| `DEFAULT_TESTNET_HTTP_GATEWAY` | `"https://http.testnet.fs.atipicial.com"` | `src/atipicial_fs/mod.rs` |
| `DEFAULT_MAINNET_REST_API` | `"https://rest.mainnet.fs.atipicial.com"` | `src/atipicial_fs/mod.rs` |
| `DEFAULT_TESTNET_REST_API` | `"https://rest.testnet.fs.atipicial.com"` | `src/atipicial_fs/mod.rs` |
| `ANY_VALUE` | `"Any"` | `src/atipicial_types/stack_item.rs` |
| `POINTER_VALUE` | `"Pointer"` | `src/atipicial_types/stack_item.rs` |
| `BOOLEAN_VALUE` | `"Boolean"` | `src/atipicial_types/stack_item.rs` |
| `INTEGER_VALUE` | `"Integer"` | `src/atipicial_types/stack_item.rs` |
| `BYTE_STRING_VALUE` | `"ByteString"` | `src/atipicial_types/stack_item.rs` |
| `BUFFER_VALUE` | `"Buffer"` | `src/atipicial_types/stack_item.rs` |
| `ARRAY_VALUE` | `"Array"` | `src/atipicial_types/stack_item.rs` |
| `STRUCT_VALUE` | `"Struct"` | `src/atipicial_types/stack_item.rs` |
| `MAP_VALUE` | `"Map"` | `src/atipicial_types/stack_item.rs` |
| `INTEROP_INTERFACE_VALUE` | `"InteropInterface"` | `src/atipicial_types/stack_item.rs` |
| `ANY_BYTE` | `0x00` | `src/atipicial_types/stack_item.rs` |
| `POINTER_BYTE` | `0x10` | `src/atipicial_types/stack_item.rs` |
| `BOOLEAN_BYTE` | `0x20` | `src/atipicial_types/stack_item.rs` |
| `INTEGER_BYTE` | `0x21` | `src/atipicial_types/stack_item.rs` |
| `BYTE_STRING_BYTE` | `0x28` | `src/atipicial_types/stack_item.rs` |
| `BUFFER_BYTE` | `0x30` | `src/atipicial_types/stack_item.rs` |
| `ARRAY_BYTE` | `0x40` | `src/atipicial_types/stack_item.rs` |
| `STRUCT_BYTE` | `0x41` | `src/atipicial_types/stack_item.rs` |
| `MAP_BYTE` | `0x48` | `src/atipicial_types/stack_item.rs` |
| `INTEROP_INTERFACE_BYTE` | `0x60` | `src/atipicial_types/stack_item.rs` |
| `HEADER_SIZE` | `Self::MAGIC_SIZE + Self::COMPILER_SIZE` | `src/atipicial_types/contract/aef_file.rs` |
| `DEFAULT_WALLET_NAME` | `"AtipicialWallet"` | `src/atipicial_wallets/wallet/wallet.rs` |
| `CURRENT_VERSION` | `"1.0"` | `src/atipicial_wallets/wallet/wallet.rs` |
| `CONTRACT_HASH` | `"74f2dc36a68fdc4682034178eb2220729231db76"` | `src/atipicial_x/bridge/bridge_contract.rs` |
| `DEPOSIT` | `"deposit"` | `src/atipicial_x/bridge/bridge_contract.rs` |
| `WITHDRAW` | `"withdraw"` | `src/atipicial_x/bridge/bridge_contract.rs` |
| `GET_FEE` | `"getFee"` | `src/atipicial_x/bridge/bridge_contract.rs` |
| `GET_CAP` | `"getCap"` | `src/atipicial_x/bridge/bridge_contract.rs` |
| `ATC_X_MAINNET_MEV_RPC` | `"https://mainnet-1.rpc.banelabs.org"` | `src/atipicial_x/evm/provider.rs` |
| `CONTRACT_MANAGEMENT` | `"0xfffdc93764dbaddd97c48f252a53ea4643faa3fd"` | `src/constants/native_contracts.rs` |
| `STD_LIB` | `"0xacce6fd80d44e1796aa0c2c625e9e4e0ce39efc0"` | `src/constants/native_contracts.rs` |
| `CRYPTO_LIB` | `"0x726cb6e0cd8628a1350a611384688911ab75f51b"` | `src/constants/native_contracts.rs` |
| `LEDGER` | `"0xda65b600f7124ce6c79950c1772a36403104f2be"` | `src/constants/native_contracts.rs` |
| `ATC_TOKEN` | `"0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5"` | `src/constants/native_contracts.rs` |
| `GAS_TOKEN` | `"0xd2a4cff31913016155e38e474a2c06d08be276cf"` | `src/constants/native_contracts.rs` |
| `POLICY` | `"0xcc5e4edd9f5f8dba8bb65734541df7a1c081c67b"` | `src/constants/native_contracts.rs` |
| `ROLE_MANAGEMENT` | `"0x49cf4e5378ffcd4dec034fd98a174c5491e395e2"` | `src/constants/native_contracts.rs` |
| `ORACLE` | `"0xfe924b7cfe89ddd271abaf7210a80a7e11178758"` | `src/constants/native_contracts.rs` |
| `NAME_SERVICE` | `"0x7a8fcf0392cd625647907afa8e45cc66872b596b"` | `src/constants/native_contracts.rs` |
| `ATC_HASH` | `hex!("ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5")` | `src/sdk/mod.rs` |
| `GAS_HASH` | `hex!("d2a4cff31913016155e38e474a2c06d08be276cf")` | `src/sdk/mod.rs` |
| `TEST_TIMEOUT_SECS` | `30` | `tests/common/test_utils.rs` |
| `TESTNET_ENDPOINT` | `"https://testnet1.atipicial.com:443"` | `tests/common/test_utils.rs` |
| `MAINNET_ENDPOINT` | `"https://mainnet1.atipicial.com:443"` | `tests/common/test_utils.rs` |

---

## 🧪 Test Inventory — Proof, Not Promises

**563 test functions across 102 files.**

**`atipicial-cli/src/generator.rs`**: `test_embedded_templates_parse`, `test_project_generation`, `test_template_variable_replacement`

**`atipicial-cli/src/main.rs`**: `cli_definition_is_valid`

**`atipicial-cli/src/commands/fs.rs`**: `with_endpoint_uses_testnet_endpoints`, `with_endpoint_propagates_http_client_creation_error`, `it ://`

**`atipicial-cli/src/commands/tools.rs`**: `base58_known_vectors`, `base58_preserves_leading_zeroes`, `base58_rejects_invalid_characters`

**`atipicial-cli/src/monitoring/logger.rs`**: `test_logger_initialization`, `test_structured_logger`, `test_performance_logger`

**`atipicial-cli/src/monitoring/metrics.rs`**: `test_metrics_registry`, `test_prometheus_export`

**`atipicial-cli/src/security/error_handler.rs`**: `test_circuit_breaker`

**`atipicial-cli/src/security/keychain.rs`**: `test_keychain_manager_creation`, `test_secure_wallet_storage`

**`atipicial-cli/src/security/network_failover.rs`**: `test_endpoint_health_scoring`

**`atipicial-cli/src/security/session.rs`**: `test_session_creation`, `test_session_validation`, `test_concurrent_session_limit`

**`atipicial-cli/tests/integration/blockchain_tests.rs`**: `test_blockchain_info`, `test_blockchain_height`, `test_blockchain_get_block_by_index`, `test_blockchain_get_block_by_hash`, `test_blockchain_get_asset`

**`atipicial-cli/tests/integration/contract_tests.rs`**: `test_contract_info`, `test_contract_test_invoke`, `test_contract_deploy`

**`atipicial-cli/tests/integration/defi_tests.rs`**: `test_defi_pools`, `test_defi_swap_info`, `test_defi_swap`, `test_defi_add_liquidity`, `test_defi_remove_liquidity`, `test_defi_token_info`, `test_defi_token_info_gas`, `test_defi_balance_with_wallet`, `test_defi_invoke_test`, `test_defi_balance`, `test_defi_invalid_token`

**`atipicial-cli/tests/integration/fs_tests.rs`**: `test_fs_endpoints_list`, `test_fs_status_no_wallet`, `test_fs_endpoints_test`, `test_fs_container_operations`, `test_fs_object_operations`, `test_fs_acl_operations`

**`atipicial-cli/tests/integration/generate_tests.rs`**: `test_generate_list_templates`

**`atipicial-cli/tests/integration/init_tests.rs`**: `test_init_default`, `test_init_custom_path`

**`atipicial-cli/tests/integration/network_tests.rs`**: `test_network_status`, `test_network_nodes`, `test_network_switch`, `test_network_add_node`, `test_network_set_default`, `test_network_ping`

**`atipicial-cli/tests/integration/wallet_tests.rs`**: `test_wallet_create`, `test_wallet_open_and_close`, `test_wallet_list_address`

**`examples/basic/02_create_wallet.rs`**: `test_wallet_creation`, `test_wif_roundtrip`, `test_address_validation`

**`examples/basic/03_check_balance.rs`**: `test_gas_formatting`, `test_address_validation`

**`examples/basic/04_send_transaction.rs`**: `test_wallet_generation`, `test_gas_calculations`, `test_address_validation`

**`examples/big-numbers/examples/conversion.rs`**: `it .`

**`examples/queries/examples/uniswapv2_pair.rs`**: `test_format_number`, `test_price_calculation`

**`src/atipicial_builder/utils.rs`**: `test_try_public_keys_to_scripthash_rejects_zero_threshold`, `test_try_public_keys_to_scripthash_rejects_threshold_above_len`, `test_transaction_attribute_to_value_is_structured_json`, `test_transaction_send_token_to_value_is_structured_json`, `test_signer_to_value_is_structured_json`

**`src/atipicial_builder/script/script_builder.rs`**: `test_push_empty_array`, `test_push_byte_array`, `test_push_string`, `test_push_integer`, `test_try_push_integer_rejects_values_larger_than_32_bytes`, `test_try_push_integer_matches_legacy_encoding_for_supported_values`, `test_verification_script`, `test_map`, `test_map_nested`

**`src/atipicial_builder/script/script_reader.rs`**: `test_convert_to_op_code_string`

**`src/atipicial_builder/transaction/gas_estimator.rs`**: `test_calculate_estimation_accuracy`

**`src/atipicial_builder/transaction/invocation_script.rs`**: `test_from_message_and_key_pair`, `test_serialize_random_invocation_script`, `test_deserialize_custom_invocation_script`, `test_deserialize_signature_invocation_script`, `test_size`, `test_get_signatures`, `test_from_serialized_script_accepts_raw_script`, `test_try_to_array_rejects_oversized_script`

**`src/atipicial_builder/transaction/production_transaction_builder.rs`**: `test_fee_calculation`, `test_witness_generation`, `test_production_builder`

**`src/atipicial_builder/transaction/proptest_tests.rs`**: `prop_script_builder_opcodes`, `prop_script_builder_integers`, `prop_script_builder_strings`, `prop_transaction_attributes_preserved`, `prop_transaction_fees_non_negative`, `prop_witness_scope_combinations`, `prop_call_flags_combinations`

**`src/atipicial_builder/transaction/transaction.rs`**: `test_transaction_equality_distinguishes_hidden_signer_state`, `test_try_to_array_rejects_oversized_witness_invocation_script`, `test_try_to_array_rejects_invalid_oracle_response_attribute`, `test_try_to_array_rejects_signer_with_too_many_allowed_contracts`, `test_deserialize_rejects_missing_script_field`, `test_deserialize_rejects_non_string_script_field`, `test_tx_id_rejects_invalid_oracle_response_attribute`, `test_try_to_array_matches_legacy_for_valid_oracle_response_attribute`

**`src/atipicial_builder/transaction/transaction_attribute.rs`**: `test_try_to_json_matches_serde_json`, `test_try_to_bytes_rejects_invalid_oracle_response_base64`, `test_try_to_bytes_matches_legacy_for_valid_oracle_response`, `test_try_encode_rejects_invalid_oracle_response_base64`, `test_try_encode_matches_safe_bytes_for_valid_oracle_response`, `test_try_size_rejects_invalid_oracle_response_base64`, `test_try_size_matches_safe_bytes_len_for_valid_oracle_response`

**`src/atipicial_builder/transaction/verification_script.rs`**: `test_from_public_key`, `test_from_public_keys`, `test_serialize_deserialize`, `test_get_signing_threshold`, `test_invalid_script`, `test_is_single_sig_script`, `test_is_multi_sig`, `test_fail_is_multi_sig_too_short`, `test_fail_is_multi_sig_n_less_than_one`, `test_fail_is_multi_sig_abrupt_end`, `test_fail_is_multi_sig_wrong_push_data`, `test_fail_is_multi_sig_n_greater_than_m`, `test_fail_is_multi_sig_m_incorrect`, `test_fail_is_multi_sig_missing_push_null`, `test_fail_is_multi_sig_missing_syscall`, `test_fail_is_multi_sig_wrong_interop_service`, `test_public_keys_from_single_sig`, `test_get_signatures_returns_empty_for_verification_script`, `test_public_keys_from_multi_sig`, `test_try_to_array_rejects_oversized_script`

**`src/atipicial_builder/transaction/signers/signer.rs`**: `test_create_signer_with_call_by_entry_scope`, `test_create_signer_with_global_scope`, `test_build_valid_signer1`, `test_build_valid_signer2`, `test_build_valid_signer3`, `test_fail_building_signer_with_global_scope_and_custom_contracts`, `test_fail_building_signer_with_global_scope_and_custom_groups`, `test_fail_building_signer_too_many_contracts`, `test_fail_building_signer_too_many_contracts_added_separately`, `test_fail_building_signer_too_many_groups`, `test_fail_building_signer_too_many_groups_added_separately`, `test_account_signer_try_to_array_rejects_too_many_allowed_contracts`, `test_contract_signer_try_to_array_rejects_too_many_allowed_groups`, `test_transaction_signer_try_to_array_rejects_rule_with_too_many_expressions`, `test_transaction_signer_try_to_array_rejects_too_many_rules`, `test_serialize_global_scope`, `test_serialize_custom_contracts_scope_produces_correct_byte_array`, `test_serialize_custom_group_scope`, `test_serialize_multiple_scopes_contracts_groups_and_rules`, `test_fail_deserialize_too_many_contracts`, `test_fail_deserialize_too_many_contract_groups`, `test_fail_deserialize_too_many_rules`, `test_get_size`, `test_serialize_deserialize_max_nested_rules`, `test_fail_adding_rules_to_global_signer`, `test_fail_adding_too_many_rules`, `test_signer_equals`, `test_to_account_signer_accepts_account_variant`, `test_to_account_signer_rejects_other_variants`, `test_to_contract_signer_accepts_contract_variant`, `test_to_contract_signer_rejects_other_variants`, `test_try_to_transaction_signer_preserves_signer_data`, `test_try_to_transaction_signer_rejects_invalid_scopes`, `test_serialize_with_multiple_scopes_contracts_groups_and_rules`, `test_deserialize`

**`src/atipicial_builder/transaction/witness_rule/witness_rule.rs`**: `test_decode_boolean_condition`, `test_script_hash_condition_serialize_deserialize`, `test_decode_not_condition`, `test_and_condition_serialize_deserialize`, `test_not_condition_serialize_deserialize`, `test_boolean_nil_values`, `test_group_condition_invalid_key_rejected`, `test_decode_or_condition`, `test_called_by_group_condition_serialize_deserialize`, `test_called_by_entry_serialize_deserialize`, `test_called_by_contract_serialize_deserialize`, `test_decode_script_hash_condition`, `test_decode_group_condition`, `test_decode_called_by_entry_condition`, `test_decode_called_by_contract_condition`, `test_condition_try_to_array_rejects_too_many_expressions`, `test_witness_rule_try_to_array_rejects_invalid_condition`, `test_and_condition_decode`, `test_not_condition_decode`, `boolean_expression`

**`src/atipicial_clients/errors.rs`**: `deterministic_provider_errors_are_not_retryable`, `provider_preserves_unknown_transaction_classification`

**`src/atipicial_clients/mod.rs`**: `test_try_http_provider_from_endpoint_rejects_invalid_url`, `test_try_http_provider_from_endpoint_accepts_valid_url`

**`src/atipicial_clients/rate_limiter.rs`**: `test_builder`

**`src/atipicial_clients/utils.rs`**: `test_try_serialize_matches_serde_json_for_valid_value`, `test_try_serialize_returns_error_on_serialization_failure`

**`src/atipicial_clients/rpc/rpc_client.rs`**: `it /`

**`src/atipicial_clients/rpc/transports/common.rs`**: `classifies_transient_and_unknown_transaction_errors`, `extracts_non_negative_provider_backoff`, `json_rpc_error_display_redacts_provider_data`, `json_rpc_error_debug_redacts_provider_data`, `deser_response`, `ser_request`, `test_roundtrip`

**`src/atipicial_clients/rpc/transports/http_provider.rs`**: `provider_debug_redacts_url_credentials_and_paths`

**`src/atipicial_clients/rpc/transports/mock.rs`**: `test_json_partial_match_object_subset`, `test_json_partial_match_array_prefix`

**`src/atipicial_clients/rpc/transports/retry.rs`**: `can_measure_unit_offset_single_request`, `can_measure_unit_offset_1x_over_budget`, `can_measure_unit_offset_2x_over_budget`, `zero_compute_units_do_not_panic`, `exponential_backoff_doubles_and_caps`, `can_extract_backoff`, `test_alchemy_ip_rate_limit`, `test_rate_limit_omitted_id`

**`src/atipicial_clients/rpc/transports/ws/types.rs`**: `it_desers_pubsub_items`

**`src/atipicial_codec/binary_decoder.rs`**: `test_read_u16_is_little_endian`, `test_read_var_int_u16_is_little_endian`, `test_read_var_bytes_bounded_rejects_excess_length`, `test_read_var_string_bounded_rejects_excess_length`, `test_read_push_data_bytes`, `test_fail_read_push_data`, `test_read_push_data_string`, `test_read_push_data_big_integer`, `test_read_u32`, `test_read_i64`, `test_read_serializable_list_rejects_length_exceeding_remaining_bytes`, `test_read_serializable_list_bounded_rejects_excess_length`

**`src/atipicial_codec/binary_encoder.rs`**: `test_write_u32`, `test_write_i64`, `test_write_u16`, `test_write_var_int`, `test_write_var_bytes`, `test_write_var_string`

**`src/atipicial_contract/atipicial_uri.rs`**: `it &`

**`src/atipicial_contract/mod.rs`**: `checked_vm_integer_rejects_negative_and_oversized_values`

**`src/atipicial_contract/notary.rs`**: `test_notary_contract_name`, `test_notary_default_constants`, `test_notary_supported_standards`, `test_notary_contract_hash`, `test_notary_deposit_from_stack_item`, `notary_deposit_rejects_invalid_block_height`

**`src/atipicial_contract/policy_contract.rs`**: `test_policy_contract_constants`, `test_policy_contract_name`

**`src/atipicial_contract/tests.rs`**: `test_contract_parameter_creation`, `test_contract_parameter_value_extraction`, `test_contract_hash_validation`, `test_script_hash_extension`, `test_aef_file_creation`, `test_contract_manifest_creation`, `test_address_to_script_hash_conversion`, `test_op_code_enum`, `test_vm_state_enum`, `test_stack_item_creation`, `test_production_ready_script_building`

**`src/atipicial_contract/treasury.rs`**: `test_treasury_contract_name`, `test_treasury_supported_standards`, `test_treasury_contract_hash`

**`src/atipicial_contract/famous/contracts.rs`**: `test_get_famous_contracts`

**`src/atipicial_crypto/base58_helper.rs`**: `test_base58_encoding_for_valid_strings`, `test_base58_decoding_for_valid_strings`, `test_base58_decoding_for_invalid_strings`, `test_base58check_encoding`, `test_base58check_decoding`, `test_try_base58check_decode_reports_invalid_characters`, `test_try_base58check_decode_reports_invalid_checksum`, `test_try_base58check_decode_reports_missing_checksum_bytes`, `test_base58check_empty_roundtrip`, `test_base58check_decoding_with_invalid_characters`, `test_base58check_decoding_with_invalid_checksum`

**`src/atipicial_crypto/crypto_lib.rs`**: `test_sha3_512`, `test_sha3_512_empty`, `test_sha3_512_short_string`, `test_blake2b_512`, `test_blake2b_512_empty`, `test_blake2b_512_short_string`, `test_verify_with_ed25519_invalid_lengths`, `test_verify_with_ed25519_valid_signature`, `test_verify_with_ed25519_valid_signature_with_message`, `test_verify_with_ed25519_invalid_signature`, `test_recover_secp256k1_invalid_lengths`, `test_recover_secp256k1_valid_signature`, `test_recover_secp256k1_with_invalid_recovery_id`, `test_crypto_lib_hashable_trait`

**`src/atipicial_crypto/hash.rs`**: `test_hash256_for_bytes`, `test_hash256_for_string`, `test_ripemd160_for_bytes`, `test_ripemd160_for_string`, `test_sha256_ripemd160_for_bytes`, `test_sha256_ripemd160_for_string`, `test_hmac_sha512_for_bytes`, `test_hmac_sha512_for_string`, `test_hash160_for_string`, `test_ripemd160_test_vectors`

**`src/atipicial_crypto/keys.rs`**: `test_new_public_key_from_point`, `test_new_public_key_from_uncompressed_point`, `test_public_key_from_slice_formats`, `test_new_public_key_from_string_with_invalid_size`, `test_new_public_key_from_point_with_hex_prefix`, `test_serialize_public_key`, `test_deserialize_public_key`, `test_public_key_size`, `test_private_key_should_be_zeroized_after_erasing`, `test_public_key_comparable`, `test_sign_message`

**`src/atipicial_crypto/key_pair.rs`**: `test_public_key_wif`, `test_public_key_only_wif_export_fails`, `test_address`, `test_script_hash`, `test_public_key_bytes_uncompressed_xy`

**`src/atipicial_crypto/proptest_tests.rs`**: `prop_base58check_roundtrip`, `prop_sha256_consistent`, `prop_sha256_output_size`, `prop_ripemd160_output_size`, `prop_hash160_output_size`, `prop_double_sha256_output_size`, `prop_sha256_collision_resistance`, `prop_keypair_deterministic`, `prop_sign_verify_roundtrip`, `prop_sign_verify_wrong_message_fails`

**`src/atipicial_crypto/wif.rs`**: `test_valid_wif_to_private_key`, `test_invalid_wif_sizes`, `test_invalid_wif_bytes`, `test_valid_private_key_to_wif`, `test_invalid_private_key_length`, `test_wif_empty_string`, `test_wif_invalid_base58_characters`, `test_wif_corrupted_checksum`, `test_wif_roundtrip`, `test_wif_different_keys_produce_different_wifs`

**`src/atipicial_error/mod.rs`**: `test_error_context`, `test_error_macros`

**`src/atipicial_error/unified.rs`**: `test_error_builder`, `test_error_display`, `kind_returns_stable_variant_classification`, `provider_conversion_preserves_retry_and_domain_classification`, `provider_debug_redacts_json_rpc_data_through_unified_error`, `codec_errors_convert_into_the_unified_boundary`, `retry_after_round_trips_for_rate_limit`, `convenience_constructors_attach_recovery_hints`, `provide_error_metadata_exposes_code_and_message`

**`src/atipicial_fs/client.rs`**: `parse_container_ids_response_rejects_missing_containers_field`, `parse_container_ids_response_rejects_missing_container_id`, `parse_object_ids_response_rejects_missing_objects_field`, `parse_object_ids_response_rejects_missing_object_id`

**`src/atipicial_protocol/aep2.rs`**: `test_decrypt_with_default_scrypt_params`, `test_encrypt_with_default_scrypt_params`, `test_encrypt_decrypt_with_custom_params`, `test_wrong_password`, `test_encrypt_decrypt_aes256_ecb`, `test_aep2_specification_test_vector`, `test_aep2_empty_password`, `test_aep2_unicode_password`, `test_aep2_invalid_format`, `test_address_hash_from_pubkey_rejects_invalid_pubkey`, `test_aep2_corrupted_data`, `test_aep2_aes_key_length_validation`, `test_aep2_different_passwords_produce_different_encrypted_keys`

**`src/atipicial_types/address.rs`**: `test_address_to_script_hash`, `test_from_script_hash`

**`src/atipicial_types/address_or_scripthash.rs`**: `test_try_from_bytes_rejects_invalid_length`, `test_try_script_hash_rejects_invalid_address`, `test_deserialize_rejects_invalid_address_variant`, `test_deserialize_accepts_valid_address_variant`

**`src/atipicial_types/hardfork.rs`**: `test_hardfork_ordering`, `test_hardfork_from_str`, `test_hardfork_display`, `test_hardfork_try_from_u8`, `test_hardfork_all`

**`src/atipicial_types/mod.rs`**: `test_base64_encode_bytes`, `test_base64_decode`, `test_try_string_to_base64`, `test_try_string_to_base64_invalid_hex`

**`src/atipicial_types/numeric.rs`**: `test_to_bytes_padded`, `test_power`, `test_var_size`, `test_to_unsigned`, `test_i32_to_bytes`, `test_i64_to_bytes`, `test_f32_to_bytes`, `test_datetime_to_ms`

**`src/atipicial_types/proptest_tests.rs`**: `prop_scripthash_address_roundtrip`, `prop_h160_conversions`, `prop_h256_conversions`, `prop_contract_parameter_integer`, `prop_contract_parameter_boolean`, `prop_contract_parameter_string`, `prop_stackitem_integer`, `prop_stackitem_boolean`, `prop_stackitem_bytestring`, `prop_reverse_hex_roundtrip`, `prop_variable_length_encoding`, `prop_address_validation_consistency`, `prop_bigint_conversions`

**`src/atipicial_types/script_hash.rs`**: `test_address_type`, `test_into_script_hash_from_address_string`, `test_into_script_hash_from_hex`, `test_address_to_script_hash_roundtrip`, `test_from_valid_hash`, `test_creation_failures`, `test_to_array`, `test_serialize_and_deserialize`, `test_equals`, `test_from_address`, `test_from_public_key_bytes`, `test_to_address`

**`src/atipicial_types/serde_with_utils.rs`**: `test_serialize_hashset_u256`, `test_serialize_hashmap_u256_hashset_u256`, `test_u64_hex_and_option_roundtrip`, `test_serialize_bytes`, `test_serialize_u32`, `test_serialize_vec_h256`, `it ,`

**`src/atipicial_types/string.rs`**: `test_base58_check_decoded_validates_checksum`, `test_base58_check_decoded_rejects_invalid_checksum`, `test_try_base58_decoded_rejects_invalid_input`, `test_try_base58_check_decoded_rejects_invalid_checksum`, `test_try_address_to_scripthash_returns_known_hash`, `test_try_reversed_hex_rejects_invalid_hex`, `test_try_reversed_hex_reverses_valid_hex`

**`src/atipicial_types/syncing.rs`**: `deserialize_sync_geth`, `deserialize_sync_minimal`, `deserialize_sync_false`

**`src/atipicial_types/tx_pool.rs`**: `it : `, `it  wei + `, `it  gas × `

**`src/atipicial_types/util.rs`**: `test_bytes_to_string`

**`src/atipicial_types/whitelisted_contract.rs`**: `test_whitelisted_contract_new`, `test_whitelisted_contract_display`

**`src/atipicial_types/contract/aef_file.rs`**: `test_deserialize_preserves_checksum_and_roundtrips`, `test_to_array_repairs_checksum_for_encodable_aef`, `test_try_to_array_rejects_invalid_checksum_length`, `test_try_to_array_rejects_mismatched_checksum`, `test_method_token_try_to_array_rejects_method_name_longer_than_max`, `test_try_to_array_rejects_method_token_with_long_method_name`, `test_aef_size_matches_serialized_length`, `test_method_token_size_matches_serialized_length`, `test_try_to_array_rejects_source_url_longer_than_max`, `test_try_to_array_rejects_empty_script`, `test_try_to_array_rejects_script_longer_than_max`, `test_try_to_array_rejects_compiler_longer_than_fixed_width`, `test_try_to_array_matches_legacy_for_encodable_aef`

**`src/atipicial_types/contract/contract_parameter.rs`**: `test_try_from_aef_file_ref_rejects_invalid_aef`, `test_from_aef_file_ref_repairs_checksum_via_legacy_wrapper`, `test_try_from_aef_file_owned_matches_legacy_for_valid_aef`, `test_string_from_string`, `test_bytes_from_bytes`, `test_bytes_from_hex_string`, `test_array_from_array`, `test_array_from_empty`, `test_nested_array`, `test_map`, `test_nested_map`, `test_serialize_deserialize`, `test_bytes_equals`, `test_bytes_from_string`, `test_bool`, `test_int`, `test_h160`, `test_h256`, `test_public_key`, `test_signature`, `create_from_various_types`, `create_array_from_vec`, `create_map_from_hashmap`, `contract_parameter_to_value_is_structured_json`, `equality_operator`

**`src/atipicial_types/contract/contract_parameter_type.rs`**: `test_contract_parameter_type_deserialization`, `test_contract_parameter_type_serialization`

**`src/atipicial_types/contract/invocation_result.rs`**: `deserialize_missing_state_defaults_to_none`, `deserialize_empty_state_is_none`, `non_halt_states_are_treated_as_faults`

**`src/atipicial_types/nns/nns_name.rs`**: `it .`

**`src/atipicial_wallets/bip39_account.rs`**: `test_create_bip39_account`, `test_recover_from_mnemonic`, `test_invalid_mnemonic`, `test_different_passwords_different_accounts`, `test_generate_and_recover_bip39_account`

**`src/atipicial_wallets/yubi.rs`**: `test_wallet_signer_creation`

**`src/atipicial_wallets/wallet/aep6account.rs`**: `test_decrypt_with_standard_scrypt_params`, `test_decrypt_encrypted_only_account_repairs_script_hash`, `test_load_account_from_aep6`, `test_load_multi_sig_account_from_aep6`, `test_to_aep6_account_with_only_an_address`, `test_to_account_rejects_missing_address_and_contract_script`, `test_to_aep6_account_with_unecrypted_private_key`, `test_to_aep6_account_with_ecrypted_private_key`, `test_to_aep6_account_with_muliti_sig_account`

**`src/atipicial_wallets/wallet/aep6wallet.rs`**: `test_read_wallet`

**`src/atipicial_wallets/wallet/backup.rs`**: `test_backup_and_recover`

**`src/atipicial_wallets/wallet/wallet.rs`**: `test_is_default`, `test_create_default_wallet`, `test_try_new_creates_single_default_account`, `test_create_wallet_with_accounts`, `test_from_account_keeps_only_supplied_account`, `test_add_account_to_empty_wallet_sets_default_account`, `test_set_default_account_with_unknown_hash_leaves_no_default`, `test_is_default_account`, `test_add_account`, `test_encrypt_wallet`, `test_encrypt_wallet_parallel`, `test_encrypt_wallet_batch_parallel`, `test_change_password_parallel`, `test_change_password_rejects_empty_new_password_without_mutating_wallet`, `test_to_aep6_rejects_unencrypted_accounts_instead_of_dropping_them`, `test_save_to_file_rejects_unencrypted_wallet`, `test_from_aep6_rejects_empty_wallet`, `test_from_aep6_surfaces_invalid_account_errors`, `test_create_wallet_creates_single_encrypted_default_account`, `test_verify_password`, `test_remove_default_account_promotes_deterministic_remaining_account`, `test_remove_default_account_promotes_remaining_account`

**`src/atipicial_x/evm/provider.rs`**: `parse_chain_id_value_rejects_invalid_strings`

**`src/atipicial_x/evm/transaction.rs`**: `test_into_alloy_request`, `preserves_values_above_u64`

**`src/constants/native_contracts.rs`**: `test_native_contract_addresses_format`, `test_native_contract_known_hashes`

**`src/sdk/hd_wallet.rs`**: `test_derivation_path_parsing`, `test_hd_wallet_generation`, `test_hd_wallet_from_phrase`, `test_account_derivation`, `test_builder`, `test_export_import_encrypted_roundtrip`, `test_export_import_preserves_bip39_passphrase`, `test_import_encrypted_rejects_tampered_ciphertext`, `it /`

**`src/sdk/mod.rs`**: `decimal_amount_rejects_invalid_raw_values`, `decimal_amount_deserialization_validates_raw_value`, `test_builder_configuration`, `endpoint_shortcut_picks_custom_network`, `config_setter_replaces_entire_config`, `token_contract_hash_returns_native_hashes`, `test_parse_balance_stack_item_u64_rejects_negative_value`, `test_parse_aep17_decimals_rejects_invalid_value`, `it .`

**`src/sdk/retry.rs`**: `backoff_grows_exponentially_and_stays_bounded`

**`src/sdk/transaction_simulator.rs`**: `test_gas_estimate_creation`, `test_simulation_result`, `test_warning_levels`, `test_extract_token_symbol_reads_first_stack_item`, `test_extract_token_symbol_rejects_non_string_items`, `test_gas_consumed_decimal_string_converts_to_base_units`, `test_gas_consumed_rejects_overly_precise_decimals`, `test_cache_key_covers_signers`

**`src/sdk/unified.rs`**: `parse_amount_uses_chain_specific_scale`, `parse_amount_rejects_invalid_and_over_precise_input`, `atipicialx_balance_formats_as_human_decimal`

**`tests/gas_estimator_integration_tests.rs`**: `test_estimation_accuracy_calculation`

**`tests/ledger_tests.rs`**: `test_hdpath_to_vec`

**`tests/sdk_integration_tests.rs`**: `test_error_builder_network`, `test_error_builder_wallet`, `test_error_builder_contract`, `test_insufficient_funds_error`, `test_timeout_error`, `test_rate_limit_error`

**`tests/common/test_utils.rs`**: `test_generate_test_key`, `test_create_test_account`

**`website/src/components/HomepageFeatures/index.js`**: `it  `


---

## ⚙️ Configuration & Manifest Inventory

**62 configuration files orchestrate this project.**

| File | Lines | Package | Version |
|---|---|---|---|
| `Cargo.toml` | 267 | atipicial | 3.0.0 |
| `cliff.toml` | 83 | - | - |
| `compose-dev.yaml` | 12 | - | - |
| `deny.toml` | 106 | ansi_term | =0.11.0 |
| `release.toml` | 3 | - | - |
| `rust-toolchain.toml` | 8 | - | - |
| `rustfmt.toml` | 13 | - | - |
| `.cargo/audit.toml` | 8 | - | - |
| `.cargo/config.toml` | 6 | - | - |
| `.github/dependabot.yml` | 62 | - | - |
| `.github/workflows/build-test.yml` | 155 | - | - |
| `.github/workflows/release.yml` | 456 | atipicial | - |
| `.serena/project.yml` | 135 | - | - |
| `atipicial-cli/Cargo.toml` | 63 | atipicial-cli | 3.0.0 |
| `atipicial-cli/.cargo/config.toml` | 17 | - | 2 |
| `atipicial-cli/templates/aep17_token.toml` | 305 | aep17-token | 1.0.6 |
| `atipicial-cli/templates/basic_dapp.toml` | 269 | basic-dapp | 1.0.6 |
| `atipicial-cli/templates/defi_protocol.toml` | 59 | defi-protocol | 1.0.6 |
| `atipicial-cli/templates/nft_collection.toml` | 73 | nft-collection | 1.0.6 |
| `atipicial-cli/templates/oracle_consumer.toml` | 59 | oracle-consumer | 1.0.6 |
| `config/development.toml` | 142 | - | 1.2 |
| `config/production.toml` | 149 | - | 1.2 |
| `docs/book.toml` | 60 | - | 3.0.3 |
| `examples/advanced/Cargo.toml` | 22 | advanced-examples | 0.1.0 |
| `examples/atipicial_aep17_tokens/Cargo.toml` | 21 | atipicial_aep17_tokens_examples | 0.1.0 |
| `examples/atipicial_contracts/Cargo.toml` | 21 | atipicial_contracts_examples | 0.1.0 |
| `examples/atipicial_crypto/Cargo.toml` | 9 | atipicial_crypto | 0.1.0 |
| `examples/atipicial_famous_contracts/Cargo.toml` | 39 | atipicial_famous_contracts | 0.1.0 |
| `examples/atipicial_fs/Cargo.toml` | 28 | atipicial_fs_examples | 0.1.0 |
| `examples/atipicial_nns/Cargo.toml` | 19 | atipicial_nns_examples | 0.1.0 |
| `examples/atipicial_nodes/Cargo.toml` | 20 | atipicial_nodes_examples | 0.1.0 |
| `examples/atipicial_smart_contracts/Cargo.toml` | 21 | atipicial_smart_contracts_examples | 0.1.0 |
| `examples/atipicial_transactions/Cargo.toml` | 19 | atipicial_transactions_examples | 0.1.0 |
| `examples/atipicial_wallets/Cargo.toml` | 20 | atipicial_wallets_examples | 0.1.0 |
| `examples/atipicial_x/Cargo.toml` | 22 | atipicial_x_examples | 0.1.0 |
| `examples/basic/Cargo.toml` | 28 | basic-examples | 0.1.0 |
| `examples/big-numbers/Cargo.toml` | 18 | examples-big-numbers | 0.0.0 |
| `examples/contracts/Cargo.toml` | 18 | examples-contracts | 0.0.0 |
| `examples/contract_interaction/Cargo.toml` | 8 | contract_interaction_example | 0.1.0 |
| `examples/events/Cargo.toml` | 18 | examples-events | 0.0.0 |
| `examples/intermediate/Cargo.toml` | 22 | intermediate-examples | 0.1.0 |
| `examples/message_signing/Cargo.toml` | 10 | message_signing_example | 0.1.0 |
| `examples/middleware/Cargo.toml` | 22 | examples-middleware | 0.0.0 |
| `examples/providers/Cargo.toml` | 22 | examples-providers | 0.0.0 |
| `examples/queries/Cargo.toml` | 18 | examples-queries | 0.0.0 |
| `examples/sgx_enclave/Cargo.toml` | 25 | atipicial-sgx-example | 0.1.0 |
| `examples/standalone/Cargo.toml` | 38 | standalone-examples | 0.1.0 |
| `examples/subscriptions/Cargo.toml` | 21 | examples-subscriptions | 0.0.0 |
| `examples/transactions/Cargo.toml` | 19 | examples-transactions | 0.0.0 |
| `examples/wallets/Cargo.toml` | 27 | wallets | 0.1.0 |
| `monitoring/docker-compose.yml` | 159 | - | - |
| `monitoring/alertmanager/alertmanager.yml` | 63 | - | - |
| `monitoring/grafana/provisioning/dashboards/dashboard.yml` | 12 | - | - |
| `monitoring/grafana/provisioning/datasources/datasource.yml` | 20 | - | - |
| `monitoring/loki/loki-config.yml` | 39 | - | - |
| `monitoring/otel/otel-collector-config.yml` | 93 | - | - |
| `monitoring/prometheus/alerts.yml` | 151 | - | - |
| `monitoring/prometheus/prometheus.yml` | 53 | - | - |
| `monitoring/promtail/promtail-config.yml` | 54 | - | - |
| `website/netlify.toml` | 8 | - | - |
| `website/package.json` | 78 | - | - |
| `website/blog/authors.yml` | 26 | - | - |

---

## 🗺️ The Symbol Atlas — Signature by Signature

Every public symbol in the codebase with its exact Rust signature.
**1,883 exact signatures, copy-paste-ready.**

If it exists here, it exists in code — copy-paste-ready.


#### `atipicial-cli/src/errors.rs` — 1 symbols

```rust
pub enum CliError
```

#### `atipicial-cli/src/generator.rs` — 6 symbols

```rust
pub struct Template
pub struct TemplateMetadata
pub enum ProjectTemplate
pub fn display_name(&self) -> &str
pub fn generate_project(
pub fn list_templates()
```

#### `atipicial-cli/src/main.rs` — 1 symbols

```rust
pub struct Cli
```

#### `atipicial-cli/src/utils_core.rs` — 30 symbols

```rust
pub fn print_info(message: &str)
pub fn print_success(message: &str)
pub fn print_error(message: &str)
pub fn print_warning(message: &str)
pub fn print_debug(message: &str)
pub fn create_table() -> Table
pub fn create_progress_bar(len: u64, message: &str) -> LightweightProgress
pub fn create_spinner(message: &str) -> LightweightProgress
pub fn prompt_yes_no(message: &str) -> Result<bool, io::Error>
pub fn prompt_password(message: &str) -> Result<String, io::Error>
pub fn prompt_input(message: &str) -> Result<String, io::Error>
pub fn prompt_input_with_default(message: &str, default: &str) -> Result<String, io::Error>
pub fn prompt_select(message: &str, options: &[&str]) -> Result<usize, io::Error>
pub struct LightweightProgress
pub fn new(len: u64, message: &str) -> Self
pub fn enable_spinner(&mut self)
pub fn inc(&mut self, delta: u64)
pub fn finish_with_message(&mut self, msg: &str)
pub fn display_key_value(key: &str, value: &str)
pub fn display_key_value_colored(key: &str, value: &str, key_color: Color, value_color: Color)
pub fn format_number(num: u64) -> String
pub fn format_bytes(bytes: u64) -> String
pub fn format_duration(seconds: u64) -> String
pub fn print_section_header(title: &str)
pub fn print_subsection_header(title: &str)
pub fn clear_screen()
pub fn wait_for_enter(message: Option<&str>)
pub fn ensure_account_loaded() -> Result<(), crate::errors::CliError>
pub fn display_error_details(error: &dyn std::error::Error)
pub fn status_indicator(status: &str) -> ColoredString
```

#### `atipicial-cli/src/commands/atipicialfs.rs` — 6 symbols

```rust
pub struct AtipicialFsArgs
pub enum AtipicialFsCommands
pub enum ContainerCommands
pub enum ObjectCommands
pub enum AclCommands
pub enum ConfigCommands
```

#### `atipicial-cli/src/commands/blockchain.rs` — 2 symbols

```rust
pub struct BlockchainArgs
pub enum BlockchainCommands
```

#### `atipicial-cli/src/commands/contract.rs` — 2 symbols

```rust
pub struct ContractArgs
pub enum ContractCommands
```

#### `atipicial-cli/src/commands/fs.rs` — 8 symbols

```rust
pub struct ContainerInfo
pub struct ObjectInfo
pub struct NetworkStatus
pub struct FSArgs
pub enum FSCommands
pub enum EndpointCommands
pub enum ContainerCommands
pub enum ObjectCommands
```

#### `atipicial-cli/src/commands/network.rs` — 3 symbols

```rust
pub struct NetworkConfig
pub struct NetworkArgs
pub enum NetworkCommands
```

#### `atipicial-cli/src/commands/nft.rs` — 2 symbols

```rust
pub struct NftArgs
pub enum NftCommands
```

#### `atipicial-cli/src/commands/tools.rs` — 2 symbols

```rust
pub struct ToolsArgs
pub enum ToolsCommands
```

#### `atipicial-cli/src/commands/wallet.rs` — 7 symbols

```rust
pub struct CliState
pub fn get_network_type_string(&self) -> String
pub fn set_network_type(&mut self, network: String)
pub fn get_rpc_client(&self) -> Result<&RpcClient<HttpProvider>, CliError>
pub fn get_account(&self) -> Result<Account, CliError>
pub struct WalletArgs
pub enum WalletCommands
```

#### `atipicial-cli/src/commands/defi/mod.rs` — 3 symbols

```rust
pub struct DefiArgs
pub enum DefiCommands
pub fn create_h160_param(value: &str) -> Result<ContractParameter, CliError>
```

#### `atipicial-cli/src/commands/defi/types.rs` — 8 symbols

```rust
pub struct TokenArgs
pub enum TokenCommands
pub struct SwapArgs
pub enum SwapCommands
pub struct LiquidityArgs
pub enum LiquidityCommands
pub struct StakingArgs
pub enum StakingCommands
```

#### `atipicial-cli/src/commands/defi/utils.rs` — 10 symbols

```rust
pub enum NetworkType
pub fn from_network(network: &str) -> Self
pub enum NetworkTypeCli
pub fn from_magic(magic: u32) -> Self
pub fn from_network_string(network: &str) -> Self
pub fn to_network_string(&self) -> String
pub fn prepare_state_from_existing(existing_state: &CliState) -> CliState
pub fn get_token_address_for_network(
pub fn format_token_amount(raw_amount: i64, decimals: u8) -> String
pub fn load_wallet_from_state(state: &mut CliState) -> Result<&mut Wallet, CliError>
```

#### `atipicial-cli/src/config/mod.rs` — 7 symbols

```rust
pub struct CliConfig
pub struct NetworkConfig
pub struct WalletConfig
pub struct StorageConfig
pub struct LoggingConfig
pub fn load() -> Result<Self>
pub fn save(&self) -> Result<()>
```

#### `atipicial-cli/src/monitoring/logger.rs` — 20 symbols

```rust
pub enum LogFormat
pub struct LoggerConfig
pub fn init_logger(config: LoggerConfig) -> Result<(), Box<dyn std::error::Error>>
pub struct StructuredLogger
pub fn new() -> Self
pub fn with_context(mut self, key: &str, value: &str) -> Self
pub fn log(&self, level: Level, message: &str)
pub fn error(&self, message: &str)
pub fn warn(&self, message: &str)
pub fn info(&self, message: &str)
pub fn debug(&self, message: &str)
pub fn trace(&self, message: &str)
pub struct PerformanceLogger
pub fn new(operation: &str) -> Self
pub fn with_threshold(mut self, threshold: std::time::Duration) -> Self
pub fn complete(self)
pub fn complete_with_result<T>(self, result: &Result<T, impl std::error::Error>)
pub struct AuditLogger
pub fn new(file_path: PathBuf) -> Result<Self, std::io::Error>
pub fn log_operation(
```

#### `atipicial-cli/src/monitoring/metrics.rs` — 25 symbols

```rust
pub struct MetricsConfig
pub enum MetricType
pub enum MetricValue
pub struct SummaryData
pub struct Metric
pub struct MetricsRegistry
pub fn new() -> Self
pub fn register(&self, metric: Metric)
pub fn get(&self, name: &str) -> Option<Metric>
pub fn update(&self, name: &str, value: MetricValue)
pub fn all(&self) -> Vec<Metric>
pub fn export_prometheus(&self) -> String
pub struct MetricsCollector
pub fn new(config: MetricsConfig) -> Result<Self, Box<dyn std::error::Error>>
pub fn start_server(&mut self) -> Result<(), Box<dyn std::error::Error>>
pub fn stop_server(&mut self)
pub fn record(&self, name: &str, value: f64, labels: Vec<(&str, &str)>)
pub fn increment(&self, name: &str, labels: Vec<(&str, &str)>)
pub fn gauge(&self, name: &str, value: f64, labels: Vec<(&str, &str)>)
pub fn histogram(&self, name: &str, value: f64, labels: Vec<(&str, &str)>)
pub fn export_prometheus(&self) -> String
pub struct Timer
pub fn new(name: &str, collector: Arc<MetricsCollector>) -> Self
pub fn with_label(mut self, key: &str, value: &str) -> Self
pub fn observe(self)
```

#### `atipicial-cli/src/monitoring/mod.rs` — 6 symbols

```rust
pub fn initialize_monitoring(
pub struct MonitoringContext
pub fn record_metric(&self, name: &str, value: f64, labels: Vec<(&str, &str)>)
pub fn increment_counter(&self, name: &str, labels: Vec<(&str, &str)>)
pub fn update_gauge(&self, name: &str, value: f64, labels: Vec<(&str, &str)>)
pub fn observe_histogram(&self, name: &str, value: f64, labels: Vec<(&str, &str)>)
```

#### `atipicial-cli/src/security/error_handler.rs` — 16 symbols

```rust
pub struct RetryConfig
pub fn critical() -> Self
pub fn fast() -> Self
pub struct RetryHandler
pub fn new(config: RetryConfig) -> Self
pub enum RecoveryStrategy
pub struct ErrorHandler
pub fn new() -> Self
pub struct ErrorContext
pub fn new(operation: &str) -> Self
pub fn with_context(mut self, key: &str, value: &str) -> Self
pub fn with_stack_trace(mut self) -> Self
pub struct ErrorReporter
pub fn new() -> Self
pub fn report(&self, error: ErrorContext)
pub fn get_recent_errors(&self, count: usize) -> Vec<ErrorContext>
```

#### `atipicial-cli/src/security/keychain.rs` — 15 symbols

```rust
pub struct KeychainManager
pub struct SecureCredential
pub fn new(service_name: &str) -> Result<Self, CliError>
pub fn store_credential(
pub fn get_credential(&self, key: &str) -> Result<SecureCredential, CliError>
pub fn delete_credential(&mut self, key: &str) -> Result<(), CliError>
pub fn list_credentials(&self) -> Result<Vec<String>, CliError>
pub struct SecureWalletStorage
pub fn new() -> Result<Self, CliError>
pub fn store_private_key(&mut self, address: &str, private_key: &[u8]) -> Result<(), CliError>
pub fn get_private_key(&self, address: &str) -> Result<Vec<u8>, CliError>
pub fn store_mnemonic(&mut self, wallet_id: &str, mnemonic: &str) -> Result<(), CliError>
pub fn get_mnemonic(&self, wallet_id: &str) -> Result<String, CliError>
pub fn delete_wallet(&mut self, address: &str) -> Result<(), CliError>
pub fn list_wallets(&self) -> Result<Vec<String>, CliError>
```

#### `atipicial-cli/src/security/mod.rs` — 4 symbols

```rust
pub fn initialize_security() -> Result<SecurityContext, crate::errors::CliError>
pub struct SecurityContext
pub fn new() -> Result<Self, crate::errors::CliError>
pub fn with_config(session_config: SessionConfig) -> Result<Self, crate::errors::CliError>
```

#### `atipicial-cli/src/security/network_failover.rs` — 23 symbols

```rust
pub struct EndpointHealth
pub fn new(url: String) -> Self
pub fn success(&mut self, response_time: Duration)
pub fn failure(&mut self, error: String)
pub fn score(&self) -> f64
pub struct FailoverConfig
pub struct NetworkFailover
pub fn new(endpoints: Vec<String>, config: FailoverConfig) -> Self
pub fn start_health_monitoring(&mut self)
pub fn stop_health_monitoring(&mut self)
pub fn get_best_endpoint(&self) -> Result<String, CliError>
pub fn get_health_stats(&self) -> Vec<EndpointHealth>
pub fn add_endpoint(&self, url: String)
pub fn remove_endpoint(&self, url: &str)
pub fn reset_health_stats(&self)
pub struct NetworkFailoverBuilder
pub fn new() -> Self
pub fn add_endpoint(mut self, url: String) -> Self
pub fn add_endpoints(mut self, urls: Vec<String>) -> Self
pub fn with_config(mut self, config: FailoverConfig) -> Self
pub fn health_check_interval(mut self, interval: Duration) -> Self
pub fn request_timeout(mut self, timeout: Duration) -> Self
pub fn build(self) -> NetworkFailover
```

#### `atipicial-cli/src/security/session.rs` — 24 symbols

```rust
pub struct SessionConfig
pub struct Session
pub fn new(user_id: &str, config: &SessionConfig) -> Self
pub fn is_expired(&self) -> bool
pub fn is_idle(&self, timeout: Duration) -> bool
pub fn touch(&mut self)
pub fn invalidate(&mut self)
pub struct SessionManager
pub fn new(config: SessionConfig) -> Self
pub fn create_session(&self, user_id: &str) -> Result<Session, CliError>
pub fn get_session(&self, session_id: &str) -> Result<Session, CliError>
pub fn validate_session(&self, session_id: &str) -> Result<bool, CliError>
pub fn refresh_session(&self, session_id: &str) -> Result<Session, CliError>
pub fn invalidate_session(&self, session_id: &str) -> Result<(), CliError>
pub fn invalidate_user_sessions(&self, user_id: &str) -> Result<(), CliError>
pub fn cleanup_expired(&self) -> Result<usize, CliError>
pub fn add_auth_callback<F>(&self, name: &str, callback: F)
pub fn active_sessions_count(&self) -> usize
pub fn get_user_sessions(&self, user_id: &str) -> Vec<Session>
pub fn load_persisted_sessions(&self) -> Result<usize, CliError>
pub struct SessionGuard
pub fn new(manager: Arc<SessionManager>, session_id: String) -> Self
pub fn session(&self) -> Result<Session, CliError>
pub fn is_valid(&self) -> bool
```

#### `atipicial-cli/src/utils/atipicialfs.rs` — 8 symbols

```rust
pub fn validate_container_id(container_id: &str) -> Result<(), CliError>
pub fn validate_file_path(path: &Path) -> Result<(), CliError>
pub fn validate_directory_path(path: &Path) -> Result<(), CliError>
pub fn format_size(size: u64) -> String
pub fn validate_endpoint(endpoint: &str) -> Result<(), CliError>
pub fn get_node_info(endpoint: &str) -> Result<String, CliError>
pub fn check_endpoint_availability(endpoint: &str) -> Result<bool, CliError>
pub fn format_permissions(is_public_read: bool, is_public_write: bool) -> String
```

#### `atipicial-cli/src/utils/config.rs` — 8 symbols

```rust
pub struct Config
pub struct NetworkConfig
pub struct AtipicialFsConfig
pub struct AtipicialFsEndpoint
pub fn get_config_dir() -> Result<PathBuf, CliError>
pub fn get_config_path() -> Result<PathBuf, CliError>
pub fn load_config() -> Result<Config, CliError>
pub fn save_config(config: &Config) -> Result<(), CliError>
```

#### `atipicial-cli/src/utils/error.rs` — 2 symbols

```rust
pub enum CliError
pub type CliResult<T> = Result<T, CliError>
```

#### `atipicial-cli/src/utils/extensions.rs` — 1 symbols

```rust
pub trait TransactionExtensions
```

#### `atipicial-cli/src/utils/mod.rs` — 7 symbols

```rust
pub fn print_success(message: &str)
pub fn print_info(message: &str)
pub fn print_warning(message: &str)
pub fn print_error(message: &str)
pub fn prompt_input<T>(prompt: &str) -> Result<T, CliError>
pub fn prompt_password(prompt: &str) -> Result<String, CliError>
pub fn prompt_yes_no(prompt: &str) -> Result<bool, CliError>
```

#### `src/lib.rs` — 1 symbols

```rust
pub const VERSION: &str = env!("CARGO_PKG_VERSION")
```

#### `src/atipicial_builder/error.rs` — 1 symbols

```rust
pub enum BuilderError
```

#### `src/atipicial_builder/utils.rs` — 4 symbols

```rust
pub fn public_keys_to_scripthash(
pub fn try_public_keys_to_scripthash(
pub fn pubkey_to_scripthash(public_key: &Secp256r1PublicKey) -> ScriptHash
pub trait VecValueExtension
```

#### `src/atipicial_builder/script/interop_service.rs` — 4 symbols

```rust
pub enum InteropService
pub fn hash(&self) -> String
pub fn from_hash(hash: String) -> Option<InteropService>
pub fn price(&self) -> u64
```

#### `src/atipicial_builder/script/script_builder.rs` — 22 symbols

```rust
pub struct ScriptBuilder
pub fn new() -> Self
pub fn op_code(&mut self, op_codes: &[OpCode]) -> &mut Self
pub fn op_code_with_arg(&mut self, opcode: OpCode, argument: Bytes) -> &mut Self
pub fn contract_call(
pub fn sys_call(&mut self, operation: InteropService) -> &mut Self
pub fn push_params(&mut self, params: &[ContractParameter]) -> Result<&mut Self, BuilderError>
pub fn push_param(&mut self, param: &ContractParameter) -> Result<&mut Self, BuilderError>
pub fn push_integer(&mut self, i: BigInt) -> &mut Self
pub fn try_push_integer(&mut self, i: BigInt) -> Result<&mut Self, BuilderError>
pub fn push_opcode_bytes(&mut self, opcode: OpCode, argument: Vec<u8>) -> &mut ScriptBuilder
pub fn push_data(&mut self, data: Vec<u8>) -> &mut Self
pub fn push_bool(&mut self, b: bool) -> &mut Self
pub fn push_array(&mut self, arr: &[ContractParameter]) -> Result<&mut Self, BuilderError>
pub fn push_map(
pub fn pack(&mut self) -> &mut Self
pub fn to_bytes(&self) -> Bytes
pub fn build_verification_script(pub_key: &Secp256r1PublicKey) -> Bytes
pub fn build_multi_sig_script(
pub fn build_contract_script(
pub fn build_contract_call_and_unwrap_iterator(
pub fn len(&self) -> usize
```

#### `src/atipicial_builder/script/script_reader.rs` — 3 symbols

```rust
pub struct ScriptReader
pub fn get_interop_service_code(_hash: String) -> Option<InteropService>
pub fn convert_to_op_code_string(script: &Bytes) -> String
```

#### `src/atipicial_builder/transaction/call_flags.rs` — 3 symbols

```rust
pub enum CallFlags
pub fn value(&self) -> u8
pub fn from_value(value: u8) -> Result<Self, &'static str>
```

#### `src/atipicial_builder/transaction/contract_parameters_context.rs` — 4 symbols

```rust
pub struct ContractParametersContext
pub fn new(
pub struct ContextItem
pub fn new(
```

#### `src/atipicial_builder/transaction/gas_estimator.rs` — 3 symbols

```rust
pub struct GasEstimator
pub fn calculate_estimation_accuracy(estimated: i64, actual: i64) -> f64
pub trait TransactionBuilderGasExt
```

#### `src/atipicial_builder/transaction/invocation_script.rs` — 10 symbols

```rust
pub struct InvocationScript
pub fn new() -> Self
pub fn new_with_script(script: Vec<u8>) -> Self
pub fn from_serialized_script(script: Vec<u8>) -> Self
pub fn from_signature(signature: Secp256r1Signature) -> Self
pub fn from_message_and_key_pair(
pub fn from_signatures(signatures: &[Secp256r1Signature]) -> Self
pub fn try_encode(&self, writer: &mut Encoder) -> Result<(), BuilderError>
pub fn try_to_array(&self) -> Result<Vec<u8>, BuilderError>
pub fn get_signatures(&self) -> Vec<Secp256r1Signature>
```

#### `src/atipicial_builder/transaction/mod.rs` — 1 symbols

```rust
pub fn init_logger()
```

#### `src/atipicial_builder/transaction/oracle_response_code.rs` — 1 symbols

```rust
pub enum OracleResponseCode
```

#### `src/atipicial_builder/transaction/production_transaction_builder.rs` — 21 symbols

```rust
pub struct FeeCalculator
pub fn new() -> Self
pub fn with_params(base_fee_per_byte: u64, witness_fee: u64, storage_fee_per_byte: u64) -> Self
pub fn calculate_network_fee(&self, transaction_size: usize, witness_count: usize) -> u64
pub fn calculate_system_fee(&self, script_length: usize, storage_changes: usize) -> u64
pub struct WitnessGenerator
pub fn new() -> Self
pub fn add_verification_script(&mut self, address: String, script: Vec<u8>)
pub fn generate_witness(&self, signature: &[u8], address: &str) -> Result<ProductionWitness, Box<dyn std::error::Error>>
pub struct ProductionWitness
pub struct ProductionSigner
pub struct ProductionTransactionAttribute
pub struct ProductionWitnessRule
pub struct ProductionTransaction
pub struct ProductionTransactionBuilder
pub fn new() -> Self
pub fn calculate_network_fee(&self, transaction_size: usize, witness_count: usize) -> u64
pub fn calculate_system_fee(&self, script_length: usize, storage_changes: usize) -> u64
pub fn add_verification_script(&mut self, address: String, script: Vec<u8>)
pub fn generate_witness(&self, signature: &[u8], address: &str) -> Result<ProductionWitness, Box<dyn std::error::Error>>
pub fn build_transaction(
```

#### `src/atipicial_builder/transaction/transaction.rs` — 7 symbols

```rust
pub struct Transaction<'a, P: JsonRpcProvider + 'static>
pub fn new() -> Self
pub fn pay<K: Into<NameOrAddress>, V: Into<U256>>(_to: K, _value: V) -> Self
pub fn add_witness(&mut self, witness: Witness)
pub fn tx_id(&self) -> Result<primitive_types::H256, TransactionError>
pub fn try_encode(&self, writer: &mut Encoder) -> Result<(), TransactionError>
pub fn try_to_array(&self) -> Result<Vec<u8>, TransactionError>
```

#### `src/atipicial_builder/transaction/transaction_attribute.rs` — 12 symbols

```rust
pub enum TransactionAttribute
pub struct OracleResponse
pub const MAX_RESULT_SIZE: usize = 0xffff
pub fn try_size(&self) -> Result<usize, TransactionError>
pub fn try_encode(&self, writer: &mut Encoder) -> Result<(), TransactionError>
pub fn try_to_bytes(&self) -> Result<Vec<u8>, TransactionError>
pub fn to_bytes(&self) -> Vec<u8>
pub fn from_bytes(bytes: &[u8]) -> Result<Self, &'static str>
pub fn try_to_json(&self) -> Result<String, serde_json::Error>
pub fn to_json(&self) -> String
pub fn get_height(&self) -> Option<&u32>
pub fn get_hash(&self) -> Option<&H256>
```

#### `src/atipicial_builder/transaction/transaction_builder.rs` — 19 symbols

```rust
pub struct TransactionBuilder<'a, P: JsonRpcProvider + 'static>
pub const BALANCE_OF_FUNCTION: &'static str = "balanceOf"
pub const DUMMY_PUB_KEY: &'static str =
pub fn new() -> Self
pub fn with_client(client: &'a RpcClient<P>) -> Self
pub fn allow_transmission_on_fault(&mut self) -> &mut Self
pub fn disallow_transmission_on_fault(&mut self) -> &mut Self
pub fn version(&mut self, version: u8) -> &mut Self
pub fn nonce(&mut self, nonce: u32) -> Result<&mut Self, TransactionError>
pub fn valid_until_block(&mut self, block: u32) -> Result<&mut Self, TransactionError>
pub fn first_signer(&mut self, sender: &Account) -> Result<&mut Self, TransactionError>
pub fn first_signer_by_hash(&mut self, sender: &H160) -> Result<&mut Self, TransactionError>
pub fn extend_script(&mut self, script: Vec<u8>) -> &mut Self
pub fn validate(&self) -> Result<(), TransactionError>
pub fn is_ready(&self) -> bool
pub fn set_signers(&mut self, signers: Vec<Signer>) -> Result<&mut Self, TransactionError>
pub fn add_attributes(
pub fn do_if_sender_cannot_cover_fees<F>(
pub fn throw_if_sender_cannot_cover_fees(
```

#### `src/atipicial_builder/transaction/transaction_error.rs` — 1 symbols

```rust
pub enum TransactionError
```

#### `src/atipicial_builder/transaction/transaction_send_token.rs` — 2 symbols

```rust
pub struct TransactionSendToken
pub fn new(token: H160, value: i32, address: String) -> Self
```

#### `src/atipicial_builder/transaction/verification_script.rs` — 14 symbols

```rust
pub struct VerificationScript
pub fn new() -> Self
pub fn from(script: Bytes) -> Self
pub fn from_public_key(public_key: &Secp256r1PublicKey) -> Self
pub fn from_multi_sig(public_keys: &mut [Secp256r1PublicKey], threshold: u8) -> Self
pub fn is_single_sig(&self) -> bool
pub fn is_multi_sig(&self) -> bool
pub fn hash(&self) -> H160
pub fn get_signatures(&self) -> Vec<Secp256r1Signature>
pub fn get_public_keys(&self) -> Result<Vec<Secp256r1PublicKey>, BuilderError>
pub fn get_signing_threshold(&self) -> Result<usize, BuilderError>
pub fn get_nr_of_accounts(&self) -> Result<usize, BuilderError>
pub fn try_encode(&self, writer: &mut Encoder) -> Result<(), BuilderError>
pub fn try_to_array(&self) -> Result<Vec<u8>, BuilderError>
```

#### `src/atipicial_builder/transaction/witness.rs` — 10 symbols

```rust
pub struct Witness
pub fn new() -> Self
pub fn from_scripts(invocation_script: Bytes, verification_script: Bytes) -> Self
pub fn from_scripts_obj(
pub fn create(message_to_sign: Bytes, key_pair: &KeyPair) -> Result<Self, BuilderError>
pub fn create_multi_sig_witness(
pub fn create_multi_sig_witness_script(
pub fn create_contract_witness(params: Vec<ContractParameter>) -> Result<Self, BuilderError>
pub fn try_encode(&self, writer: &mut Encoder) -> Result<(), BuilderError>
pub fn try_to_array(&self) -> Result<Vec<u8>, BuilderError>
```

#### `src/atipicial_builder/transaction/witness_scope.rs` — 5 symbols

```rust
pub enum WitnessScope
pub fn byte_repr(&self) -> u8
pub fn validate(scopes: &[WitnessScope]) -> Result<(), BuilderError>
pub fn combine(scopes: &[Self]) -> u8
pub fn split(flags: u8) -> Vec<Self>
```

#### `src/atipicial_builder/transaction/signers/account_signer.rs` — 12 symbols

```rust
pub struct AccountSigner
pub fn none(account: &Account) -> Result<Self, TransactionError>
pub fn called_by_entry(account: &Account) -> Result<Self, TransactionError>
pub fn global(account: &Account) -> Result<Self, TransactionError>
pub fn is_multi_sig(&self) -> bool
pub fn get_script_hash(&self) -> H160
pub fn try_encode(&self, writer: &mut Encoder) -> Result<(), TransactionError>
pub fn try_to_array(&self) -> Result<Vec<u8>, TransactionError>
pub fn new(account: &Account, scope: WitnessScope) -> Self
pub fn none_hash160(account_hash: H160) -> Result<Self, TransactionError>
pub fn called_by_entry_hash160(account_hash: H160) -> Result<Self, TransactionError>
pub fn global_hash160(account_hash: H160) -> Result<Self, TransactionError>
```

#### `src/atipicial_builder/transaction/signers/contract_signer.rs` — 5 symbols

```rust
pub struct ContractSigner
pub fn called_by_entry(contract_hash: H160, verify_params: &[ContractParameter]) -> Self
pub fn global(contract_hash: H160, verify_params: &[ContractParameter]) -> Self
pub fn try_encode(&self, writer: &mut Encoder) -> Result<(), TransactionError>
pub fn try_to_array(&self) -> Result<Vec<u8>, TransactionError>
```

#### `src/atipicial_builder/transaction/signers/signer.rs` — 14 symbols

```rust
pub enum SignerType
pub trait SignerTrait
pub enum Signer
pub fn from_bytes(data: &[u8]) -> Result<Signer, TransactionError>
pub fn get_type(&self) -> SignerType
pub fn get_signer_hash(&self) -> &H160
pub fn as_account_signer(&self) -> Option<&AccountSigner>
pub fn as_contract_signer(&self) -> Option<&ContractSigner>
pub fn as_transaction_signer(&self) -> Option<&TransactionSigner>
pub fn to_account_signer(self) -> Result<AccountSigner, BuilderError>
pub fn to_contract_signer(self) -> Result<ContractSigner, BuilderError>
pub fn try_to_transaction_signer(&self) -> Result<TransactionSigner, BuilderError>
pub fn try_encode(&self, writer: &mut Encoder) -> Result<(), TransactionError>
pub fn try_to_array(&self) -> Result<Vec<u8>, TransactionError>
```

#### `src/atipicial_builder/transaction/signers/transaction_signer.rs` — 5 symbols

```rust
pub struct TransactionSigner
pub fn new(account: H160, scopes: Vec<WitnessScope>) -> Result<Self, BuilderError>
pub fn new_full(
pub fn try_encode(&self, writer: &mut Encoder) -> Result<(), TransactionError>
pub fn try_to_array(&self) -> Result<Vec<u8>, TransactionError>
```

#### `src/atipicial_builder/transaction/witness_rule/witness_action.rs` — 1 symbols

```rust
pub enum WitnessAction
```

#### `src/atipicial_builder/transaction/witness_rule/witness_condition.rs` — 11 symbols

```rust
pub enum WitnessCondition
pub fn json_value(&self) -> &'static str
pub fn byte(&self) -> u8
pub fn boolean_expression(&self) -> Option<bool>
pub fn expression(&self) -> Option<&WitnessCondition>
pub fn expression_list(&self) -> Option<&[WitnessCondition]>
pub fn script_hash(&self) -> Option<&H160>
pub fn group(&self) -> Option<&Secp256r1PublicKey>
pub fn from_bytes(bytes: &[u8]) -> Result<WitnessCondition, TransactionError>
pub fn try_encode(&self, writer: &mut Encoder) -> Result<(), TransactionError>
pub fn try_to_array(&self) -> Result<Vec<u8>, TransactionError>
```

#### `src/atipicial_builder/transaction/witness_rule/witness_rule.rs` — 4 symbols

```rust
pub struct WitnessRule
pub fn new(action: WitnessAction, condition: WitnessCondition) -> Self
pub fn try_encode(&self, writer: &mut Encoder) -> Result<(), TransactionError>
pub fn try_to_array(&self) -> Result<Vec<u8>, TransactionError>
```

#### `src/atipicial_clients/api_trait.rs` — 1 symbols

```rust
pub trait APITrait: Sync + Send + Debug
```

#### `src/atipicial_clients/cache.rs` — 15 symbols

```rust
pub struct CacheConfig
pub fn builder() -> CacheConfigBuilder
pub struct CacheConfigBuilder
pub fn max_entries(mut self, val: usize) -> Self
pub fn default_ttl(mut self, val: Duration) -> Self
pub fn cleanup_interval(mut self, val: Duration) -> Self
pub fn enable_lru(mut self, val: bool) -> Self
pub fn build(self) -> CacheConfig
pub struct Cache<K, V>
pub struct CacheStats
pub fn hit_rate(&self) -> f64
pub fn new(config: CacheConfig) -> Self
pub fn start_cleanup_task(&self) -> tokio::task::JoinHandle<()>
pub type RpcCache = Cache<String, serde_json::Value>
pub fn new_rpc_cache() -> Self
```

#### `src/atipicial_clients/circuit_breaker.rs` — 13 symbols

```rust
pub enum CircuitState
pub struct CircuitBreakerConfig
pub fn builder() -> CircuitBreakerConfigBuilder
pub struct CircuitBreakerConfigBuilder
pub fn failure_threshold(mut self, val: u32) -> Self
pub fn timeout(mut self, val: Duration) -> Self
pub fn success_threshold(mut self, val: u32) -> Self
pub fn failure_window(mut self, val: Duration) -> Self
pub fn half_open_max_requests(mut self, val: u32) -> Self
pub fn build(self) -> CircuitBreakerConfig
pub struct CircuitBreakerStats
pub struct CircuitBreaker
pub fn new(config: CircuitBreakerConfig) -> Self
```

#### `src/atipicial_clients/connection_pool.rs` — 15 symbols

```rust
pub struct PoolConfig
pub fn builder() -> PoolConfigBuilder
pub struct PoolConfigBuilder
pub fn max_connections(mut self, val: usize) -> Self
pub fn min_idle(mut self, val: usize) -> Self
pub fn max_idle_time(mut self, val: Duration) -> Self
pub fn connection_timeout(mut self, val: Duration) -> Self
pub fn request_timeout(mut self, val: Duration) -> Self
pub fn max_retries(mut self, val: u32) -> Self
pub fn retry_delay(mut self, val: Duration) -> Self
pub fn build(self) -> PoolConfig
pub struct ConnectionPool
pub struct PoolStats
pub fn new(endpoint: String, config: PoolConfig) -> Self
pub fn start_maintenance_task(&self) -> tokio::task::JoinHandle<()>
```

#### `src/atipicial_clients/errors.rs` — 8 symbols

```rust
pub enum ProviderError
pub fn is_retryable(&self) -> bool
pub fn is_rate_limited(&self) -> bool
pub fn retry_after(&self) -> Option<std::time::Duration>
pub fn http_status(&self) -> Option<reqwest::StatusCode>
pub fn is_unknown_transaction(&self) -> bool
pub fn is_already_known_transaction(&self) -> bool
pub fn is_transaction_rejection(&self) -> bool
```

#### `src/atipicial_clients/mock_client.rs` — 2 symbols

```rust
pub struct MockClient
pub fn into_client(&self) -> RpcClient<MockProvider>
```

#### `src/atipicial_clients/mod.rs` — 7 symbols

```rust
pub trait RpcError: std::error::Error + Send + Sync
pub fn try_http_provider_from_endpoint(endpoint: &str) -> Result<RpcClient<Http>, ProviderError>
pub fn try_http_provider_from_env() -> Result<RpcClient<Http>, ProviderError>
pub struct TestProvider
pub fn new(endpoints: &'static [&'static str], network: impl Into<String>) -> Self
pub fn url(&self) -> String
pub fn provider(&self) -> RpcClient<Http>
```

#### `src/atipicial_clients/production_client.rs` — 12 symbols

```rust
pub struct ProductionRpcClient
pub struct ProductionClientConfig
pub fn builder() -> ProductionClientConfigBuilder
pub struct ProductionClientConfigBuilder
pub fn pool_config(mut self, config: PoolConfig) -> Self
pub fn cache_config(mut self, config: CacheConfig) -> Self
pub fn circuit_breaker_config(mut self, config: CircuitBreakerConfig) -> Self
pub fn enable_logging(mut self, enable: bool) -> Self
pub fn enable_metrics(mut self, enable: bool) -> Self
pub fn build(self) -> ProductionClientConfig
pub struct ProductionClientStats
pub fn new(endpoint: String, config: ProductionClientConfig) -> Self
```

#### `src/atipicial_clients/rate_limiter.rs` — 14 symbols

```rust
pub struct RateLimiter
pub fn new(max_requests: u32, window: Duration, max_concurrent: usize) -> Self
pub struct RateLimitPermit<'a>
pub struct RateLimiterBuilder
pub fn new() -> Self
pub fn max_requests(mut self, max: u32) -> Self
pub fn window(mut self, window: Duration) -> Self
pub fn max_concurrent(mut self, max: usize) -> Self
pub fn build(self) -> RateLimiter
pub struct RateLimiterPresets
pub fn conservative() -> RateLimiter
pub fn standard() -> RateLimiter
pub fn aggressive() -> RateLimiter
pub fn custom(requests_per_second: u32, max_concurrent: usize) -> RateLimiter
```

#### `src/atipicial_clients/utils.rs` — 19 symbols

```rust
pub type EscalationPolicy = Box<dyn Fn(U256, usize) -> U256 + Send + Sync>
pub type AtipicialHttpClient = super::RpcClient<super::Http>
pub type ProviderResult<T> = Result<T, ProviderError>
pub type AsyncProviderResult<'a, T> = Pin<Box<dyn Future<Output = ProviderResult<T>> + Send + 'a>>
pub type AsyncProviderResult<'a, T> = Pin<Box<dyn Future<Output = ProviderResult<T>> + 'a>>
pub fn interval(duration: Duration) -> impl stream::Stream<Item = ()> + Send + Unpin
pub fn try_serialize<T: serde::Serialize>(t: &T) -> Result<serde_json::Value, serde_json::Error>
pub fn serialize<T: serde::Serialize>(t: &T) -> serde_json::Value
pub fn script_hash_from_script(script: &[u8]) -> ScriptHash
pub fn public_key_to_address(public_key: &Secp256r1PublicKey) -> String
pub fn public_key_to_script_hash(public_key: &Secp256r1PublicKey) -> ScriptHash
pub fn private_key_to_script_hash(private_key: &Secp256r1PrivateKey) -> ScriptHash
pub fn private_key_to_address(private_key: &Secp256r1PrivateKey) -> String
pub fn script_hash_to_address(script_hash: &ScriptHash) -> String
pub fn address_to_script_hash(address: &str) -> Result<ScriptHash, ProviderError>
pub fn script_hash_to_hex(script_hash: &ScriptHash) -> String
pub fn script_hash_from_hex(hex: &str) -> Result<ScriptHash, ProviderError>
pub fn address_to_hex(address: &str) -> Result<String, ProviderError>
pub fn hex_to_address(hex: &str) -> Result<String, ProviderError>
```

#### `src/atipicial_clients/ext/dev_rpc.rs` — 3 symbols

```rust
pub struct DevRpcMiddleware<M>(M)
pub enum DevRpcMiddlewareError<M: Middleware>
pub fn new(inner: M) -> Self
```

#### `src/atipicial_clients/rpc/connections.rs` — 1 symbols

```rust
pub trait JsonRpcProvider: Debug + Send + Sync
```

#### `src/atipicial_clients/rpc/pubsub.rs` — 4 symbols

```rust
pub trait PubsubClient: JsonRpcProvider
pub struct SubscriptionStream<'a, P: PubsubClient, R: DeserializeOwned>
pub fn new(id: U256, provider: &'a RpcClient<P>) -> Result<Self, P::Error>
pub fn set_loaded_elements(&mut self, loaded_elements: VecDeque<R>)
```

#### `src/atipicial_clients/rpc/rpc_client.rs` — 9 symbols

```rust
pub enum AtipicialClient
pub struct RpcClient<P>
pub fn new(provider: P) -> Self
pub fn with_sender(mut self, address: impl Into<Address>) -> Self
pub fn set_interval<T: Into<Duration>>(&mut self, interval: T) -> &mut Self
pub fn interval<T: Into<Duration>>(mut self, interval: T) -> Self
pub fn url(&self) -> &Url
pub fn url_mut(&mut self) -> &mut Url
pub fn rw(r: Read, w: Write) -> Self
```

#### `src/atipicial_clients/rpc/transports/common.rs` — 28 symbols

```rust
pub struct JsonRpcError
pub fn is_retryable(&self) -> bool
pub fn is_rate_limited(&self) -> bool
pub fn retry_after(&self) -> Option<Duration>
pub fn is_unknown_transaction(&self) -> bool
pub fn is_already_known_transaction(&self) -> bool
pub fn is_transaction_rejection(&self) -> bool
pub fn is_revert(&self) -> bool
pub fn as_revert_data(&self) -> Option<Bytes>
pub struct Request<'a, T>
pub fn new(id: u64, method: &'a str, params: T) -> Self
pub enum Response<'a>
pub struct Params<'a>
pub enum Authorization
pub fn basic(username: impl AsRef<str>, password: impl AsRef<str>) -> Self
pub fn bearer(token: impl Into<String>) -> Self
pub fn raw(token: impl Into<String>) -> Self
pub const JWT_SECRET_LENGTH: usize = 32
pub struct JwtKey([u8
pub fn from_slice(key: &[u8]) -> Result<Self, String>
pub fn from_hex(hex: &str) -> Result<Self, String>
pub fn as_bytes(&self) -> &[u8
pub fn into_bytes(self) -> [u8
pub struct JwtAuth
pub fn new(secret: JwtKey, id: Option<String>, clv: Option<String>) -> Self
pub fn generate_token(&self) -> Result<String, Error>
pub fn validate_token(
pub struct Claims
```

#### `src/atipicial_clients/rpc/transports/http_provider.rs` — 8 symbols

```rust
pub struct HttpProvider
pub enum ClientError
pub fn new<T: TryInto<Url>>(url: T) -> Result<Self, T::Error>
pub fn url(&self) -> &Url
pub fn url_mut(&mut self) -> &mut Url
pub fn new_with_auth(
pub fn new_with_client(url: impl Into<Url>, client: reqwest::Client) -> Self
pub enum HttpClientError
```

#### `src/atipicial_clients/rpc/transports/ipc.rs` — 2 symbols

```rust
pub struct Ipc
pub enum IpcError
```

#### `src/atipicial_clients/rpc/transports/legacy_ws.rs` — 4 symbols

```rust
pub struct Ws
pub fn new<S>(ws: S) -> Self
pub fn ready(&self) -> bool
pub enum ClientError
```

#### `src/atipicial_clients/rpc/transports/mock.rs` — 10 symbols

```rust
pub enum MockResponse
pub struct MockProvider
pub fn new() -> Self
pub fn push_result(&self, method: impl Into<String>, result: Value)
pub fn push_result_with_params(&self, method: impl Into<String>, params: Value, result: Value)
pub fn push_result_with_partial_params(
pub fn push_error_any(&self, error: JsonRpcError)
pub fn push_error(&self, method: impl Into<String>, error: JsonRpcError)
pub fn take_requests(&self) -> VecDeque<(String, Value)>
pub fn assert_request<T: Serialize>(
```

#### `src/atipicial_clients/rpc/transports/retry.rs` — 12 symbols

```rust
pub trait RetryPolicy<E>: Send + Sync + Debug
pub struct RetryClient<T>
pub fn new(
pub fn set_compute_units(&mut self, cpus: u64) -> &mut Self
pub struct RetryClientBuilder
pub fn timeout_retries(mut self, timeout_retries: u32) -> Self
pub fn rate_limit_retries(mut self, rate_limit_retries: u32) -> Self
pub fn compute_units_per_second(mut self, compute_units_per_second: u64) -> Self
pub fn initial_backoff(mut self, initial_backoff: Duration) -> Self
pub fn build<T>(self, client: T, policy: Box<dyn RetryPolicy<T::Error>>) -> RetryClient<T>
pub enum RetryClientError
pub struct HttpRateLimitRetryPolicy
```

#### `src/atipicial_clients/rpc/transports/rw.rs` — 7 symbols

```rust
pub struct RwClient<Read, Write>
pub fn new(r: Read, w: Write) -> RwClient<Read, Write>
pub fn read_client(&self) -> &Read
pub fn write_client(&self) -> &Write
pub fn transpose(self) -> RwClient<Write, Read>
pub fn split(self) -> (Read, Write)
pub enum RwClientError<Read, Write>
```

#### `src/atipicial_clients/rpc/transports/ws/error.rs` — 1 symbols

```rust
pub enum WsClientError
```

#### `src/atipicial_clients/rpc/transports/ws/mod.rs` — 1 symbols

```rust
pub struct WsClient
```

#### `src/atipicial_clients/rpc/transports/ws/types.rs` — 3 symbols

```rust
pub struct ConnectionDetails
pub fn new(url: impl AsRef<str>, auth: Option<Authorization>) -> Self
pub fn new(url: impl AsRef<str>) -> Self
```

#### `src/atipicial_codec/binary_decoder.rs` — 30 symbols

```rust
pub struct Decoder<'a>
pub fn new(data: &'a [u8]) -> Self
pub fn read_u8_safe(&mut self) -> Result<u8, CodecError>
pub fn read_bool_safe(&mut self) -> Result<bool, CodecError>
pub fn read_bool(&mut self) -> bool
pub fn read_u8(&mut self) -> u8
pub fn read_u16(&mut self) -> Result<u16, CodecError>
pub fn read_i16(&mut self) -> Result<i16, CodecError>
pub fn read_u32(&mut self) -> Result<u32, CodecError>
pub fn read_i32(&mut self) -> Result<i32, CodecError>
pub fn read_u64(&mut self) -> Result<u64, CodecError>
pub fn read_i64(&mut self) -> Result<i64, CodecError>
pub fn read_bigint(&mut self) -> Result<BigInt, CodecError>
pub fn read_encoded_ec_point(&mut self) -> Result<Vec<u8>, CodecError>
pub fn read_bytes(&mut self, length: usize) -> Result<Vec<u8>, CodecError>
pub fn read_var_bytes(&mut self) -> Result<Vec<u8>, CodecError>
pub fn read_var_bytes_bounded(&mut self, max_len: usize) -> Result<Vec<u8>, CodecError>
pub fn read_var_int(&mut self) -> Result<i64, CodecError>
pub fn read_var_string(&mut self) -> Result<String, CodecError>
pub fn read_var_string_bounded(&mut self, max_len: usize) -> Result<String, CodecError>
pub fn read_push_bytes(&mut self) -> Result<Vec<u8>, CodecError>
pub fn read_push_int(&mut self) -> Result<BigInt, CodecError>
pub fn read_push_string(&mut self) -> Result<String, CodecError>
pub fn read_serializable<T: AtipicialSerializable>(&mut self) -> Result<T, CodecError>
pub fn read_serializable_list_bounded<T: AtipicialSerializable>(
pub fn read_serializable_list<T: AtipicialSerializable>(&mut self) -> Result<Vec<T>, CodecError>
pub fn read_serializable_list_var_bytes<T: AtipicialSerializable>(
pub fn mark(&mut self)
pub fn reset(&mut self)
pub fn available(&self) -> usize
```

#### `src/atipicial_codec/binary_encoder.rs` — 23 symbols

```rust
pub struct Encoder
pub fn new() -> Self
pub fn size(&self) -> usize
pub fn write_bool(&mut self, value: bool)
pub fn write_u8(&mut self, value: u8)
pub fn write_i16(&mut self, v: i16)
pub fn write_i32(&mut self, v: i32)
pub fn write_i64(&mut self, v: i64)
pub fn write_u16(&mut self, v: u16)
pub fn write_u32(&mut self, v: u32)
pub fn write_u64(&mut self, v: u64)
pub fn write_bytes(&mut self, bytes: &[u8])
pub fn write_var_int(&mut self, value: i64) -> Result<(), std::io::Error>
pub fn write_var_string(&mut self, v: &str)
pub fn write_fixed_string(
pub fn write_var_bytes(&mut self, bytes: &[u8]) -> Result<(), std::io::Error>
pub fn write_serializable_fixed<S: AtipicialSerializable>(&mut self, value: &S)
pub fn write_serializable_list_fixed<S: AtipicialSerializable>(&mut self, value: &[S])
pub fn write_serializable_variable_bytes<S: AtipicialSerializable>(
pub fn write_serializable_variable_list<S: AtipicialSerializable>(
pub fn write_serializable_variable_list_bytes<S: AtipicialSerializable>(
pub fn reset(&mut self)
pub fn to_bytes(&self) -> Vec<u8>
```

#### `src/atipicial_codec/encode.rs` — 2 symbols

```rust
pub trait AtipicialSerializable
pub trait VarSizeTrait
```

#### `src/atipicial_codec/error.rs` — 1 symbols

```rust
pub enum CodecError
```

#### `src/atipicial_config/config.rs` — 19 symbols

```rust
pub enum AtipicialNetwork
pub fn to_magic(&self) -> u32
pub fn from_magic(magic: u32) -> Option<AtipicialNetwork>
pub const DEFAULT_BLOCK_TIME: u64 = 15_000
pub const DEFAULT_ADDRESS_VERSION: u8 = 0x35
pub const MAX_VALID_UNTIL_BLOCK_INCREMENT_BASE: u64 = 86_400_000
pub struct AtipicialConfig
pub fn atipicial_config_lock() -> MutexGuard<'static, AtipicialConfig>
pub fn new(json_config: &str) -> Result<Self, serde_json::Error>
pub fn set_network(&mut self, magic: u32) -> Result<(), &'static str>
pub fn get_max_valid_until_block_increment(&self) -> u32
pub fn is_hardfork_enabled(&self, hardfork: &str, block_height: u32) -> bool
pub fn get_hardfork_height(&self, hardfork: &str) -> Option<u32>
pub fn set_hardfork_height(&mut self, hardfork: &str, height: u32)
pub fn mainnet() -> Self
pub fn testnet() -> Self
pub struct Counter
pub fn new() -> Self
pub fn get_and_increment(&self) -> u32
```

#### `src/atipicial_config/constant.rs` — 37 symbols

```rust
pub struct AtipicialConstants
pub const MAGIC_NUMBER_MAINNET: u32 = 860833102
pub const MAGIC_NUMBER_TESTNET: u32 = 894710606
pub const MAX_PUBLIC_KEYS_PER_MULTI_SIG: u32 = 1024
pub const HASH160_SIZE: u32 = 20
pub const HASH256_SIZE: u32 = 32
pub const PRIVATE_KEY_SIZE: u32 = 32
pub const PUBLIC_KEY_SIZE_COMPRESSED: u32 = 33
pub const SIGNATURE_SIZE: u32 = 64
pub const VERIFICATION_SCRIPT_SIZE: u32 = 40
pub const MAX_ITERATOR_ITEMS_DEFAULT: u32 = 100
pub const MAX_SUBITEMS: u32 = 16
pub const MAX_NESTING_DEPTH: u8 = 2
pub const CURRENT_TX_VERSION: u8 = 0
pub const MAX_TRANSACTION_SIZE: u32 = 102400
pub const MAX_TRANSACTION_ATTRIBUTES: u32 = 16
pub const MAX_SIGNER_SUBITEMS: u32 = 16
pub const MAX_MANIFEST_SIZE: u32 = 0xFFFF
pub const MAX_RPC_MESSAGE_SIZE: usize = 16 * 1024 * 1024
pub fn max_rpc_message_size() -> usize
pub fn max_rpc_message_size() -> usize
pub fn rpc_request_timeout() -> Option<core::time::Duration>
pub fn rpc_request_timeout() -> Option<core::time::Duration>
pub const SEED_1: &'static str = "http://seed1.atipicial.com:10332"
pub const SEED_2: &'static str = "http://seed2.atipicial.com:10332"
pub const SEED_3: &'static str = "http://seed3.atipicial.com:10332"
pub const SEED_4: &'static str = "http://seed4.atipicial.com:10332"
pub const SEED_5: &'static str = "http://seed5.atipicial.com:10332"
pub const SCRYPT_N: usize = 16384
pub const SCRYPT_R: u32 = 8
pub const SCRYPT_P: u32 = 8
pub const SCRYPT_LOG_N: u8 = 14
pub const SCRYPT_DK_LEN: usize = 64
pub const AEP_HEADER_1: u8 = 0x01
pub const AEP_HEADER_2: u8 = 0x42
pub const AEP_FLAG: u8 = 0xe0
pub fn new() -> Self
```

#### `src/atipicial_config/test_properties.rs` — 25 symbols

```rust
pub struct TestConstants
pub const TEST_RESOURCE_PATH: &'static str = "../../../test_resources/"
pub const DEFAULT_ACCOUNT_ADDRESS: &'static str = "NM7Aky765FG8NhhwtxjXRx7jEL1cnw7PBP"
pub const DEFAULT_ACCOUNT_SCRIPT_HASH: &'static str =
pub const DEFAULT_ACCOUNT_VERIFICATION_SCRIPT: &'static str =
pub const DEFAULT_ACCOUNT_PUBLIC_KEY: &'static str =
pub const DEFAULT_ACCOUNT_PRIVATE_KEY: &'static str =
pub const DEFAULT_ACCOUNT_ENCRYPTED_PRIVATE_KEY: &'static str =
pub const DEFAULT_ACCOUNT_WIF: &'static str =
pub const DEFAULT_ACCOUNT_PASSWORD: &'static str = "atipicial"
pub const COMMITTEE_ACCOUNT_ADDRESS: &'static str = "NXXazKH39yNFWWZF5MJ8tEN98VYHwzn7g3"
pub const COMMITTEE_ACCOUNT_SCRIPT_HASH: &'static str =
pub const COMMITTEE_ACCOUNT_VERIFICATION_SCRIPT: &'static str =
pub const CONTRACT_MANAGEMENT_HASH: &'static str = "fffdc93764dbaddd97c48f252a53ea4643faa3fd"
pub const STD_LIB_HASH: &'static str = "acce6fd80d44e1796aa0c2c625e9e4e0ce39efc0"
pub const CRYPTO_LIB_HASH: &'static str = "726cb6e0cd8628a1350a611384688911ab75f51b"
pub const LEDGER_CONTRACT_HASH: &'static str = "da65b600f7124ce6c79950c1772a36403104f2be"
pub const ATC_TOKEN_HASH: &'static str = "ef4073a0f2b305a38ec4050e4d3d28bc40ea63f5"
pub const GAS_TOKEN_HASH: &'static str = "d2a4cff31913016155e38e474a2c06d08be276cf"
pub const GAS_TOKEN_NAME: &'static str = "GasToken"
pub const POLICY_CONTRACT_HASH: &'static str = "cc5e4edd9f5f8dba8bb65734541df7a1c081c67b"
pub const ROLE_MANAGEMENT_HASH: &'static str = "49cf4e5378ffcd4dec034fd98a174c5491e395e2"
pub const ORACLE_CONTRACT_HASH: &'static str = "fe924b7cfe89ddd271abaf7210a80a7e11178758"
pub const NAME_SERVICE_HASH: &'static str = "7a8fcf0392cd625647907afa8e45cc66872b596b"
pub const CLIENT1_ACCOUNT_WIF: &'static str =
```

#### `src/atipicial_contract/atipicial_token.rs` — 8 symbols

```rust
pub struct AtipicialCoin<'a, P: JsonRpcProvider>
pub const NAME: &'static str = "AtipicialCoin"
pub const DECIMALS: u8 = 0
pub const SYMBOL: &'static str = "ATC"
pub const TOTAL_SUPPLY: u64 = 100_000_000
pub struct Candidate
pub struct AccountState
pub fn with_no_balance() -> Self
```

#### `src/atipicial_contract/atipicial_uri.rs` — 8 symbols

```rust
pub struct AtipicialURI<'a, P: JsonRpcProvider>
pub fn new(provider: Option<&'a RpcClient<P>>) -> Self
pub fn from_uri(uri_string: &str) -> Result<Self, ContractError>
pub fn uri_string(&self) -> Option<String>
pub fn recipient_address(&self) -> Option<String>
pub fn token_string(&self) -> Option<String>
pub fn token_str(&mut self, token_str: &str)
pub fn build_uri(&mut self) -> Result<Url, ContractError>
```

#### `src/atipicial_contract/contract_error.rs` — 1 symbols

```rust
pub enum ContractError
```

#### `src/atipicial_contract/contract_management.rs` — 4 symbols

```rust
pub struct ContractManagement<'a, P: JsonRpcProvider>
pub const NAME: &'static str = "ContractManagement"
pub fn new(provider: Option<&'a RpcClient<P>>) -> Self
pub fn with_script_hash(script_hash: H160, provider: Option<&'a RpcClient<P>>) -> Self
```

#### `src/atipicial_contract/fungible_token_contract.rs` — 2 symbols

```rust
pub struct FungibleTokenContract<'a, P: JsonRpcProvider>
pub fn new(script_hash: &H160, provider: Option<&'a RpcClient<P>>) -> Self
```

#### `src/atipicial_contract/gas_token.rs` — 5 symbols

```rust
pub struct GasToken<'a, P: JsonRpcProvider>
pub const NAME: &'static str = "GasToken"
pub const DECIMALS: u8 = 8
pub const SYMBOL: &'static str = "GAS"
pub fn new(provider: Option<&'a RpcClient<P>>) -> Self
```

#### `src/atipicial_contract/iterator.rs` — 2 symbols

```rust
pub struct AtipicialIterator<'a, T, P: JsonRpcProvider>
pub fn new(
```

#### `src/atipicial_contract/name_service.rs` — 6 symbols

```rust
pub enum RecordType
pub struct NameState
pub struct AtipicialNameService<'a, P: JsonRpcProvider>
pub const NAME: &'static str = "NameService"
pub fn new(provider: Option<&'a RpcClient<P>>) -> Result<Self, ContractError>
pub fn with_script_hash(script_hash: H160, provider: Option<&'a RpcClient<P>>) -> Self
```

#### `src/atipicial_contract/nft_contract.rs` — 2 symbols

```rust
pub struct NftContract<'a, P: JsonRpcProvider>
pub fn new(script_hash: &H160, provider: Option<&'a RpcClient<P>>) -> Self
```

#### `src/atipicial_contract/notary.rs` — 8 symbols

```rust
pub struct NotaryDeposit
pub fn from_stack_item(item: &StackItem) -> Result<Self, String>
pub struct NotaryContract<'a, P: JsonRpcProvider>
pub const NAME: &'static str = "Notary"
pub const DEFAULT_MAX_NOT_VALID_BEFORE_DELTA: u32 = 140
pub const DEFAULT_DEPOSIT_DELTA_TILL: u32 = 5760
pub fn new(provider: Option<&'a RpcClient<P>>) -> Self
pub fn supported_standards() -> Vec<&'static str>
```

#### `src/atipicial_contract/policy_contract.rs` — 14 symbols

```rust
pub struct PolicyContract<'a, P: JsonRpcProvider>
pub const NAME: &'static str = "PolicyContract"
pub const DEFAULT_EXEC_FEE_FACTOR: u32 = 30
pub const DEFAULT_STORAGE_PRICE: u32 = 100000
pub const DEFAULT_FEE_PER_BYTE: u32 = 1000
pub const DEFAULT_ATTRIBUTE_FEE: u32 = 0
pub const DEFAULT_NOTARY_ASSISTED_ATTRIBUTE_FEE: u32 = 10_000_000
pub const MAX_EXEC_FEE_FACTOR: u64 = 100
pub const MAX_ATTRIBUTE_FEE: u32 = 10_0000_0000
pub const MAX_STORAGE_PRICE: u32 = 10_000_000
pub const MAX_MILLISECONDS_PER_BLOCK: u32 = 30_000
pub const MAX_MAX_VALID_UNTIL_BLOCK_INCREMENT: u32 = 86400
pub const MAX_MAX_TRACEABLE_BLOCKS: u32 = 2_102_400
pub fn new(provider: Option<&'a RpcClient<P>>) -> Self
```

#### `src/atipicial_contract/role_management.rs` — 5 symbols

```rust
pub struct RoleManagement<'a, P: JsonRpcProvider>
pub const NAME: &'static str = "RoleManagement"
pub fn new(provider: Option<&'a RpcClient<P>>) -> Self
pub enum Role
pub const fn byte(self) -> u8
```

#### `src/atipicial_contract/treasury.rs` — 4 symbols

```rust
pub struct TreasuryContract<'a, P: JsonRpcProvider>
pub const NAME: &'static str = "Treasury"
pub fn new(provider: Option<&'a RpcClient<P>>) -> Self
pub fn supported_standards() -> Vec<&'static str>
```

#### `src/atipicial_contract/famous/atipicialburger.rs` — 10 symbols

```rust
pub struct AtipicialburgerContract<'a, P: JsonRpcProvider>
pub const CONTRACT_HASH: &'static str = "48c40d4666f93408be1bef038b6722404f5c4a5a"
pub const SYMBOL: &'static str = "bATC"
pub const DECIMALS: u8 = 8
pub const WRAP: &'static str = "wrap"
pub const UNWRAP: &'static str = "unwrap"
pub const CLAIM_GAS: &'static str = "claimGas"
pub const GET_RATE: &'static str = "getRate"
pub fn new(provider: Option<&'a RpcClient<P>>) -> Self
pub fn with_script_hash(script_hash: ScriptHash, provider: Option<&'a RpcClient<P>>) -> Self
```

#### `src/atipicial_contract/famous/atipicialcompound.rs` — 8 symbols

```rust
pub struct AtipicialCompoundContract<'a, P: JsonRpcProvider>
pub const CONTRACT_HASH: &'static str = "f0151f528127558851b39c2cd8aa47da7418ab28"
pub const DEPOSIT: &'static str = "deposit"
pub const WITHDRAW: &'static str = "withdraw"
pub const COMPOUND: &'static str = "compound"
pub const GET_APY: &'static str = "getAPY"
pub fn new(provider: Option<&'a RpcClient<P>>) -> Self
pub fn with_script_hash(script_hash: ScriptHash, provider: Option<&'a RpcClient<P>>) -> Self
```

#### `src/atipicial_contract/famous/contracts.rs` — 15 symbols

```rust
pub enum Network
pub struct FamousContract
pub fn new_unchecked(
pub fn new(
pub fn flamingo_flm_token() -> FamousContract
pub fn flamingo_flamingo_finance() -> FamousContract
pub fn ghostmarket() -> FamousContract
pub fn atipicialburger_dao() -> FamousContract
pub fn atipicialcompound() -> FamousContract
pub fn atipicial_name_service() -> FamousContract
pub fn bridge_atipicial_to_eth() -> FamousContract
pub fn testnet_nns() -> FamousContract
pub fn testnet_faucet() -> FamousContract
pub fn get_famous_contracts(network: Network) -> Vec<FamousContract>
pub fn get_all_famous_contracts() -> Vec<FamousContract>
```

#### `src/atipicial_contract/famous/flamingo.rs` — 9 symbols

```rust
pub struct FlamingoContract<'a, P: JsonRpcProvider>
pub const CONTRACT_HASH: &'static str = "f970f4cddcd087ab5d8a5697a32b3cfd32c8b465"
pub const SWAP: &'static str = "swap"
pub const ADD_LIQUIDITY: &'static str = "addLiquidity"
pub const REMOVE_LIQUIDITY: &'static str = "removeLiquidity"
pub const STAKE: &'static str = "stake"
pub const CLAIM_REWARDS: &'static str = "claimRewards"
pub fn new(provider: Option<&'a RpcClient<P>>) -> Self
pub fn with_script_hash(script_hash: ScriptHash, provider: Option<&'a RpcClient<P>>) -> Self
```

#### `src/atipicial_contract/famous/grandshare.rs` — 8 symbols

```rust
pub struct GrandShareContract<'a, P: JsonRpcProvider>
pub const CONTRACT_HASH: &'static str = "74f2dc36a68fdc4682034178eb2220729231db76"
pub const SUBMIT_PROPOSAL: &'static str = "submitProposal"
pub const VOTE: &'static str = "vote"
pub const FUND_PROJECT: &'static str = "fundProject"
pub const CLAIM_FUNDS: &'static str = "claimFunds"
pub fn new(provider: Option<&'a RpcClient<P>>) -> Self
pub fn with_script_hash(script_hash: ScriptHash, provider: Option<&'a RpcClient<P>>) -> Self
```

#### `src/atipicial_contract/traits/fungible_token.rs` — 1 symbols

```rust
pub trait FungibleTokenTrait<'a, P: JsonRpcProvider>: TokenTrait<'a, P>
```

#### `src/atipicial_contract/traits/nft.rs` — 1 symbols

```rust
pub trait NonFungibleTokenTrait<'a, P: JsonRpcProvider>: TokenTrait<'a, P> + Send
```

#### `src/atipicial_contract/traits/smart_contract.rs` — 1 symbols

```rust
pub trait SmartContractTrait<'a>: Send + Sync
```

#### `src/atipicial_contract/traits/token.rs` — 1 symbols

```rust
pub trait TokenTrait<'a, P: JsonRpcProvider>: SmartContractTrait<'a, P = P>
```

#### `src/atipicial_crypto/base58_helper.rs` — 5 symbols

```rust
pub enum Base58CheckError
pub fn base58check_encode(bytes: &[u8]) -> String
pub fn try_base58check_decode(input: &str) -> Result<Vec<u8>, Base58CheckError>
pub fn base58check_decode(input: &str) -> Option<Vec<u8>>
pub fn calculate_checksum(input: &[u8]) -> [u8
```

#### `src/atipicial_crypto/crypto_lib.rs` — 5 symbols

```rust
pub fn sha3_512(data: &[u8]) -> Vec<u8>
pub fn blake2b_512(data: &[u8]) -> Vec<u8>
pub fn verify_with_ed25519(message: &[u8], pubkey: &[u8], signature: &[u8]) -> bool
pub fn recover_secp256k1(message_hash: &[u8], signature: &[u8]) -> Option<Vec<u8>>
pub trait CryptoLibHashable
```

#### `src/atipicial_crypto/error.rs` — 3 symbols

```rust
pub enum CryptoError
pub enum Aep2Error
pub enum SignError
```

#### `src/atipicial_crypto/hash.rs` — 2 symbols

```rust
pub trait HashableForVec
pub trait HashableForString
```

#### `src/atipicial_crypto/key_pair.rs` — 19 symbols

```rust
pub struct KeyPair
pub fn new(private_key: Secp256r1PrivateKey, public_key: Secp256r1PublicKey) -> Self
pub fn private_key(&self) -> Result<Secp256r1PrivateKey, CryptoError>
pub fn private_key_ref(&self) -> Result<&Secp256r1PrivateKey, CryptoError>
pub fn public_key(&self) -> Secp256r1PublicKey
pub fn public_key_ref(&self) -> &Secp256r1PublicKey
pub fn has_private_key(&self) -> bool
pub fn from_secret_key(private_key: &Secp256r1PrivateKey) -> Self
pub fn private_key_bytes(&self) -> Result<[u8
pub fn public_key_bytes(&self) -> [u8
pub fn sign(&self, message: &[u8]) -> Result<Secp256r1Signature, CryptoError>
pub fn verify(
pub fn new_random() -> Self
pub fn from_private_key(private_key: &[u8
pub fn from_wif(wif: &str) -> Result<Self, CryptoError>
pub fn from_public_key(public_key: &[u8
pub fn export_as_wif(&self) -> Result<String, CryptoError>
pub fn get_script_hash(&self) -> ScriptHash
pub fn get_address(&self) -> String
```

#### `src/atipicial_crypto/keys.rs` — 26 symbols

```rust
pub struct Secp256r1PublicKey
pub struct Secp256r1PrivateKey
pub struct Secp256r1Signature
pub struct Secp256r1SignedMsg<T: Serialize>
pub fn new(gx: [u8
pub fn from_public_key(public_key: PublicKey) -> Self
pub fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError>
pub fn verify(
pub fn get_encoded(&self, compressed: bool) -> Vec<u8>
pub fn get_encoded_point(&self, compressed: bool) -> EncodedPoint
pub fn get_encoded_compressed_hex(&self) -> String
pub fn from_encoded(encoded: &str) -> Option<Self>
pub fn new_random() -> Self
pub fn random(rng: &mut OsRng) -> Self
pub fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError>
pub fn to_raw_bytes(&self) -> [u8
pub fn to_public_key(&self) -> Secp256r1PublicKey
pub fn erase(&mut self)
pub fn sign_tx(&self, message: &[u8]) -> Result<Secp256r1Signature, CryptoError>
pub fn sign_prehash(&self, message: &[u8]) -> Result<Secp256r1Signature, CryptoError>
pub fn from_scalars(r: &[u8
pub fn from_u256(r: U256, s: U256) -> Result<Self, CryptoError>
pub fn from_bytes(bytes: &[u8]) -> Result<Self, CryptoError>
pub fn to_bytes(&self) -> [u8
pub trait PrivateKeyExtension
pub trait PublicKeyExtension
```

#### `src/atipicial_crypto/utils.rs` — 10 symbols

```rust
pub fn private_key_to_public_key(private_key: &Secp256r1PrivateKey) -> Secp256r1PublicKey
pub fn private_key_to_hex_string(private_key: &Secp256r1PrivateKey) -> String
pub fn private_key_from_hex(hex: &str) -> Result<Secp256r1PrivateKey, CryptoError>
pub fn public_key_to_hex_string(public_key: &[u8]) -> String
pub fn public_key_from_hex(hex: &str) -> Result<Secp256r1PublicKey, CryptoError>
pub trait ToArray32
pub trait ToHexString
pub trait FromHexString
pub trait FromBase64String
pub trait ToBase64String
```

#### `src/atipicial_crypto/wif.rs` — 2 symbols

```rust
pub fn private_key_from_wif(wif: &str) -> Result<Secp256r1PrivateKey, CryptoError>
pub fn wif_from_private_key(private_key: &Secp256r1PrivateKey) -> String
```

#### `src/atipicial_error/mod.rs` — 9 symbols

```rust
pub enum LegacyError
pub enum CryptoError
pub enum WalletError
pub enum NetworkError
pub enum TransactionError
pub enum ContractError
pub enum SerializationError
pub type AtipicialResult<T> = Result<T, LegacyError>
pub trait ErrorContext<T>
```

#### `src/atipicial_error/unified.rs` — 32 symbols

```rust
pub enum AtipicialError
pub struct ErrorRecovery
pub fn new() -> Self
pub fn suggest(mut self, suggestion: impl Into<String>) -> Self
pub fn retryable(mut self, retryable: bool) -> Self
pub fn retry_after(mut self, duration: std::time::Duration) -> Self
pub fn doc(mut self, link: impl Into<String>) -> Self
pub type Result<T> = std::result::Result<T, AtipicialError>
pub trait ProvideErrorMetadata
pub enum AtipicialErrorKind
pub fn provider(context: &str, err: crate::atipicial_clients::ProviderError) -> Self
pub fn kind(&self) -> AtipicialErrorKind
pub fn recovery(&self) -> &ErrorRecovery
pub fn is_retryable(&self) -> bool
pub fn retry_after(&self) -> Option<std::time::Duration>
pub fn message(&self) -> &str
pub fn network<E: fmt::Display>(context: &str, err: E) -> Self
pub fn transaction<E: fmt::Display>(context: &str, err: E) -> Self
pub fn contract<E: fmt::Display>(
pub fn validation<V: Into<String>>(field: &str, value: Option<V>, message: &str) -> Self
pub fn wallet<E: fmt::Display>(context: &str, err: E) -> Self
pub struct ErrorBuilder
pub fn network(message: impl Into<String>) -> Self
pub fn wallet(message: impl Into<String>) -> Self
pub fn contract(message: impl Into<String>) -> Self
pub fn source(mut self, source: impl std::error::Error + Send + Sync + 'static) -> Self
pub fn with_contract(mut self, contract: impl Into<String>) -> Self
pub fn with_method(mut self, method: impl Into<String>) -> Self
pub fn suggest(mut self, suggestion: impl Into<String>) -> Self
pub fn retryable(mut self) -> Self
pub fn build(self) -> AtipicialError
pub trait ErrorContextExt
```

#### `src/atipicial_fs/acl.rs` — 14 symbols

```rust
pub enum Operation
pub enum ContainerOperation
pub enum ObjectOperation
pub struct Target
pub enum TargetRole
pub enum Action
pub struct Filter
pub enum FilterOperation
pub struct EACLRecord
pub struct EACL
pub fn new(container_id: ContainerId) -> Self
pub fn add_record(&mut self, record: EACLRecord)
pub struct BearerToken
pub struct SessionToken
```

#### `src/atipicial_fs/client.rs` — 4 symbols

```rust
pub struct AtipicialFsClient
pub fn new(config: AtipicialFsConfig) -> Self
pub fn with_account(mut self, account: Account) -> Self
pub fn get_owner_id(&self) -> AtipicialFsResult<OwnerId>
```

#### `src/atipicial_fs/container.rs` — 13 symbols

```rust
pub struct Version
pub struct Container
pub fn new(id: ContainerId, owner_id: OwnerId) -> Self
pub fn with_basic_acl(mut self, acl: u32) -> Self
pub fn with_name(mut self, name: String) -> Self
pub fn with_creation(mut self, creation: DateTime<Utc>) -> Self
pub fn with_version(mut self, version: Version) -> Self
pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self
pub struct BasicACL
pub fn full_access() -> Self
pub fn read_only() -> Self
pub fn to_bitmask(&self) -> u32
pub fn from_bitmask(bitmask: u32) -> Self
```

#### `src/atipicial_fs/error.rs` — 2 symbols

```rust
pub enum AtipicialFsError
pub type AtipicialFsResult<T> = std::result::Result<T, AtipicialFsError>
```

#### `src/atipicial_fs/mod.rs` — 17 symbols

```rust
pub const DEFAULT_MAINNET_ENDPOINT: &str = "grpc.mainnet.fs.atipicial.com:8082"
pub const DEFAULT_TESTNET_ENDPOINT: &str = "grpc.testnet.fs.atipicial.com:8082"
pub const DEFAULT_ENDPOINT: &str = DEFAULT_MAINNET_ENDPOINT
pub const DEFAULT_MAINNET_HTTP_GATEWAY: &str = "https://http.mainnet.fs.atipicial.com"
pub const DEFAULT_TESTNET_HTTP_GATEWAY: &str = "https://http.testnet.fs.atipicial.com"
pub const DEFAULT_MAINNET_REST_API: &str = "https://rest.mainnet.fs.atipicial.com"
pub const DEFAULT_TESTNET_REST_API: &str = "https://rest.testnet.fs.atipicial.com"
pub struct AtipicialFsConfig
pub fn builder() -> AtipicialFsConfigBuilder
pub struct AtipicialFsConfigBuilder
pub fn endpoint(mut self, val: String) -> Self
pub fn auth(mut self, val: AtipicialFsAuth) -> Self
pub fn timeout_sec(mut self, val: u64) -> Self
pub fn insecure(mut self, val: bool) -> Self
pub fn build(self) -> AtipicialFsConfig
pub struct AtipicialFsAuth
pub trait AtipicialFsService
```

#### `src/atipicial_fs/object.rs` — 12 symbols

```rust
pub struct Object
pub fn new(container_id: ContainerId, owner_id: OwnerId) -> Self
pub fn with_payload(mut self, payload: Vec<u8>) -> Self
pub fn with_type(mut self, object_type: ObjectType) -> Self
pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self
pub fn with_filename(self, filename: impl Into<String>) -> Self
pub fn with_content_type(self, content_type: impl Into<String>) -> Self
pub fn size(&self) -> usize
pub struct MultipartUpload
pub struct Part
pub fn new(part_number: u32, payload: Vec<u8>) -> Self
pub struct MultipartUploadResult
```

#### `src/atipicial_fs/types.rs` — 15 symbols

```rust
pub struct ContainerId(pub String)
pub struct ObjectId(pub String)
pub struct OwnerId(pub String)
pub struct PlacementPolicy
pub struct Selector
pub struct Filter
pub enum ClauseOperator
pub enum MatchOperator
pub enum ObjectType
pub struct Attributes
pub fn new() -> Self
pub fn add(&mut self, key: impl Into<String>, value: impl Into<String>)
pub fn get(&self, key: &str) -> Option<&String>
pub enum AccessPermission
pub struct SessionToken
```

#### `src/atipicial_protocol/account.rs` — 9 symbols

```rust
pub trait AccountTrait: Sized + PartialEq + Send + Sync + Debug + Clone
pub struct Account
pub fn get_address(&self) -> String
pub fn get_script_hash(&self) -> H160
pub fn get_verification_script(&self) -> Option<VerificationScript>
pub fn get_public_key(&self) -> Option<Secp256r1PublicKey>
pub fn decrypt_private_key_with_params(
pub fn encrypt_private_key_with_params(
pub fn to_aep6_account(&self) -> Result<AEP6Account, ProviderError>
```

#### `src/atipicial_protocol/aep2.rs` — 11 symbols

```rust
pub struct AEP2
pub fn encrypt(password: &str, key_pair: &KeyPair) -> Result<String, Aep2Error>
pub fn encrypt_with_params(
pub fn decrypt(password: &str, aep2: &str) -> Result<KeyPair, Aep2Error>
pub fn decrypt_with_params(
pub fn encrypt_for_test_vector(
pub fn decrypt_for_test_vector(password: &str, aep2: &str) -> Result<KeyPair, Aep2Error>
pub fn encrypt_test_vector() -> Result<String, Aep2Error>
pub fn decrypt_test_vector(_password: &str, _aep2: &str) -> Result<KeyPair, Aep2Error>
pub fn get_aep2_from_private_key(
pub fn get_private_key_from_aep2(
```

#### `src/atipicial_protocol/protocol_error.rs` — 1 symbols

```rust
pub enum ProtocolError
```

#### `src/atipicial_protocol/responses/atipicial_account_state.rs` — 3 symbols

```rust
pub struct AccountState
pub fn with_no_vote(balance: i64, update_height: i64) -> Self
pub fn with_no_balance() -> Self
```

#### `src/atipicial_protocol/responses/atipicial_address.rs` — 1 symbols

```rust
pub struct AtipicialAddress
```

#### `src/atipicial_protocol/responses/atipicial_application_log.rs` — 8 symbols

```rust
pub struct ApplicationLog
pub fn get_first_execution(&self) -> Result<&Execution, TypeError>
pub fn get_execution(&self, index: usize) -> Result<&Execution, TypeError>
pub struct Execution
pub fn get_first_stack_item(&self) -> Result<&StackItem, TypeError>
pub fn get_stack_item(&self, index: usize) -> Result<&StackItem, TypeError>
pub fn get_first_notification(&self) -> Result<&LogNotification, TypeError>
pub fn get_notification(&self, index: usize) -> Result<&LogNotification, TypeError>
```

#### `src/atipicial_protocol/responses/atipicial_balances.rs` — 8 symbols

```rust
pub struct Aep11Balances
pub struct Aep11Balance
pub fn new(
pub struct Aep11Token
pub fn new(token_id: String, amount: String, last_updated_block: u32) -> Self
pub struct Aep17Balances
pub struct Aep17Balance
pub fn new(
```

#### `src/atipicial_protocol/responses/atipicial_block.rs` — 2 symbols

```rust
pub struct AtipicialBlock
pub fn get_nonce_as_u64(&self) -> Result<u64, ParseIntError>
```

#### `src/atipicial_protocol/responses/atipicial_find_states.rs` — 2 symbols

```rust
pub struct States
pub struct StateResult
```

#### `src/atipicial_protocol/responses/atipicial_get_claimable.rs` — 2 symbols

```rust
pub struct Claimables
pub struct Claim
```

#### `src/atipicial_protocol/responses/atipicial_get_mem_pool.rs` — 1 symbols

```rust
pub struct MemPoolDetails
```

#### `src/atipicial_protocol/responses/atipicial_get_next_block_validators.rs` — 2 symbols

```rust
pub struct Validator
pub fn new(public_key: String, votes: String, active: bool) -> Self
```

#### `src/atipicial_protocol/responses/atipicial_get_peers.rs` — 3 symbols

```rust
pub struct Peers
pub struct AddressEntry
pub fn new(address: String, port: u16) -> Self
```

#### `src/atipicial_protocol/responses/atipicial_get_state_height.rs` — 1 symbols

```rust
pub struct StateHeight
```

#### `src/atipicial_protocol/responses/atipicial_get_state_root.rs` — 1 symbols

```rust
pub struct StateRoot
```

#### `src/atipicial_protocol/responses/atipicial_get_token_balances.rs` — 2 symbols

```rust
pub trait TokenBalances<'a>: Serialize + Deserialize<'a> + Clone + PartialEq + Eq + Hash
pub trait TokenBalance<'a>: Serialize + Deserialize<'a> + Clone
```

#### `src/atipicial_protocol/responses/atipicial_get_token_transfers.rs` — 2 symbols

```rust
pub trait TokenTransfers<'a>: Serialize + Deserialize<'a> + Clone + PartialEq + Eq + Hash
pub trait TokenTransfer<'a>: Serialize + Deserialize<'a> + Clone + PartialEq + Eq + Hash
```

#### `src/atipicial_protocol/responses/atipicial_get_unclaimed_gas.rs` — 1 symbols

```rust
pub struct UnclaimedGas
```

#### `src/atipicial_protocol/responses/atipicial_get_unspents.rs` — 3 symbols

```rust
pub struct Unspents
pub struct Balance
pub struct UnspentTransaction
```

#### `src/atipicial_protocol/responses/atipicial_get_version.rs` — 4 symbols

```rust
pub struct AtipicialVersion
pub struct AtipicialRpcSettings
pub struct AtipicialProtocol
pub struct HardForks
```

#### `src/atipicial_protocol/responses/atipicial_get_wallet_balance.rs` — 1 symbols

```rust
pub struct Balance
```

#### `src/atipicial_protocol/responses/atipicial_list_plugins.rs` — 1 symbols

```rust
pub struct Plugin
```

#### `src/atipicial_protocol/responses/atipicial_network_fee.rs` — 1 symbols

```rust
pub struct AtipicialNetworkFee
```

#### `src/atipicial_protocol/responses/atipicial_send_raw_transaction.rs` — 2 symbols

```rust
pub struct RawTransaction
pub fn new(hash: H256) -> Self
```

#### `src/atipicial_protocol/responses/atipicial_submit_block.rs` — 2 symbols

```rust
pub struct SubmitBlock(bool)
pub fn get_submit_block(&self) -> bool
```

#### `src/atipicial_protocol/responses/atipicial_transaction_result.rs` — 2 symbols

```rust
pub struct TransactionResult
pub struct AtipicialTransactionSigner
```

#### `src/atipicial_protocol/responses/atipicial_transfers.rs` — 6 symbols

```rust
pub struct Aep11Transfers
pub struct Aep11Transfer
pub fn new(
pub struct Aep17Transfers
pub struct Aep17Transfer
pub fn new(
```

#### `src/atipicial_protocol/responses/atipicial_validate_address.rs` — 2 symbols

```rust
pub struct ValidateAddress
pub fn new(address: String, is_valid: bool) -> Self
```

#### `src/atipicial_protocol/responses/atipicial_witness.rs` — 3 symbols

```rust
pub struct AtipicialWitness
pub fn new(invocation: String, verification: String) -> Self
pub fn from_witness(witness: Witness) -> Self
```

#### `src/atipicial_protocol/responses/diagnostics.rs` — 4 symbols

```rust
pub struct Diagnostics
pub fn new(invoked_contracts: InvokedContract, storage_changes: Vec<StorageChange>) -> Self
pub struct InvokedContract
pub struct StorageChange
```

#### `src/atipicial_protocol/responses/express_contract_state.rs` — 2 symbols

```rust
pub struct ExpressContractState
pub fn new(hash: H160, manifest: ContractManifest) -> Self
```

#### `src/atipicial_protocol/responses/express_shutdown.rs` — 2 symbols

```rust
pub struct ExpressShutdown
pub fn new(process_id: i32) -> Self
```

#### `src/atipicial_protocol/responses/notification.rs` — 2 symbols

```rust
pub struct LogNotification
pub fn new(contract: H160, event_name: String, state: StackItem) -> Self
```

#### `src/atipicial_protocol/responses/oracle_request.rs` — 2 symbols

```rust
pub struct OracleRequest
pub fn new(
```

#### `src/atipicial_protocol/responses/populated_blocks.rs` — 2 symbols

```rust
pub struct PopulatedBlocks
pub fn new(cache_id: String, blocks: Vec<i32>) -> Self
```

#### `src/atipicial_protocol/responses/response_transaction.rs` — 6 symbols

```rust
pub struct RTransaction
pub fn new(
pub fn get_first_signer(&self) -> Result<&RTransactionSigner, TypeError>
pub fn get_signer(&self, index: usize) -> Result<&RTransactionSigner, TypeError>
pub fn get_first_attribute(&self) -> Result<&TransactionAttributeEnum, TypeError>
pub fn get_attribute(&self, index: usize) -> Result<&TransactionAttributeEnum, TypeError>
```

#### `src/atipicial_protocol/responses/response_transaction_attribute.rs` — 7 symbols

```rust
pub enum TransactionAttributeType
pub struct HighPriorityAttribute
pub struct OracleResponseAttribute
pub struct NotValidBeforeAttribute
pub struct ConflictsAttribute
pub enum TransactionAttributeEnum
pub struct OracleResponse
```

#### `src/atipicial_protocol/responses/response_transaction_signer.rs` — 11 symbols

```rust
pub struct RTransactionSigner
pub fn new(account: H160, scopes: Vec<WitnessScope>) -> Self
pub fn new_full(
pub fn get_first_scope(&self) -> Result<&WitnessScope, TypeError>
pub fn get_scope(&self, index: usize) -> Result<&WitnessScope, TypeError>
pub fn get_first_allowed_contract(&self) -> Result<&H160, TypeError>
pub fn get_allowed_contract(&self, index: usize) -> Result<&H160, TypeError>
pub fn get_first_allowed_group(&self) -> Result<&Secp256r1PublicKey, TypeError>
pub fn get_allowed_group(&self, index: usize) -> Result<&Secp256r1PublicKey, TypeError>
pub fn get_first_rule(&self) -> Result<&WitnessRule, TypeError>
pub fn get_rule(&self, index: usize) -> Result<&WitnessRule, TypeError>
```

#### `src/atipicial_sgx/allocator.rs` — 10 symbols

```rust
pub struct SgxAllocator
pub const fn new(heap_base: usize, heap_size: usize) -> Self
pub fn memory_usage(&self) -> usize
pub fn available_memory(&self) -> usize
pub fn init_allocator() -> Result<(), SgxError>
pub struct SgxAllocator
pub fn new(_heap_base: usize, _heap_size: usize) -> Self
pub fn memory_usage(&self) -> usize
pub fn available_memory(&self) -> usize
pub fn init_allocator() -> Result<(), SgxError>
```

#### `src/atipicial_sgx/attestation.rs` — 16 symbols

```rust
pub struct RemoteAttestation
pub fn new() -> Self
pub fn configure_spid(&mut self, spid: [u8
pub fn init_attestation(&mut self, sp_pub_key: &[u8
pub fn init_attestation(&mut self, sp_pub_key: &[u8
pub fn generate_quote(&mut self, user_data: &[u8]) -> Result<Vec<u8>, SgxError>
pub fn generate_quote(&mut self, user_data: &[u8]) -> Result<Vec<u8>, SgxError>
pub fn get_quote(&self) -> Option<&[u8]>
pub fn close(&mut self) -> Result<(), SgxError>
pub fn close(&mut self) -> Result<(), SgxError>
pub struct QuoteVerifier
pub fn new() -> Self
pub fn configure_ias(&mut self, api_key: String, url: String)
pub fn verify_quote(&self, quote: &[u8]) -> Result<QuoteVerificationResult, SgxError>
pub struct QuoteVerificationResult
pub enum TcbStatus
```

#### `src/atipicial_sgx/crypto.rs` — 15 symbols

```rust
pub struct SgxCrypto
pub fn compute_shared_secret(
pub fn new() -> Result<Self, SgxError>
pub fn sha256(&self, data: &[u8]) -> Result<[u8
pub fn sign_ecdsa(&self, message: &[u8], private_key: &[u8
pub fn verify_ecdsa(
pub fn random_bytes(&self, size: usize) -> Result<Vec<u8>, SgxError>
pub fn generate_keypair(&self) -> Result<([u8
pub struct SgxKeyManager
pub fn new() -> Self
pub fn seal_key(&mut self, key_id: &[u8
pub fn unseal_key(&self, key_id: &[u8
pub fn seal_key(&mut self, _key_id: &[u8
pub fn unseal_key(&self, _key_id: &[u8
pub fn init_crypto() -> Result<(), SgxError>
```

#### `src/atipicial_sgx/enclave.rs` — 16 symbols

```rust
pub struct EnclaveConfig
pub struct EnclaveAttributes
pub struct SgxEnclave
pub fn new(config: EnclaveConfig) -> Result<Self, SgxError>
pub fn initialize(&mut self) -> Result<(), SgxError>
pub fn initialize(&mut self) -> Result<(), SgxError>
pub fn is_initialized(&self) -> bool
pub fn config(&self) -> &EnclaveConfig
pub fn ecall<T, R>(&self, function_id: u32, input: &T) -> Result<R, SgxError>
pub fn ecall<T, R>(&self, _function_id: u32, _input: &T) -> Result<R, SgxError>
pub fn destroy(&mut self) -> Result<(), SgxError>
pub fn destroy(&mut self) -> Result<(), SgxError>
pub trait EnclaveSerializable
pub trait EnclaveDeserializable: Sized
pub fn generate_enclave_config(config: &EnclaveConfig) -> String
pub fn generate_edl() -> &'static str
```

#### `src/atipicial_sgx/mod.rs` — 2 symbols

```rust
pub fn init_sgx() -> Result<(), SgxError>
pub enum SgxError
```

#### `src/atipicial_sgx/networking.rs` — 11 symbols

```rust
pub struct SgxNetworking
pub struct SecureChannel
pub fn new() -> Self
pub fn establish_channel(&mut self, remote_id: &[u8
pub fn establish_channel(&mut self, remote_id: &[u8
pub fn send_secure(
pub fn receive_secure(
pub fn complete_handshake(&mut self, remote_public_key: &[u8
pub fn is_established(&self) -> bool
pub fn ocall_network_request(request: &[u8]) -> Result<Vec<u8>, SgxError>
pub fn ocall_network_request(_request: &[u8]) -> Result<Vec<u8>, SgxError>
```

#### `src/atipicial_sgx/storage.rs` — 10 symbols

```rust
pub struct SecureStorage
pub fn new() -> Self
pub fn store(&mut self, key_id: &[u8
pub fn store(&mut self, _key_id: &[u8
pub fn retrieve(&self, key_id: &[u8
pub fn retrieve(&self, _key_id: &[u8
pub fn delete(&mut self, key_id: &[u8
pub fn delete(&mut self, _key_id: &[u8
pub fn list_keys(&self) -> Vec<[u8
pub fn list_keys(&self) -> Vec<[u8
```

#### `src/atipicial_types/address.rs` — 4 symbols

```rust
pub type Address = String
pub enum NameOrAddress
pub trait AddressExtension
pub fn from_script_hash(script_hash: &H160) -> Result<String, AtipicialError>
```

#### `src/atipicial_types/address_or_scripthash.rs` — 5 symbols

```rust
pub enum AddressOrScriptHash
pub fn try_from_script_hash_bytes(bytes: &[u8]) -> Result<Self, TypeError>
pub fn address(&self) -> Address
pub fn try_script_hash(&self) -> Result<H160, TypeError>
pub fn script_hash(&self) -> H160
```

#### `src/atipicial_types/bytes.rs` — 1 symbols

```rust
pub trait ReverseTrait
```

#### `src/atipicial_types/error.rs` — 1 symbols

```rust
pub enum TypeError
```

#### `src/atipicial_types/hardfork.rs` — 4 symbols

```rust
pub enum Hardfork
pub fn all() -> &'static [Hardfork]
pub fn name(&self) -> &'static str
pub fn description(&self) -> &'static str
```

#### `src/atipicial_types/mod.rs` — 8 symbols

```rust
pub type Byte = u8
pub type Bytes = Vec<u8>
pub type TxHash = H256
pub trait ExternBase64
pub struct ScryptParamsDef
pub trait Base64Encode
pub trait TryBase64Encode
pub fn to_checksum(addr: &ScriptHash, chain_id: Option<u8>) -> String
```

#### `src/atipicial_types/numeric.rs` — 2 symbols

```rust
pub trait ToBytesPadded
pub trait ToBytes
```

#### `src/atipicial_types/op_code.rs` — 8 symbols

```rust
pub enum OpCode
pub fn price(self) -> u32
pub fn opcode(self) -> u8
pub fn to_hex_string(self) -> String
pub fn operand_size(self) -> Option<OperandSize>
pub struct OperandSize
pub fn with_size(size: u8) -> Self
pub fn with_prefix_size(prefix_size: u8) -> Self
```

#### `src/atipicial_types/path_or_string.rs` — 2 symbols

```rust
pub enum PathOrString
pub fn read(&self) -> Result<String, std::io::Error>
```

#### `src/atipicial_types/plugin_type.rs` — 2 symbols

```rust
pub enum NodePluginType
pub fn value_of_name(name: &str) -> Result<Self, &'static str>
```

#### `src/atipicial_types/script_hash.rs` — 7 symbols

```rust
pub type ScriptHash = H160
pub trait ScriptHashExtension
pub struct Address(pub String)
pub fn new(address: impl Into<String>) -> Self
pub fn as_str(&self) -> &str
pub fn to_script_hash(&self) -> Result<ScriptHash, TypeError>
pub trait IntoScriptHash
```

#### `src/atipicial_types/serde_value.rs` — 1 symbols

```rust
pub trait ValueExtension
```

#### `src/atipicial_types/serde_with_utils.rs` — 70 symbols

```rust
pub fn serialize_h160_without_0x<S>(h160: &H160, serializer: S) -> Result<S::Ok, S::Error>
pub fn serialize_h160<S>(item: &H160, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_h160<'de, D>(deserializer: D) -> Result<H160, D::Error>
pub fn serialize_scopes<S>(scopes: &[WitnessScope], serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_scopes<'de, D>(deserializer: D) -> Result<Vec<WitnessScope>, D::Error>
pub fn serialize_boolean_expression<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_boolean_expression<'de, D>(deserializer: D) -> Result<bool, D::Error>
pub fn serialize_bytes<S>(item: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_bytes<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
pub fn serialize_url<S>(item: Url, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_pubkey<'de, D>(deserializer: D) -> Result<Secp256r1PublicKey, D::Error>
pub fn serialize_pubkey<S>(item: Secp256r1PublicKey, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
pub fn serialize_url_option<S>(item: &Option<Url>, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_url_option<'de, D>(deserializer: D) -> Result<Option<Url>, D::Error>
pub fn serialize_u256<S>(item: &U256, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_u256<'de, D>(deserializer: D) -> Result<U256, D::Error>
pub fn serialize_u256_option<S>(item: &Option<U256>, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_u256_option<'de, D>(deserializer: D) -> Result<Option<U256>, D::Error>
pub fn serialize_u32<S>(item: &u32, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
pub fn serialize_u64<S>(item: &u64, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
pub fn serialize_u64_option<S>(item: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_u64_option<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
pub fn deserialize_script_hash<'de, D>(deserializer: D) -> Result<ScriptHash, D::Error>
pub fn serialize_script_hash<S>(item: &ScriptHash, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_address_or_script_hash<'de, D>(
pub fn serialize_address_or_script_hash<S>(
pub fn deserialize_vec_script_hash<'de, D>(deserializer: D) -> Result<Vec<ScriptHash>, D::Error>
pub fn serialize_vec_script_hash<S>(
pub fn deserialize_vec_script_hash_option<'de, D>(
pub fn serialize_vec_script_hash_option<S>(
pub fn serialize_script_hash_option<S>(
pub fn deserialize_script_hash_option<'de, D>(
pub fn serialize_hash_map_h160_account<S, Account>(
pub fn deserialize_hash_map_h160_account<'de, D, Account>(
pub fn deserialize_private_key<'de, D>(deserializer: D) -> Result<Secp256r1PrivateKey, D::Error>
pub fn serialize_private_key<S>(
pub fn deserialize_public_key<'de, D>(deserializer: D) -> Result<Secp256r1PublicKey, D::Error>
pub fn serialize_public_key<S>(item: &Secp256r1PublicKey, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_vec_public_key<'de, D>(
pub fn deserialize_vec_public_key_option<'de, D>(
pub fn serialize_vec_public_key<S>(
pub fn serialize_vec_public_key_option<S>(
pub fn serialize_public_key_option<S>(
pub fn deserialize_public_key_option<'de, D>(
pub fn serialize_h256<S>(item: &H256, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_h256<'de, D>(deserializer: D) -> Result<H256, D::Error>
pub fn serialize_hashset_u256<S>(item: &HashSet<U256>, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_hashset_u256<'de, D>(deserializer: D) -> Result<HashSet<U256>, D::Error>
pub fn serialize_vec_h256<S>(item: &Vec<H256>, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_vec_h256<'de, D>(deserializer: D) -> Result<Vec<H256>, D::Error>
pub fn serialize_vec_u256<S>(item: &Vec<U256>, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_vec_u256<'de, D>(deserializer: D) -> Result<Vec<U256>, D::Error>
pub fn serialize_h256_option<S>(item: &Option<H256>, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_h256_option<'de, D>(deserializer: D) -> Result<Option<H256>, D::Error>
pub fn serialize_hashmap_u256_hashset_u256<S>(
pub fn deserialize_hashmap_u256_hashset_u256<'de, D>(
pub fn serialize_hashmap_address_u256<S>(
pub fn deserialize_hashmap_address_u256<'de, D>(
pub fn serialize_hashmap_u256_hashset_h256<S>(
pub fn deserialize_hashmap_u256_hashset_h256<'de, D>(
pub fn serialize_hashmap_u256_vec_u256<S>(
pub fn deserialize_hashmap_u256_vec_u256<'de, D>(
pub fn serialize_map<S>(
pub fn deserialize_map<'de, D>(
pub fn serialize_wildcard<S>(methods: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
pub fn deserialize_wildcard<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
pub fn deserialize_hardforks<'de, D>(deserializer: D) -> Result<Vec<HardForks>, D::Error>
```

#### `src/atipicial_types/stack_item.rs` — 42 symbols

```rust
pub enum StackItem
pub struct MapEntry
pub const ANY_VALUE: &'static str = "Any"
pub const POINTER_VALUE: &'static str = "Pointer"
pub const BOOLEAN_VALUE: &'static str = "Boolean"
pub const INTEGER_VALUE: &'static str = "Integer"
pub const BYTE_STRING_VALUE: &'static str = "ByteString"
pub const BUFFER_VALUE: &'static str = "Buffer"
pub const ARRAY_VALUE: &'static str = "Array"
pub const STRUCT_VALUE: &'static str = "Struct"
pub const MAP_VALUE: &'static str = "Map"
pub const INTEROP_INTERFACE_VALUE: &'static str = "InteropInterface"
pub const ANY_BYTE: u8 = 0x00
pub const POINTER_BYTE: u8 = 0x10
pub const BOOLEAN_BYTE: u8 = 0x20
pub const INTEGER_BYTE: u8 = 0x21
pub const BYTE_STRING_BYTE: u8 = 0x28
pub const BUFFER_BYTE: u8 = 0x30
pub const ARRAY_BYTE: u8 = 0x40
pub const STRUCT_BYTE: u8 = 0x41
pub const MAP_BYTE: u8 = 0x48
pub const INTEROP_INTERFACE_BYTE: u8 = 0x60
pub fn new_byte_string(byte_array: Vec<u8>) -> Self
pub fn as_bool(&self) -> Option<bool>
pub fn as_string(&self) -> Option<String>
pub fn to_string(&self) -> String
pub fn as_bytes(&self) -> Option<Vec<u8>>
pub fn as_array(&self) -> Option<Vec<StackItem>>
pub fn as_array_ref(&self) -> Option<&[StackItem]>
pub fn as_int(&self) -> Option<i64>
pub fn as_map(&self) -> Option<HashMap<StackItem, StackItem>>
pub fn as_map_entries(&self) -> Option<&[MapEntry]>
pub fn as_address(&self) -> Option<Address>
pub fn as_public_key(&self) -> Option<Secp256r1PublicKey>
pub fn as_hash160(&self) -> Option<H160>
pub fn as_hash256(&self) -> Option<H256>
pub fn as_interop(&self, interface_name: &str) -> Option<StackItem>
pub fn len(&self) -> Option<usize>
pub fn is_empty(&self) -> Option<bool>
pub fn get(&self, index: usize) -> Option<StackItem>
pub fn get_iterator_id(&self) -> Option<&String>
pub fn get_interface_name(&self) -> Option<&String>
```

#### `src/atipicial_types/string.rs` — 2 symbols

```rust
pub trait TryStringExt
pub trait StringExt
```

#### `src/atipicial_types/syncing.rs` — 2 symbols

```rust
pub enum SyncingStatus
pub struct SyncProgress
```

#### `src/atipicial_types/tx_pool.rs` — 4 symbols

```rust
pub struct TxPoolInspectSummary
pub struct TxpoolContent<TX>
pub struct TxpoolInspect
pub struct TxpoolStatus
```

#### `src/atipicial_types/url_session.rs` — 1 symbols

```rust
pub struct URLSession
```

#### `src/atipicial_types/util.rs` — 19 symbols

```rust
pub fn parse_string_u64(u64_str: &str) -> Result<u64, TypeError>
pub fn parse_string_u256(u256_str: &str) -> Result<U256, TypeError>
pub fn parse_address(address: &str) -> Result<ScriptHash, TypeError>
pub fn encode_string_h160(h160: &H160) -> String
pub fn parse_string_h160(h160_str: &str) -> Result<H160, TypeError>
pub fn parse_string_h256(h256_str: &str) -> Result<H256, TypeError>
pub fn encode_string_h256(h256: &H256) -> String
pub fn encode_string_u256(u256: &U256) -> String
pub fn encode_vec_string_vec_u256(item: Vec<U256>) -> Vec<String>
pub fn parse_vec_string_vec_u256(item: Vec<String>) -> Result<Vec<U256>, TypeError>
pub fn h256_to_u256(item: H256) -> U256
pub fn bytes_to_string(mybytes: &[u8]) -> String
pub fn string_to_bytes(mystring: &str) -> Result<Vec<u8>, TypeError>
pub fn u256_sqrt(input: &U256) -> U256
pub fn u256_min(x: U256, y: U256) -> U256
pub fn vec_to_array32(vec: Vec<u8>) -> Result<[u8
pub fn var_size(value: usize) -> usize
pub trait ToBase58
pub trait ToBase64
```

#### `src/atipicial_types/vm_state.rs` — 1 symbols

```rust
pub enum VMState
```

#### `src/atipicial_types/whitelisted_contract.rs` — 3 symbols

```rust
pub struct WhitelistedContract
pub fn new(contract_hash: H160, method: String, arg_count: i32, fixed_fee: i64) -> Self
pub fn from_stack_item(item: &StackItem) -> Result<Self, String>
```

#### `src/atipicial_types/contract/aef_file.rs` — 9 symbols

```rust
pub struct AefFile
pub const HEADER_SIZE: usize = Self::MAGIC_SIZE + Self::COMPILER_SIZE
pub fn new(
pub fn deserialize(bytes: &[u8]) -> Result<Self, TypeError>
pub fn try_encode(&self, writer: &mut Encoder) -> Result<(), TypeError>
pub fn try_to_array(&self) -> Result<Vec<u8>, TypeError>
pub struct MethodToken
pub fn try_encode(&self, writer: &mut Encoder) -> Result<(), TypeError>
pub fn try_to_array(&self) -> Result<Vec<u8>, TypeError>
```

#### `src/atipicial_types/contract/aep17contract.rs` — 2 symbols

```rust
pub struct Aep17Contract
pub fn new(script_hash: H160, symbol: String, decimals: u8) -> Self
```

#### `src/atipicial_types/contract/contract_aef.rs` — 4 symbols

```rust
pub struct ContractAef
pub fn new(
pub fn get_first_token(&self) -> Result<&ContractMethodToken, TypeError>
pub fn get_token(&self, index: usize) -> Result<&ContractMethodToken, TypeError>
```

#### `src/atipicial_types/contract/contract_manifest.rs` — 20 symbols

```rust
pub struct ContractManifest
pub fn new(
pub fn get_supported_standard(&self, index: usize) -> Result<&String, TypeError>
pub fn get_first_supported_standard(&self) -> Result<&String, TypeError>
pub fn get_permission(&self, index: usize) -> Result<&ContractPermission, TypeError>
pub fn get_first_permission(&self) -> Result<&ContractPermission, TypeError>
pub fn get_first_trust(&self) -> Result<&String, TypeError>
pub fn get_trust(&self, index: usize) -> Result<&String, TypeError>
pub struct ContractGroup
pub struct ContractABI
pub fn new(methods: Option<Vec<ContractMethod>>, events: Option<Vec<ContractEvent>>) -> Self
pub fn get_first_method(&self) -> Result<&ContractMethod, TypeError>
pub fn get_method(&self, index: usize) -> Result<&ContractMethod, TypeError>
pub fn get_first_event(&self) -> Result<&ContractEvent, TypeError>
pub fn get_event(&self, index: usize) -> Result<&ContractEvent, TypeError>
pub struct ContractMethod
pub fn new(
pub struct ContractEvent
pub struct ContractPermission
pub fn new(contract: String, methods: Vec<String>) -> Self
```

#### `src/atipicial_types/contract/contract_method_token.rs` — 2 symbols

```rust
pub struct ContractMethodToken
pub fn new(
```

#### `src/atipicial_types/contract/contract_parameter.rs` — 50 symbols

```rust
pub struct ContractParameter2
pub fn new(name: String, typ: ContractParameterType) -> Self
pub struct ContractParameter
pub fn try_from_aef_file(value: &AefFile) -> Result<Self, crate::atipicial_types::TypeError>
pub fn from_json(value: Value) -> Self
pub fn as_public_key(&self) -> Result<Secp256r1PublicKey, String>
pub fn as_signature(&self) -> Result<Vec<u8>, String>
pub fn as_bytes(&self) -> Result<Vec<u8>, String>
pub fn as_string(&self) -> Result<String, String>
pub enum ParameterValue
pub fn new(typ: ContractParameterType) -> Self
pub fn get_type(&self) -> ContractParameterType
pub fn with_value(typ: ContractParameterType, value: ParameterValue) -> Self
pub fn bool(value: bool) -> Self
pub fn to_bool(&self) -> Result<bool, String>
pub fn integer(value: i64) -> Self
pub fn to_integer(&self) -> Result<i64, String>
pub fn byte_array(value: Vec<u8>) -> Self
pub fn to_byte_array(&self) -> Result<Vec<u8>, String>
pub fn string(value: String) -> Self
pub fn to_string(&self) -> Result<String, String>
pub fn h160(value: &H160) -> Self
pub fn to_h160(&self) -> Result<H160, String>
pub fn h256(value: &H256) -> Self
pub fn to_h256(&self) -> Result<H256, String>
pub fn public_key(value: &Secp256r1PublicKey) -> Self
pub fn to_public_key(&self) -> Result<Secp256r1PublicKey, String>
pub fn signature(value: &str) -> Self
pub fn to_signature(&self) -> Result<String, String>
pub fn array(values: Vec<Self>) -> Self
pub fn to_array(&self) -> Result<Vec<ContractParameter>, String>
pub fn map(values: ContractParameterMap) -> Self
pub fn to_map(&self) -> Result<ContractParameterMap, String>
pub fn any() -> Self
pub fn hash(self) -> Vec<u8>
pub struct ContractParameterMap(
pub fn new() -> Self
pub fn from_map(map: HashMap<ContractParameter, ContractParameter>) -> Self
pub fn to_map(&mut self) -> &HashMap<ContractParameter, ContractParameter>
pub fn to_bool(&self) -> Result<bool, String>
pub fn to_integer(&self) -> Result<i64, String>
pub fn to_byte_array(&self) -> Result<Vec<u8>, String>
pub fn to_string(&self) -> Result<String, String>
pub fn to_h160(&self) -> Result<H160, String>
pub fn to_h256(&self) -> Result<H256, String>
pub fn to_public_key(&self) -> Result<Secp256r1PublicKey, String>
pub fn to_signature(&self) -> Result<String, String>
pub fn to_array(&self) -> Result<Vec<ContractParameter>, String>
pub fn to_map(&self) -> Result<ContractParameterMap, String>
pub fn hash(&self) -> Vec<u8>
```

#### `src/atipicial_types/contract/contract_parameter_type.rs` — 1 symbols

```rust
pub enum ContractParameterType
```

#### `src/atipicial_types/contract/contract_state.rs` — 5 symbols

```rust
pub struct ContractState
pub fn new(
pub fn contract_identifiers(
pub struct ContractIdentifiers
pub fn from_invocation_result(result: InvocationResult) -> Result<Self, String>
```

#### `src/atipicial_types/contract/contract_storage_entry.rs` — 2 symbols

```rust
pub struct ContractStorageEntry
pub fn new(key: String, value: String) -> Self
```

#### `src/atipicial_types/contract/invocation_result.rs` — 19 symbols

```rust
pub struct InvocationResult
pub enum AtipicialVMStateType
pub fn new(
pub fn has_state_fault(&self) -> bool
pub fn get_first_stack_item(&self) -> Result<&StackItem, TypeError>
pub fn get_stack_item(&self, index: usize) -> Result<&StackItem, TypeError>
pub fn get_first_notification(&self) -> Result<&Notification, TypeError>
pub fn get_notification(&self, index: usize) -> Result<&Notification, TypeError>
pub struct PendingSignature
pub struct Item
pub struct Diagnostics
pub fn new(invoked_contracts: InvokedContract, storage_changes: Vec<StorageChange>) -> Self
pub struct InvokedContract
pub fn new(hash: H160, invoked_contracts: Vec<InvokedContract>) -> Self
pub fn new_hash(hash: H160) -> Self
pub struct StorageChange
pub fn new(state: String, key: String, value: String) -> Self
pub struct Notification
pub enum NotificationState
```

#### `src/atipicial_types/contract/native_contract_state.rs` — 2 symbols

```rust
pub struct NativeContractState
pub fn new(
```

#### `src/atipicial_types/nns/nns_name.rs` — 8 symbols

```rust
pub struct NNSName
pub fn new(name: &str) -> Result<Self, TypeError>
pub fn is_valid(name: &str, allow_multi_fragments: bool) -> Result<(), TypeError>
pub fn validate(name: &str, allow_multi_fragments: bool) -> Result<(), TypeError>
pub fn bytes(&self) -> Vec<u8>
pub fn is_second_level_domain(&self) -> bool
pub struct NNSRoot
pub fn new(root: &str) -> Result<Self, TypeError>
```

#### `src/atipicial_types/nns/record_state.rs` — 3 symbols

```rust
pub struct RecordState
pub fn new(name: String, record_type: RecordType, data: String) -> Self
pub fn from_stack_item(item: &StackItem) -> Result<Self, &'static str>
```

#### `src/atipicial_types/nns/record_type.rs` — 2 symbols

```rust
pub enum RecordType
pub fn byte_repr(self) -> u8
```

#### `src/atipicial_utils/error.rs` — 3 symbols

```rust
pub fn option_to_result<T, E, F>(option: Option<T>, err_fn: F) -> Result<T, E>
pub fn with_context<T, E, C, F, G>(
pub fn result_to_option<T, E: std::fmt::Display>(result: Result<T, E>) -> Option<T>
```

#### `src/atipicial_wallets/bip39_account.rs` — 5 symbols

```rust
pub struct Bip39Account
pub fn mnemonic(&self) -> &str
pub fn account(&self) -> &Account
pub fn create(password: &str) -> Result<Self, Box<dyn std::error::Error>>
pub fn from_bip39_mnemonic(
```

#### `src/atipicial_wallets/error.rs` — 1 symbols

```rust
pub enum SignerError
```

#### `src/atipicial_wallets/ledger.rs` — 3 symbols

```rust
pub enum HDPath
pub fn to_vec(&self) -> Vec<u32>
pub struct LedgerWallet<T: LedgerAsync>
```

#### `src/atipicial_wallets/mod.rs` — 2 symbols

```rust
pub type LocalWallet = WalletSigner<Account>
pub type YubiWallet = WalletSigner<yubihsm::ecdsa::Signer<NistP256>>
```

#### `src/atipicial_wallets/wallet_signer.rs` — 7 symbols

```rust
pub struct WalletSigner<D: PrehashSigner<Signature>>
pub fn new_with_signer(signer: D, address: Address) -> Self
pub fn sign_hash(&self, hash: H256) -> Result<Signature, WalletError>
pub fn signer(&self) -> &D
pub fn address(&self) -> Address
pub fn network(&self) -> Option<u64>
pub fn with_network<T: Into<u64>>(mut self, network: T) -> Self
```

#### `src/atipicial_wallets/wallet_trait.rs` — 1 symbols

```rust
pub trait WalletTrait
```

#### `src/atipicial_wallets/yubi.rs` — 3 symbols

```rust
pub fn connect(
pub fn new(
pub fn from_key(
```

#### `src/atipicial_wallets/wallet/aep6account.rs` — 4 symbols

```rust
pub struct AEP6Account
pub fn new(
pub fn from_account(account: &Account) -> Result<AEP6Account, WalletError>
pub fn to_account(&self) -> Result<Account, WalletError>
```

#### `src/atipicial_wallets/wallet/aep6contract.rs` — 2 symbols

```rust
pub struct AEP6Contract
pub struct AEP6Parameter
```

#### `src/atipicial_wallets/wallet/aep6wallet.rs` — 2 symbols

```rust
pub struct Aep6Wallet
pub fn new(
```

#### `src/atipicial_wallets/wallet/backup.rs` — 3 symbols

```rust
pub struct WalletBackup
pub fn backup(wallet: &Wallet, path: PathBuf) -> Result<(), WalletError>
pub fn recover(path: PathBuf) -> Result<Wallet, WalletError>
```

#### `src/atipicial_wallets/wallet/wallet.rs` — 30 symbols

```rust
pub struct Wallet
pub const DEFAULT_WALLET_NAME: &'static str = "AtipicialWallet"
pub const CURRENT_VERSION: &'static str = "1.0"
pub fn new() -> Self
pub fn try_new() -> Result<Self, WalletError>
pub fn to_aep6(&self) -> Result<Aep6Wallet, WalletError>
pub fn from_aep6(aep6: Aep6Wallet) -> Result<Self, WalletError>
pub fn from_account(account: &Account) -> Result<Wallet, WalletError>
pub fn from_accounts(accounts: Vec<Account>) -> Result<Wallet, WalletError>
pub fn save_to_file(&self, path: PathBuf) -> Result<(), WalletError>
pub fn get_account(&self, script_hash: &H160) -> Option<&Account>
pub fn remove_account(&mut self, script_hash: &H160) -> bool
pub fn encrypt_accounts(&mut self, password: &str) -> Result<(), WalletError>
pub fn encrypt_accounts_parallel(&mut self, password: &str) -> Result<(), WalletError>
pub fn encrypt_accounts_parallel_with_threads(
pub fn encrypt_accounts_batch_parallel(
pub fn create(path: &Path, password: &str) -> Result<Self, WalletError>
pub fn open(path: &Path, password: &str) -> Result<Self, WalletError>
pub fn get_accounts(&self) -> Vec<&Account>
pub fn create_account(&mut self) -> Result<&Account, WalletError>
pub fn import_private_key(&mut self, wif: &str) -> Result<&Account, WalletError>
pub fn verify_password(&self, password: &str) -> bool
pub fn change_password(
pub fn change_password_parallel(
pub fn create_wallet(path: &Path, password: &str) -> Result<Self, WalletError>
pub fn open_wallet(path: &Path, password: &str) -> Result<Self, WalletError>
pub fn get_all_accounts(&self) -> Vec<&Account>
pub fn create_new_account(&mut self) -> Result<&Account, WalletError>
pub fn import_from_wif(&mut self, private_key: &str) -> Result<&Account, WalletError>
pub fn with_network(mut self, network: u32) -> Self
```

#### `src/atipicial_wallets/wallet/wallet_error.rs` — 1 symbols

```rust
pub enum WalletError
```

#### `src/atipicial_x/bridge/bridge_contract.rs` — 8 symbols

```rust
pub struct AtipicialXBridgeContract<'a, P: JsonRpcProvider>
pub const CONTRACT_HASH: &'static str = "74f2dc36a68fdc4682034178eb2220729231db76"
pub const DEPOSIT: &'static str = "deposit"
pub const WITHDRAW: &'static str = "withdraw"
pub const GET_FEE: &'static str = "getFee"
pub const GET_CAP: &'static str = "getCap"
pub fn new(provider: Option<&'a RpcClient<P>>) -> Result<Self, ContractError>
pub fn with_script_hash(script_hash: ScriptHash, provider: Option<&'a RpcClient<P>>) -> Self
```

#### `src/atipicial_x/bridge/evm_bridge.rs` — 4 symbols

```rust
pub struct AtipicialXBridgeContractEVM
pub fn new(address: Address, provider: Arc<RootProvider>) -> Self
pub fn default_bridge(provider: Arc<RootProvider>) -> Result<Self, ContractError>
pub fn address(&self) -> Address
```

#### `src/atipicial_x/evm/provider.rs` — 7 symbols

```rust
pub const ATC_X_MAINNET_MEV_RPC: &str = "https://mainnet-1.rpc.banelabs.org"
pub struct AtipicialXProvider<'a, P: JsonRpcProvider>
pub fn new(rpc_url: &str, provider: Option<&'a RpcClient<P>>) -> Self
pub fn new_anti_mev(provider: Option<&'a RpcClient<P>>) -> Self
pub fn rpc_url(&self) -> &str
pub fn set_rpc_url(&mut self, rpc_url: &str)
pub fn evm_provider(&self) -> Option<Arc<RootProvider>>
```

#### `src/atipicial_x/evm/transaction.rs` — 9 symbols

```rust
pub struct AtipicialXTransaction
pub fn new(
pub fn to(&self) -> Option<H160>
pub fn data(&self) -> &Vec<u8>
pub fn value(&self) -> u64
pub fn gas_limit(&self) -> u64
pub fn gas_price(&self) -> u64
pub fn into_alloy_request(self) -> TransactionRequest
pub fn build_alloy_request(
```

#### `src/atipicial_x/evm/wallet.rs` — 7 symbols

```rust
pub struct AtipicialXWallet
pub fn from_private_key(hex_key: &str) -> Result<Self, LocalSignerError>
pub fn create_random() -> Self
pub fn address(&self) -> AlloyAddress
pub fn inner_wallet(&self) -> &PrivateKeySigner
pub struct AtipicialXClient<'a, P: JsonRpcProvider>
pub fn new(wallet: AtipicialXWallet, provider: AtipicialXProvider<'a, P>) -> Self
```

#### `src/constants/native_contracts.rs` — 10 symbols

```rust
pub const CONTRACT_MANAGEMENT: &str = "0xfffdc93764dbaddd97c48f252a53ea4643faa3fd"
pub const STD_LIB: &str = "0xacce6fd80d44e1796aa0c2c625e9e4e0ce39efc0"
pub const CRYPTO_LIB: &str = "0x726cb6e0cd8628a1350a611384688911ab75f51b"
pub const LEDGER: &str = "0xda65b600f7124ce6c79950c1772a36403104f2be"
pub const ATC_TOKEN: &str = "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5"
pub const GAS_TOKEN: &str = "0xd2a4cff31913016155e38e474a2c06d08be276cf"
pub const POLICY: &str = "0xcc5e4edd9f5f8dba8bb65734541df7a1c081c67b"
pub const ROLE_MANAGEMENT: &str = "0x49cf4e5378ffcd4dec034fd98a174c5491e395e2"
pub const ORACLE: &str = "0xfe924b7cfe89ddd271abaf7210a80a7e11178758"
pub const NAME_SERVICE: &str = "0x7a8fcf0392cd625647907afa8e45cc66872b596b"
```

#### `src/monitoring/health.rs` — 12 symbols

```rust
pub enum HealthStatus
pub struct HealthCheck
pub struct HealthRegistry
pub fn register(&self, name: String, check: HealthCheck)
pub fn update(&self, name: &str, status: HealthStatus, message: Option<String>)
pub fn overall_status(&self) -> HealthStatus
pub fn get_all(&self) -> Vec<HealthCheck>
pub struct HealthResponse
pub fn init(port: u16) -> Result<(), Box<dyn std::error::Error>>
pub fn update_health(name: &str, status: HealthStatus, message: Option<String>)
pub fn register_health_check(name: String, initial_status: HealthStatus)
pub fn shutdown()
```

#### `src/monitoring/metrics.rs` — 11 symbols

```rust
pub struct MetricsSnapshot
pub fn init(port: u16) -> Result<(), Box<dyn std::error::Error>>
pub fn increment_counter(name: &str, value: f64)
pub fn set_gauge(name: &str, value: f64)
pub fn observe_histogram(name: &str, value: f64)
pub fn snapshot() -> MetricsSnapshot
pub fn record_transaction(tx_type: &str, network: &str, duration: f64, success: bool)
pub fn record_rpc_request(method: &str, endpoint: &str, duration: f64, success: bool)
pub fn update_blockchain_metrics(network: &str, height: u64, synced: bool, nodes: u64)
pub fn record_contract_invocation(contract: &str, method: &str, gas_used: f64)
pub fn shutdown()
```

#### `src/monitoring/mod.rs` — 14 symbols

```rust
pub struct MonitoringConfig
pub fn builder() -> MonitoringConfigBuilder
pub struct MonitoringConfigBuilder
pub fn metrics_enabled(mut self, val: bool) -> Self
pub fn metrics_port(mut self, val: u16) -> Self
pub fn tracing_enabled(mut self, val: bool) -> Self
pub fn tracing_endpoint(mut self, val: String) -> Self
pub fn log_level(mut self, val: String) -> Self
pub fn health_check_enabled(mut self, val: bool) -> Self
pub fn health_check_port(mut self, val: u16) -> Self
pub fn build(self) -> MonitoringConfig
pub fn from_env() -> Self
pub fn init() -> Result<(), Box<dyn std::error::Error>>
pub fn shutdown()
```

#### `src/monitoring/tracing.rs` — 5 symbols

```rust
pub fn init(endpoint: &str, log_level: &str) -> Result<(), Box<dyn std::error::Error>>
pub fn add_event(name: &str, attributes: Vec<(&str, String)>)
pub fn set_status(success: bool, message: Option<&str>)
pub fn record_error(error: &dyn std::error::Error)
pub fn shutdown()
```

#### `src/sdk/hd_wallet.rs` — 20 symbols

```rust
pub struct DerivationPath
pub fn new_atipicial(account: u32, index: u32) -> Result<Self, AtipicialError>
pub fn from_string(path: &str) -> Result<Self, AtipicialError>
pub struct HDWallet
pub fn generate(word_count: usize, passphrase: Option<&str>) -> Result<Self, AtipicialError>
pub fn from_mnemonic(
pub fn from_phrase(
pub fn mnemonic_phrase(&self) -> &str
pub fn derive_account(&mut self, path: &str) -> Result<Account, AtipicialError>
pub fn derive_accounts(
pub fn get_default_account(&mut self) -> Result<Account, AtipicialError>
pub fn export_encrypted(&self, password: &str) -> Result<String, AtipicialError>
pub fn import_encrypted(json: &str, password: &str) -> Result<Self, AtipicialError>
pub struct HDWalletBuilder
pub fn new() -> Self
pub fn word_count(mut self, count: usize) -> Self
pub fn passphrase(mut self, passphrase: impl Into<String>) -> Self
pub fn language(mut self, language: Language) -> Self
pub fn mnemonic(mut self, mnemonic: impl Into<String>) -> Self
pub fn build(self) -> Result<HDWallet, AtipicialError>
```

#### `src/sdk/mod.rs` — 41 symbols

```rust
pub struct Atipicial
pub enum Network
pub struct SdkConfig
pub fn builder() -> SdkConfigBuilder
pub struct SdkConfigBuilder
pub fn timeout(mut self, val: Duration) -> Self
pub fn retries(mut self, val: u32) -> Self
pub fn cache_enabled(mut self, val: bool) -> Self
pub fn metrics_enabled(mut self, val: bool) -> Self
pub fn build(self) -> SdkConfig
pub struct DecimalAmount
pub enum DecimalAmountParseError
pub fn try_from_raw(
pub fn from_raw(raw: impl Into<String>, decimals: u8) -> Self
pub fn parse(amount: &str, decimals: u8) -> Result<Self, DecimalAmountParseError>
pub fn raw(&self) -> &str
pub fn decimals(&self) -> u8
pub fn to_fixed_string(&self) -> String
pub fn raw_i64(&self) -> Option<i64>
pub struct Balance
pub struct TokenBalance
pub type TxHash = String
pub enum Token
pub const ATC_HASH: [u8
pub const GAS_HASH: [u8
pub fn contract_hash(&self) -> ScriptHash
pub fn builder() -> AtipicialBuilder
pub fn client(&self) -> &RpcClient<HttpProvider>
pub fn endpoint(&self) -> &str
pub fn network(&self) -> &Network
pub struct AtipicialBuilder
pub fn network(mut self, network: Network) -> Self
pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self
pub fn config(mut self, config: SdkConfig) -> Self
pub fn timeout(mut self, timeout: Duration) -> Self
pub fn retries(mut self, retries: u32) -> Self
pub fn cache(mut self, enabled: bool) -> Self
pub fn metrics(mut self, enabled: bool) -> Self
pub struct Transfer
pub fn new(from: Wallet, to: impl Into<String>, amount: u64, token: Token) -> Self
pub fn with_memo(mut self, memo: impl Into<String>) -> Self
```

#### `src/sdk/transaction_simulator.rs` — 22 symbols

```rust
pub struct SimulationResult
pub struct StateChanges
pub struct StorageChange
pub struct BalanceChange
pub struct TokenTransfer
pub struct ContractDeployment
pub struct ContractUpdate
pub struct Notification
pub struct SimulationWarning
pub enum WarningLevel
pub struct OptimizationSuggestion
pub enum OptimizationType
pub struct TransactionSimulator
pub fn new(client: Arc<RpcClient<HttpProvider>>) -> Self
pub enum OptimizationRule
pub struct GasEstimate
pub struct TransactionSimulatorBuilder
pub fn new() -> Self
pub fn client(mut self, client: Arc<RpcClient<HttpProvider>>) -> Self
pub fn cache_duration(mut self, duration: std::time::Duration) -> Self
pub fn add_optimization_rule(mut self, rule: OptimizationRule) -> Self
pub fn build(self) -> Result<TransactionSimulator, AtipicialError>
```

#### `src/sdk/unified.rs` — 4 symbols

```rust
pub enum EcosystemClient<'a>
pub fn new_n3(provider: crate::sdk::Atipicial, wallet: N3Wallet) -> Self
pub fn new_atipicialx(wallet: AtipicialXWallet, provider: AtipicialXProvider<'a, N3HttpProvider>) -> Self
pub fn new_atipicialx_anti_mev(wallet: AtipicialXWallet) -> Self
```

#### `src/sdk/websocket.rs` — 13 symbols

```rust
pub enum SubscriptionType
pub enum EventData
pub struct SubscriptionHandle
pub fn id(&self) -> &str
pub fn subscription_type(&self) -> &SubscriptionType
pub fn cancel(self)
pub struct WebSocketClient
pub fn take_event_receiver(
pub fn set_reconnect_params(&mut self, interval: Duration, max_attempts: u32)
pub struct WebSocketClientBuilder
pub fn new(url: impl Into<String>) -> Self
pub fn reconnect_interval(mut self, interval: Duration) -> Self
pub fn max_reconnect_attempts(mut self, attempts: u32) -> Self
```

---

## 🧱 Type Anatomy — Structs & Enums, Field by Field


#### `atipicial-cli/src/errors.rs`

**444 types dissected, field by field.**

**enum `CliError`** (0 members)
```rust
```


#### `atipicial-cli/src/generator.rs`

**struct `Template`** (2 members)
```rust
    pub template: TemplateMetadata
    pub files: HashMap<String, String>
```

**struct `TemplateMetadata`** (4 members)
```rust
    pub name: String
    pub description: String
    pub version: String
    pub author: String
```

**enum `ProjectTemplate`** (5 members)
```rust
    BasicDapp
    Aep17Token
    NftCollection
    DefiProtocol
    OracleConsumer
```


#### `atipicial-cli/src/main.rs`

**struct `Cli`** (5 members)
```rust
    config: Option<PathBuf>
    verbose: bool
    format: OutputFormat
    network: Option<String>
    command: Commands
```


#### `atipicial-cli/src/utils_core.rs`

**struct `LightweightProgress`** (6 members)
```rust
    len: u64
    pos: u64
    message: String
    spinner: bool
    stop_flag: Option<std::sync::Arc<AtomicBool>>
    worker: Option<thread::JoinHandle<()>>
```


#### `atipicial-cli/src/commands/atipicialfs.rs`

**struct `AtipicialFsArgs`** (2 members)
```rust
    pub endpoint: Option<String>
    pub command: AtipicialFsCommands
```

**enum `AtipicialFsCommands`** (2 members)
```rust
    Container {
    command: ContainerCommands
```

**enum `ContainerCommands`** (4 members)
```rust
    Create {
    name: String
    acl: Option<String>
    options: Option<String>
```

**enum `ObjectCommands`** (4 members)
```rust
    Put {
    file: PathBuf
    container: String
    path: Option<String>
```

**enum `AclCommands`** (2 members)
```rust
    Get {
    container: String
```

**enum `ConfigCommands`** (3 members)
```rust
    SetEndpoint {
    url: String
    env: Option<String>
```


#### `atipicial-cli/src/commands/blockchain.rs`

**struct `BlockchainArgs`** (1 members)
```rust
    pub command: BlockchainCommands
```

**enum `BlockchainCommands`** (5 members)
```rust
    Status
    Export {
    path: PathBuf
    start: u32
    end: Option<u32>
```


#### `atipicial-cli/src/commands/contract.rs`

**struct `ContractArgs`** (1 members)
```rust
    pub command: ContractCommands
```

**enum `ContractCommands`** (2 members)
```rust
    Get {
    script_hash: String
```


#### `atipicial-cli/src/commands/fs.rs`

**struct `ContainerInfo`** (7 members)
```rust
    pub id: String
    pub name: String
    pub owner: String
    pub basic_acl: u32
    pub placement_policy: String
    pub created_at: String
    pub attributes: HashMap<String, String>
```

**struct `ObjectInfo`** (8 members)
```rust
    pub id: String
    pub container_id: String
    pub owner: String
    pub size: u64
    pub checksum: String
    pub content_type: String
    pub created_at: String
    pub attributes: HashMap<String, String>
```

**struct `NetworkStatus`** (6 members)
```rust
    pub status: String
    pub network: String
    pub version: String
    pub nodes: u32
    pub epoch: u64
    pub uptime: u64
```

**struct `FSArgs`** (2 members)
```rust
    pub endpoint: Option<String>
    pub command: FSCommands
```

**enum `FSCommands`** (2 members)
```rust
    Container {
    command: ContainerCommands
```

**enum `EndpointCommands`** (2 members)
```rust
    List {
    network: String
```

**enum `ContainerCommands`** (2 members)
```rust
    Create {
    name: String
```

**enum `ObjectCommands`** (3 members)
```rust
    Put {
    container: String
    file: PathBuf
```


#### `atipicial-cli/src/commands/network.rs`

**struct `NetworkConfig`** (5 members)
```rust
    pub name: String
    pub rpc_url: String
    pub network_type: String
    pub chain_id: u32
    pub is_default: bool
```

**struct `NetworkArgs`** (1 members)
```rust
    pub command: NetworkCommands
```

**enum `NetworkCommands`** (2 members)
```rust
    Connect {
    network: Option<String>
```


#### `atipicial-cli/src/commands/nft.rs`

**struct `NftArgs`** (1 members)
```rust
    pub command: NftCommands
```

**enum `NftCommands`** (7 members)
```rust
    Mint {
    contract: String
    to: String
    token_id: String
    metadata: Option<String>
    properties: Option<String>
    account: Option<String>
```


#### `atipicial-cli/src/commands/tools.rs`

**struct `ToolsArgs`** (1 members)
```rust
    pub command: ToolsCommands
```

**enum `ToolsCommands`** (4 members)
```rust
    Encode {
    input: String
    format: String
    input_format: String
```


#### `atipicial-cli/src/commands/wallet.rs`

**struct `CliState`** (7 members)
```rust
    pub wallet: Option<Wallet>
    pub wallet_path: Option<PathBuf>
    pub wallet_password: Option<String>
    pub rpc_client: Option<RpcClient<HttpProvider>>
    pub network_type: Option<String>
    pub current_network: Option<crate::commands::network::NetworkConfig>
    pub networks: Vec<crate::commands::network::NetworkConfig>
```

**struct `WalletArgs`** (1 members)
```rust
    pub command: WalletCommands
```

**enum `WalletCommands`** (4 members)
```rust
    Create {
    path: Option<PathBuf>
    name: Option<String>
    password: Option<String>
```


#### `atipicial-cli/src/commands/defi/mod.rs`

**struct `DefiArgs`** (3 members)
```rust
    pub wallet: Option<PathBuf>
    pub password: Option<String>
    pub command: DefiCommands
```

**enum `DefiCommands`** (2 members)
```rust
    Token {
    contract: String
```


#### `atipicial-cli/src/commands/defi/types.rs`

**struct `TokenArgs`** (3 members)
```rust
    pub wallet: Option<PathBuf>
    pub password: Option<String>
    pub command: TokenCommands
```

**enum `TokenCommands`** (2 members)
```rust
    Info {
    contract: String
```

**struct `SwapArgs`** (3 members)
```rust
    pub wallet: Option<PathBuf>
    pub password: Option<String>
    pub command: SwapCommands
```

**enum `SwapCommands`** (5 members)
```rust
    Swap {
    from_token: String
    to_token: String
    amount: String
    slippage: f64
```

**struct `LiquidityArgs`** (3 members)
```rust
    pub wallet: Option<PathBuf>
    pub password: Option<String>
    pub command: LiquidityCommands
```

**enum `LiquidityCommands`** (5 members)
```rust
    Add {
    token_a: String
    token_b: String
    amount_a: String
    amount_b: String
```

**struct `StakingArgs`** (3 members)
```rust
    pub wallet: Option<PathBuf>
    pub password: Option<String>
    pub command: StakingCommands
```

**enum `StakingCommands`** (4 members)
```rust
    Stake {
    token: String
    amount: String
    period: Option<u32>
```


#### `atipicial-cli/src/commands/defi/utils.rs`

**enum `NetworkType`** (3 members)
```rust
    MainNet
    TestNet
    PrivateNet
```

**enum `NetworkTypeCli`** (3 members)
```rust
    MainNet, // Updated to match the Network enum in wallet module
    TestNet, // Updated to match the Network enum in wallet module
    AtipicialX
```


#### `atipicial-cli/src/config/mod.rs`

**struct `CliConfig`** (4 members)
```rust
    pub network: NetworkConfig
    pub wallet: WalletConfig
    pub storage: StorageConfig
    pub logging: LoggingConfig
```

**struct `NetworkConfig`** (3 members)
```rust
    pub rpc_url: String
    pub network_magic: u32
    pub address_version: u8
```

**struct `WalletConfig`** (2 members)
```rust
    pub default_path: Option<String>
    pub auto_unlock: bool
```

**struct `StorageConfig`** (1 members)
```rust
    pub path: PathBuf
```

**struct `LoggingConfig`** (2 members)
```rust
    pub level: String
    pub file: Option<PathBuf>
```


#### `atipicial-cli/src/monitoring/logger.rs`

**enum `LogFormat`** (3 members)
```rust
    Pretty
    Json
    Compact
```

**struct `LoggerConfig`** (6 members)
```rust
    pub level: String
    pub format: LogFormat
    pub file_path: Option<PathBuf>
    pub enable_colors: bool
    pub enable_timestamps: bool
    pub enable_module_path: bool
```

**struct `StructuredLogger`** (1 members)
```rust
    context: std::collections::HashMap<String, String>
```

**struct `PerformanceLogger`** (3 members)
```rust
    operation: String
    start_time: std::time::Instant
    threshold: std::time::Duration
```

**struct `AuditLogger`** (1 members)
```rust
    file_path: PathBuf
```


#### `atipicial-cli/src/monitoring/metrics.rs`

**struct `MetricsConfig`** (4 members)
```rust
    pub enabled: bool
    pub port: u16
    pub path: String
    pub export_interval: Duration
```

**enum `MetricType`** (4 members)
```rust
    Counter
    Gauge
    Histogram
    Summary
```

**enum `MetricValue`** (4 members)
```rust
    Counter(u64)
    Gauge(f64)
    Histogram(Vec<f64>)
    Summary(SummaryData)
```

**struct `SummaryData`** (3 members)
```rust
    pub count: u64
    pub sum: f64
    pub quantiles: Vec<(f64, f64)>, // (quantile, value)
```

**struct `Metric`** (6 members)
```rust
    pub name: String
    pub help: String
    pub metric_type: MetricType
    pub labels: HashMap<String, String>
    pub value: MetricValue
    pub timestamp: Instant
```

**struct `MetricsRegistry`** (1 members)
```rust
    metrics: Arc<RwLock<HashMap<String, Metric>>>
```

**struct `MetricsCollector`** (3 members)
```rust
    registry: Arc<MetricsRegistry>
    config: MetricsConfig
    server_handle: Option<tokio::task::JoinHandle<()>>
```

**struct `Timer`** (4 members)
```rust
    name: String
    labels: Vec<(String, String)>
    start: Instant
    collector: Arc<MetricsCollector>
```


#### `atipicial-cli/src/monitoring/mod.rs`

**struct `MonitoringContext`** (1 members)
```rust
    pub metrics: Arc<MetricsCollector>
```


#### `atipicial-cli/src/security/error_handler.rs`

**struct `RetryConfig`** (5 members)
```rust
    pub max_attempts: u32
    pub initial_delay: Duration
    pub max_delay: Duration
    pub exponential_base: f64
    pub jitter: bool
```

**struct `RetryHandler`** (1 members)
```rust
    config: RetryConfig
```

**enum `RecoveryStrategy`** (3 members)
```rust
    Retry(RetryConfig)
    Fallback(String)
    CircuitBreaker { threshold: u32, timeout: Duration
```

**struct `ErrorHandler`** (3 members)
```rust
    retry_handler: RetryHandler
    circuit_breakers:
    std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, CircuitBreaker>>>
```

**struct `ErrorContext`** (4 members)
```rust
    pub operation: String
    pub timestamp: chrono::DateTime<chrono::Utc>
    pub context: std::collections::HashMap<String, String>
    pub stack_trace: Option<String>
```

**struct `ErrorReporter`** (1 members)
```rust
    errors: std::sync::Arc<std::sync::Mutex<Vec<ErrorContext>>>
```


#### `atipicial-cli/src/security/keychain.rs`

**struct `KeychainManager`** (4 members)
```rust
    service_name: String
    keychain: security_framework::os::macos::keychain::SecKeychain
    credential_store: HashMap<String, Vec<u8>>
    secret_service: Option<()>
```

**struct `SecureCredential`** (3 members)
```rust
    pub account: String
    pub data: Vec<u8>
    pub metadata: HashMap<String, String>
```

**struct `SecureWalletStorage`** (1 members)
```rust
    keychain: KeychainManager
```


#### `atipicial-cli/src/security/mod.rs`

**struct `SecurityContext`** (3 members)
```rust
    pub keychain: SecureWalletStorage
    pub session_manager: SessionManager
    pub error_handler: ErrorHandler
```


#### `atipicial-cli/src/security/network_failover.rs`

**struct `EndpointHealth`** (7 members)
```rust
    pub url: String
    pub is_healthy: bool
    pub response_time: Duration
    pub failure_count: u32
    pub success_count: u32
    pub last_check: Instant
    pub last_error: Option<String>
```

**struct `FailoverConfig`** (5 members)
```rust
    pub health_check_interval: Duration
    pub request_timeout: Duration
    pub max_retries: u32
    pub failover_threshold: u32
    pub recovery_interval: Duration
```

**struct `NetworkFailover`** (4 members)
```rust
    endpoints: Arc<RwLock<Vec<EndpointHealth>>>
    current_index: Arc<RwLock<usize>>
    config: FailoverConfig
    health_check_handle: Option<tokio::task::JoinHandle<()>>
```

**struct `NetworkFailoverBuilder`** (2 members)
```rust
    endpoints: Vec<String>
    config: FailoverConfig
```


#### `atipicial-cli/src/security/session.rs`

**struct `SessionConfig`** (5 members)
```rust
    pub max_duration: Duration
    pub idle_timeout: Duration
    pub persistent: bool
    pub max_concurrent: usize
    pub require_reauth: bool
```

**struct `Session`** (8 members)
```rust
    pub id: String
    pub user_id: String
    pub created_at: DateTime<Utc>
    pub last_activity: DateTime<Utc>
    pub expires_at: DateTime<Utc>
    pub metadata: HashMap<String, String>
    pub permissions: Vec<String>
    pub is_active: bool
```

**struct `SessionManager`** (3 members)
```rust
    sessions: Arc<RwLock<HashMap<String, Session>>>
    config: SessionConfig
    auth_callbacks: AuthCallbackMap
```

**struct `SessionGuard`** (2 members)
```rust
    manager: Arc<SessionManager>
    session_id: String
```


#### `atipicial-cli/src/utils/config.rs`

**struct `Config`** (5 members)
```rust
    pub networks: Vec<NetworkConfig>
    pub default_network: String
    pub wallet_path: Option<String>
    pub auto_connect: bool
    pub atipicialfs: AtipicialFsConfig
```

**struct `NetworkConfig`** (2 members)
```rust
    pub name: String
    pub rpc_url: String
```

**struct `AtipicialFsConfig`** (2 members)
```rust
    pub endpoints: Vec<AtipicialFsEndpoint>
    pub default_endpoint: Option<String>
```

**struct `AtipicialFsEndpoint`** (4 members)
```rust
    pub name: String
    pub url: String
    pub network: String,       // mainnet, testnet, private
    pub endpoint_type: String, // grpc, http, rest
```


#### `atipicial-cli/src/utils/error.rs`

**enum `CliError`** (0 members)
```rust
```


#### `atipicial-cli/tests/integration/utils.rs`

**struct `CliTest`** (2 members)
```rust
    pub temp_dir: TempDir
    pub binary_path: PathBuf
```


#### `examples/providers/examples/retry.rs`

**struct `RetryPolicy`** (4 members)
```rust
    pub max_retries: u32
    pub initial_delay: Duration
    pub max_delay: Duration
    pub backoff_multiplier: f64
```

**enum `RpcError`** (5 members)
```rust
    NetworkError(String)
    TimeoutError
    RateLimitError
    ServerError(u16)
    ParseError(String)
```

**struct `RetryClient`** (1 members)
```rust
    policy: RetryPolicy
```


#### `src/atipicial_builder/error.rs`

**enum `BuilderError`** (0 members)
```rust
```


#### `src/atipicial_builder/script/interop_service.rs`

**enum `InteropService`** (34 members)
```rust
    SystemCryptoCheckSig
    SystemCryptoCheckMultiSig
    SystemContractCall
    SystemContractCallNative
    SystemContractGetCallFlags
    SystemContractCreateStandardAccount
    SystemContractCreateMultiSigAccount
    SystemContractNativeOnPersist
    SystemContractNativePostPersist
    SystemIteratorNext
    SystemIteratorValue
    SystemRuntimePlatform
    SystemRuntimeGetTrigger
    SystemRuntimeGetTime
    SystemRuntimeGetScriptContainer
    SystemRuntimeGetExecutingScriptHash
    SystemRuntimeGetCallingScriptHash
    SystemRuntimeGetEntryScriptHash
    SystemRuntimeCheckWitness
    SystemRuntimeGetInvocationCounter
    SystemRuntimeLog
    SystemRuntimeNotify
    SystemRuntimeGetNotifications
    SystemRuntimeGasLeft
    SystemRuntimeBurnGas
    // …9 more members
```


#### `src/atipicial_builder/script/script_builder.rs`

**struct `ScriptBuilder`** (1 members)
```rust
    pub script: Encoder
```


#### `src/atipicial_builder/transaction/call_flags.rs`

**enum `CallFlags`** (8 members)
```rust
    None
    ReadStates
    WriteStates
    AllowCall
    AllowNotify
    States
    ReadOnly
    All
```


#### `src/atipicial_builder/transaction/contract_parameters_context.rs`

**struct `ContractParametersContext`** (5 members)
```rust
    pub type_: String
    pub hash: String
    pub data: String
    pub items: HashMap<String, ContextItem>
    pub network: u32
```

**struct `ContextItem`** (3 members)
```rust
    pub script: String
    pub parameters: Option<Vec<ContractParameter>>
    pub signatures: HashMap<String, String>
```


#### `src/atipicial_builder/transaction/invocation_script.rs`

**struct `InvocationScript`** (1 members)
```rust
    script: Vec<u8>
```


#### `src/atipicial_builder/transaction/oracle_response_code.rs`

**enum `OracleResponseCode`** (10 members)
```rust
    Success = 0x00
    ProtocolNotSupported = 0x10
    ConsensusUnreachable = 0x12
    NotFound = 0x14
    Timeout = 0x16
    Forbidden = 0x18
    ResponseTooLarge = 0x1A
    InsufficientFunds = 0x1C
    ContentTypeNotSupported = 0x1F
    Error = 0xFF
```


#### `src/atipicial_builder/transaction/production_transaction_builder.rs`

**struct `FeeCalculator`** (3 members)
```rust
    base_fee_per_byte: u64
    witness_fee: u64
    storage_fee_per_byte: u64
```

**struct `WitnessGenerator`** (1 members)
```rust
    verification_scripts: HashMap<String, Vec<u8>>
```

**struct `ProductionWitness`** (2 members)
```rust
    pub invocation: Vec<u8>
    pub verification: Vec<u8>
```

**struct `ProductionSigner`** (5 members)
```rust
    pub account: String
    pub scopes: String
    pub allowed_contracts: Option<Vec<String>>
    pub allowed_groups: Option<Vec<String>>
    pub rules: Option<Vec<ProductionWitnessRule>>
```

**struct `ProductionTransactionAttribute`** (2 members)
```rust
    pub attr_type: String
    pub value: serde_json::Value
```

**struct `ProductionWitnessRule`** (2 members)
```rust
    pub action: String
    pub condition: serde_json::Value
```

**struct `ProductionTransaction`** (11 members)
```rust
    pub hash: String
    pub size: i32
    pub version: u8
    pub nonce: u32
    pub system_fee: i64
    pub network_fee: i64
    pub valid_until_block: u32
    pub signers: Vec<ProductionSigner>
    pub attributes: Vec<ProductionTransactionAttribute>
    pub script: Vec<u8>
    pub witnesses: Vec<ProductionWitness>
```

**struct `ProductionTransactionBuilder`** (2 members)
```rust
    fee_calculator: FeeCalculator
    witness_generator: WitnessGenerator
```


#### `src/atipicial_builder/transaction/transaction.rs`

**struct `Transaction`** (12 members)
```rust
    pub network: Option<&'a RpcClient<P>>
    pub version: u8
    pub nonce: u32
    pub valid_until_block: u32
    pub signers: Vec<Signer>
    pub size: i32
    pub sys_fee: i64
    pub net_fee: i64
    pub attributes: Vec<TransactionAttribute>
    pub script: Bytes
    pub witnesses: Vec<Witness>
    pub(crate) block_count_when_sent: Option<u32>
```


#### `src/atipicial_builder/transaction/transaction_attribute.rs`

**enum `TransactionAttribute`** (3 members)
```rust
    HighPriority
    OracleResponse(OracleResponse)
    NotValidBefore { height: u32
```

**struct `OracleResponse`** (3 members)
```rust
    pub id: u64
    pub response_code: OracleResponseCode
    pub result: String
```


#### `src/atipicial_builder/transaction/transaction_builder.rs`

**struct `TransactionBuilder`** (12 members)
```rust
    pub(crate) client: Option<&'a RpcClient<P>>
    version: u8
    nonce: u32
    valid_until_block: Option<u32>
    pub(crate) signers: Vec<Signer>
    additional_network_fee: u64
    additional_system_fee: u64
    attributes: Vec<TransactionAttribute>
    script: Option<Bytes>
    fee_consumer: Option<Box<dyn Fn(i64, i64)>>
    fee_error: Option<TransactionError>
    allows_transmission_on_fault: Option<bool>
```


#### `src/atipicial_builder/transaction/transaction_error.rs`

**enum `TransactionError`** (0 members)
```rust
```


#### `src/atipicial_builder/transaction/transaction_send_token.rs`

**struct `TransactionSendToken`** (3 members)
```rust
    pub token: H160
    pub value: i32
    pub address: String
```


#### `src/atipicial_builder/transaction/verification_script.rs`

**struct `VerificationScript`** (1 members)
```rust
    script: Bytes
```


#### `src/atipicial_builder/transaction/witness.rs`

**struct `Witness`** (2 members)
```rust
    pub invocation: InvocationScript
    pub verification: VerificationScript
```


#### `src/atipicial_builder/transaction/witness_scope.rs`

**enum `WitnessScope`** (6 members)
```rust
    None = 0x00
    CalledByEntry = 0x01
    CustomContracts = 0x10
    CustomGroups = 0x20
    WitnessRules = 0x40
    Global = 0x80
```


#### `src/atipicial_builder/transaction/signers/account_signer.rs`

**struct `AccountSigner`** (15 members)
```rust
    serialize_with = "serialize_script_hash"
    deserialize_with = "deserialize_script_hash"
    )]
    pub(crate) signer_hash: H160
    pub(crate) scopes: Vec<WitnessScope>
    serialize_with = "serialize_vec_script_hash"
    deserialize_with = "deserialize_vec_script_hash"
    )]
    pub(crate) allowed_contracts: Vec<H160>
    serialize_with = "serialize_vec_public_key"
    deserialize_with = "deserialize_vec_public_key"
    )]
    pub(crate) allowed_groups: Vec<Secp256r1PublicKey>
    rules: Vec<WitnessRule>
    pub account: Account
```


#### `src/atipicial_builder/transaction/signers/contract_signer.rs`

**struct `ContractSigner`** (20 members)
```rust
    serialize_with = "serialize_script_hash"
    deserialize_with = "deserialize_script_hash"
    )]
    signer_hash: H160
    scopes: Vec<WitnessScope>
    serialize_with = "serialize_vec_script_hash"
    deserialize_with = "deserialize_vec_script_hash"
    )]
    allowed_contracts: Vec<H160>
    serialize_with = "serialize_vec_public_key"
    deserialize_with = "deserialize_vec_public_key"
    )]
    allowed_groups: Vec<Secp256r1PublicKey>
    rules: Vec<WitnessRule>
    verify_params: Vec<ContractParameter>
    serialize_with = "serialize_script_hash"
    deserialize_with = "deserialize_script_hash"
    )]
    contract_hash: H160
    scope: WitnessScope
```


#### `src/atipicial_builder/transaction/signers/signer.rs`

**enum `SignerType`** (3 members)
```rust
    AccountSigner
    ContractSigner
    TransactionSigner
```

**enum `Signer`** (3 members)
```rust
    AccountSigner(AccountSigner)
    ContractSigner(ContractSigner)
    TransactionSigner(TransactionSigner)
```


#### `src/atipicial_builder/transaction/signers/transaction_signer.rs`

**struct `TransactionSigner`** (5 members)
```rust
    pub account: H160
    pub scopes: Vec<WitnessScope>
    pub allowed_contracts: Option<Vec<H160>>
    pub allowed_groups: Option<Vec<Secp256r1PublicKey>>
    pub rules: Option<Vec<WitnessRule>>
```


#### `src/atipicial_builder/transaction/witness_rule/witness_action.rs`

**enum `WitnessAction`** (2 members)
```rust
    Deny = 0
    Allow = 1
```


#### `src/atipicial_builder/transaction/witness_rule/witness_condition.rs`

**enum `WitnessCondition`** (9 members)
```rust
    Boolean(bool)
    Not(Box<WitnessCondition>)
    And(Vec<WitnessCondition>)
    Or(Vec<WitnessCondition>)
    ScriptHash(H160)
    Group(Secp256r1PublicKey)
    CalledByEntry
    CalledByContract(H160)
    CalledByGroup(Secp256r1PublicKey)
```


#### `src/atipicial_builder/transaction/witness_rule/witness_rule.rs`

**struct `WitnessRule`** (2 members)
```rust
    pub action: WitnessAction
    pub condition: WitnessCondition
```


#### `src/atipicial_clients/cache.rs`

**struct `CacheConfig`** (4 members)
```rust
    pub max_entries: usize
    pub default_ttl: Duration
    pub cleanup_interval: Duration
    pub enable_lru: bool
```

**struct `CacheConfigBuilder`** (4 members)
```rust
    max_entries: Option<usize>
    default_ttl: Option<Duration>
    cleanup_interval: Option<Duration>
    enable_lru: Option<bool>
```

**struct `Cache`** (3 members)
```rust
    config: CacheConfig
    entries: Arc<RwLock<HashMap<K, CacheEntry<V>>>>
    stats: Arc<RwLock<CacheStats>>
```

**struct `CacheStats`** (6 members)
```rust
    pub hits: u64
    pub misses: u64
    pub evictions: u64
    pub expired_removals: u64
    pub current_size: usize
    pub max_size_reached: u64
```


#### `src/atipicial_clients/circuit_breaker.rs`

**enum `CircuitState`** (3 members)
```rust
    Closed
    Open
    HalfOpen
```

**struct `CircuitBreakerConfig`** (5 members)
```rust
    pub failure_threshold: u32
    pub timeout: Duration
    pub success_threshold: u32
    pub failure_window: Duration
    pub half_open_max_requests: u32
```

**struct `CircuitBreakerConfigBuilder`** (5 members)
```rust
    failure_threshold: Option<u32>
    timeout: Option<Duration>
    success_threshold: Option<u32>
    failure_window: Option<Duration>
    half_open_max_requests: Option<u32>
```

**struct `CircuitBreakerStats`** (8 members)
```rust
    pub total_requests: u64
    pub successful_requests: u64
    pub failed_requests: u64
    pub rejected_requests: u64
    pub state_transitions: u64
    pub current_state: CircuitState
    pub last_failure_time: Option<Instant>
    pub last_success_time: Option<Instant>
```

**struct `CircuitBreaker`** (8 members)
```rust
    config: CircuitBreakerConfig
    state: Arc<RwLock<CircuitState>>
    failure_count: AtomicU32
    success_count: AtomicU32
    half_open_requests: AtomicU32
    last_failure_time: Arc<RwLock<Option<Instant>>>
    last_success_time: Arc<RwLock<Option<Instant>>>
    stats: Arc<RwLock<CircuitBreakerStats>>
```


#### `src/atipicial_clients/connection_pool.rs`

**struct `PoolConfig`** (7 members)
```rust
    pub max_connections: usize
    pub min_idle: usize
    pub max_idle_time: Duration
    pub connection_timeout: Duration
    pub request_timeout: Duration
    pub max_retries: u32
    pub retry_delay: Duration
```

**struct `PoolConfigBuilder`** (7 members)
```rust
    max_connections: Option<usize>
    min_idle: Option<usize>
    max_idle_time: Option<Duration>
    connection_timeout: Option<Duration>
    request_timeout: Option<Duration>
    max_retries: Option<u32>
    retry_delay: Option<Duration>
```

**struct `ConnectionPool`** (5 members)
```rust
    config: PoolConfig
    endpoint: String
    connections: Arc<RwLock<VecDeque<PooledConnection>>>
    semaphore: Arc<Semaphore>
    stats: Arc<RwLock<PoolStats>>
```

**struct `PoolStats`** (7 members)
```rust
    pub total_connections_created: u64
    pub total_requests: u64
    pub successful_requests: u64
    pub failed_requests: u64
    pub retried_requests: u64
    pub current_active_connections: usize
    pub current_idle_connections: usize
```


#### `src/atipicial_clients/errors.rs`

**enum `ProviderError`** (0 members)
```rust
```


#### `src/atipicial_clients/mock_client.rs`

**struct `MockClient`** (1 members)
```rust
    provider: MockProvider
```


#### `src/atipicial_clients/mod.rs`

**struct `TestProvider`** (2 members)
```rust
    network: String
    endpoints: Mutex<Cycle<Iter<'static, &'static str>>>
```


#### `src/atipicial_clients/production_client.rs`

**struct `ProductionRpcClient`** (5 members)
```rust
    pool: ConnectionPool
    cache: RpcCache
    circuit_breaker: CircuitBreaker
    config: ProductionClientConfig
    stats: Arc<RwLock<ProductionClientStats>>
```

**struct `ProductionClientConfig`** (5 members)
```rust
    pub pool_config: PoolConfig
    pub cache_config: CacheConfig
    pub circuit_breaker_config: CircuitBreakerConfig
    pub enable_logging: bool
    pub enable_metrics: bool
```

**struct `ProductionClientConfigBuilder`** (5 members)
```rust
    pool_config: Option<PoolConfig>
    cache_config: Option<CacheConfig>
    circuit_breaker_config: Option<CircuitBreakerConfig>
    enable_logging: Option<bool>
    enable_metrics: Option<bool>
```

**struct `ProductionClientStats`** (7 members)
```rust
    pub total_requests: u64
    pub cache_hits: u64
    pub cache_misses: u64
    pub circuit_breaker_rejections: u64
    pub successful_requests: u64
    pub failed_requests: u64
    pub average_response_time_ms: f64
```


#### `src/atipicial_clients/rate_limiter.rs`

**struct `RateLimiter`** (4 members)
```rust
    max_requests: u32
    window: Duration
    semaphore: Arc<Semaphore>
    bucket: Arc<Mutex<TokenBucket>>
```

**struct `RateLimitPermit`** (1 members)
```rust
    _semaphore: tokio::sync::SemaphorePermit<'a>
```

**struct `RateLimiterBuilder`** (3 members)
```rust
    max_requests: u32
    window: Duration
    max_concurrent: usize
```


#### `src/atipicial_clients/ext/dev_rpc.rs`

**enum `DevRpcMiddlewareError`** (0 members)
```rust
```


#### `src/atipicial_clients/rpc/pubsub.rs`

**struct `SubscriptionStream`** (5 members)
```rust
    pub id: U256
    loaded_elements: VecDeque<R>
    pub(crate) provider: &'a RpcClient<P>
    rx: P::NotificationStream
    ret: PhantomData<R>
```


#### `src/atipicial_clients/rpc/rpc_client.rs`

**enum `AtipicialClient`** (1 members)
```rust
    ATC
```

**struct `RpcClient`** (4 members)
```rust
    provider: P
    interval: Option<Duration>
    from: Option<Address>
    _node_client: Arc<Mutex<Option<AtipicialVersion>>>
```


#### `src/atipicial_clients/rpc/transports/common.rs`

**struct `JsonRpcError`** (3 members)
```rust
    pub code: i64
    pub message: String
    pub data: Option<Value>
```

**struct `Request`** (4 members)
```rust
    id: u64
    jsonrpc: &'a str
    method: &'a str
    params: T
```

**enum `Response`** (1 members)
```rust
    Success { id: u64, result: &'a RawValue
```

**struct `Params`** (2 members)
```rust
    pub subscription: U256
    pub result: &'a RawValue
```

**enum `Authorization`** (3 members)
```rust
    Basic(String)
    Bearer(String)
    Raw(String)
```

**struct `JwtAuth`** (3 members)
```rust
    key: EncodingKey
    id: Option<String>
    clv: Option<String>
```

**struct `Claims`** (3 members)
```rust
    iat: u64
    id: Option<String>
    clv: Option<String>
```


#### `src/atipicial_clients/rpc/transports/http_provider.rs`

**struct `HttpProvider`** (3 members)
```rust
    id: Arc<AtomicU64>
    client: Client
    url: Url
```

**enum `ClientError`** (2 members)
```rust
    ReqwestError(#[from] ReqwestError)
    JsonRpcError(#[from] JsonRpcError)
```

**enum `HttpClientError`** (2 members)
```rust
    InvalidHeader(#[from] header::InvalidHeaderValue)
    ClientBuild(#[from] reqwest::Error)
```


#### `src/atipicial_clients/rpc/transports/ipc.rs`

**struct `Ipc`** (2 members)
```rust
    id: Arc<AtomicU64>
    request_tx: mpsc::UnboundedSender<TransportMessage>
```

**enum `IpcError`** (3 members)
```rust
    JsonError(#[from] serde_json::Error)
    IoError(#[from] io::Error)
    JsonRpcError(#[from] JsonRpcError)
```


#### `src/atipicial_clients/rpc/transports/legacy_ws.rs`

**struct `Ws`** (2 members)
```rust
    id: Arc<AtomicU64>
    instructions: mpsc::UnboundedSender<Instruction>
```

**enum `ClientError`** (4 members)
```rust
    JsonError(#[from] serde_json::Error)
    JsonRpcError(#[from] JsonRpcError)
    UnexpectedBinary(Vec<u8>)
    TungsteniteError(#[from] WsError)
```


#### `src/atipicial_clients/rpc/transports/mock.rs`

**enum `MockResponse`** (2 members)
```rust
    Result(Value)
    Error(JsonRpcError)
```

**struct `MockProvider`** (2 members)
```rust
    rules: Arc<Mutex<Vec<MockRule>>>
    requests: Arc<Mutex<VecDeque<(String, Value)>>>
```


#### `src/atipicial_clients/rpc/transports/retry.rs`

**struct `RetryClientBuilder`** (4 members)
```rust
    timeout_retries: u32
    rate_limit_retries: u32
    initial_backoff: Duration
    compute_units_per_second: u64
```

**enum `RetryClientError`** (3 members)
```rust
    ProviderError(ProviderError)
    TimeoutError
    SerdeJson(serde_json::Error)
```


#### `src/atipicial_clients/rpc/transports/rw.rs`

**struct `RwClient`** (2 members)
```rust
    r: Read
    w: Write
```


#### `src/atipicial_clients/rpc/transports/ws/error.rs`

**enum `WsClientError`** (6 members)
```rust
    JsonError(#[from] serde_json::Error)
    JsonRpcError(#[from] JsonRpcError)
    InternalError(#[from] WsError)
    UnexpectedClose
    DeadChannel
    UnexpectedBinary(Vec<u8>)
```


#### `src/atipicial_clients/rpc/transports/ws/mod.rs`

**struct `WsClient`** (2 members)
```rust
    instructions: mpsc::UnboundedSender<Instruction>
    channel_map: SharedChannelMap
```


#### `src/atipicial_clients/rpc/transports/ws/types.rs`

**struct `ConnectionDetails`** (2 members)
```rust
    pub url: String
    pub auth: Option<Authorization>
```


#### `src/atipicial_codec/binary_decoder.rs`

**struct `Decoder`** (3 members)
```rust
    data: &'a [u8]
    pointer: usize
    marker: usize
```


#### `src/atipicial_codec/binary_encoder.rs`

**struct `Encoder`** (1 members)
```rust
    data: Vec<u8>
```


#### `src/atipicial_codec/error.rs`

**enum `CodecError`** (0 members)
```rust
```


#### `src/atipicial_config/config.rs`

**enum `AtipicialNetwork`** (3 members)
```rust
    MainNet = 0x334f454e
    TestNet = 0x74746e41
    PrivateNet = 0x4e454e
```

**struct `AtipicialConfig`** (13 members)
```rust
    pub network: Option<u32>
    pub address_version: u8
    pub milliseconds_per_block: u32
    pub max_transactions_per_block: u32
    pub memory_pool_max_transactions: u32
    pub max_traceable_blocks: u32
    pub hardforks: HashMap<String, u32>
    pub initial_gas_distribution: u64
    pub validators_count: u32
    pub standby_committee: Vec<String>
    pub seed_list: Vec<String>
    pub nns_resolver: H160
    pub allows_transmission_on_fault: bool
```

**struct `Counter`** (1 members)
```rust
    count: Arc<Mutex<u32>>
```


#### `src/atipicial_config/constant.rs`

**struct `AtipicialConstants`** (0 members)
```rust
```


#### `src/atipicial_config/test_properties.rs`

**struct `TestConstants`** (0 members)
```rust
```


#### `src/atipicial_contract/atipicial_token.rs`

**struct `AtipicialCoin`** (5 members)
```rust
    script_hash: ScriptHash
    total_supply: Option<u64>
    decimals: Option<u8>
    symbol: Option<String>
    provider: Option<&'a RpcClient<P>>
```

**struct `Candidate`** (2 members)
```rust
    pub public_key: Secp256r1PublicKey
    pub votes: i64
```

**struct `AccountState`** (4 members)
```rust
    pub balance: i64
    pub balance_height: Option<i64>
    pub public_key: Option<Secp256r1PublicKey>
    pub last_gas_per_vote: Option<i64>
```


#### `src/atipicial_contract/atipicial_uri.rs`

**struct `AtipicialURI`** (5 members)
```rust
    uri: Option<Url>
    recipient: Option<ScriptHash>
    token: Option<ScriptHash>
    amount: Option<u64>
    provider: Option<&'a RpcClient<P>>
```


#### `src/atipicial_contract/contract_error.rs`

**enum `ContractError`** (0 members)
```rust
```


#### `src/atipicial_contract/contract_management.rs`

**struct `ContractManagement`** (2 members)
```rust
    script_hash: ScriptHash
    provider: Option<&'a RpcClient<P>>
```


#### `src/atipicial_contract/fungible_token_contract.rs`

**struct `FungibleTokenContract`** (5 members)
```rust
    script_hash: H160
    total_supply: Option<u64>
    decimals: Option<u8>
    symbol: Option<String>
    provider: Option<&'a RpcClient<P>>
```


#### `src/atipicial_contract/gas_token.rs`

**struct `GasToken`** (5 members)
```rust
    script_hash: ScriptHash
    total_supply: Option<u64>
    decimals: Option<u8>
    symbol: Option<String>
    provider: Option<&'a RpcClient<P>>
```


#### `src/atipicial_contract/iterator.rs`

**struct `AtipicialIterator`** (4 members)
```rust
    session_id: String
    iterator_id: String
    mapper: Arc<dyn Fn(StackItem) -> Result<T, ContractError> + Send + Sync>
    provider: Option<&'a RpcClient<P>>
```


#### `src/atipicial_contract/name_service.rs`

**enum `RecordType`** (26 members)
```rust
    None = 0
    Txt = 1
    A = 2
    Aaaa = 3
    Cname = 4
    Srv = 5
    Url = 6
    Oauth = 7
    Ipfs = 8
    Email = 9
    Dnssec = 10
    Tlsa = 11
    Smimea = 12
    Hippo = 13
    Http = 14
    Sshfp = 15
    Onion = 16
    Xmpp = 17
    Magnet = 18
    Tor = 19
    I2p = 20
    Git = 21
    Keybase = 22
    Briar = 23
    Zcash = 24
    // …1 more members
```

**struct `NameState`** (3 members)
```rust
    pub name: String
    pub expiration: u32
    pub admin: Option<ScriptHash>
```

**struct `AtipicialNameService`** (2 members)
```rust
    script_hash: ScriptHash
    provider: Option<&'a RpcClient<P>>
```


#### `src/atipicial_contract/nft_contract.rs`

**struct `NftContract`** (5 members)
```rust
    script_hash: H160
    total_supply: Option<u64>
    decimals: Option<u8>
    symbol: Option<String>
    provider: Option<&'a RpcClient<P>>
```


#### `src/atipicial_contract/notary.rs`

**struct `NotaryDeposit`** (2 members)
```rust
    pub amount: i64
    pub till: u32
```

**struct `NotaryContract`** (2 members)
```rust
    script_hash: ScriptHash
    provider: Option<&'a RpcClient<P>>
```


#### `src/atipicial_contract/policy_contract.rs`

**struct `PolicyContract`** (2 members)
```rust
    script_hash: ScriptHash
    provider: Option<&'a RpcClient<P>>
```


#### `src/atipicial_contract/role_management.rs`

**struct `RoleManagement`** (2 members)
```rust
    script_hash: ScriptHash
    provider: Option<&'a RpcClient<P>>
```

**enum `Role`** (4 members)
```rust
    StateValidator = 4
    Oracle = 8
    AtipicialFsAlphabetNode = 16
    P2PNotary = 32
```


#### `src/atipicial_contract/treasury.rs`

**struct `TreasuryContract`** (2 members)
```rust
    script_hash: ScriptHash
    provider: Option<&'a RpcClient<P>>
```


#### `src/atipicial_contract/famous/atipicialburger.rs`

**struct `AtipicialburgerContract`** (5 members)
```rust
    script_hash: ScriptHash
    total_supply: Option<u64>
    decimals: Option<u8>
    symbol: Option<String>
    provider: Option<&'a RpcClient<P>>
```


#### `src/atipicial_contract/famous/atipicialcompound.rs`

**struct `AtipicialCompoundContract`** (2 members)
```rust
    script_hash: ScriptHash
    provider: Option<&'a RpcClient<P>>
```


#### `src/atipicial_contract/famous/contracts.rs`

**enum `Network`** (3 members)
```rust
    MainNet
    TestNet
    PrivateNet
```

**struct `FamousContract`** (5 members)
```rust
    pub script_hash: ScriptHash
    pub name: String
    pub description: Option<String>
    pub network: Network
    pub contract_type: String
```


#### `src/atipicial_contract/famous/flamingo.rs`

**struct `FlamingoContract`** (2 members)
```rust
    script_hash: ScriptHash
    provider: Option<&'a RpcClient<P>>
```


#### `src/atipicial_contract/famous/grandshare.rs`

**struct `GrandShareContract`** (2 members)
```rust
    script_hash: ScriptHash
    provider: Option<&'a RpcClient<P>>
```


#### `src/atipicial_crypto/base58_helper.rs`

**enum `Base58CheckError`** (0 members)
```rust
```


#### `src/atipicial_crypto/error.rs`

**enum `CryptoError`** (0 members)
```rust
```

**enum `Aep2Error`** (0 members)
```rust
```

**enum `SignError`** (0 members)
```rust
```


#### `src/atipicial_crypto/key_pair.rs`

**struct `KeyPair`** (2 members)
```rust
    private_key: Option<Secp256r1PrivateKey>
    public_key: Secp256r1PublicKey
```


#### `src/atipicial_crypto/keys.rs`

**struct `Secp256r1PublicKey`** (1 members)
```rust
    inner: PublicKey
```

**struct `Secp256r1PrivateKey`** (1 members)
```rust
    inner: SecretKey
```

**struct `Secp256r1Signature`** (1 members)
```rust
    inner: Signature
```

**struct `Secp256r1SignedMsg`** (2 members)
```rust
    pub msg: T
    pub signature: Secp256r1Signature
```


#### `src/atipicial_error/mod.rs`

**enum `LegacyError`** (0 members)
```rust
```

**enum `CryptoError`** (0 members)
```rust
```

**enum `WalletError`** (0 members)
```rust
```

**enum `NetworkError`** (0 members)
```rust
```

**enum `TransactionError`** (0 members)
```rust
```

**enum `ContractError`** (0 members)
```rust
```

**enum `SerializationError`** (0 members)
```rust
```


#### `src/atipicial_error/unified.rs`

**enum `AtipicialError`** (0 members)
```rust
```

**struct `ErrorRecovery`** (4 members)
```rust
    pub suggestions: Vec<String>
    pub retryable: bool
    pub retry_after: Option<std::time::Duration>
    pub docs: Vec<String>
```

**enum `AtipicialErrorKind`** (10 members)
```rust
    Network
    Wallet
    Contract
    Transaction
    Configuration
    Validation
    InsufficientFunds
    Timeout
    RateLimit
    Other
```

**struct `ErrorBuilder`** (5 members)
```rust
    kind: ErrorKind
    message: String
    source: Option<Box<dyn std::error::Error + Send + Sync>>
    recovery: ErrorRecovery
    context: ErrorContext
```


#### `src/atipicial_fs/acl.rs`

**enum `Operation`** (2 members)
```rust
    Container(ContainerOperation)
    Object(ObjectOperation)
```

**enum `ContainerOperation`** (5 members)
```rust
    Get
    Put
    Delete
    GetEACL
    SetEACL
```

**enum `ObjectOperation`** (7 members)
```rust
    Get
    Put
    Head
    Search
    Delete
    Range
    Hash
```

**struct `Target`** (2 members)
```rust
    pub role: TargetRole
    pub keys: Vec<String>
```

**enum `TargetRole`** (4 members)
```rust
    Owner
    Group
    Users
    Others
```

**enum `Action`** (2 members)
```rust
    Allow
    Deny
```

**struct `Filter`** (3 members)
```rust
    pub key: String
    pub value: String
    pub operation: FilterOperation
```

**enum `FilterOperation`** (6 members)
```rust
    Eq
    Ne
    Gt
    Ge
    Lt
    Le
```

**struct `EACLRecord`** (4 members)
```rust
    pub operation: Operation
    pub action: Action
    pub target: Target
    pub filters: Vec<Filter>
```

**struct `EACL`** (2 members)
```rust
    pub container_id: ContainerId
    pub records: Vec<EACLRecord>
```

**struct `BearerToken`** (6 members)
```rust
    pub owner_id: OwnerId
    pub token_id: String
    pub expiration: DateTime<Utc>
    pub operations: Vec<AccessPermission>
    pub container_id: ContainerId
    pub signature: Vec<u8>
```

**struct `SessionToken`** (5 members)
```rust
    pub token_id: String
    pub owner_id: OwnerId
    pub expiration: DateTime<Utc>
    pub session_key: String
    pub signature: Vec<u8>
```


#### `src/atipicial_fs/client.rs`

**struct `AtipicialFsClient`** (4 members)
```rust
    config: AtipicialFsConfig
    account: Option<Account>
    http_client: Client
    base_url: String
```


#### `src/atipicial_fs/container.rs`

**struct `Version`** (2 members)
```rust
    pub major: u32
    pub minor: u32
```

**struct `Container`** (8 members)
```rust
    pub id: Option<ContainerId>
    pub owner_id: OwnerId
    pub basic_acl: u32
    pub name: String
    pub creation: Option<DateTime<Utc>>
    pub version: Option<Version>
    pub attributes: Attributes
    pub placement_policy: PlacementPolicy
```

**struct `BasicACL`** (5 members)
```rust
    pub put_allowed: bool
    pub get_allowed: bool
    pub head_allowed: bool
    pub delete_allowed: bool
    pub list_allowed: bool
```


#### `src/atipicial_fs/error.rs`

**enum `AtipicialFsError`** (0 members)
```rust
```


#### `src/atipicial_fs/mod.rs`

**struct `AtipicialFsConfig`** (4 members)
```rust
    pub endpoint: String
    pub auth: Option<AtipicialFsAuth>
    pub timeout_sec: u64
    pub insecure: bool
```

**struct `AtipicialFsConfigBuilder`** (4 members)
```rust
    endpoint: Option<String>
    auth: Option<AtipicialFsAuth>
    timeout_sec: Option<u64>
    insecure: Option<bool>
```

**struct `AtipicialFsAuth`** (2 members)
```rust
    pub wallet_address: String
    pub private_key: Option<String>
```


#### `src/atipicial_fs/object.rs`

**struct `Object`** (7 members)
```rust
    pub id: Option<ObjectId>
    pub container_id: ContainerId
    pub owner_id: OwnerId
    pub object_type: ObjectType
    pub payload: Vec<u8>
    pub attributes: Attributes
    pub created_at: Option<DateTime<Utc>>
```

**struct `MultipartUpload`** (7 members)
```rust
    pub id: Option<ObjectId>
    pub container_id: ContainerId
    pub owner_id: OwnerId
    pub upload_id: String
    pub attributes: Attributes
    pub part_size: u64
    pub max_parts: u64
```

**struct `Part`** (2 members)
```rust
    pub part_number: u32
    pub payload: Vec<u8>
```

**struct `MultipartUploadResult`** (2 members)
```rust
    pub object_id: ObjectId
    pub container_id: ContainerId
```


#### `src/atipicial_fs/types.rs`

**struct `PlacementPolicy`** (3 members)
```rust
    pub replicas: u32
    pub selectors: Vec<Selector>
    pub filters: Vec<Filter>
```

**struct `Selector`** (5 members)
```rust
    pub name: String
    pub count: u32
    pub attribute: String
    pub clause: ClauseOperator
    pub value: String
```

**struct `Filter`** (4 members)
```rust
    pub name: String
    pub key: String
    pub operation: MatchOperator
    pub value: String
```

**enum `ClauseOperator`** (6 members)
```rust
    EQ
    NE
    GT
    GE
    LT
    LE
```

**enum `MatchOperator`** (4 members)
```rust
    EQ
    NE
    RE
    NRE
```

**enum `ObjectType`** (3 members)
```rust
    Regular
    Tombstone
    StorageGroup
```

**struct `Attributes`** (1 members)
```rust
    pub attributes: HashMap<String, String>
```

**enum `AccessPermission`** (7 members)
```rust
    PutObject
    GetObject
    HeadObject
    DeleteObject
    SearchObject
    GetEACL
    SetEACL
```

**struct `SessionToken`** (4 members)
```rust
    pub token_id: String
    pub owner_id: OwnerId
    pub expires_at: u64
    pub signature: Vec<u8>
```


#### `src/atipicial_protocol/account.rs`

**struct `Account`** (13 members)
```rust
    pub key_pair: Option<KeyPair>
    serialize_with = "serialize_address_or_script_hash"
    deserialize_with = "deserialize_address_or_script_hash"
    )]
    pub address_or_scripthash: AddressOrScriptHash
    pub label: Option<String>
    pub verification_script: Option<VerificationScript>
    pub is_default: bool
    pub is_locked: bool
    pub encrypted_private_key: Option<String>
    pub signing_threshold: Option<u32>
    pub nr_of_participants: Option<u32>
    pub wallet: Option<Weak<Wallet>>
```


#### `src/atipicial_protocol/protocol_error.rs`

**enum `ProtocolError`** (0 members)
```rust
```


#### `src/atipicial_protocol/responses/atipicial_account_state.rs`

**struct `AccountState`** (3 members)
```rust
    pub balance: i64
    pub balance_height: Option<i64>
    pub public_key: Option<Secp256r1PublicKey>
```


#### `src/atipicial_protocol/responses/atipicial_address.rs`

**struct `AtipicialAddress`** (4 members)
```rust
    pub address: String
    pub has_key: bool
    pub label: Option<String>
    pub watch_only: bool
```


#### `src/atipicial_protocol/responses/atipicial_application_log.rs`

**struct `ApplicationLog`** (2 members)
```rust
    pub transaction_id: H256
    pub executions: Vec<Execution>
```

**struct `Execution`** (6 members)
```rust
    pub trigger: String
    pub state: VMState
    pub exception: Option<String>
    pub gas_consumed: String
    pub stack: Vec<StackItem>
    pub notifications: Vec<LogNotification>
```


#### `src/atipicial_protocol/responses/atipicial_balances.rs`

**struct `Aep11Balances`** (2 members)
```rust
    pub address: String
    pub balances: Vec<Aep11Balance>
```

**struct `Aep11Balance`** (5 members)
```rust
    pub name: String
    pub symbol: String
    pub decimals: String
    pub tokens: Vec<Aep11Token>
    pub asset_hash: ScriptHash
```

**struct `Aep11Token`** (3 members)
```rust
    pub token_id: String
    pub amount: String
    pub last_updated_block: u32
```

**struct `Aep17Balances`** (2 members)
```rust
    pub address: String
    pub balances: Vec<Aep17Balance>
```

**struct `Aep17Balance`** (6 members)
```rust
    pub name: Option<String>
    pub symbol: Option<String>
    pub decimals: Option<String>
    pub amount: String
    pub last_updated_block: u32
    pub asset_hash: ScriptHash
```


#### `src/atipicial_protocol/responses/atipicial_block.rs`

**struct `AtipicialBlock`** (14 members)
```rust
    pub hash: H256
    pub size: i32
    pub version: i32
    pub prev_block_hash: H256
    pub merkle_root_hash: H256
    pub time: u64
    pub nonce: String
    pub index: u32
    pub primary: Option<i32>
    pub next_consensus: String
    pub witnesses: Option<Vec<AtipicialWitness>>
    pub transactions: Option<Vec<RTransaction>>
    pub confirmations: i32
    pub next_block_hash: Option<H256>
```


#### `src/atipicial_protocol/responses/atipicial_find_states.rs`

**struct `States`** (4 members)
```rust
    pub first_proof: Option<String>
    pub last_proof: Option<String>
    pub truncated: bool
    pub results: Vec<StateResult>
```

**struct `StateResult`** (2 members)
```rust
    pub key: String
    pub value: String
```


#### `src/atipicial_protocol/responses/atipicial_get_claimable.rs`

**struct `Claimables`** (3 members)
```rust
    pub claims: Vec<Claim>
    pub address: String
    pub total_unclaimed: String
```

**struct `Claim`** (8 members)
```rust
    pub tx_id: String
    pub index: u64
    pub atipicial_value: u64
    pub start_height: u64
    pub end_height: u64
    pub generated_gas: String
    pub system_fee: String
    pub unclaimed_gas: String
```


#### `src/atipicial_protocol/responses/atipicial_get_mem_pool.rs`

**struct `MemPoolDetails`** (3 members)
```rust
    pub height: u32
    pub verified: Vec<H256>
    pub unverified: Vec<H256>
```


#### `src/atipicial_protocol/responses/atipicial_get_next_block_validators.rs`

**struct `Validator`** (3 members)
```rust
    pub public_key: String
    pub votes: String
    pub active: bool
```


#### `src/atipicial_protocol/responses/atipicial_get_peers.rs`

**struct `Peers`** (3 members)
```rust
    pub connected: Vec<AddressEntry>
    pub bad: Vec<AddressEntry>
    pub unconnected: Vec<AddressEntry>
```

**struct `AddressEntry`** (2 members)
```rust
    pub address: String
    pub port: u16
```


#### `src/atipicial_protocol/responses/atipicial_get_state_height.rs`

**struct `StateHeight`** (2 members)
```rust
    pub local_root_index: u32
    pub validated_root_index: u32
```


#### `src/atipicial_protocol/responses/atipicial_get_state_root.rs`

**struct `StateRoot`** (4 members)
```rust
    pub version: u32
    pub index: u32
    pub root_hash: H256
    pub witnesses: Vec<AtipicialWitness>
```


#### `src/atipicial_protocol/responses/atipicial_get_unclaimed_gas.rs`

**struct `UnclaimedGas`** (2 members)
```rust
    pub unclaimed: String
    pub address: String
```


#### `src/atipicial_protocol/responses/atipicial_get_unspents.rs`

**struct `Unspents`** (2 members)
```rust
    pub address: String
    pub balances: Vec<Balance>
```

**struct `Balance`** (5 members)
```rust
    pub(crate) unspent_transactions: Vec<UnspentTransaction>
    pub(crate) asset_hash: String
    pub(crate) asset_name: String
    pub(crate) asset_symbol: String
    pub(crate) amount: f64
```

**struct `UnspentTransaction`** (3 members)
```rust
    pub tx_id: String
    pub index: u32
    pub value: f64
```


#### `src/atipicial_protocol/responses/atipicial_get_version.rs`

**struct `AtipicialVersion`** (6 members)
```rust
    pub tcp_port: Option<u16>
    pub ws_port: Option<u16>
    pub nonce: u32
    pub user_agent: String
    pub rpc: Option<AtipicialRpcSettings>
    pub protocol: Option<AtipicialProtocol>
```

**struct `AtipicialRpcSettings`** (2 members)
```rust
    pub max_iterator_result_items: u32
    pub session_enabled: bool
```

**struct `AtipicialProtocol`** (18 members)
```rust
    pub network: u32
    pub validators_count: Option<u32>
    pub ms_per_block: u32
    rename = "maxvaliduntilblockincrement"
    default = "default_max_valid_until_block_increment"
    )]
    pub max_valid_until_block_increment: u32
    pub max_traceable_blocks: u32
    pub address_version: u32
    pub max_transactions_per_block: u32
    rename = "memorypoolmaxtransactions"
    default = "default_memory_pool_max_transactions"
    )]
    pub memory_pool_max_transactions: u32
    pub initial_gas_distribution: u64
    pub hard_forks: Vec<HardForks>
    pub standby_committee: Vec<String>
    pub seed_list: Vec<String>
```

**struct `HardForks`** (2 members)
```rust
    pub name: String
    pub block_height: u32
```


#### `src/atipicial_protocol/responses/atipicial_get_wallet_balance.rs`

**struct `Balance`** (1 members)
```rust
    pub balance: String
```


#### `src/atipicial_protocol/responses/atipicial_list_plugins.rs`

**struct `Plugin`** (3 members)
```rust
    pub name: String
    pub version: String
    pub interfaces: Vec<String>
```


#### `src/atipicial_protocol/responses/atipicial_network_fee.rs`

**struct `AtipicialNetworkFee`** (1 members)
```rust
    pub network_fee: i64
```


#### `src/atipicial_protocol/responses/atipicial_send_raw_transaction.rs`

**struct `RawTransaction`** (1 members)
```rust
    pub hash: H256
```


#### `src/atipicial_protocol/responses/atipicial_transaction_result.rs`

**struct `TransactionResult`** (16 members)
```rust
    pub hash: H256
    pub size: i32
    pub version: i32
    pub nonce: u32
    pub sender: String
    pub sys_fee: String
    pub net_fee: String
    pub valid_until_block: i32
    pub signers: Vec<TransactionSigner>
    pub attributes: Vec<TransactionAttribute>
    pub script: String
    pub witnesses: Vec<AtipicialWitness>
    pub block_hash: Option<H256>
    pub confirmations: Option<i32>
    pub block_time: Option<u64>
    pub vm_state: Option<AtipicialVMStateType>
```

**struct `AtipicialTransactionSigner`** (5 members)
```rust
    account: H160
    scopes: Vec<WitnessScope>
    allowed_contracts: Option<Vec<String>>
    allowed_groups: Option<Vec<String>>
    rules: Option<Vec<WitnessRule>>
```


#### `src/atipicial_protocol/responses/atipicial_transfers.rs`

**struct `Aep11Transfers`** (3 members)
```rust
    pub sent: Vec<Aep11Transfer>
    pub received: Vec<Aep11Transfer>
    pub transfer_address: String
```

**struct `Aep11Transfer`** (8 members)
```rust
    pub token_id: String
    pub timestamp: u64
    pub asset_hash: ScriptHash
    pub transfer_address: String
    pub amount: u64
    pub block_index: u32
    pub transfer_notify_index: u32
    pub tx_hash: H256
```

**struct `Aep17Transfers`** (3 members)
```rust
    pub sent: Vec<Aep17Transfer>
    pub received: Vec<Aep17Transfer>
    pub transfer_address: String
```

**struct `Aep17Transfer`** (7 members)
```rust
    pub timestamp: u64
    pub asset_hash: ScriptHash
    pub transfer_address: String
    pub amount: u64
    pub block_index: u32
    pub transfer_notify_index: u32
    pub tx_hash: H256
```


#### `src/atipicial_protocol/responses/atipicial_validate_address.rs`

**struct `ValidateAddress`** (2 members)
```rust
    pub address: String
    pub is_valid: bool
```


#### `src/atipicial_protocol/responses/atipicial_witness.rs`

**struct `AtipicialWitness`** (2 members)
```rust
    pub invocation: String
    pub verification: String
```


#### `src/atipicial_protocol/responses/diagnostics.rs`

**struct `Diagnostics`** (2 members)
```rust
    pub invoked_contracts: InvokedContract
    pub storage_changes: Vec<StorageChange>
```

**struct `InvokedContract`** (2 members)
```rust
    pub hash: ScriptHash
    pub invoked_contracts: Option<Vec<InvokedContract>>
```

**struct `StorageChange`** (3 members)
```rust
    pub state: String
    pub key: String
    pub value: String
```


#### `src/atipicial_protocol/responses/express_contract_state.rs`

**struct `ExpressContractState`** (2 members)
```rust
    pub hash: ScriptHash
    pub manifest: ContractManifest
```


#### `src/atipicial_protocol/responses/express_shutdown.rs`

**struct `ExpressShutdown`** (1 members)
```rust
    process_id: i32
```


#### `src/atipicial_protocol/responses/notification.rs`

**struct `LogNotification`** (3 members)
```rust
    pub contract: ScriptHash
    pub event_name: String
    pub state: StackItem
```


#### `src/atipicial_protocol/responses/oracle_request.rs`

**struct `OracleRequest`** (8 members)
```rust
    pub request_id: i32
    pub original_transaction_hash: H256
    pub gas_for_response: i32
    pub url: String
    pub filter: String
    pub callback_contract: H160
    pub callback_method: String
    pub user_data: String
```


#### `src/atipicial_protocol/responses/populated_blocks.rs`

**struct `PopulatedBlocks`** (2 members)
```rust
    pub cache_id: String
    pub blocks: Vec<i32>
```


#### `src/atipicial_protocol/responses/response_transaction.rs`

**struct `RTransaction`** (16 members)
```rust
    pub hash: H256
    pub size: u64
    pub version: u8
    pub nonce: u32
    pub sender: String
    pub sys_fee: String
    pub net_fee: String
    pub valid_until_block: u64
    pub signers: Vec<RTransactionSigner>
    pub attributes: Vec<TransactionAttributeEnum>
    pub script: String
    pub witnesses: Vec<AtipicialWitness>
    pub block_hash: H256
    pub confirmations: i32
    pub block_time: i64
    pub vmstate: VMState
```


#### `src/atipicial_protocol/responses/response_transaction_attribute.rs`

**enum `TransactionAttributeType`** (4 members)
```rust
    HighPriority
    OracleResponse
    NotValidBefore
    Conflicts
```

**struct `HighPriorityAttribute`** (0 members)
```rust
```

**struct `OracleResponseAttribute`** (1 members)
```rust
    pub oracle_response: OracleResponse
```

**struct `NotValidBeforeAttribute`** (1 members)
```rust
    pub height: i64
```

**struct `ConflictsAttribute`** (1 members)
```rust
    pub hash: H256
```

**enum `TransactionAttributeEnum`** (4 members)
```rust
    HighPriority(HighPriorityAttribute)
    OracleResponse(OracleResponseAttribute)
    NotValidBefore(NotValidBeforeAttribute)
    Conflicts(ConflictsAttribute)
```

**struct `OracleResponse`** (3 members)
```rust
    pub(crate) id: u64
    pub(crate) response_code: OracleResponseCode
    pub(crate) result: String
```


#### `src/atipicial_protocol/responses/response_transaction_signer.rs`

**struct `RTransactionSigner`** (5 members)
```rust
    pub account: H160
    pub scopes: Vec<WitnessScope>
    pub allowed_contracts: Vec<H160>
    pub allowed_groups: Vec<Secp256r1PublicKey>
    pub rules: Vec<WitnessRule>
```


#### `src/atipicial_sgx/allocator.rs`

**struct `SgxAllocator`** (3 members)
```rust
    heap_base: usize
    heap_size: usize
    allocated: core::sync::atomic::AtomicUsize
```


#### `src/atipicial_sgx/attestation.rs`

**struct `RemoteAttestation`** (4 members)
```rust
    context: sgx_ra_context_t
    sp_public_key: Option<[u8; 64]>
    spid: Option<[u8; 16]>
    quote: Option<Vec<u8>>
```

**struct `QuoteVerifier`** (2 members)
```rust
    ias_api_key: Option<String>
    ias_url: Option<String>
```

**struct `QuoteVerificationResult`** (6 members)
```rust
    pub verified: bool
    pub mrenclave: [u8; 32]
    pub mrsigner: [u8; 32]
    pub product_id: u16
    pub isv_svn: u16
    pub tcb_status: TcbStatus
```

**enum `TcbStatus`** (5 members)
```rust
    UpToDate
    SWHardeningNeeded
    ConfigurationNeeded
    OutOfDate
    Revoked
```


#### `src/atipicial_sgx/crypto.rs`

**struct `SgxCrypto`** (1 members)
```rust
    context: SgxCryptoContext
```

**struct `SgxKeyManager`** (1 members)
```rust
    sealed_keys: Vec<SealedKey>
```


#### `src/atipicial_sgx/enclave.rs`

**struct `EnclaveConfig`** (8 members)
```rust
    pub heap_size: usize
    pub stack_size: usize
    pub tcs_num: u32
    pub debug: bool
    pub product_id: u16
    pub isv_svn: u16
    pub misc_select: u32
    pub attributes: EnclaveAttributes
```

**struct `EnclaveAttributes`** (3 members)
```rust
    pub provision_key: bool
    pub einit_token_key: bool
    pub kss: bool
```

**struct `SgxEnclave`** (3 members)
```rust
    config: EnclaveConfig
    enclave_id: sgx_enclave_id_t
    initialized: bool
```


#### `src/atipicial_sgx/mod.rs`

**enum `SgxError`** (6 members)
```rust
    InitializationFailed(String)
    CryptoError(String)
    NetworkError(String)
    AttestationError(String)
    MemoryError(String)
    EnclaveError(String)
```


#### `src/atipicial_sgx/networking.rs`

**struct `SgxNetworking`** (1 members)
```rust
    secure_channels: Vec<SecureChannel>
```

**struct `SecureChannel`** (4 members)
```rust
    channel_id: [u8; 16]
    remote_public_key: Option<[u8; 64]>
    session_key: Option<[u8; 32]>
    nonce: u64
```


#### `src/atipicial_sgx/storage.rs`

**struct `SecureStorage`** (1 members)
```rust
    sealed_items: Vec<SealedItem>
```


#### `src/atipicial_types/address.rs`

**enum `NameOrAddress`** (2 members)
```rust
    Name(String)
    Address(Address)
```


#### `src/atipicial_types/address_or_scripthash.rs`

**enum `AddressOrScriptHash`** (2 members)
```rust
    Address(Address)
    ScriptHash(H160)
```


#### `src/atipicial_types/error.rs`

**enum `TypeError`** (0 members)
```rust
```


#### `src/atipicial_types/hardfork.rs`

**enum `Hardfork`** (7 members)
```rust
    Aspidochelone = 0
    Basilisk = 1
    Cockatrice = 2
    Domovoi = 3
    Echidna = 4
    Faun = 5
    Gorgon = 6
```


#### `src/atipicial_types/mod.rs`

**struct `ScryptParamsDef`** (3 members)
```rust
    pub log_n: u8
    pub r: u32
    pub p: u32
```


#### `src/atipicial_types/op_code.rs`

**enum `OpCode`** (196 members)
```rust
    PushInt8 = 0x00
    PushInt16 = 0x01
    PushInt32 = 0x02
    PushInt64 = 0x03
    PushInt128 = 0x04
    PushInt256 = 0x05
    PushTrue = 0x08
    PushFalse = 0x09
    PushA = 0x0A
    PushNull = 0x0B
    PushData1 = 0x0C
    PushData2 = 0x0D
    PushData4 = 0x0E
    PushM1 = 0x0F
    Push0 = 0x10
    Push1 = 0x11
    Push2 = 0x12
    Push3 = 0x13
    Push4 = 0x14
    Push5 = 0x15
    Push6 = 0x16
    Push7 = 0x17
    Push8 = 0x18
    Push9 = 0x19
    Push10 = 0x1A
    // …171 more members
```

**struct `OperandSize`** (2 members)
```rust
    prefix_size: u8
    size: u8
```


#### `src/atipicial_types/path_or_string.rs`

**enum `PathOrString`** (2 members)
```rust
    Path(PathBuf)
    String(String)
```


#### `src/atipicial_types/plugin_type.rs`

**enum `NodePluginType`** (12 members)
```rust
    ApplicationLogs
    CoreMetrics
    ImportBlocks
    LevelDbStore
    RocksDbStore
    RpcAep17Tracker
    RpcSecurity
    RpcServerPlugin
    RpcSystemAssetTracker
    SimplePolicy
    StatesDumper
    SystemLog
```


#### `src/atipicial_types/stack_item.rs`

**enum `StackItem`** (3 members)
```rust
    Any
    Pointer {
    value: i64
```

**struct `MapEntry`** (2 members)
```rust
    pub key: StackItem
    pub value: StackItem
```


#### `src/atipicial_types/syncing.rs`

**enum `SyncingStatus`** (2 members)
```rust
    IsFalse
    IsSyncing(Box<SyncProgress>)
```

**struct `SyncProgress`** (17 members)
```rust
    pub current_block: u64
    pub highest_block: u64
    pub starting_block: u64
    pub pulled_states: Option<u64>
    pub known_states: Option<u64>
    pub healed_bytecode_bytes: Option<u64>
    pub healed_bytecodes: Option<u64>
    pub healed_trienode_bytes: Option<u64>
    pub healed_trienodes: Option<u64>
    pub healing_bytecode: Option<u64>
    pub healing_trienodes: Option<u64>
    pub synced_account_bytes: Option<u64>
    pub synced_accounts: Option<u64>
    pub synced_bytecode_bytes: Option<u64>
    pub synced_bytecodes: Option<u64>
    pub synced_storage: Option<u64>
    pub synced_storage_bytes: Option<u64>
```


#### `src/atipicial_types/tx_pool.rs`

**struct `TxPoolInspectSummary`** (3 members)
```rust
    pub to: Option<Address>
    pub value: U256
    pub gas: U256
```

**struct `TxpoolContent`** (2 members)
```rust
    pub pending: BTreeMap<Address, BTreeMap<String, TX>>
    pub queued: BTreeMap<Address, BTreeMap<String, TX>>
```

**struct `TxpoolInspect`** (2 members)
```rust
    pub pending: BTreeMap<Address, BTreeMap<String, TxPoolInspectSummary>>
    pub queued: BTreeMap<Address, BTreeMap<String, TxPoolInspectSummary>>
```

**struct `TxpoolStatus`** (2 members)
```rust
    pub pending: u64
    pub queued: u64
```


#### `src/atipicial_types/vm_state.rs`

**enum `VMState`** (4 members)
```rust
    None = 0
    Halt = 1
    Fault = 2
    Break = 4
```


#### `src/atipicial_types/whitelisted_contract.rs`

**struct `WhitelistedContract`** (4 members)
```rust
    pub contract_hash: H160
    pub method: String
    pub arg_count: i32
    pub fixed_fee: i64
```


#### `src/atipicial_types/contract/aef_file.rs`

**struct `AefFile`** (5 members)
```rust
    pub(crate) compiler: Option<String>
    source_url: String
    method_tokens: Vec<MethodToken>
    pub(crate) script: Bytes
    pub(crate) checksum: Bytes
```

**struct `MethodToken`** (5 members)
```rust
    hash: H160
    method: String
    params_count: u16
    has_return_value: bool
    call_flags: u8
```


#### `src/atipicial_types/contract/aep17contract.rs`

**struct `Aep17Contract`** (3 members)
```rust
    pub script_hash: H160
    pub symbol: String
    pub decimals: u8
```


#### `src/atipicial_types/contract/contract_aef.rs`

**struct `ContractAef`** (6 members)
```rust
    pub magic: i32
    pub compiler: String
    pub source: String
    pub tokens: Vec<ContractMethodToken>
    pub script: String
    pub checksum: i64
```


#### `src/atipicial_types/contract/contract_manifest.rs`

**struct `ContractManifest`** (8 members)
```rust
    pub name: Option<String>
    pub groups: Vec<ContractGroup>
    pub features: HashMap<String, serde_json::Value>
    pub supported_standards: Vec<String>
    pub abi: Option<ContractABI>
    pub permissions: Vec<ContractPermission>
    pub trusts: Vec<String>
    pub extra: Option<HashMap<String, serde_json::Value>>
```

**struct `ContractGroup`** (2 members)
```rust
    pub pub_key: String
    pub signature: String
```

**struct `ContractABI`** (2 members)
```rust
    pub methods: Vec<ContractMethod>
    pub events: Vec<ContractEvent>
```

**struct `ContractMethod`** (5 members)
```rust
    pub name: String
    pub parameters: Vec<ContractParameter2>
    pub offset: usize
    pub return_type: ContractParameterType
    pub safe: bool
```

**struct `ContractEvent`** (2 members)
```rust
    pub name: String
    pub parameters: Vec<ContractParameter>
```

**struct `ContractPermission`** (2 members)
```rust
    pub contract: String
    pub methods: Vec<String>
```


#### `src/atipicial_types/contract/contract_method_token.rs`

**struct `ContractMethodToken`** (5 members)
```rust
    hash: ScriptHash
    method: String
    param_count: u32
    has_return_value: bool
    call_flags: String
```


#### `src/atipicial_types/contract/contract_parameter.rs`

**struct `ContractParameter2`** (2 members)
```rust
    pub name: String
    pub typ: ContractParameterType
```

**struct `ContractParameter`** (3 members)
```rust
    name: Option<String>
    typ: ContractParameterType
    pub value: Option<ParameterValue>
```

**enum `ParameterValue`** (11 members)
```rust
    Boolean(bool)
    Integer(i64)
    ByteArray(String)
    String(String)
    H160(String)
    H256(String)
    PublicKey(String)
    Signature(String)
    Array(Vec<ContractParameter>)
    Map(ContractParameterMap)
    Any
```


#### `src/atipicial_types/contract/contract_parameter_type.rs`

**enum `ContractParameterType`** (13 members)
```rust
    Any = 0x00
    Boolean = 0x10
    Integer = 0x11
    ByteArray = 0x12
    String = 0x13
    H160 = 0x14
    H256 = 0x15
    PublicKey = 0x16
    Signature = 0x17
    Array = 0x20
    Map = 0x22
    InteropInterface = 0x30
    Void = 0xff
```


#### `src/atipicial_types/contract/contract_state.rs`

**struct `ContractState`** (5 members)
```rust
    pub id: i32
    pub aef: ContractAef
    pub update_counter: i32
    pub hash: H160
    pub manifest: ContractManifest
```

**struct `ContractIdentifiers`** (2 members)
```rust
    pub id: i32
    pub hash: H160
```


#### `src/atipicial_types/contract/contract_storage_entry.rs`

**struct `ContractStorageEntry`** (2 members)
```rust
    pub key: String
    pub value: String
```


#### `src/atipicial_types/contract/invocation_result.rs`

**struct `InvocationResult`** (10 members)
```rust
    pub script: String
    pub state: AtipicialVMStateType
    pub gas_consumed: String
    pub exception: Option<String>
    pub notifications: Option<Vec<Notification>>
    pub diagnostics: Option<Diagnostics>
    pub stack: Vec<StackItem>
    pub tx: Option<String>
    pub pending_signature: Option<PendingSignature>
    pub session_id: Option<String>
```

**enum `AtipicialVMStateType`** (4 members)
```rust
    None
    Halt
    Fault
    Break
```

**struct `PendingSignature`** (4 members)
```rust
    pub typ: String
    pub data: String
    pub items: HashMap<String, Item>
    pub network: u32
```

**struct `Item`** (3 members)
```rust
    pub script: String
    pub parameters: Vec<ContractParameter>
    pub signatures: HashMap<String, String>
```

**struct `Diagnostics`** (2 members)
```rust
    pub invoked_contracts: InvokedContract
    pub storage_changes: Vec<StorageChange>
```

**struct `InvokedContract`** (2 members)
```rust
    pub hash: H160
    pub invoked_contracts: Vec<InvokedContract>
```

**struct `StorageChange`** (3 members)
```rust
    pub state: String
    pub key: String
    pub value: String
```

**struct `Notification`** (3 members)
```rust
    pub contract: H160
    pub event_name: String
    pub state: StackItem
```

**enum `NotificationState`** (5 members)
```rust
    Failure
    Success
    Halt
    Fault
    Break
```


#### `src/atipicial_types/contract/native_contract_state.rs`

**struct `NativeContractState`** (4 members)
```rust
    pub id: i32
    pub aef: ContractAef
    hash: H160
    manifest: ContractManifest
```


#### `src/atipicial_types/nns/nns_name.rs`

**struct `NNSName`** (1 members)
```rust
    name: String
```

**struct `NNSRoot`** (1 members)
```rust
    root: String
```


#### `src/atipicial_types/nns/record_state.rs`

**struct `RecordState`** (3 members)
```rust
    pub name: String
    pub record_type: RecordType
    pub data: String
```


#### `src/atipicial_types/nns/record_type.rs`

**enum `RecordType`** (4 members)
```rust
    A = 1
    CNAME = 5
    TXT = 16
    AAAA = 28
```


#### `src/atipicial_wallets/bip39_account.rs`

**struct `Bip39Account`** (2 members)
```rust
    account: Account
    mnemonic: String
```


#### `src/atipicial_wallets/error.rs`

**enum `SignerError`** (0 members)
```rust
```


#### `src/atipicial_wallets/ledger.rs`

**enum `HDPath`** (3 members)
```rust
    LedgerLive(u32)
    Legacy(u32)
    Custom(Vec<u32>)
```

**struct `LedgerWallet`** (4 members)
```rust
    pub(crate) ledger: Arc<T>
    pub(crate) derivation_path: HDPath
    pub(crate) address: Option<Address>
    pub(crate) network: Option<u64>
```


#### `src/atipicial_wallets/wallet_signer.rs`

**struct `WalletSigner`** (3 members)
```rust
    pub(crate) signer: D
    pub(crate) address: Address
    pub(crate) network: Option<u64>
```


#### `src/atipicial_wallets/wallet/aep6account.rs`

**struct `AEP6Account`** (7 members)
```rust
    pub address: Address
    pub label: Option<String>
    pub is_default: bool
    pub lock: bool
    pub key: Option<String>
    pub contract: Option<AEP6Contract>
    pub extra: Option<HashMap<String, String>>
```


#### `src/atipicial_wallets/wallet/aep6contract.rs`

**struct `AEP6Contract`** (3 members)
```rust
    pub script: Option<String>
    pub is_deployed: bool
    pub aep6_parameters: Vec<AEP6Parameter>
```

**struct `AEP6Parameter`** (2 members)
```rust
    pub param_name: String
    pub param_type: ContractParameterType
```


#### `src/atipicial_wallets/wallet/aep6wallet.rs`

**struct `Aep6Wallet`** (5 members)
```rust
    pub(crate) name: String
    pub(crate) version: String
    pub(crate) scrypt: ScryptParamsDef
    pub(crate) accounts: Vec<AEP6Account>
    pub(crate) extra: Option<HashMap<String, String>>
```


#### `src/atipicial_wallets/wallet/wallet.rs`

**struct `Wallet`** (6 members)
```rust
    pub name: String
    pub version: String
    pub scrypt_params: ScryptParamsDef
    pub accounts: HashMap<H160, Account>
    pub(crate) default_account: H160
    pub extra: Option<HashMap<String, String>>
```


#### `src/atipicial_wallets/wallet/wallet_error.rs`

**enum `WalletError`** (0 members)
```rust
```


#### `src/atipicial_x/bridge/bridge_contract.rs`

**struct `AtipicialXBridgeContract`** (2 members)
```rust
    script_hash: ScriptHash
    provider: Option<&'a RpcClient<P>>
```


#### `src/atipicial_x/bridge/evm_bridge.rs`

**struct `AtipicialXBridgeContractEVM`** (1 members)
```rust
    contract: AtipicialXBridgeEVM::AtipicialXBridgeEVMInstance<Arc<RootProvider>>
```


#### `src/atipicial_x/evm/provider.rs`

**struct `AtipicialXProvider`** (3 members)
```rust
    rpc_url: String
    provider: Option<&'a RpcClient<P>>
    evm_provider: Option<Arc<RootProvider>>
```


#### `src/atipicial_x/evm/transaction.rs`

**struct `AtipicialXTransaction`** (5 members)
```rust
    to: Option<H160>
    data: Vec<u8>
    value: u64
    gas_limit: u64
    gas_price: u64
```


#### `src/atipicial_x/evm/wallet.rs`

**struct `AtipicialXWallet`** (1 members)
```rust
    inner: PrivateKeySigner
```

**struct `AtipicialXClient`** (2 members)
```rust
    pub wallet: AtipicialXWallet
    pub provider: AtipicialXProvider<'a, P>
```


#### `src/monitoring/health.rs`

**enum `HealthStatus`** (3 members)
```rust
    Healthy
    Degraded
    Unhealthy
```

**struct `HealthCheck`** (5 members)
```rust
    pub name: String
    pub status: HealthStatus
    pub message: Option<String>
    pub last_check: Option<Instant>
    pub metadata: HashMap<String, String>
```

**struct `HealthRegistry`** (1 members)
```rust
    checks: RwLock<HashMap<String, HealthCheck>>
```

**struct `HealthResponse`** (4 members)
```rust
    pub status: HealthStatus
    pub timestamp: u64
    pub version: String
    pub checks: Vec<HealthCheck>
```


#### `src/monitoring/metrics.rs`

**struct `MetricsSnapshot`** (3 members)
```rust
    pub counters: HashMap<String, f64>
    pub gauges: HashMap<String, f64>
    pub histograms: HashMap<String, Vec<f64>>
```


#### `src/monitoring/mod.rs`

**struct `MonitoringConfig`** (7 members)
```rust
    pub metrics_enabled: bool
    pub metrics_port: u16
    pub tracing_enabled: bool
    pub tracing_endpoint: String
    pub log_level: String
    pub health_check_enabled: bool
    pub health_check_port: u16
```

**struct `MonitoringConfigBuilder`** (7 members)
```rust
    metrics_enabled: Option<bool>
    metrics_port: Option<u16>
    tracing_enabled: Option<bool>
    tracing_endpoint: Option<String>
    log_level: Option<String>
    health_check_enabled: Option<bool>
    health_check_port: Option<u16>
```


#### `src/sdk/hd_wallet.rs`

**struct `DerivationPath`** (5 members)
```rust
    purpose: u32
    coin_type: u32
    account: u32
    change: u32
    index: u32
```

**struct `HDWallet`** (7 members)
```rust
    mnemonic: Mnemonic
    mnemonic_phrase: String
    seed: Vec<u8>
    passphrase: Option<String>
    master_key: ExtendedPrivateKey
    accounts: HashMap<String, Account>
    language: Language
```

**struct `HDWalletBuilder`** (4 members)
```rust
    word_count: usize
    passphrase: Option<String>
    language: Language
    mnemonic: Option<String>
```


#### `src/sdk/mod.rs`

**struct `Atipicial`** (5 members)
```rust
    client: Arc<RpcClient<HttpProvider>>
    network: Network
    endpoint: String
    cache: Option<RpcCache>
    config: SdkConfig
```

**enum `Network`** (3 members)
```rust
    MainNet
    TestNet
    Custom(String)
```

**struct `SdkConfig`** (4 members)
```rust
    pub timeout: Duration
    pub retries: u32
    pub cache_enabled: bool
    pub metrics_enabled: bool
```

**struct `SdkConfigBuilder`** (4 members)
```rust
    timeout: Option<Duration>
    retries: Option<u32>
    cache_enabled: Option<bool>
    metrics_enabled: Option<bool>
```

**struct `DecimalAmount`** (2 members)
```rust
    raw: String
    decimals: u8
```

**enum `DecimalAmountParseError`** (7 members)
```rust
    Empty
    NegativeNotAllowed
    InvalidFormat
    InvalidCharacter
    TooManyFractionalDigits {
    provided: usize
    allowed: u8
```

**struct `Balance`** (3 members)
```rust
    pub atipicial: u64
    pub gas: DecimalAmount
    pub tokens: Vec<TokenBalance>
```

**struct `TokenBalance`** (3 members)
```rust
    pub contract: ScriptHash
    pub symbol: String
    pub amount: DecimalAmount
```

**enum `Token`** (3 members)
```rust
    ATC
    GAS
    Custom(ScriptHash)
```

**struct `AtipicialBuilder`** (2 members)
```rust
    network: Network
    config: SdkConfig
```

**struct `Transfer`** (5 members)
```rust
    from: Wallet
    to: String
    amount: u64
    token: Token
    memo: Option<String>
```


#### `src/sdk/transaction_simulator.rs`

**struct `SimulationResult`** (11 members)
```rust
    pub success: bool
    pub vm_state: AtipicialVMStateType
    pub gas_consumed: u64
    pub system_fee: u64
    pub network_fee: u64
    pub total_fee: u64
    pub state_changes: StateChanges
    pub notifications: Vec<Notification>
    pub return_values: Vec<StackItem>
    pub warnings: Vec<SimulationWarning>
    pub suggestions: Vec<OptimizationSuggestion>
```

**struct `StateChanges`** (5 members)
```rust
    pub storage: HashMap<ScriptHash, Vec<StorageChange>>
    pub balances: HashMap<String, BalanceChange>
    pub transfers: Vec<TokenTransfer>
    pub deployments: Vec<ContractDeployment>
    pub updates: Vec<ContractUpdate>
```

**struct `StorageChange`** (4 members)
```rust
    pub key: Vec<u8>
    pub old_value: Option<Vec<u8>>
    pub new_value: Option<Vec<u8>>
    pub description: Option<String>
```

**struct `BalanceChange`** (4 members)
```rust
    pub address: String
    pub atipicial_delta: i64
    pub gas_delta: i64
    pub token_changes: HashMap<String, i64>
```

**struct `TokenTransfer`** (5 members)
```rust
    pub token: ScriptHash
    pub symbol: String
    pub from: String
    pub to: String
    pub amount: String
```

**struct `ContractDeployment`** (3 members)
```rust
    pub hash: ScriptHash
    pub name: String
    pub cost: u64
```

**struct `ContractUpdate`** (3 members)
```rust
    pub hash: ScriptHash
    pub update_type: String
    pub cost: u64
```

**struct `Notification`** (3 members)
```rust
    pub contract: ScriptHash
    pub event_name: String
    pub state: serde_json::Value
```

**struct `SimulationWarning`** (3 members)
```rust
    pub level: WarningLevel
    pub message: String
    pub suggestion: Option<String>
```

**enum `WarningLevel`** (3 members)
```rust
    Info
    Warning
    Error
```

**struct `OptimizationSuggestion`** (4 members)
```rust
    pub optimization_type: OptimizationType
    pub description: String
    pub gas_savings: Option<u64>
    pub implementation: Option<String>
```

**enum `OptimizationType`** (6 members)
```rust
    BatchOperations
    CacheResults
    OptimizeScript
    ReduceStorageOperations
    UseNativeContracts
    Other(String)
```

**struct `TransactionSimulator`** (5 members)
```rust
    client: Arc<RpcClient<HttpProvider>>
    cache: HashMap<String, CachedSimulation>
    cache_ttl: std::time::Duration
    symbol_cache: HashMap<ScriptHash, String>
    optimization_rules: Vec<OptimizationRule>
```

**enum `OptimizationRule`** (4 members)
```rust
    BatchTransfers
    UseNativeContracts
    MinimizeStorageOps
    CacheRepeatedCalls
```

**struct `GasEstimate`** (6 members)
```rust
    pub system_fee: u64
    pub network_fee: u64
    pub total_fee: u64
    pub gas_consumed: u64
    pub safety_margin: u64
    pub warnings: Vec<SimulationWarning>
```

**struct `TransactionSimulatorBuilder`** (3 members)
```rust
    client: Option<Arc<RpcClient<HttpProvider>>>
    cache_duration: std::time::Duration
    optimization_rules: Vec<OptimizationRule>
```


#### `src/sdk/unified.rs`

**enum `EcosystemClient`** (3 members)
```rust
    N3 {
    provider: crate::sdk::Atipicial
    wallet: N3Wallet
```


#### `src/sdk/websocket.rs`

**enum `SubscriptionType`** (6 members)
```rust
    NewBlocks
    NewTransactions
    TransactionConfirmation(String)
    ContractEvents(ScriptHash)
    AddressActivity(Address)
    TokenTransfers { token: ScriptHash, address: Option<Address>
```

**enum `EventData`** (1 members)
```rust
    NewBlock { height: u32, hash: String, timestamp: u64, transactions: Vec<String>
```

**struct `SubscriptionHandle`** (3 members)
```rust
    id: String
    subscription_type: SubscriptionType
    cancel_tx: oneshot::Sender<()>
```

**struct `WebSocketClient`** (7 members)
```rust
    url: String
    subscriptions: Arc<RwLock<HashMap<String, SubscriptionType>>>
    event_tx: mpsc::UnboundedSender<(SubscriptionType, EventData)>
    event_rx: Option<mpsc::UnboundedReceiver<(SubscriptionType, EventData)>>
    reconnect_interval: Duration
    max_reconnect_attempts: u32
    command_tx: Option<mpsc::UnboundedSender<Command>>
```

**struct `WebSocketClientBuilder`** (3 members)
```rust
    url: String
    reconnect_interval: Duration
    max_reconnect_attempts: u32
```


---

## 🚨 Error Message Atlas — Every Way This System Can Fail


**`atipicial-cli/src/errors.rs`**

**286 distinct error messages.**

- `Invalid argument: {0} - {1}`
- `Wallet error: {0}`
- `File system error: {0}`
- `JSON error: {0}`
- `Network error: {0}`
- `Transaction error: {0}`
- `RPC error: {0}`
- `AtipicialFs error: {0}`
- `Contract error: {0}`
- `Authentication error: {0}`
- `Security error: {0}`
- `Timeout error: {0}`
- `Configuration error: {0}`
- `Input error: {0}`
- `SDK error: {0}`
- `Builder error: {0}`
- `Unknown error: {0}`
- `Invalid input: {0}`
- `Invalid format: {0}`
- `Provider error: {0}`
- `External error: {0}`
- `User cancelled: {0}`
- `Transaction failed: {0}`
- `No RPC client connected`
- `No account loaded`
- `No wallet loaded`
- `Wallet not loaded: {0}`
- `Wallet operation failed: {0}`
- `Invalid operation: {0}`
- `IO error: {0}`
- `Serde JSON error: {0}`
- `Reqwest error: {0}`
- `Tokio join error: {0}`
- `Error: {0}`

**`atipicial-cli/src/utils/error.rs`**

- `Configuration error: {0}`
- `Wallet error: {0}`
- `Network error: {0}`
- `RPC error: {0}`
- `Input error: {0}`
- `IO error: {0}`
- `SDK error: {0}`
- `Transaction error: {0}`
- `Transaction builder error: {0}`
- `Builder error: {0}`
- `Unknown error: {0}`
- `Anyhow error: {0}`
- `Invalid input: {0}`
- `Invalid format: {0}`
- `External error: {0}`

**`examples/middleware/examples/create_custom_middleware.rs`**

- `Gas limit too low: {0}`
- `Invalid transaction amount: {0}`
- `Transaction validation failed: {0}`
- `Middleware chain error: {0}`

**`src/atipicial_builder/error.rs`**

- `Invalid script: {0}`
- `Invalid operation`
- `Invalid argument`
- `Invalid state`
- `Invalid invocation`
- `Stack overflow`
- `Out of gas`
- `Out of memory`
- `Out of cycles`
- `UnknownError`
- `Unsupported operation: {0}`
- `Invalid signer configuration: {0}`
- `Invalid transaction configuration: {0}`
- `Invalid configuration: {0}`
- `Too many signers: {0}`
- `Illegal state: {0}`
- `Illegal argument: {0}`
- `Invalid public key: {0}`
- `Crypto error: {0}`

**`src/atipicial_builder/transaction/transaction_error.rs`**

- `Script format error: {0}`
- `Signer configuration error: {0}`
- `Invalid nonce`
- `Invalid block`
- `Invalid transaction`
- `Invalid witness condition`
- `Too many signers`
- `Duplicate signer`
- `No signers`
- `No script`
- `Empty script`
- `Invalid sender`
- `Invalid state: {0}`
- `Transaction too large`
- `Transaction configuration error: {0}`
- `Codec error: {0}`
- `Crypto error: {0}`
- `Insufficient funds`
- `Invalid script`
- `Unknown transaction`
- `Builder error: {0}`

**`src/atipicial_clients/errors.rs`**

- `nns name not found: {0}`
- `reverse nns name not pointing to itself: {0}`
- `custom error: {0}`
- `unsupported RPC`
- `unsupported node client`
- `Illegal state: {0}`
- `Invalid address`
- `Invalid password`
- `Parse error: {0}`
- `Lock error`
- `Protocol not found`
- `Network not found`

**`src/atipicial_clients/ext/dev_rpc.rs`**

- `Could not revert to snapshot`

**`src/atipicial_clients/rpc/transports/http_provider.rs`**

- `Deserialization Error: {err}. Response: <redacted>`

**`src/atipicial_clients/rpc/transports/ipc.rs`**

- `The IPC server has exited`

**`src/atipicial_clients/rpc/transports/legacy_ws.rs`**

- `Websocket responded with unexpected binary data`
- `Websocket closed with info: {0:?}`
- `Websocket closed`
- `WebSocket connection closed unexpectedly`

**`src/atipicial_clients/rpc/transports/retry.rs`**

- `request timed out`

**`src/atipicial_clients/rpc/transports/rw.rs`**

- `Read error: {0}`
- `Write error: {0}`

**`src/atipicial_clients/rpc/transports/ws/error.rs`**

- `Websocket closed unexpectedly`
- `Unexpected internal channel closure. This is likely a bug. Please report via github`
- `Websocket responded with unexpected binary data`
- `Attempted to listen to unknown subscription: {0:?}`
- `Reconnect limit reached`

**`src/atipicial_codec/error.rs`**

- `Invalid passphrase: {0}`
- `Invalid format`
- `Index out of bounds: {0}`
- `Invalid encoding: {0}`
- `Invalid op code`

**`src/atipicial_contract/contract_error.rs`**

- `Invalid NNS name {0}`
- `Invalid NNS root {0}`
- `Unexpected return type {0}`
- `Unresolvable domain name {0}`
- `Domain name {0} is not available`
- `Domain name {0} is not registered`
- `Unsupported operation: {0}`
- `Runtime error: {0}`
- `Invalid state error: {0}`
- `Invalid argument error: {0}`
- `Provider not set: {0}`
- `Invocation failed: {0}`
- `Invalid response: {0}`
- `Invalid account: {0}`
- `Invalid script hash: {0}`

**`src/atipicial_crypto/base58_helper.rs`**

- `invalid base58 string: {0}`
- `base58check payload is missing checksum bytes`
- `base58check checksum mismatch`

**`src/atipicial_crypto/error.rs`**

- `Invalid passphrase: {0}`
- `Invalid format: {0}`
- `invalid signature length, got {0}, expected 65`
- `Could not recover public key from signature`
- `Invalid public key`
- `Invalid private key`
- `Elliptic curve error: {0}`
- `Signing error`
- `Signature verification error`
- `Decryption error: {0}`
- `Key error: {0}`
- `Invalid passphrase: {0}`
- `Invalid format: {0}`
- `Invalid private key: {0}`
- `Encryption error: {0}`
- `Decryption error: {0}`
- `Verification failed: {0}`
- `Scrypt error: {0}`
- `Base58 error: {0}`
- `Header byte out of range: {0}`
- `Could not recover public key from signature`

**`src/atipicial_error/mod.rs`**

- `Cryptographic error: {0}`
- `Wallet error: {0}`
- `Network error: {0}`
- `Transaction error: {0}`
- `Contract error: {0}`
- `Serialization error: {0}`
- `Configuration error: {0}`
- `Error: {message}`
- `Unsupported operation: {0}`
- `Invalid private key: {0}`
- `Invalid public key: {0}`
- `Signature verification failed`
- `Key generation failed: {0}`
- `Hash operation failed: {0}`
- `Encryption failed: {0}`
- `Decryption failed: {0}`
- `Wallet not found: {0}`
- `Invalid password`
- `Account not found: {0}`
- `Wallet is locked`
- `Backup operation failed: {0}`
- `Recovery operation failed: {0}`
- `YubiHSM error: {0}`
- `Invalid wallet format: {0}`
- `IO error: {0}`
- `Connection failed: {0}`
- `Request timeout`
- `Invalid response: {0}`
- `RPC error: {code} - {message}`
- `Network unreachable: {0}`
- `Rate limit exceeded`
- `HTTP error: {0}`
- `Invalid transaction: {0}`
- `Insufficient funds: required {required}, available {available}`
- `Transaction too large: {size} bytes (max: {max})`
- `Invalid signature`
- `Transaction expired`
- `Nonce too low: {provided} (expected: {expected})`
- `Gas limit exceeded: {used} (limit: {limit})`
- `Contract not found: {0}`
- `Method not found: {0}`
- `Invalid parameters: {0}`
- `Execution failed: {0}`
- `Insufficient gas: {0}`
- `Contract deployment failed: {0}`
- `JSON error: {0}`
- `Invalid format: {0}`
- `Encoding error: {0}`
- `Decoding error: {0}`

**`src/atipicial_error/unified.rs`**

- `Network error: {message}`
- `Wallet error: {message}`
- `Contract error: {message}`
- `Transaction failed: {message}`
- `Configuration error: {message}`
- `Validation error: {message}`
- `Insufficient funds: need {required} but have {available}`
- `Operation timed out after {duration:?}`
- `Rate limit exceeded: {message}`
- `{message}`

**`src/atipicial_fs/error.rs`**

- `Connection error: {0}`
- `Authentication error: {0}`
- `Container error: {0}`
- `Object error: {0}`
- `Access control error: {0}`
- `Serialization error: {0}`
- `Permission denied: {0}`
- `Resource not found: {0}`
- `Invalid argument: {0}`
- `Operation timeout: {0}`
- `Internal error: {0}`
- `Conversion error: {0}`
- `Unsupported AtipicialFs operation: {0}`
- `IO error: {0}`
- `Unexpected response: {0}`

**`src/atipicial_protocol/protocol_error.rs`**

- `RPC responses error: {error}`
- `Invocation fault state: {error}`
- `Client connection error: {message}`
- `Cannot cast {item} to {target}`
- `Illegal state: {message}`
- `HTTP error: {0}`

**`src/atipicial_types/error.rs`**

- `Illegal argument: {0}`
- `Deserialization error: {0}`
- `Illegal state: {0}`
- `Index out of bounds: {0}`
- `Invalid configuration: {0}`
- `Runtime error: {0}`
- `Invalid data: {0}`
- `Unsupported operation: {0}`
- `Transaction error: {0}`
- `Invalid script: {0}`
- `Invalid format: {0}`
- `atipicial-rs not initialized`
- `Contract error: {0}`
- `Unexpected returned type: {0}`
- `Invalid private key`
- `Invalid public key`
- `Invalid address`
- `Invalid signature`
- `Invalid encoding {0}`
- `Invalid op code`
- `Invalid argument {0}`
- `Invalid atipicial name {0}`
- `Numeric overflow`
- `Wif error {0}`

**`src/atipicial_wallets/error.rs`**

- `Invalid passphrase: {0}`
- `Invalid address`
- `Hex decoding error: {0}`

**`src/atipicial_wallets/wallet/wallet_error.rs`**

- `Account state error: {0}`
- `No key pair`
- `No default account`
- `Invalid key pair`
- `Invalid signature`
- `Ledger error: {0}`
- `No accounts in wallet`
- `YubiHSM error: {0}`
- `Decryption error: {0}`
- `Signing error: {0}`
- `File error: {0}`
- `Parse error: {0}`
- `Import error: {0}`
- `Invalid password`
- `Deserialization error: {0}`

---

## ⚙️ Function Inventory — src/, Complete


**`src/lib.rs`** — 2 functions:
`test_create_and_send_transaction`, `setup_test_client`

**3,405 functions inventoried.**


**`src/atipicial_builder/error.rs`** — 1 functions:
`from`


**`src/atipicial_builder/utils.rs`** — 21 functions:
`public_keys_to_scripthash`, `try_public_keys_to_scripthash`, `pubkey_to_scripthash`, `to_value`, `serialize_to_value`, `to_value`, `to_value`, `to_value`, `to_value`, `to_value`, `to_value`, `to_value`, `to_value`, `serialize`, `test_try_public_keys_to_scripthash_rejects_zero_threshold`, `test_try_public_keys_to_scripthash_rejects_threshold_above_len`, `test_public_keys_to_scripthash_panics_instead_of_returning_zero_hash`, `test_transaction_attribute_to_value_is_structured_json`, `test_transaction_send_token_to_value_is_structured_json`, `test_signer_to_value_is_structured_json`, `test_serialize_to_value_panics_on_serialization_failure`


**`src/atipicial_builder/script/interop_service.rs`** — 3 functions:
`hash`, `from_hash`, `price`


**`src/atipicial_builder/script/script_builder.rs`** — 34 functions:
`new`, `op_code`, `op_code_with_arg`, `contract_call`, `sys_call`, `push_params`, `push_param`, `push_integer`, `try_push_integer`, `push_opcode_bytes`, `pad_right`, `push_data`, `push_bool`, `push_array`, `push_map`, `pack`, `to_bytes`, `build_verification_script`, `build_multi_sig_script`, `build_contract_script`, `build_contract_call_and_unwrap_iterator`, `len`, `test_push_empty_array`, `test_push_byte_array`, `test_push_string`, `test_push_integer`, `test_try_push_integer_rejects_values_larger_than_32_bytes`, `test_push_integer_panics_on_values_larger_than_32_bytes`, `test_try_push_integer_matches_legacy_encoding_for_supported_values`, `test_verification_script`, `test_map`, `test_map_nested`, `assert_builder`, `byte_array`


**`src/atipicial_builder/script/script_reader.rs`** — 4 functions:
`get_interop_service_code`, `convert_to_op_code_string`, `get_prefix_size`, `test_convert_to_op_code_string`


**`src/atipicial_builder/transaction/call_flags.rs`** — 2 functions:
`value`, `from_value`


**`src/atipicial_builder/transaction/contract_parameters_context.rs`** — 2 functions:
`new`, `new`


**`src/atipicial_builder/transaction/gas_estimator.rs`** — 7 functions:
`estimate_gas_realtime`, `estimate_gas_with_margin`, `batch_estimate_gas`, `calculate_estimation_accuracy`, `estimate_gas_realtime`, `estimate_gas_with_margin`, `test_calculate_estimation_accuracy`


**`src/atipicial_builder/transaction/invocation_script.rs`** — 22 functions:
`default`, `new`, `new_with_script`, `from_serialized_script`, `from_signature`, `from_message_and_key_pair`, `from_signatures`, `try_encode`, `try_to_array`, `get_signatures`, `size`, `encode`, `decode`, `to_array`, `test_from_message_and_key_pair`, `test_serialize_random_invocation_script`, `test_deserialize_custom_invocation_script`, `test_deserialize_signature_invocation_script`, `test_size`, `test_get_signatures`, `test_from_serialized_script_accepts_raw_script`, `test_try_to_array_rejects_oversized_script`


**`src/atipicial_builder/transaction/mod.rs`** — 1 functions:
`init_logger`


**`src/atipicial_builder/transaction/production_transaction_builder.rs`** — 18 functions:
`default`, `new`, `with_params`, `calculate_network_fee`, `calculate_system_fee`, `new`, `add_verification_script`, `generate_witness`, `default`, `new`, `calculate_network_fee`, `calculate_system_fee`, `add_verification_script`, `generate_witness`, `build_transaction`, `test_fee_calculation`, `test_witness_generation`, `test_production_builder`


**`src/atipicial_builder/transaction/proptest_tests.rs`** — 8 functions:
`opcode_strategy`, `prop_script_builder_opcodes`, `prop_script_builder_integers`, `prop_script_builder_strings`, `prop_transaction_attributes_preserved`, `prop_transaction_fees_non_negative`, `prop_witness_scope_combinations`, `prop_call_flags_combinations`


**`src/atipicial_builder/transaction/transaction.rs`** — 33 functions:
`default`, `deserialize`, `hash`, `new`, `add_witness`, `get_hash_data`, `get_tx_id`, `tx_id`, `serialize_without_witnesses`, `try_serialize_without_witnesses`, `try_encode`, `try_to_array`, `send_tx`, `track_tx`, `get_application_log`, `eq`, `size`, `encode`, `decode`, `to_array`, `transaction_with_attribute`, `transaction_with_signer`, `transaction_with_witness`, `test_transaction_equality_distinguishes_hidden_signer_state`, `test_try_to_array_rejects_oversized_witness_invocation_script`, `test_try_to_array_rejects_invalid_oracle_response_attribute`, `test_to_array_panics_on_invalid_oracle_response_attribute`, `test_try_to_array_rejects_signer_with_too_many_allowed_contracts`, `test_deserialize_rejects_missing_script_field`, `test_deserialize_rejects_non_string_script_field`, `test_send_tx_rejects_signer_with_too_many_allowed_contracts`, `test_tx_id_rejects_invalid_oracle_response_attribute`, `test_try_to_array_matches_legacy_for_valid_oracle_response_attribute`


**`src/atipicial_builder/transaction/transaction_attribute.rs`** — 22 functions:
`try_oracle_response_result_bytes`, `oracle_response_serialized_size`, `try_size`, `try_encode`, `try_to_bytes`, `to_bytes`, `from_bytes`, `try_to_json`, `to_json`, `get_height`, `get_hash`, `size`, `encode`, `decode`, `to_array`, `test_try_to_json_matches_serde_json`, `test_try_to_bytes_rejects_invalid_oracle_response_base64`, `test_try_to_bytes_matches_legacy_for_valid_oracle_response`, `test_try_encode_rejects_invalid_oracle_response_base64`, `test_try_encode_matches_safe_bytes_for_valid_oracle_response`, `test_try_size_rejects_invalid_oracle_response_base64`, `test_try_size_matches_safe_bytes_len_for_valid_oracle_response`


**`src/atipicial_builder/transaction/transaction_builder.rs`** — 43 functions:
`fmt`, `clone`, `eq`, `hash`, `default`, `new`, `with_client`, `allow_transmission_on_fault`, `disallow_transmission_on_fault`, `version`, `nonce`, `valid_until_block`, `first_signer`, `first_signer_by_hash`, `extend_script`, `call_invoke_script`, `build`, `validate`, `is_ready`, `get_unsigned_tx`, `get_system_fee`, `get_network_fee`, `fetch_current_block_count`, `get_sender_balance`, `create_estimated_single_sig_verification_script`, `create_estimated_multi_sig_verification_script`, `is_account_signer`, `sign`, `signers_contain_multi_sig_with_committee_member`, `set_signers`, `add_attributes`, `add_high_priority_attribute`, `add_not_valid_before_attribute`, `add_conflicts_attribute`, `has_attribute_of_type`, `has_attribute`, `is_high_priority`, `contains_duplicate_signers`, `check_and_throw_if_max_attributes_exceeded`, `is_allowed_for_high_priority`, `do_if_sender_cannot_cover_fees`, `throw_if_sender_cannot_cover_fees`, `can_send_cover_fees`


**`src/atipicial_builder/transaction/transaction_builder_tests.rs`** — 59 functions:
`test_build_transaction_with_correct_nonce`, `test_build_transaction_automatically_set_nonce`, `test_build_transaction_fail_building_tx_without_signer`, `test_build_transaction_fail_adding_multiple_signers_concerning_the_same_account`, `test_invoke_script`, `test_invoke_script_without_setting_script`, `test_build_without_setting_script`, `test_sign_transaction_with_additional_signers`, `test_send_invoke_function`, `test_fail_building_transaction_with_incorrect_nonce`, `test_fail_building_transaction_with_invalid_block_number`, `test_override_signer`, `test_attributes_high_priority_multisig_containing_committee_Member`, `test_attributes_high_priority`, `test_attributes_high_priority_not_committee_member`, `test_attributes_high_priority_error_when_multiple`, `test_attributes_not_valid_before`, `test_attributes_not_valid_before_error_when_multiple`, `test_attributes_conflicts`, `test_attributes_conflicts_multiple`, `test_attributes_conflicts_same_exist_already`, `test_attributes_compare_not_valid_before_attributes`, `test_fail_adding_more_than_max_attributes_to_tx_just_attributes`, `test_fail_adding_more_than_max_attributes_to_tx_attributes_and_signers`, `test_fail_adding_more_than_max_attributes_to_tx_signers`, `test_automatic_setting_of_valid_until_block_variable`, `test_automatic_setting_of_system_fee_and_network_fee`, `test_fail_trying_to_sign_transaction_with_account_missing_a_private_key`, `test_fail_automatically_signing_with_multi_sig_account_signer`, `test_fail_with_no_signing_account`, `test_fail_signing_with_account_without_ec_keypair`, `test_fail_sending_transaction_because_it_doesnt_contain_the_right_number_of_witnesses`, `test_contract_witness`, `test_transfer_atipicial_from_normal_account`, `test_extend_script`, `test_invoking_with_params_should_produce_the_correct_request`, `test_fail_signing_with_account_without_ec_key_pair`, `test_do_if_sender_cannot_cover_fees`, `test_do_if_sender_cannot_cover_fees_already_specified_a_supplier`, `test_throw_if_sender_cannot_cover_fees`, `test_throw_if_sender_cannot_cover_fees_already_specified_a_consumer`, `test_build_with_invalid_script`, `test_build_with_script_vm_faults`, `test_get_unsigned_transaction`, `test_version`, `test_additional_network_fee`, `test_additional_system_fee`, `test_set_first_signer`, `test_set_first_signer_fee_only_present`, `test_set_first_signer_not_present`, `test_tracking_transaction_should_return_correct_block`, `test_tracking_transaction_tx_not_sent`, `test_get_application_log`, `test_get_application_log_tx_not_sent`, `test_get_application_log_not_existing`, `test_transmission_on_fault`, `test_prevent_transmission_on_fault`, `test_get_unsigned_tx_rejects_invalid_oracle_response_attribute`, `test_sign_with_multiple_accounts`


**`src/atipicial_builder/transaction/transaction_send_token.rs`** — 1 functions:
`new`


**`src/atipicial_builder/transaction/verification_script.rs`** — 38 functions:
`default`, `new`, `from`, `from_public_key`, `from_multi_sig`, `is_single_sig`, `is_multi_sig`, `hash`, `get_signatures`, `get_public_keys`, `get_signing_threshold`, `get_nr_of_accounts`, `try_encode`, `try_to_array`, `size`, `encode`, `decode`, `to_array`, `test_from_public_key`, `test_from_public_keys`, `test_serialize_deserialize`, `test_get_signing_threshold`, `test_invalid_script`, `test_is_single_sig_script`, `test_is_multi_sig`, `test_fail_is_multi_sig_too_short`, `test_fail_is_multi_sig_n_less_than_one`, `test_fail_is_multi_sig_abrupt_end`, `test_fail_is_multi_sig_wrong_push_data`, `test_fail_is_multi_sig_n_greater_than_m`, `test_fail_is_multi_sig_m_incorrect`, `test_fail_is_multi_sig_missing_push_null`, `test_fail_is_multi_sig_missing_syscall`, `test_fail_is_multi_sig_wrong_interop_service`, `test_public_keys_from_single_sig`, `test_get_signatures_returns_empty_for_verification_script`, `test_public_keys_from_multi_sig`, `test_try_to_array_rejects_oversized_script`


**`src/atipicial_builder/transaction/witness.rs`** — 14 functions:
`default`, `new`, `from_scripts`, `from_scripts_obj`, `create`, `create_multi_sig_witness`, `create_multi_sig_witness_script`, `create_contract_witness`, `try_encode`, `try_to_array`, `size`, `encode`, `decode`, `to_array`


**`src/atipicial_builder/transaction/witness_scope.rs`** — 4 functions:
`byte_repr`, `validate`, `combine`, `split`


**`src/atipicial_builder/transaction/signers/account_signer.rs`** — 30 functions:
`none`, `called_by_entry`, `global`, `is_multi_sig`, `get_script_hash`, `try_encode`, `try_to_array`, `size`, `encode`, `decode`, `read_bounded_list`, `to_array`, `eq`, `hash`, `get_type`, `get_signer_hash`, `set_signer_hash`, `get_scopes`, `get_scopes_mut`, `set_scopes`, `get_allowed_contracts`, `get_allowed_contracts_mut`, `get_allowed_groups`, `get_allowed_groups_mut`, `get_rules`, `get_rules_mut`, `new`, `none_hash160`, `called_by_entry_hash160`, `global_hash160`


**`src/atipicial_builder/transaction/signers/contract_signer.rs`** — 23 functions:
`hash`, `get_type`, `get_signer_hash`, `set_signer_hash`, `get_scopes`, `get_scopes_mut`, `set_scopes`, `get_allowed_contracts`, `get_allowed_contracts_mut`, `get_allowed_groups`, `get_allowed_groups_mut`, `get_rules`, `get_rules_mut`, `new`, `called_by_entry`, `global`, `try_encode`, `try_to_array`, `size`, `encode`, `decode`, `read_bounded_list`, `to_array`


**`src/atipicial_builder/transaction/signers/signer.rs`** — 93 functions:
`get_type`, `get_signer_hash`, `set_signer_hash`, `get_scopes`, `get_scopes_mut`, `set_scopes`, `get_allowed_contracts`, `get_allowed_contracts_mut`, `get_allowed_groups`, `get_allowed_groups_mut`, `get_rules`, `get_rules_mut`, `set_allowed_contracts`, `set_allowed_groups`, `set_rules`, `check_depth`, `validate_subitems`, `eq`, `get_type`, `get_signer_hash`, `set_signer_hash`, `get_scopes`, `get_scopes_mut`, `set_scopes`, `get_allowed_contracts`, `get_allowed_contracts_mut`, `get_allowed_groups`, `get_allowed_groups_mut`, `get_rules`, `get_rules_mut`, `validate_serializable_subitems`, `validate_signer_serialization`, `from_bytes`, `get_type`, `get_signer_hash`, `as_account_signer`, `as_contract_signer`, `as_transaction_signer`, `to_account_signer`, `to_contract_signer`, `try_to_transaction_signer`, `try_encode`, `try_to_array`, `hash`, `from`, `from`, `try_build_transaction_signer`, `into`, `into`, `into`, `into`, `into`, `into`, `serialize`, `size`, `encode`, `decode`, `to_array`, `test_create_signer_with_call_by_entry_scope`, `test_create_signer_with_global_scope`, `test_build_valid_signer1`, `test_build_valid_signer2`, `test_build_valid_signer3`, `test_fail_building_signer_with_global_scope_and_custom_contracts`, `test_fail_building_signer_with_global_scope_and_custom_groups`, `test_fail_building_signer_too_many_contracts`, `test_fail_building_signer_too_many_contracts_added_separately`, `test_fail_building_signer_too_many_groups`, `test_fail_building_signer_too_many_groups_added_separately`, `test_account_signer_try_to_array_rejects_too_many_allowed_contracts`, `test_contract_signer_try_to_array_rejects_too_many_allowed_groups`, `test_transaction_signer_try_to_array_rejects_rule_with_too_many_expressions`, `test_transaction_signer_try_to_array_rejects_too_many_rules`, `test_serialize_global_scope`, `test_serialize_custom_contracts_scope_produces_correct_byte_array`, `test_serialize_custom_group_scope`, `test_serialize_multiple_scopes_contracts_groups_and_rules`, `test_fail_deserialize_too_many_contracts`, `test_fail_deserialize_too_many_contract_groups`, `test_fail_deserialize_too_many_rules`, `test_get_size`, `test_serialize_deserialize_max_nested_rules`, `test_fail_adding_rules_to_global_signer`, `test_fail_adding_too_many_rules`, `test_signer_equals`, `test_to_account_signer_accepts_account_variant`, `test_to_account_signer_rejects_other_variants`, `test_to_contract_signer_accepts_contract_variant`, `test_to_contract_signer_rejects_other_variants`, `test_try_to_transaction_signer_preserves_signer_data`, `test_try_to_transaction_signer_rejects_invalid_scopes`, `test_serialize_with_multiple_scopes_contracts_groups_and_rules`, `test_deserialize`


**`src/atipicial_builder/transaction/signers/transaction_signer.rs`** — 22 functions:
`hash`, `new`, `new_full`, `try_encode`, `try_to_array`, `get_type`, `get_signer_hash`, `set_signer_hash`, `get_scopes`, `get_scopes_mut`, `set_scopes`, `get_allowed_contracts`, `get_allowed_contracts_mut`, `get_allowed_groups`, `get_allowed_groups_mut`, `get_rules`, `get_rules_mut`, `size`, `encode`, `decode`, `read_bounded_list`, `to_array`


**`src/atipicial_builder/transaction/witness_rule/witness_condition.rs`** — 20 functions:
`serialize`, `deserialize_witness_condition`, `deserialize`, `hash`, `json_value`, `byte`, `boolean_expression`, `expression`, `expression_list`, `script_hash`, `group`, `from_bytes`, `validate_serialization`, `encode_legacy`, `try_encode`, `try_to_array`, `size`, `encode`, `decode`, `to_array`


**`src/atipicial_builder/transaction/witness_rule/witness_rule.rs`** — 28 functions:
`new`, `try_encode`, `try_to_array`, `size`, `encode`, `decode`, `to_array`, `test_decode_boolean_condition`, `test_script_hash_condition_serialize_deserialize`, `test_decode_not_condition`, `test_and_condition_serialize_deserialize`, `test_not_condition_serialize_deserialize`, `test_boolean_nil_values`, `test_group_condition_invalid_key_rejected`, `test_decode_or_condition`, `test_called_by_group_condition_serialize_deserialize`, `test_called_by_entry_serialize_deserialize`, `test_called_by_contract_serialize_deserialize`, `test_decode_script_hash_condition`, `test_decode_group_condition`, `test_decode_called_by_entry_condition`, `test_decode_called_by_contract_condition`, `test_condition_try_to_array_rejects_too_many_expressions`, `test_witness_rule_try_to_array_rejects_invalid_condition`, `test_and_condition_decode`, `test_not_condition_decode`, `boolean_expression`, `parse_condition`


**`src/atipicial_clients/api_trait.rs`** — 91 functions:
`rpc_client`, `network`, `nns_resolver`, `block_interval`, `polling_interval`, `max_valid_until_block_increment`, `get_best_block_hash`, `get_block_hash`, `get_block`, `get_raw_block`, `get_block_header_count`, `get_block_count`, `get_block_header`, `get_block_header_by_index`, `get_raw_block_header`, `get_raw_block_header_by_index`, `get_native_contracts`, `get_contract_state`, `get_contract_state_by_id`, `get_native_contract_state`, `get_mem_pool`, `get_raw_mem_pool`, `get_transaction`, `get_raw_transaction`, `get_storage`, `find_storage`, `find_storage_with_id`, `get_transaction_height`, `get_next_block_validators`, `get_committee`, `get_connection_count`, `get_peers`, `get_version`, `send_raw_transaction`, `send_transaction`, `submit_block`, `invoke_function`, `invoke_script`, `get_unclaimed_gas`, `list_plugins`, `validate_address`, `close_wallet`, `dump_priv_key`, `get_wallet_balance`, `get_new_address`, `get_wallet_unclaimed_gas`, `get_wallet_height`, `import_priv_key`, `calculate_network_fee`, `list_address`, `open_wallet`, `send_from`, `send_many`, `send_to_address`, `cancel_transaction`, `get_application_log`, `get_aep17_balances`, `get_aep17_transfers`, `get_aep17_transfers_from`, `get_aep17_transfers_range`, `get_aep11_balances`, `get_aep11_transfers`, `get_aep11_transfers_from`, `get_aep11_transfers_range`, `get_aep11_properties`, `get_state_root`, `get_proof`, `verify_proof`, `get_state_height`, `get_state`, `find_states`, `get_block_by_hash`, `broadcast_address`, `broadcast_block`, `broadcast_get_blocks`, `broadcast_transaction`, `create_contract_deployment_transaction`, `create_contract_update_transaction`, `create_invocation_transaction`, `get_block_by_index`, `get_raw_block_by_index`, `invoke_function_diagnostics`, `invoke_script_diagnostics`, `traverse_iterator`, `terminate_session`, `invoke_contract_verify`, `get_raw_mempool`, `import_private_key`, `get_block_header_hash`, `send_to_address_send_token`, `send_from_send_token`


**`src/atipicial_clients/cache.rs`** — 34 functions:
`builder`, `max_entries`, `default_ttl`, `cleanup_interval`, `enable_lru`, `build`, `default`, `new`, `is_expired`, `access`, `hit_rate`, `new`, `get`, `insert`, `insert_with_ttl`, `remove`, `clear`, `stats`, `cleanup_expired`, `size`, `contains_key`, `evict_lru`, `invalidate_where`, `start_cleanup_task`, `new_rpc_cache`, `cache_block`, `cache_transaction`, `cache_contract_state`, `cache_balance`, `invalidate_by_prefix`, `test_cache_basic_operations`, `test_cache_expiration`, `test_cache_stats`, `test_rpc_cache`


**`src/atipicial_clients/circuit_breaker.rs`** — 25 functions:
`builder`, `failure_threshold`, `timeout`, `success_threshold`, `failure_window`, `half_open_max_requests`, `build`, `default`, `new`, `call`, `should_allow_request`, `on_success`, `on_failure`, `transition_to_closed`, `transition_to_open`, `transition_to_half_open`, `get_state`, `get_stats`, `reset`, `force_open`, `get_failure_rate`, `test_circuit_breaker_closed_state`, `test_circuit_breaker_opens_on_failures`, `test_circuit_breaker_half_open_transition`, `test_circuit_breaker_stats`


**`src/atipicial_clients/connection_pool.rs`** — 24 functions:
`builder`, `max_connections`, `min_idle`, `max_idle_time`, `connection_timeout`, `request_timeout`, `max_retries`, `retry_delay`, `build`, `default`, `new`, `is_expired`, `mark_used`, `health_check`, `new`, `execute`, `get_connection`, `return_connection`, `health_check`, `get_stats`, `start_maintenance_task`, `close`, `test_pool_creation`, `test_pool_stats`


**`src/atipicial_clients/errors.rs`** — 12 functions:
`is_retryable`, `is_rate_limited`, `retry_after`, `http_status`, `is_unknown_transaction`, `is_already_known_transaction`, `is_transaction_rejection`, `is_retryable_http_error`, `eq`, `clone`, `deterministic_provider_errors_are_not_retryable`, `provider_preserves_unknown_transaction_classification`


**`src/atipicial_clients/mock_blocks.rs`** — 3 functions:
`unix_timestamp_ms`, `random_uint256`, `random_uint160`


**`src/atipicial_clients/mock_client.rs`** — 17 functions:
`new`, `mock_response`, `mock_response_error`, `mock_response_ignore_param`, `mock_response_with_file`, `mock_response_with_file_ignore_param`, `mock_response_for_balance_of`, `mock_default_responses`, `mock_invoke_script`, `mock_get_block_count`, `mock_send_raw_transaction`, `mock_get_version`, `mock_invoke_function`, `mock_get_application_log`, `mount_mocks`, `into_client`, `load_jsonrpc_response`


**`src/atipicial_clients/mod.rs`** — 12 functions:
`as_error_response`, `as_serde_error`, `rpc_client_from_parsed_url`, `default_http_provider_client`, `try_http_provider_from_endpoint`, `try_http_provider_from_env`, `new`, `url`, `provider`, `ws`, `test_try_http_provider_from_endpoint_rejects_invalid_url`, `test_try_http_provider_from_endpoint_accepts_valid_url`


**`src/atipicial_clients/production_client.rs`** — 25 functions:
`builder`, `pool_config`, `cache_config`, `circuit_breaker_config`, `enable_logging`, `enable_metrics`, `build`, `default`, `new`, `call`, `get_block_count`, `get_block`, `get_transaction`, `get_contract_state`, `get_aep17_balances`, `send_raw_transaction`, `get_stats`, `get_health`, `health_check`, `create_cache_key`, `is_cacheable_method`, `get_cache_ttl`, `test_production_client_creation`, `test_cache_key_generation`, `test_cacheable_methods`


**`src/atipicial_clients/rate_limiter.rs`** — 19 functions:
`new`, `acquire`, `try_acquire`, `available_tokens`, `reset`, `new`, `max_requests`, `window`, `max_concurrent`, `build`, `default`, `conservative`, `standard`, `aggressive`, `custom`, `test_rate_limiter_basic`, `test_rate_limiter_refill`, `test_concurrent_limiting`, `test_builder`


**`src/atipicial_clients/utils.rs`** — 19 functions:
`maybe`, `interval`, `try_serialize`, `serialize`, `script_hash_from_script`, `public_key_to_address`, `public_key_to_script_hash`, `private_key_to_script_hash`, `private_key_to_address`, `script_hash_to_address`, `address_to_script_hash`, `script_hash_to_hex`, `script_hash_from_hex`, `address_to_hex`, `hex_to_address`, `serialize`, `test_try_serialize_matches_serde_json_for_valid_value`, `test_try_serialize_returns_error_on_serialization_failure`, `test_serialize_panics_on_serialization_failure`


**`src/atipicial_clients/ext/dev_rpc.rs`** — 8 functions:
`inner`, `from_err`, `as_inner`, `from`, `new`, `snapshot`, `revert_to_snapshot`, `test_snapshot`


**`src/atipicial_clients/rpc/connections.rs`** — 1 functions:
`fetch`


**`src/atipicial_clients/rpc/pubsub.rs`** — 4 functions:
`new`, `set_loaded_elements`, `poll_next`, `drop`


**`src/atipicial_clients/rpc/rpc_client.rs`** — 113 functions:
`encode_hex_parameter_as_base64`, `provider_error_from_builder`, `try_transaction_signers`, `from_str`, `as_ref`, `new`, `node_client`, `with_sender`, `request`, `rpc_client`, `network`, `get_best_block_hash`, `get_block_hash`, `get_block`, `get_block_by_hash`, `get_raw_block`, `get_block_header_count`, `get_block_count`, `get_block_header`, `get_block_header_by_index`, `get_raw_block_header`, `get_raw_block_header_by_index`, `get_native_contracts`, `get_contract_state`, `get_contract_state_by_id`, `get_native_contract_state`, `get_mem_pool`, `get_raw_mem_pool`, `get_transaction`, `get_raw_transaction`, `get_storage`, `find_storage`, `find_storage_with_id`, `get_transaction_height`, `get_next_block_validators`, `get_committee`, `get_connection_count`, `get_peers`, `get_version`, `send_raw_transaction`, `send_transaction`, `submit_block`, `broadcast_address`, `broadcast_block`, `broadcast_get_blocks`, `broadcast_transaction`, `create_contract_deployment_transaction`, `create_contract_update_transaction`, `create_invocation_transaction`, `invoke_function`, `invoke_script`, `get_unclaimed_gas`, `list_plugins`, `validate_address`, `close_wallet`, `dump_priv_key`, `get_wallet_balance`, `get_new_address`, `get_wallet_unclaimed_gas`, `get_wallet_height`, `import_priv_key`, `calculate_network_fee`, `list_address`, `open_wallet`, `send_from`, `send_many`, `send_to_address`, `cancel_transaction`, `get_application_log`, `get_aep17_balances`, `get_aep17_transfers`, `get_aep17_transfers_from`, `get_aep17_transfers_range`, `get_aep11_balances`, `get_aep11_transfers`, `get_aep11_transfers_from`, `get_aep11_transfers_range`, `get_aep11_properties`, `get_state_root`, `get_proof`, `verify_proof`, `get_state_height`, `get_state`, `find_states`, `get_block_by_index`, `get_raw_block_by_index`, `invoke_function_diagnostics`, `invoke_script_diagnostics`, `traverse_iterator`, `terminate_session`, `invoke_contract_verify`, `get_raw_mempool`, `import_private_key`, `get_block_header_hash`, `send_to_address_send_token`, `send_from_send_token`, `connect_ipc`, `url`, `url_mut`, `rw`, `assert_parse_error`, `assert_illegal_state`, `invalid_signer`, `get_storage_rejects_invalid_hex_key_before_request`, `send_raw_transaction_rejects_invalid_hex_before_request`, `verify_proof_rejects_invalid_hex_proof_before_request`, `encodable_test_aef`, `create_contract_deployment_transaction_rejects_invalid_aef_before_request`, `create_contract_update_transaction_rejects_invalid_aef_before_request`, `send_transaction_rejects_invalid_transaction_before_request`, `find_states_rejects_invalid_start_key_before_request`, `invoke_script_rejects_invalid_signer_before_request`, `invoke_function_rejects_invalid_signer_before_request`


**`src/atipicial_clients/rpc/transports/common.rs`** — 37 functions:
`fmt`, `spelunk_revert`, `is_retryable`, `is_rate_limited`, `retry_after`, `is_unknown_transaction`, `is_already_known_transaction`, `is_transaction_rejection`, `is_revert`, `as_revert_data`, `fmt`, `is_zst`, `new`, `deserialize`, `expecting`, `visit_map`, `fmt`, `basic`, `bearer`, `raw`, `fmt`, `from_slice`, `from_hex`, `as_bytes`, `into_bytes`, `new`, `generate_token`, `generate_token_with_claims`, `generate_claims_at_timestamp`, `validate_token`, `classifies_transient_and_unknown_transaction_errors`, `extracts_non_negative_provider_backoff`, `json_rpc_error_display_redacts_provider_data`, `json_rpc_error_debug_redacts_provider_data`, `deser_response`, `ser_request`, `test_roundtrip`


**`src/atipicial_clients/rpc/transports/http_provider.rs`** — 22 functions:
`fmt`, `fmt`, `from`, `fetch`, `parse_retry_after`, `attach_retry_after`, `collect_body_with_limit`, `default`, `url`, `url_mut`, `new_with_auth`, `new_with_client`, `clone`, `provider_returning_status`, `provider_returning_response`, `provider_debug_redacts_url_credentials_and_paths`, `preserves_retryable_http_error_statuses`, `sanitizes_status_error_urls_with_compatible_error_variant`, `preserves_json_rpc_errors_from_http_error_responses`, `classifies_truncated_response_body_as_retryable`, `rejects_oversized_content_length_without_reading_body`, `poll_next`


**`src/atipicial_clients/rpc/transports/ipc.rs`** — 30 functions:
`new`, `connect`, `split`, `connect`, `poll_read`, `poll_write`, `poll_write_vectored`, `is_write_vectored`, `poll_flush`, `poll_shutdown`, `split`, `connect`, `send`, `fetch`, `spawn_ipc_server`, `run_ipc_server`, `next_pending_deadline`, `expire_timed_out_requests`, `handle_ipc_reads`, `handle_ipc_writes`, `handle_write_message`, `handle_bytes`, `send_response`, `send_notification`, `from`, `as_error_response`, `as_serde_error`, `connect`, `request`, `subscription`


**`src/atipicial_clients/rpc/transports/legacy_ws.rs`** — 39 functions:
`new`, `is_canceled`, `send`, `truncate_for_log`, `fmt`, `new`, `ready`, `connect`, `connect`, `connect_with_auth`, `send`, `fetch`, `new`, `is_done`, `spawn`, `close_all_subscriptions`, `service_request`, `service_subscribe`, `service_unsubscribe`, `service`, `handle_ping`, `handle_text`, `next_request_deadline`, `expire_timed_out_requests`, `handle_notification`, `handle`, `handle`, `tick`, `tick`, `to_client_error`, `as_error_response`, `as_serde_error`, `from`, `request`, `subscription`, `deserialization_fails`, `connect`, `connect`, `connect_with_auth`


**`src/atipicial_clients/rpc/transports/mock.rs`** — 20 functions:
`kind`, `matches`, `matches`, `kind`, `new`, `push_result`, `push_result_with_params`, `push_result_with_partial_params`, `push_error_any`, `push_error`, `push_rule`, `take_requests`, `assert_request`, `fetch`, `select_best_rule`, `json_partial_match`, `test_json_partial_match_object_subset`, `test_json_partial_match_array_prefix`, `test_mock_provider_selects_most_specific_rule`, `test_mock_provider_records_requests`


**`src/atipicial_clients/rpc/transports/retry.rs`** — 35 functions:
`should_retry`, `backoff_hint`, `new`, `set_compute_units`, `timeout_retries`, `rate_limit_retries`, `compute_units_per_second`, `initial_backoff`, `build`, `default`, `new`, `drop`, `retry_backoff`, `with_jitter`, `sleep_backoff`, `from`, `fetch`, `should_retry`, `backoff_hint`, `compute_unit_offset_in_secs`, `maybe_connectivity`, `compute_offset`, `can_measure_unit_offset_single_request`, `can_measure_unit_offset_1x_over_budget`, `can_measure_unit_offset_2x_over_budget`, `zero_compute_units_do_not_panic`, `exponential_backoff_doubles_and_caps`, `retry_client_for_responses`, `retry_client_for_responses_with_backoff`, `retries_truncated_response_then_succeeds`, `connectivity_retries_wait_before_retrying`, `retry_exhaustion_releases_queue_accounting`, `can_extract_backoff`, `test_alchemy_ip_rate_limit`, `test_rate_limit_omitted_id`


**`src/atipicial_clients/rpc/transports/rw.rs`** — 7 functions:
`new`, `read_client`, `write_client`, `transpose`, `split`, `from`, `fetch`


**`src/atipicial_clients/rpc/transports/ws/backend.rs`** — 11 functions:
`truncate_for_log`, `shutdown`, `new_for_test`, `connect`, `connect`, `connect_with_config`, `new`, `handle_text`, `handle`, `handle`, `spawn`


**`src/atipicial_clients/rpc/transports/ws/error.rs`** — 3 functions:
`as_error_response`, `as_serde_error`, `from`


**`src/atipicial_clients/rpc/transports/ws/manager.rs`** — 32 functions:
`new`, `reset_server_ids`, `count`, `add_alias`, `remove_alias`, `end_subscription`, `handle_notification`, `req_success`, `has`, `to_reissue`, `service_subscription_request`, `make_manager`, `expires_timed_out_requests`, `cleans_up_subscription_on_timeout`, `next_id`, `connect`, `connect_internal`, `connect_with_reconnects`, `connect_with_reconnects`, `connect_with_config`, `connect_with_config_and_reconnects`, `reconnect_backend`, `reconnect_backend`, `reconnect`, `req_success`, `req_fail`, `handle`, `service_request`, `next_request_deadline`, `expire_timed_out_requests`, `service_instruction`, `spawn`


**`src/atipicial_clients/rpc/transports/ws/mod.rs`** — 11 functions:
`connect`, `connect_with_reconnects`, `connect_with_config`, `connect_with_config_and_reconnects`, `make_request`, `fmt`, `fetch`, `connect`, `connect_with_reconnects`, `connect_with_auth`, `connect_with_auth_and_reconnects`


**`src/atipicial_clients/rpc/transports/ws/types.rs`** — 15 functions:
`serialize_raw`, `deserialize`, `expecting`, `visit_map`, `fmt`, `new`, `new`, `from`, `from`, `to_request`, `serialize_raw`, `to_request`, `serialize_raw`, `into_client_request`, `it_desers_pubsub_items`


**`src/atipicial_clients/rx/atipicial_rust_rx_trait.rs`** — 6 functions:
`block_stream`, `replay_blocks_stream`, `replay_blocks_stream_ordered`, `catch_up_to_latest_block_stream`, `catch_up_to_latest_and_subscribe_to_new_blocks_stream`, `subscribe_to_new_blocks_stream`


**`src/atipicial_codec/binary_decoder.rs`** — 43 functions:
`next`, `new`, `read_u8_safe`, `read_bool_safe`, `read_bool`, `read_u8`, `read_u16`, `read_i16`, `read_u32`, `read_i32`, `read_u64`, `read_i64`, `read_bigint`, `read_encoded_ec_point`, `read_bytes`, `read_var_bytes`, `read_var_bytes_bounded`, `read_var_int`, `read_var_string`, `read_var_string_bounded`, `read_push_bytes`, `read_push_int`, `read_push_string`, `read_serializable`, `read_serializable_list_len`, `read_serializable_list_bounded`, `read_serializable_list`, `read_serializable_list_var_bytes`, `mark`, `reset`, `available`, `test_read_u16_is_little_endian`, `test_read_var_int_u16_is_little_endian`, `test_read_var_bytes_bounded_rejects_excess_length`, `test_read_var_string_bounded_rejects_excess_length`, `test_read_push_data_bytes`, `test_fail_read_push_data`, `test_read_push_data_string`, `test_read_push_data_big_integer`, `test_read_u32`, `test_read_i64`, `test_read_serializable_list_rejects_length_exceeding_remaining_bytes`, `test_read_serializable_list_bounded_rejects_excess_length`


**`src/atipicial_codec/binary_encoder.rs`** — 31 functions:
`default`, `new`, `size`, `write_bool`, `write_u8`, `write_i16`, `write_i32`, `write_i64`, `write_u16`, `write_u32`, `write_u64`, `write_bytes`, `write_var_int`, `write_var_string`, `write_fixed_string`, `write_var_bytes`, `write_serializable_fixed`, `write_serializable_list_fixed`, `write_serializable_variable_bytes`, `write_serializable_variable_list`, `write_serializable_variable_list_bytes`, `reset`, `to_bytes`, `finish`, `write`, `test_write_u32`, `test_write_i64`, `test_write_u16`, `test_write_var_int`, `test_write_var_bytes`, `test_write_var_string`


**`src/atipicial_codec/encode.rs`** — 20 functions:
`size`, `encode`, `decode`, `to_array`, `size`, `encode`, `decode`, `to_array`, `size`, `encode`, `decode`, `to_array`, `size`, `encode`, `decode`, `to_array`, `var_size`, `get_var_size`, `var_size`, `var_size`


**`src/atipicial_codec/error.rs`** — 1 functions:
`hash`


**`src/atipicial_config/config.rs`** — 18 functions:
`to_magic`, `from_magic`, `atipicial_config_lock`, `hash`, `default`, `new`, `set_network`, `get_max_valid_until_block_increment`, `is_hardfork_enabled`, `get_hardfork_height`, `set_hardfork_height`, `mainnet`, `testnet`, `hash`, `eq`, `default`, `new`, `get_and_increment`


**`src/atipicial_config/constant.rs`** — 8 functions:
`max_rpc_message_size`, `max_rpc_message_size`, `rpc_request_timeout`, `rpc_request_timeout`, `new`, `default`, `with_env_var`, `rpc_request_timeout_parses_env_var`


**`src/atipicial_contract/atipicial_token.rs`** — 32 functions:
`new`, `unclaimed_gas`, `unclaimed_gas_contract`, `register_candidate`, `unregister_candidate`, `get_committee`, `get_candidates`, `is_candidate`, `vote`, `cancel_vote`, `build_vote_script`, `get_gas_per_block`, `set_gas_per_block`, `get_register_price`, `set_register_price`, `get_committee_address`, `get_candidate_vote`, `get_account_state`, `call_function_returning_list_of_public_keys`, `resolve_nns_text_record`, `total_supply`, `set_total_supply`, `decimals`, `set_decimals`, `symbol`, `set_symbol`, `resolve_nns_text_record`, `script_hash`, `set_script_hash`, `provider`, `from`, `with_no_balance`


**`src/atipicial_contract/atipicial_uri.rs`** — 10 functions:
`new`, `parse_token`, `from_uri`, `uri_string`, `recipient_address`, `token_string`, `build_transfer_from`, `token_str`, `build_query`, `build_uri`


**`src/atipicial_contract/contract_management.rs`** — 15 functions:
`new`, `with_script_hash`, `get_minimum_deployment_fee`, `set_minimum_deployment_fee`, `get_contract`, `get_contract_by_id`, `get_contract_hash_by_id`, `get_contract_hashes`, `has_method`, `deploy`, `update`, `destroy`, `script_hash`, `set_script_hash`, `provider`


**`src/atipicial_contract/fungible_token_contract.rs`** — 11 functions:
`new`, `total_supply`, `set_total_supply`, `decimals`, `set_decimals`, `symbol`, `set_symbol`, `resolve_nns_text_record`, `script_hash`, `set_script_hash`, `provider`


**`src/atipicial_contract/gas_token.rs`** — 11 functions:
`new`, `total_supply`, `set_total_supply`, `decimals`, `set_decimals`, `symbol`, `set_symbol`, `resolve_nns_text_record`, `script_hash`, `set_script_hash`, `provider`


**`src/atipicial_contract/iterator.rs`** — 4 functions:
`fmt`, `new`, `traverse`, `terminate_session`


**`src/atipicial_contract/mod.rs`** — 2 functions:
`checked_vm_integer`, `checked_vm_integer_rejects_negative_and_oversized_values`


**`src/atipicial_contract/name_service.rs`** — 25 functions:
`new`, `with_script_hash`, `add_root`, `get_roots`, `get_symbol`, `get_decimals`, `register`, `set_admin`, `set_record`, `delete_record`, `is_available`, `renew`, `get_name_state`, `check_domain_name_availability`, `total_supply`, `set_total_supply`, `decimals`, `set_decimals`, `symbol`, `set_symbol`, `resolve_nns_text_record`, `set_name`, `script_hash`, `set_script_hash`, `provider`


**`src/atipicial_contract/nft_contract.rs`** — 11 functions:
`new`, `total_supply`, `set_total_supply`, `decimals`, `set_decimals`, `symbol`, `set_symbol`, `resolve_nns_text_record`, `script_hash`, `set_script_hash`, `provider`


**`src/atipicial_contract/notary.rs`** — 18 functions:
`from_stack_item`, `new`, `balance_of`, `expiration_of`, `get_max_not_valid_before_delta`, `lock_deposit_until`, `withdraw`, `set_max_not_valid_before_delta`, `supported_standards`, `script_hash`, `set_script_hash`, `provider`, `test_notary_contract_name`, `test_notary_default_constants`, `test_notary_supported_standards`, `test_notary_contract_hash`, `test_notary_deposit_from_stack_item`, `notary_deposit_rejects_invalid_block_height`


**`src/atipicial_contract/policy_contract.rs`** — 42 functions:
`new`, `get_fee_per_byte`, `get_exec_fee_factor`, `get_exec_pico_fee_factor`, `get_storage_price`, `get_milliseconds_per_block`, `set_milliseconds_per_block`, `get_max_valid_until_block_increment`, `set_max_valid_until_block_increment`, `get_max_traceable_blocks`, `set_max_traceable_blocks`, `get_attribute_fee`, `set_attribute_fee`, `is_blocked`, `get_blocked_accounts`, `get_blocked_accounts_all`, `get_blocked_accounts_all_with_batch`, `block_account`, `block_account_address`, `unblock_account`, `unblock_account_address`, `set_fee_per_byte`, `set_exec_fee_factor`, `set_storage_price`, `get_whitelist_fee_contracts`, `get_whitelist_fee_contracts_all`, `get_whitelist_fee_contracts_all_with_batch`, `checked_iterator_batch_size`, `collect_all`, `set_whitelist_fee_contract`, `remove_whitelist_fee_contract`, `recover_fund`, `script_hash`, `set_script_hash`, `provider`, `iterator_invocation_result`, `test_policy_contract_constants`, `test_policy_contract_name`, `test_get_blocked_accounts_iterator_rejects_invalid_items`, `test_get_whitelist_fee_contracts_iterator_rejects_invalid_items`, `blocked_accounts_reject_invalid_batch_sizes_before_opening_iterator`, `whitelist_rejects_invalid_batch_sizes_before_opening_iterator`


**`src/atipicial_contract/role_management.rs`** — 9 functions:
`new`, `get_designated_by_role`, `check_block_index_validity`, `designate_as_role`, `script_hash`, `set_script_hash`, `provider`, `from`, `from`


**`src/atipicial_contract/tests.rs`** — 20 functions:
`create_test_client`, `create_test_account`, `get_test_contract_hash`, `create_test_aef`, `create_test_manifest`, `create_test_signers`, `test_contract_management_deploy`, `test_contract_management_update`, `test_contract_parameter_creation`, `test_contract_parameter_value_extraction`, `test_contract_hash_validation`, `test_script_hash_extension`, `test_aef_file_creation`, `test_contract_manifest_creation`, `test_address_to_script_hash_conversion`, `test_op_code_enum`, `test_vm_state_enum`, `test_stack_item_creation`, `test_contract_parameter_serialization`, `test_production_ready_script_building`


**`src/atipicial_contract/treasury.rs`** — 9 functions:
`new`, `verify`, `supported_standards`, `script_hash`, `set_script_hash`, `provider`, `test_treasury_contract_name`, `test_treasury_supported_standards`, `test_treasury_contract_hash`


**`src/atipicial_contract/famous/atipicialburger.rs`** — 16 functions:
`new`, `with_script_hash`, `wrap`, `unwrap`, `claim_gas`, `get_rate`, `script_hash`, `set_script_hash`, `provider`, `total_supply`, `set_total_supply`, `decimals`, `set_decimals`, `symbol`, `set_symbol`, `resolve_nns_text_record`


**`src/atipicial_contract/famous/atipicialcompound.rs`** — 9 functions:
`new`, `with_script_hash`, `deposit`, `withdraw`, `compound`, `get_apy`, `script_hash`, `set_script_hash`, `provider`


**`src/atipicial_contract/famous/contracts.rs`** — 16 functions:
`fmt`, `from_str`, `new_unchecked`, `new`, `flamingo_flm_token`, `flamingo_flamingo_finance`, `ghostmarket`, `atipicialburger_dao`, `atipicialcompound`, `atipicial_name_service`, `bridge_atipicial_to_eth`, `testnet_nns`, `testnet_faucet`, `get_famous_contracts`, `get_all_famous_contracts`, `test_get_famous_contracts`


**`src/atipicial_contract/famous/flamingo.rs`** — 10 functions:
`new`, `with_script_hash`, `swap`, `add_liquidity`, `remove_liquidity`, `stake`, `claim_rewards`, `script_hash`, `set_script_hash`, `provider`


**`src/atipicial_contract/famous/grandshare.rs`** — 9 functions:
`new`, `with_script_hash`, `submit_proposal`, `vote`, `fund_project`, `claim_funds`, `script_hash`, `set_script_hash`, `provider`


**`src/atipicial_contract/traits/fungible_token.rs`** — 8 functions:
`get_balance_of`, `get_balance_of_hash160`, `get_total_balance`, `transfer_from_account`, `transfer_from_hash160`, `build_transfer_script`, `transfer_from_account_to_nns`, `transfer_from_hash160_to_nns`


**`src/atipicial_contract/traits/nft.rs`** — 21 functions:
`balance_of`, `tokens_of`, `transfer`, `transfer_inner`, `transfer_from_name`, `transfer_to_name`, `build_non_divisible_transfer_script`, `owner_of`, `throw_if_divisible_nft`, `throw_if_sender_is_not_owner`, `transfer_divisible`, `transfer_divisible_from_hashes`, `transfer_divisible_from_name`, `transfer_divisible_to_name`, `build_divisible_transfer_script`, `owners_of`, `throw_if_non_divisible_nft`, `balance_of_divisible`, `tokens`, `properties`, `custom_properties`


**`src/atipicial_contract/traits/smart_contract.rs`** — 34 functions:
`try_name`, `name`, `set_name`, `script_hash`, `set_script_hash`, `provider`, `invoke_function`, `build_invoke_function_script`, `call_function_returning_string`, `call_function_returning_int`, `call_function_returning_bool`, `call_invoke_function`, `throw_if_fault_state`, `call_function_returning_script_hash`, `call_function_returning_iterator`, `call_function_and_unwrap_iterator`, `calc_native_contract_hash`, `calc_native_contract_hash_unchecked`, `calc_contract_hash`, `try_get_manifest`, `get_manifest`, `without_provider`, `with_provider`, `script_hash`, `provider`, `test_manifest`, `test_contract_state`, `test_try_get_manifest_returns_provider_not_set_without_provider`, `test_try_get_manifest_returns_manifest_from_provider`, `test_try_name_returns_manifest_name`, `test_try_name_rejects_missing_manifest_name`, `test_name_returns_contract_hash_when_manifest_name_missing`, `test_get_manifest_returns_default_without_provider`, `test_name_returns_contract_hash_without_provider`


**`src/atipicial_contract/traits/token.rs`** — 11 functions:
`total_supply`, `set_total_supply`, `decimals`, `set_decimals`, `symbol`, `set_symbol`, `get_total_supply`, `get_decimals`, `get_symbol`, `to_fractions`, `resolve_nns_text_record`


**`src/atipicial_crypto/base58_helper.rs`** — 15 functions:
`base58check_encode`, `try_base58check_decode`, `base58check_decode`, `calculate_checksum`, `test_base58_encoding_for_valid_strings`, `test_base58_decoding_for_valid_strings`, `test_base58_decoding_for_invalid_strings`, `test_base58check_encoding`, `test_base58check_decoding`, `test_try_base58check_decode_reports_invalid_characters`, `test_try_base58check_decode_reports_invalid_checksum`, `test_try_base58check_decode_reports_missing_checksum_bytes`, `test_base58check_empty_roundtrip`, `test_base58check_decoding_with_invalid_characters`, `test_base58check_decoding_with_invalid_checksum`


**`src/atipicial_crypto/crypto_lib.rs`** — 24 functions:
`sha3_512`, `blake2b_512`, `verify_with_ed25519`, `recover_secp256k1`, `sha3_512`, `blake2b_512`, `sha3_512`, `blake2b_512`, `sha3_512`, `blake2b_512`, `test_sha3_512`, `test_sha3_512_empty`, `test_sha3_512_short_string`, `test_blake2b_512`, `test_blake2b_512_empty`, `test_blake2b_512_short_string`, `test_verify_with_ed25519_invalid_lengths`, `test_verify_with_ed25519_valid_signature`, `test_verify_with_ed25519_valid_signature_with_message`, `test_verify_with_ed25519_invalid_signature`, `test_recover_secp256k1_invalid_lengths`, `test_recover_secp256k1_valid_signature`, `test_recover_secp256k1_with_invalid_recovery_id`, `test_crypto_lib_hashable_trait`


**`src/atipicial_crypto/hash.rs`** — 37 functions:
`hash256`, `double_sha256`, `ripemd160`, `sha256_ripemd160`, `hmac_sha512`, `hash256`, `double_sha256`, `ripemd160`, `sha256_ripemd160`, `hmac_sha512`, `hash256`, `double_sha256`, `ripemd160`, `sha256_ripemd160`, `hmac_sha512`, `hex_encode`, `hash256`, `ripemd160`, `sha256_ripemd160`, `hmac_sha512`, `hash160`, `hash256`, `ripemd160`, `sha256_ripemd160`, `hmac_sha512`, `hash160`, `test_hash256_for_bytes`, `test_hash256_for_string`, `test_ripemd160_for_bytes`, `test_ripemd160_for_string`, `test_sha256_ripemd160_for_bytes`, `test_sha256_ripemd160_for_string`, `test_hmac_sha512_for_bytes`, `test_hmac_sha512_for_string`, `test_hash160_for_string`, `test_ripemd160_test_vectors`, `to_hex_string`


**`src/atipicial_crypto/key_pair.rs`** — 27 functions:
`fmt`, `missing_private_key_error`, `new`, `private_key`, `private_key_ref`, `public_key`, `public_key_ref`, `has_private_key`, `from_secret_key`, `private_key_bytes`, `public_key_bytes`, `sign`, `verify`, `new_random`, `from_private_key`, `from_wif`, `from_public_key`, `export_as_wif`, `get_script_hash`, `get_address`, `eq`, `zeroize`, `test_public_key_wif`, `test_public_key_only_wif_export_fails`, `test_address`, `test_script_hash`, `test_public_key_bytes_uncompressed_xy`


**`src/atipicial_crypto/keys.rs`** — 67 functions:
`fmt`, `zeroize`, `drop`, `fmt`, `new`, `from_public_key`, `from_bytes`, `verify`, `get_encoded`, `get_encoded_point`, `get_encoded_compressed_hex`, `from_encoded`, `get_size`, `new_random`, `random`, `from_bytes`, `to_raw_bytes`, `to_public_key`, `erase`, `sign_tx`, `sign_prehash`, `from_scalars`, `from_u256`, `from_bytes`, `to_bytes`, `fmt`, `fmt`, `fmt`, `serialize`, `serialize`, `serialize`, `deserialize`, `deserialize`, `deserialize`, `eq`, `partial_cmp`, `cmp`, `hash`, `hash`, `hash`, `eq`, `eq`, `try_from`, `try_from`, `to_vec`, `from_slice`, `to_vec`, `from_slice`, `to_vec`, `from_slice`, `to_vec`, `from_slice`, `size`, `encode`, `decode`, `to_array`, `test_new_public_key_from_point`, `test_new_public_key_from_uncompressed_point`, `test_public_key_from_slice_formats`, `test_new_public_key_from_string_with_invalid_size`, `test_new_public_key_from_point_with_hex_prefix`, `test_serialize_public_key`, `test_deserialize_public_key`, `test_public_key_size`, `test_private_key_should_be_zeroized_after_erasing`, `test_public_key_comparable`, `test_sign_message`


**`src/atipicial_crypto/proptest_tests.rs`** — 10 functions:
`prop_base58check_roundtrip`, `prop_sha256_consistent`, `prop_sha256_output_size`, `prop_ripemd160_output_size`, `prop_hash160_output_size`, `prop_double_sha256_output_size`, `prop_sha256_collision_resistance`, `prop_keypair_deterministic`, `prop_sign_verify_roundtrip`, `prop_sign_verify_wrong_message_fails`


**`src/atipicial_crypto/utils.rs`** — 19 functions:
`private_key_to_public_key`, `private_key_to_hex_string`, `private_key_from_hex`, `public_key_to_hex_string`, `public_key_from_hex`, `to_array32`, `to_array32`, `to_hex_string`, `to_hex_string`, `to_hex_string`, `to_hex_string`, `from_hex_string`, `from_hex_string`, `from_hex_string`, `from_base64_string`, `from_base64_string`, `from_base64_string`, `to_base64_string`, `to_base64_string`


**`src/atipicial_crypto/wif.rs`** — 12 functions:
`private_key_from_wif`, `wif_from_private_key`, `test_valid_wif_to_private_key`, `test_invalid_wif_sizes`, `test_invalid_wif_bytes`, `test_valid_private_key_to_wif`, `test_invalid_private_key_length`, `test_wif_empty_string`, `test_wif_invalid_base58_characters`, `test_wif_corrupted_checksum`, `test_wif_roundtrip`, `test_wif_different_keys_produce_different_wifs`


**`src/atipicial_error/mod.rs`** — 6 functions:
`from`, `from`, `with_context`, `with_context`, `test_error_context`, `test_error_macros`


**`src/atipicial_error/unified.rs`** — 56 functions:
`new`, `suggest`, `retryable`, `retry_after`, `doc`, `fmt`, `code`, `message`, `code`, `message`, `provider`, `kind`, `recovery`, `is_retryable`, `retry_after`, `message`, `from`, `network`, `transaction`, `contract`, `wallet`, `network`, `wallet`, `contract`, `source`, `with_contract`, `with_method`, `suggest`, `retryable`, `build`, `context`, `recover`, `context`, `recover`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `test_error_builder`, `test_error_display`, `kind_returns_stable_variant_classification`, `provider_conversion_preserves_retry_and_domain_classification`, `provider_debug_redacts_json_rpc_data_through_unified_error`, `codec_errors_convert_into_the_unified_boundary`, `retry_after_round_trips_for_rate_limit`, `convenience_constructors_attach_recovery_hints`, `provide_error_metadata_exposes_code_and_message`


**`src/atipicial_fs/acl.rs`** — 4 functions:
`new`, `add_record`, `fmt`, `fmt`


**`src/atipicial_fs/client.rs`** — 30 functions:
`default`, `parse_container_ids_response`, `parse_object_ids_response`, `new`, `with_account`, `get_owner_id`, `create_auth_headers`, `make_request`, `init_multipart_upload`, `upload_part`, `complete_multipart_upload`, `abort_multipart_upload`, `create_container`, `get_container`, `list_containers`, `delete_container`, `put_object`, `get_object`, `list_objects`, `delete_object`, `create_bearer_token`, `get_session_token`, `initiate_multipart_upload`, `upload_part`, `complete_multipart_upload`, `abort_multipart_upload`, `parse_container_ids_response_rejects_missing_containers_field`, `parse_container_ids_response_rejects_missing_container_id`, `parse_object_ids_response_rejects_missing_objects_field`, `parse_object_ids_response_rejects_missing_object_id`


**`src/atipicial_fs/container.rs`** — 11 functions:
`default`, `new`, `with_basic_acl`, `with_name`, `with_creation`, `with_version`, `with_attribute`, `full_access`, `read_only`, `to_bitmask`, `from_bitmask`


**`src/atipicial_fs/mod.rs`** — 22 functions:
`builder`, `endpoint`, `auth`, `timeout_sec`, `insecure`, `build`, `default`, `fmt`, `create_container`, `get_container`, `list_containers`, `delete_container`, `put_object`, `get_object`, `list_objects`, `delete_object`, `create_bearer_token`, `get_session_token`, `initiate_multipart_upload`, `upload_part`, `complete_multipart_upload`, `abort_multipart_upload`


**`src/atipicial_fs/object.rs`** — 8 functions:
`new`, `with_payload`, `with_type`, `with_attribute`, `with_filename`, `with_content_type`, `size`, `new`


**`src/atipicial_fs/types.rs`** — 4 functions:
`default`, `new`, `add`, `get`


**`src/atipicial_protocol/account.rs`** — 85 functions:
`key_pair`, `address_or_scripthash`, `label`, `verification_script`, `is_locked`, `encrypted_private_key`, `signing_threshold`, `nr_of_participants`, `set_key_pair`, `set_address_or_scripthash`, `set_label`, `set_verification_script`, `set_locked`, `set_encrypted_private_key`, `set_signing_threshold`, `set_nr_of_participants`, `new`, `from_key_pair`, `from_key_pair_opt`, `from_wif`, `decrypt_private_key`, `encrypt_private_key`, `get_script_hash`, `get_signing_threshold`, `get_nr_of_participants`, `from_verification_script`, `from_public_key`, `set_wallet`, `get_wallet`, `multi_sig_from_public_keys`, `multi_sig_from_addr`, `from_address`, `from_script_hash`, `create`, `is_multi_sig`, `fmt`, `get_address`, `get_script_hash`, `get_verification_script`, `get_public_key`, `from`, `from`, `eq`, `hash`, `drop`, `key_pair`, `address_or_scripthash`, `label`, `verification_script`, `is_locked`, `encrypted_private_key`, `signing_threshold`, `nr_of_participants`, `set_key_pair`, `set_address_or_scripthash`, `set_label`, `set_verification_script`, `set_locked`, `set_encrypted_private_key`, `set_signing_threshold`, `set_nr_of_participants`, `new`, `from_key_pair`, `from_key_pair_opt`, `from_wif`, `decrypt_private_key`, `encrypt_private_key`, `get_script_hash`, `get_signing_threshold`, `get_nr_of_participants`, `from_verification_script`, `from_public_key`, `set_wallet`, `get_wallet`, `multi_sig_from_public_keys`, `multi_sig_from_addr`, `from_address`, `from_script_hash`, `create`, `is_multi_sig`, `sign_prehash`, `decrypt_private_key_with_params`, `encrypt_private_key_with_params`, `to_aep6_account`, `get_aep17_balances`


**`src/atipicial_protocol/aep2.rs`** — 34 functions:
`encrypt`, `encrypt_with_params`, `decrypt`, `decrypt_with_params`, `get_default_scrypt_params`, `get_test_scrypt_params`, `get_test_vector_scrypt_params`, `encrypt_for_test_vector`, `encrypt_aes256_ecb`, `decrypt_aes256_ecb`, `address_hash_from_pubkey`, `decrypt_for_test_vector`, `encrypt_test_vector`, `decrypt_test_vector`, `encrypt_async`, `encrypt_with_params_async`, `decrypt_async`, `decrypt_with_params_async`, `get_aep2_from_private_key`, `get_private_key_from_aep2`, `test_async_encrypt_decrypt_roundtrip_matches_sync`, `test_decrypt_with_default_scrypt_params`, `test_encrypt_with_default_scrypt_params`, `test_encrypt_decrypt_with_custom_params`, `test_wrong_password`, `test_encrypt_decrypt_aes256_ecb`, `test_aep2_specification_test_vector`, `test_aep2_empty_password`, `test_aep2_unicode_password`, `test_aep2_invalid_format`, `test_address_hash_from_pubkey_rejects_invalid_pubkey`, `test_aep2_corrupted_data`, `test_aep2_aes_key_length_validation`, `test_aep2_different_passwords_produce_different_encrypted_keys`


**`src/atipicial_protocol/role.rs`** — 1 functions:
`byte_repr`


**`src/atipicial_protocol/responses/atipicial_account_state.rs`** — 3 functions:
`hash`, `with_no_vote`, `with_no_balance`


**`src/atipicial_protocol/responses/atipicial_application_log.rs`** — 8 functions:
`default`, `get_first_execution`, `get_execution`, `get_first_stack_item`, `get_stack_item`, `get_first_notification`, `get_notification`, `default`


**`src/atipicial_protocol/responses/atipicial_balances.rs`** — 3 functions:
`new`, `new`, `new`


**`src/atipicial_protocol/responses/atipicial_block.rs`** — 3 functions:
`get_nonce_as_u64`, `hash`, `default_transactions`


**`src/atipicial_protocol/responses/atipicial_get_next_block_validators.rs`** — 1 functions:
`new`


**`src/atipicial_protocol/responses/atipicial_get_peers.rs`** — 1 functions:
`new`


**`src/atipicial_protocol/responses/atipicial_get_token_balances.rs`** — 3 functions:
`address`, `balances`, `asset_hash`


**`src/atipicial_protocol/responses/atipicial_get_token_transfers.rs`** — 10 functions:
`sent`, `received`, `transfer_address`, `timestamp`, `asset_hash`, `transfer_address`, `amount`, `block_index`, `transfer_notify_index`, `tx_hash`


**`src/atipicial_protocol/responses/atipicial_get_unclaimed_gas.rs`** — 2 functions:
`add`, `add_assign`


**`src/atipicial_protocol/responses/atipicial_get_unspents.rs`** — 4 functions:
`eq`, `hash`, `eq`, `hash`


**`src/atipicial_protocol/responses/atipicial_get_version.rs`** — 19 functions:
`default`, `default_tcp_port`, `default_ws_port`, `default_nonce`, `default_user_agent`, `default_protocol`, `eq`, `default_max_iterator_result_items`, `default_session_enabled`, `default`, `default_network`, `default_validators_count`, `default_ms_per_block`, `default_max_valid_until_block_increment`, `default_max_traceable_blocks`, `default_address_version`, `default_max_transactions_per_block`, `default_memory_pool_max_transactions`, `default_initial_gas_distribution`


**`src/atipicial_protocol/responses/atipicial_send_raw_transaction.rs`** — 1 functions:
`new`


**`src/atipicial_protocol/responses/atipicial_submit_block.rs`** — 1 functions:
`get_submit_block`


**`src/atipicial_protocol/responses/atipicial_transfers.rs`** — 4 functions:
`new`, `new`, `deserialize_amount`, `serialize_amount`


**`src/atipicial_protocol/responses/atipicial_validate_address.rs`** — 1 functions:
`new`


**`src/atipicial_protocol/responses/atipicial_witness.rs`** — 2 functions:
`new`, `from_witness`


**`src/atipicial_protocol/responses/diagnostics.rs`** — 1 functions:
`new`


**`src/atipicial_protocol/responses/express_contract_state.rs`** — 1 functions:
`new`


**`src/atipicial_protocol/responses/express_shutdown.rs`** — 1 functions:
`new`


**`src/atipicial_protocol/responses/notification.rs`** — 1 functions:
`new`


**`src/atipicial_protocol/responses/oracle_request.rs`** — 1 functions:
`new`


**`src/atipicial_protocol/responses/populated_blocks.rs`** — 1 functions:
`new`


**`src/atipicial_protocol/responses/response_transaction.rs`** — 6 functions:
`new`, `get_first_signer`, `get_signer`, `get_first_attribute`, `get_attribute`, `eq`


**`src/atipicial_protocol/responses/response_transaction_attribute.rs`** — 1 functions:
`deserialize_height`


**`src/atipicial_protocol/responses/response_transaction_signer.rs`** — 23 functions:
`hash`, `new`, `new_full`, `get_first_scope`, `get_scope`, `get_first_allowed_contract`, `get_allowed_contract`, `get_first_allowed_group`, `get_allowed_group`, `get_first_rule`, `get_rule`, `get_type`, `get_signer_hash`, `set_signer_hash`, `get_scopes`, `get_scopes_mut`, `set_scopes`, `get_allowed_contracts`, `get_allowed_contracts_mut`, `get_allowed_groups`, `get_allowed_groups_mut`, `get_rules`, `get_rules_mut`


**`src/atipicial_sgx/allocator.rs`** — 9 functions:
`memory_usage`, `available_memory`, `init_allocator`, `panic`, `oom`, `new`, `memory_usage`, `available_memory`, `init_allocator`


**`src/atipicial_sgx/attestation.rs`** — 19 functions:
`new`, `configure_spid`, `init_attestation`, `init_attestation`, `generate_quote`, `generate_quote`, `get_quote`, `close`, `close`, `new`, `verify_with_ias`, `configure_ias`, `verify_quote`, `extract_mrenclave`, `extract_mrsigner`, `sgx_ra_init`, `sgx_ra_close`, `sgx_create_report`, `sgx_get_quote`


**`src/atipicial_sgx/crypto.rs`** — 13 functions:
`compute_shared_secret`, `new`, `sha256`, `sign_ecdsa`, `verify_ecdsa`, `random_bytes`, `generate_keypair`, `new`, `seal_key`, `unseal_key`, `seal_key`, `unseal_key`, `init_crypto`


**`src/atipicial_sgx/enclave.rs`** — 25 functions:
`default`, `default`, `new`, `initialize`, `initialize`, `is_initialized`, `config`, `ecall`, `ecall`, `destroy`, `destroy`, `serialize_for_enclave`, `expected_size`, `deserialize_from_enclave`, `serialize_for_enclave`, `expected_size`, `deserialize_from_enclave`, `serialize_for_enclave`, `expected_size`, `deserialize_from_enclave`, `generate_enclave_config`, `sgx_create_enclave`, `sgx_destroy_enclave`, `sgx_ecall`, `generate_edl`


**`src/atipicial_sgx/mod.rs`** — 2 functions:
`init_sgx`, `fmt`


**`src/atipicial_sgx/networking.rs`** — 13 functions:
`new`, `establish_channel`, `establish_channel`, `send_secure`, `receive_secure`, `encrypt_message`, `decrypt_message`, `clone`, `complete_handshake`, `is_established`, `ocall_network_request`, `ocall_atipicial_rpc_request`, `ocall_network_request`


**`src/atipicial_sgx/storage.rs`** — 19 functions:
`default`, `new`, `store`, `store`, `retrieve`, `retrieve`, `delete`, `delete`, `list_keys`, `list_keys`, `persist_to_disk`, `persist_to_disk`, `delete_from_disk`, `delete_from_disk`, `get_timestamp`, `get_timestamp`, `ocall_secure_save`, `ocall_secure_load`, `sgx_create_monotonic_counter`


**`src/atipicial_types/address.rs`** — 15 functions:
`address_to_script_hash`, `script_to_script_hash`, `hex_to_script_hash`, `random`, `address_to_script_hash`, `script_to_script_hash`, `hex_to_script_hash`, `random`, `address_to_script_hash`, `script_to_script_hash`, `hex_to_script_hash`, `random`, `test_address_to_script_hash`, `test_from_script_hash`, `from_script_hash`


**`src/atipicial_types/address_or_scripthash.rs`** — 16 functions:
`hash`, `default`, `from`, `from`, `try_from`, `deserialize`, `try_from_script_hash_bytes`, `address`, `try_script_hash`, `script_hash`, `test_try_from_bytes_rejects_invalid_length`, `test_try_script_hash_rejects_invalid_address`, `test_from_address_panics_on_invalid_address`, `test_from_bytes_panics_on_invalid_length_instead_of_zero_hash`, `test_deserialize_rejects_invalid_address_variant`, `test_deserialize_accepts_valid_address_variant`


**`src/atipicial_types/block.rs`** — 3 functions:
`hash`, `hash`, `eq`


**`src/atipicial_types/bytes.rs`** — 12 functions:
`b_int`, `base64_encoded`, `base58_encoded`, `base58_check_encoded`, `no_prefix_hex`, `var_size`, `scripthash_to_address`, `to_padded`, `trim_trailing_bytes`, `bitxor`, `reverse`, `reverse`


**`src/atipicial_types/hardfork.rs`** — 12 functions:
`all`, `name`, `description`, `fmt`, `from_str`, `try_from`, `from`, `test_hardfork_ordering`, `test_hardfork_from_str`, `test_hardfork_display`, `test_hardfork_try_from_u8`, `test_hardfork_all`


**`src/atipicial_types/mod.rs`** — 17 functions:
`to_base64`, `to_base64`, `serialize`, `default`, `to_base64`, `try_to_base64`, `try_to_base64`, `try_to_base64`, `to_base64`, `to_base64`, `to_base64`, `to_checksum`, `test_base64_encode_bytes`, `test_base64_decode`, `test_try_string_to_base64`, `test_try_string_to_base64_invalid_hex`, `test_string_to_base64_panics_on_invalid_hex`


**`src/atipicial_types/numeric.rs`** — 19 functions:
`to_bytes_padded`, `to_bytes_padded`, `power_of`, `var_size`, `to_unsigned`, `to_bytes`, `to_bytes`, `to_bytes`, `to_bytes`, `to_bytes`, `to_milliseconds`, `test_to_bytes_padded`, `test_power`, `test_var_size`, `test_to_unsigned`, `test_i32_to_bytes`, `test_i64_to_bytes`, `test_f32_to_bytes`, `test_datetime_to_ms`


**`src/atipicial_types/op_code.rs`** — 6 functions:
`price`, `opcode`, `to_hex_string`, `operand_size`, `with_size`, `with_prefix_size`


**`src/atipicial_types/path_or_string.rs`** — 3 functions:
`from`, `from`, `read`


**`src/atipicial_types/plugin_type.rs`** — 1 functions:
`value_of_name`


**`src/atipicial_types/proptest_tests.rs`** — 13 functions:
`prop_scripthash_address_roundtrip`, `prop_h160_conversions`, `prop_h256_conversions`, `prop_contract_parameter_integer`, `prop_contract_parameter_boolean`, `prop_contract_parameter_string`, `prop_stackitem_integer`, `prop_stackitem_boolean`, `prop_stackitem_bytestring`, `prop_reverse_hex_roundtrip`, `prop_variable_length_encoding`, `prop_address_validation_consistency`, `prop_bigint_conversions`


**`src/atipicial_types/script_hash.rs`** — 58 functions:
`to_bs58_string`, `zero`, `from_slice`, `from_hex`, `from_address`, `to_address`, `to_hex`, `to_hex_big_endian`, `to_vec`, `to_le_vec`, `from_script`, `from_public_key`, `to_bs58_string`, `zero`, `from_slice`, `from_hex`, `from_address`, `to_address`, `to_hex`, `to_hex_big_endian`, `to_vec`, `to_le_vec`, `from_script`, `from_public_key`, `new`, `as_str`, `to_script_hash`, `fmt`, `from`, `from`, `try_from`, `try_from`, `from`, `from`, `into_script_hash`, `into_script_hash`, `into_script_hash`, `into_script_hash`, `into_script_hash`, `into_script_hash`, `into_script_hash`, `into_script_hash`, `into_script_hash`, `into_script_hash`, `into_script_hash`, `test_address_type`, `test_into_script_hash_from_address_string`, `test_into_script_hash_from_hex`, `test_address_to_script_hash_roundtrip`, `test_from_valid_hash`, `test_creation_failures`, `test_to_array`, `test_serialize_and_deserialize`, `test_equals`, `test_from_address`, `test_from_invalid_address`, `test_from_public_key_bytes`, `test_to_address`


**`src/atipicial_types/serde_value.rs`** — 12 functions:
`to_value`, `to_value`, `to_value`, `to_value`, `to_value`, `to_value`, `to_value`, `to_value`, `to_value`, `to_value`, `to_value`, `to_value`


**`src/atipicial_types/serde_with_utils.rs`** — 82 functions:
`serialize_h160_without_0x`, `serialize_h160`, `deserialize_h160`, `serialize_scopes`, `deserialize_scopes`, `serialize_boolean_expression`, `deserialize_boolean_expression`, `serialize_bytes`, `deserialize_bytes`, `serialize_url`, `deserialize_pubkey`, `serialize_pubkey`, `deserialize_url`, `serialize_url_option`, `deserialize_url_option`, `serialize_u256`, `deserialize_u256`, `serialize_u256_option`, `deserialize_u256_option`, `serialize_u32`, `deserialize_u32`, `serialize_u64`, `deserialize_u64`, `serialize_u64_option`, `deserialize_u64_option`, `deserialize_script_hash`, `serialize_script_hash`, `deserialize_address_or_script_hash`, `serialize_address_or_script_hash`, `deserialize_vec_script_hash`, `serialize_vec_script_hash`, `deserialize_vec_script_hash_option`, `serialize_vec_script_hash_option`, `serialize_script_hash_option`, `deserialize_script_hash_option`, `serialize_hash_map_h160_account`, `deserialize_hash_map_h160_account`, `deserialize_private_key`, `serialize_private_key`, `deserialize_public_key`, `serialize_public_key`, `deserialize_vec_public_key`, `deserialize_vec_public_key_option`, `serialize_vec_public_key`, `serialize_vec_public_key_option`, `serialize_public_key_option`, `deserialize_public_key_option`, `serialize_h256`, `deserialize_h256`, `serialize_hashset_u256`, `deserialize_hashset_u256`, `serialize_vec_h256`, `deserialize_vec_h256`, `serialize_vec_u256`, `deserialize_vec_u256`, `serialize_h256_option`, `deserialize_h256_option`, `serialize_hashmap_u256_hashset_u256`, `deserialize_hashmap_u256_hashset_u256`, `serialize_hashmap_address_u256`, `deserialize_hashmap_address_u256`, `serialize_hashmap_u256_hashset_h256`, `deserialize_hashmap_u256_hashset_h256`, `serialize_hashmap_u256_vec_u256`, `deserialize_hashmap_u256_vec_u256`, `serialize_map`, `deserialize_map`, `serialize_wildcard`, `deserialize_wildcard`, `expecting`, `visit_str`, `visit_seq`, `deserialize_hardforks`, `expecting`, `visit_seq`, `visit_map`, `test_serialize_hashset_u256`, `test_serialize_hashmap_u256_hashset_u256`, `test_u64_hex_and_option_roundtrip`, `test_serialize_bytes`, `test_serialize_u32`, `test_serialize_vec_h256`


**`src/atipicial_types/stack_item.rs`** — 36 functions:
`deserialize_integer_from_string`, `expecting`, `visit_str`, `visit_string`, `visit_i64`, `visit_u64`, `new_byte_string`, `as_bool`, `as_string`, `to_string`, `as_bytes`, `as_array`, `as_array_ref`, `as_int`, `as_map`, `as_map_entries`, `as_address`, `as_public_key`, `as_hash160`, `as_hash256`, `as_interop`, `len`, `is_empty`, `get`, `get_iterator_id`, `get_interface_name`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`


**`src/atipicial_types/string.rs`** — 41 functions:
`try_base58_decoded`, `try_base58_check_decoded`, `try_address_to_scripthash`, `bytes_from_hex`, `base64_decoded`, `base64_encoded`, `base58_decoded`, `base58_check_decoded`, `base58_encoded`, `var_size`, `is_valid_address`, `is_valid_hex`, `address_to_scripthash`, `try_reversed_hex`, `reversed_hex`, `try_base58_decoded`, `try_base58_check_decoded`, `try_address_to_scripthash`, `try_base58_decoded`, `try_base58_check_decoded`, `try_address_to_scripthash`, `bytes_from_hex`, `base64_decoded`, `base64_encoded`, `base58_decoded`, `base58_check_decoded`, `base58_encoded`, `var_size`, `is_valid_address`, `is_valid_hex`, `address_to_scripthash`, `try_reversed_hex`, `reversed_hex`, `test_base58_check_decoded_validates_checksum`, `test_base58_check_decoded_rejects_invalid_checksum`, `test_try_base58_decoded_rejects_invalid_input`, `test_try_base58_check_decoded_rejects_invalid_checksum`, `test_try_address_to_scripthash_returns_known_hash`, `test_try_reversed_hex_rejects_invalid_hex`, `test_try_reversed_hex_reverses_valid_hex`, `test_reversed_hex_panics_on_invalid_hex`


**`src/atipicial_types/syncing.rs`** — 5 functions:
`serialize`, `deserialize`, `deserialize_sync_geth`, `deserialize_sync_minimal`, `deserialize_sync_false`


**`src/atipicial_types/tx_pool.rs`** — 5 functions:
`expecting`, `visit_string`, `visit_str`, `deserialize`, `serialize`


**`src/atipicial_types/url_session.rs`** — 1 functions:
`data`


**`src/atipicial_types/util.rs`** — 22 functions:
`parse_string_u64`, `parse_string_u256`, `parse_address`, `encode_string_h160`, `parse_string_h160`, `parse_string_h256`, `encode_string_h256`, `encode_string_u256`, `encode_vec_string_vec_u256`, `parse_vec_string_vec_u256`, `h256_to_u256`, `bytes_to_string`, `string_to_bytes`, `u256_sqrt`, `u256_min`, `vec_to_array32`, `var_size`, `to_base58`, `to_base58`, `to_base64`, `to_base64`, `test_bytes_to_string`


**`src/atipicial_types/whitelisted_contract.rs`** — 5 functions:
`new`, `from_stack_item`, `fmt`, `test_whitelisted_contract_new`, `test_whitelisted_contract_display`


**`src/atipicial_types/contract/aef_file.rs`** — 39 functions:
`from`, `new`, `get_checksum_as_integer`, `compute_checksum`, `compute_checksum_from_bytes`, `encode_without_checksum`, `compute_checksum_for_payload`, `encode_legacy`, `encode_best_effort`, `read_from_file`, `deserialize`, `read_from_stack_item`, `try_encode`, `try_to_array`, `size`, `encode`, `decode`, `to_array`, `try_encode`, `try_to_array`, `size`, `encode`, `decode`, `to_array`, `create_valid_serialized_aef_bytes`, `create_encodable_aef`, `test_deserialize_preserves_checksum_and_roundtrips`, `test_to_array_repairs_checksum_for_encodable_aef`, `test_try_to_array_rejects_invalid_checksum_length`, `test_try_to_array_rejects_mismatched_checksum`, `test_method_token_try_to_array_rejects_method_name_longer_than_max`, `test_try_to_array_rejects_method_token_with_long_method_name`, `test_aef_size_matches_serialized_length`, `test_method_token_size_matches_serialized_length`, `test_try_to_array_rejects_source_url_longer_than_max`, `test_try_to_array_rejects_empty_script`, `test_try_to_array_rejects_script_longer_than_max`, `test_try_to_array_rejects_compiler_longer_than_fixed_width`, `test_try_to_array_matches_legacy_for_encodable_aef`


**`src/atipicial_types/contract/aep17contract.rs`** — 1 functions:
`new`


**`src/atipicial_types/contract/contract_aef.rs`** — 3 functions:
`new`, `get_first_token`, `get_token`


**`src/atipicial_types/contract/contract_manifest.rs`** — 16 functions:
`new`, `get_supported_standard`, `get_first_supported_standard`, `get_permission`, `get_first_permission`, `get_first_trust`, `get_trust`, `eq`, `hash`, `new`, `get_first_method`, `get_method`, `get_first_event`, `get_event`, `new`, `new`


**`src/atipicial_types/contract/contract_method_token.rs`** — 1 functions:
`new`


**`src/atipicial_types/contract/contract_parameter.rs`** — 100 functions:
`new`, `deserialize`, `expecting`, `visit_map`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `from`, `try_from_aef_file`, `from_json`, `as_public_key`, `as_signature`, `as_bytes`, `as_string`, `from`, `from`, `to_value`, `to_value`, `hash`, `new`, `get_type`, `with_value`, `bool`, `to_bool`, `integer`, `to_integer`, `byte_array`, `to_byte_array`, `string`, `to_string`, `h160`, `to_h160`, `h256`, `to_h256`, `public_key`, `to_public_key`, `signature`, `to_signature`, `array`, `to_array`, `map`, `to_map`, `any`, `hash`, `new`, `from_map`, `to_map`, `to_bool`, `to_integer`, `to_byte_array`, `to_string`, `to_h160`, `to_h256`, `to_public_key`, `to_signature`, `to_array`, `to_map`, `hash`, `test_try_from_aef_file_ref_rejects_invalid_aef`, `test_from_aef_file_ref_repairs_checksum_via_legacy_wrapper`, `test_try_from_aef_file_owned_matches_legacy_for_valid_aef`, `test_string_from_string`, `test_bytes_from_bytes`, `test_bytes_from_hex_string`, `test_array_from_array`, `test_array_from_empty`, `test_nested_array`, `test_map`, `test_nested_map`, `test_serialize_deserialize`, `test_bytes_equals`, `test_bytes_from_string`, `test_bool`, `test_int`, `test_h160`, `test_h256`, `test_public_key`, `test_signature`, `create_from_various_types`, `create_array_from_vec`, `create_map_from_hashmap`, `contract_parameter_to_value_is_structured_json`, `equality_operator`


**`src/atipicial_types/contract/contract_parameter_type.rs`** — 2 functions:
`test_contract_parameter_type_deserialization`, `test_contract_parameter_type_serialization`


**`src/atipicial_types/contract/contract_state.rs`** — 3 functions:
`new`, `contract_identifiers`, `from_invocation_result`


**`src/atipicial_types/contract/contract_storage_entry.rs`** — 1 functions:
`new`


**`src/atipicial_types/contract/invocation_result.rs`** — 18 functions:
`default_gas_consumed`, `deserialize`, `new`, `has_state_fault`, `get_first_stack_item`, `get_stack_item`, `get_first_notification`, `get_notification`, `default`, `hash`, `hash`, `new`, `new`, `new_hash`, `new`, `deserialize_missing_state_defaults_to_none`, `deserialize_empty_state_is_none`, `non_halt_states_are_treated_as_faults`


**`src/atipicial_types/contract/native_contract_state.rs`** — 1 functions:
`new`


**`src/atipicial_types/nns/name_state.rs`** — 1 functions:
`new`


**`src/atipicial_types/nns/nns_name.rs`** — 8 functions:
`new`, `is_valid`, `validate_fragment`, `validate`, `bytes`, `is_second_level_domain`, `new`, `validate`


**`src/atipicial_types/nns/record_state.rs`** — 2 functions:
`new`, `from_stack_item`


**`src/atipicial_types/nns/record_type.rs`** — 1 functions:
`byte_repr`


**`src/atipicial_utils/error.rs`** — 6 functions:
`option_to_result`, `with_context`, `result_to_option`, `retry`, `test_retry_zero_attempts_runs_once_ok`, `test_retry_zero_attempts_runs_once_err`


**`src/atipicial_wallets/bip39_account.rs`** — 11 functions:
`fmt`, `drop`, `mnemonic`, `account`, `create`, `from_bip39_mnemonic`, `test_create_bip39_account`, `test_recover_from_mnemonic`, `test_invalid_mnemonic`, `test_different_passwords_different_accounts`, `test_generate_and_recover_bip39_account`


**`src/atipicial_wallets/ledger.rs`** — 9 functions:
`get_address`, `sign_tx`, `sign_message`, `to_vec`, `new`, `get_address`, `sign_transaction`, `sign_message`, `fmt`


**`src/atipicial_wallets/wallet_signer.rs`** — 8 functions:
`new_with_signer`, `sign_transaction`, `sign_hash`, `sign_message`, `signer`, `address`, `network`, `fmt`


**`src/atipicial_wallets/wallet_trait.rs`** — 12 functions:
`name`, `version`, `scrypt_params`, `accounts`, `default_account`, `default_account_or_err`, `set_name`, `set_version`, `set_scrypt_params`, `set_default_account`, `add_account`, `remove_account`


**`src/atipicial_wallets/yubi.rs`** — 7 functions:
`connect`, `new`, `from_key`, `from`, `from_key`, `new_key`, `test_wallet_signer_creation`


**`src/atipicial_wallets/wallet/aep6account.rs`** — 15 functions:
`fmt`, `new`, `from_account`, `to_account`, `eq`, `drop`, `test_decrypt_with_standard_scrypt_params`, `test_decrypt_encrypted_only_account_repairs_script_hash`, `test_load_account_from_aep6`, `test_load_multi_sig_account_from_aep6`, `test_to_aep6_account_with_only_an_address`, `test_to_account_rejects_missing_address_and_contract_script`, `test_to_aep6_account_with_unecrypted_private_key`, `test_to_aep6_account_with_ecrypted_private_key`, `test_to_aep6_account_with_muliti_sig_account`


**`src/atipicial_wallets/wallet/aep6contract.rs`** — 1 functions:
`eq`


**`src/atipicial_wallets/wallet/aep6wallet.rs`** — 2 functions:
`new`, `test_read_wallet`


**`src/atipicial_wallets/wallet/backup.rs`** — 3 functions:
`backup`, `recover`, `test_backup_and_recover`


**`src/atipicial_wallets/wallet/wallet.rs`** — 73 functions:
`default`, `name`, `version`, `scrypt_params`, `accounts`, `default_account`, `set_name`, `set_version`, `set_scrypt_params`, `set_default_account`, `add_account`, `remove_account`, `sync_default_account_flags`, `promote_default_account`, `effective_scrypt_params`, `new`, `try_new`, `try_new_with_account_factory`, `to_aep6`, `from_aep6`, `from_account`, `from_accounts`, `save_to_file`, `get_account`, `remove_account`, `encrypt_accounts`, `encrypt_accounts_parallel`, `encrypt_accounts_parallel_with_threads`, `encrypt_accounts_batch_parallel`, `create`, `open`, `get_accounts`, `create_account`, `import_private_key`, `verify_password`, `change_password`, `change_password_parallel`, `get_unclaimed_gas`, `get_witness`, `sign_transaction`, `address`, `create_wallet`, `open_wallet`, `get_all_accounts`, `create_new_account`, `import_from_wif`, `get_unclaimed_gas_as_float`, `network`, `with_network`, `apply_fast_scrypt`, `test_is_default`, `test_create_default_wallet`, `test_try_new_creates_single_default_account`, `test_new_panics_when_account_creation_fails`, `test_create_wallet_with_accounts`, `test_from_account_keeps_only_supplied_account`, `test_add_account_to_empty_wallet_sets_default_account`, `test_set_default_account_with_unknown_hash_leaves_no_default`, `test_is_default_account`, `test_add_account`, `test_encrypt_wallet`, `test_encrypt_wallet_parallel`, `test_encrypt_wallet_batch_parallel`, `test_change_password_parallel`, `test_change_password_rejects_empty_new_password_without_mutating_wallet`, `test_to_aep6_rejects_unencrypted_accounts_instead_of_dropping_them`, `test_save_to_file_rejects_unencrypted_wallet`, `test_from_aep6_rejects_empty_wallet`, `test_from_aep6_surfaces_invalid_account_errors`, `test_create_wallet_creates_single_encrypted_default_account`, `test_verify_password`, `test_remove_default_account_promotes_deterministic_remaining_account`, `test_remove_default_account_promotes_remaining_account`


**`src/atipicial_x/bridge/bridge_contract.rs`** — 9 functions:
`new`, `with_script_hash`, `deposit`, `withdraw`, `get_fee`, `get_cap`, `script_hash`, `set_script_hash`, `provider`


**`src/atipicial_x/bridge/evm_bridge.rs`** — 4 functions:
`new`, `default_bridge`, `address`, `get_fee`


**`src/atipicial_x/evm/provider.rs`** — 11 functions:
`new`, `new_anti_mev`, `rpc_url`, `set_rpc_url`, `evm_provider`, `chain_id`, `get_balance`, `parse_chain_id_value`, `chain_id_queries_provider`, `chain_id_requires_provider`, `parse_chain_id_value_rejects_invalid_strings`


**`src/atipicial_x/evm/transaction.rs`** — 11 functions:
`new`, `to`, `data`, `value`, `gas_limit`, `gas_price`, `into_alloy_request`, `build_alloy_request`, `from`, `test_into_alloy_request`, `preserves_values_above_u64`


**`src/atipicial_x/evm/wallet.rs`** — 8 functions:
`from_private_key`, `create_random`, `address`, `inner_wallet`, `new`, `get_balance`, `send_transaction`, `send_transaction_request`


**`src/constants/native_contracts.rs`** — 3 functions:
`is_valid_script_hash`, `test_native_contract_addresses_format`, `test_native_contract_known_hashes`


**`src/monitoring/health.rs`** — 15 functions:
`new`, `register`, `update`, `overall_status`, `get_all`, `init`, `register_default_checks`, `check_rpc_health`, `check_memory_health`, `current_memory_usage_percent`, `current_memory_usage_percent`, `check_blockchain_health`, `update_health`, `register_health_check`, `shutdown`


**`src/monitoring/metrics.rs`** — 11 functions:
`init`, `with_registry`, `increment_counter`, `set_gauge`, `observe_histogram`, `snapshot`, `record_transaction`, `record_rpc_request`, `update_blockchain_metrics`, `record_contract_invocation`, `shutdown`


**`src/monitoring/mod.rs`** — 12 functions:
`builder`, `metrics_enabled`, `metrics_port`, `tracing_enabled`, `tracing_endpoint`, `log_level`, `health_check_enabled`, `health_check_port`, `build`, `from_env`, `init`, `shutdown`


**`src/monitoring/tracing.rs`** — 5 functions:
`init`, `add_event`, `set_status`, `record_error`, `shutdown`


**`src/sdk/hd_wallet.rs`** — 41 functions:
`new_atipicial`, `from_string`, `fmt`, `fmt`, `drop`, `generate`, `from_mnemonic`, `from_phrase`, `mnemonic_phrase`, `derive_account`, `derive_accounts`, `get_default_account`, `derive_key`, `export_encrypted`, `import_encrypted`, `encrypted_wallet_error`, `derive_hd_wallet_key_material`, `aes256_ctr_crypt`, `hd_wallet_auth_tag`, `verify_hd_wallet_auth_tag`, `fmt`, `drop`, `from_seed`, `derive_child`, `fmt`, `drop`, `default`, `new`, `word_count`, `passphrase`, `language`, `mnemonic`, `build`, `test_derivation_path_parsing`, `test_hd_wallet_generation`, `test_hd_wallet_from_phrase`, `test_account_derivation`, `test_builder`, `test_export_import_encrypted_roundtrip`, `test_export_import_preserves_bip39_passphrase`, `test_import_encrypted_rejects_tampered_ciphertext`


**`src/sdk/mod.rs`** — 77 functions:
`builder`, `timeout`, `retries`, `cache_enabled`, `metrics_enabled`, `build`, `default`, `deserialize`, `fmt`, `try_from_raw`, `from_raw`, `parse`, `raw`, `decimals`, `to_fixed_string`, `raw_i64`, `fmt`, `contract_hash`, `no_default_account_error`, `invalid_address_error`, `send_tx_with_retry`, `build_and_send_transfer`, `testnet`, `mainnet`, `connect`, `from_env`, `builder`, `get_balance`, `get_balance_inner`, `fetch_aep17_decimals`, `fetch_aep17_symbol`, `transfer`, `transfer_inner`, `deploy_contract`, `deploy_contract_inner`, `invoke_read`, `invoke_write`, `invoke_write_inner`, `wait_for_confirmation`, `get_block_height`, `get_block_height_inner`, `client`, `endpoint`, `network`, `network_label`, `record_operation`, `record_transaction_metric`, `default`, `network`, `endpoint`, `config`, `timeout`, `retries`, `cache`, `metrics`, `build`, `new`, `with_memo`, `execute`, `invalid_balance_response`, `parse_balance_stack_item_u64`, `parse_aep17_decimals`, `decimal_amount_rejects_invalid_raw_values`, `decimal_amount_deserialization_validates_raw_value`, `deprecated_decimal_amount_constructor_preserves_invalid_input_fallback`, `transaction_with_broadcast_responses`, `send_retry_accepts_transactions_already_known_by_the_node`, `send_retry_classifies_node_rejection_as_transaction_error`, `test_builder_configuration`, `endpoint_shortcut_picks_custom_network`, `config_setter_replaces_entire_config`, `token_contract_hash_returns_native_hashes`, `connect_with_malformed_url_returns_network_error`, `from_env_without_env_var_falls_back_to_testnet`, `from_env_with_malformed_url_yields_network_error`, `test_parse_balance_stack_item_u64_rejects_negative_value`, `test_parse_aep17_decimals_rejects_invalid_value`


**`src/sdk/retry.rs`** — 7 functions:
`backoff_delay`, `retry_network`, `backoff_grows_exponentially_and_stays_bounded`, `returns_first_success_without_extra_attempts`, `maps_final_failure_with_context`, `stops_after_a_deterministic_provider_error`, `honors_provider_retry_after_hint`


**`src/sdk/transaction_simulator.rs`** — 39 functions:
`new`, `simulate_transaction`, `simulate_script`, `estimate_gas`, `preview_state_changes`, `perform_simulation`, `parse_invocation_result`, `parse_gas_consumed`, `parse_notifications`, `analyze_state_changes`, `parse_transfer_notification`, `stack_item_to_address`, `get_token_symbol`, `extract_token_symbol`, `calculate_system_fee`, `calculate_network_fee`, `apply_optimization_rules`, `analyze_for_warnings`, `calculate_cache_key`, `default_optimization_rules`, `check`, `is_valid`, `default`, `new`, `client`, `cache_duration`, `add_optimization_rule`, `build`, `test_gas_estimate_creation`, `test_simulation_result`, `test_warning_levels`, `test_parse_invocation_result_rejects_invalid_gas_consumed`, `test_extract_token_symbol_reads_first_stack_item`, `test_extract_token_symbol_rejects_non_string_items`, `test_gas_consumed_decimal_string_converts_to_base_units`, `test_gas_consumed_rejects_overly_precise_decimals`, `test_cache_key_covers_signers`, `test_transfer_notification_parses_real_state`, `test_transfer_notification_rejects_non_transfer_shapes`


**`src/sdk/unified.rs`** — 11 functions:
`amount_validation_error`, `parse_amount_base_units`, `new_n3`, `new_atipicialx`, `new_atipicialx_anti_mev`, `get_balance`, `transfer`, `bridge_to_other_chain`, `parse_amount_uses_chain_specific_scale`, `parse_amount_rejects_invalid_and_over_precise_input`, `atipicialx_balance_formats_as_human_decimal`


**`src/sdk/websocket.rs`** — 35 functions:
`limited_websocket_config`, `id`, `subscription_type`, `cancel`, `new`, `is_connected`, `connect`, `disconnect`, `subscribe`, `unsubscribe`, `take_event_receiver`, `set_reconnect_params`, `start_event_loop`, `process_text_message`, `parse_event`, `send_message`, `generate_subscription_id`, `create_subscription_request`, `create_subscription_request_static`, `create_unsubscribe_request`, `create_unsubscribe_request_static`, `new`, `reconnect_interval`, `max_reconnect_attempts`, `build`, `test_websocket_client_creation`, `test_websocket_builder`, `test_subscription_id_generation`, `subscribe_receives_event`, `unsubscribe_sends_request`, `cancel_sends_request`, `dropping_handle_does_not_cancel`, `reconnects_on_close_and_receives_event`, `parse_event_rejects_block_event_missing_required_fields`, `parse_event_rejects_notification_missing_state`


---

## 📚 Documentation Index

**1. [`docs/API_DOCUMENTATION.md`](docs/API_DOCUMENTATION.md)** — Api Documentation
> AtipicialRust API Documentation (v2.0.0)

**2. [`docs/GAS_ESTIMATION_GUIDE.md`](docs/GAS_ESTIMATION_GUIDE.md)** — Gas Estimation Guide
> Gas Estimation Guide - AtipicialRust v2.0.0

**3. [`docs/PRODUCTION_DEPLOYMENT_GUIDE.md`](docs/PRODUCTION_DEPLOYMENT_GUIDE.md)** — Production Deployment Guide
> 🚀 AtipicialRust SDK Production Deployment Guide

**4. [`docs/RATE_LIMITING_GUIDE.md`](docs/RATE_LIMITING_GUIDE.md)** — Rate Limiting Guide
> Rate Limiting Guide - AtipicialRust v2.0.0

**5. [`docs/README.md`](docs/README.md)** — Readme
> AtipicialRust v2.1.0 - Complete Atipicial Development Suite

**6. [`docs/RELEASE_PROCESS.md`](docs/RELEASE_PROCESS.md)** — Release Process
> AtipicialRust Automated Release Process

**7. [`docs/SECURITY_AUDIT.md`](docs/SECURITY_AUDIT.md)** — Security Audit
> 🔒 AtipicialRust SDK Security Audit

**8. [`docs/SGX_GUIDE.md`](docs/SGX_GUIDE.md)** — Sgx Guide
> SGX Support Guide - AtipicialRust (experimental)

**9. [`docs/atipicial-x/README.md`](docs/atipicial-x/README.md)** — Readme
> Atipicial X

**10. [`docs/atipicial-x/bridge.md`](docs/atipicial-x/bridge.md)** — Bridge
> Atipicial X Bridge

**11. [`docs/atipicial-x/evm-contracts.md`](docs/atipicial-x/evm-contracts.md)** — Evm Contracts
> EVM Contracts on Atipicial X

**12. [`docs/contracts/README.md`](docs/contracts/README.md)** — Readme
> Atipicial Smart Contracts

**13. [`docs/contracts/contract-deployment.md`](docs/contracts/contract-deployment.md)** — Contract Deployment
> Smart Contract Deployment

**14. [`docs/contracts/contract-invocation.md`](docs/contracts/contract-invocation.md)** — Contract Invocation
> Smart Contract Invocation

**15. [`docs/contracts/system-contracts.md`](docs/contracts/system-contracts.md)** — System Contracts
> Atipicial System Contracts

**16. [`docs/contracts/token-standards.md`](docs/contracts/token-standards.md)** — Token Standards
> Atipicial Token Standards

**17. [`docs/crypto/AEP2.md`](docs/crypto/AEP2.md)** — Aep2
> AEP2 Implementation

**18. [`docs/crypto/README.md`](docs/crypto/README.md)** — Readme
> Atipicial Cryptography

**19. [`docs/guides/README.md`](docs/guides/README.md)** — Readme
> Guides

**20. [`docs/guides/build-configuration.md`](docs/guides/build-configuration.md)** — Build Configuration
> Build Configuration Guide

**21. [`docs/guides/choosing-an-api.md`](docs/guides/choosing-an-api.md)** — Choosing An Api
> Choosing the Right API Layer

**22. [`docs/guides/hd-wallet.md`](docs/guides/hd-wallet.md)** — Hd Wallet
> HD Wallet Guide (v2.0.0)

**23. [`docs/guides/installation.md`](docs/guides/installation.md)** — Installation
> Installation Guide

**24. [`docs/guides/migration-v1.0.md`](docs/guides/migration-v1.0.md)** — Migration V1.0
> Migration Guide: Pre-1.0 → v2.0.0

**25. [`docs/guides/production-implementations.md`](docs/guides/production-implementations.md)** — Production Implementations
> Production-Ready Implementations Guide

**26. [`docs/guides/transaction-simulation.md`](docs/guides/transaction-simulation.md)** — Transaction Simulation
> Transaction Simulation Guide (v2.0.0)

**27. [`docs/guides/websocket.md`](docs/guides/websocket.md)** — Websocket
> AtipicialRust WebSocket Guide (v2.0.0)

**28. [`docs/plans/2026-03-06-sdk-safety-refactor.md`](docs/plans/2026-03-06-sdk-safety-refactor.md)** — 2026 03 06 Sdk Safety Refactor
> SDK Safety Refactor Implementation Plan

**29. [`docs/src/README.md`](docs/src/README.md)** — Readme
> AtipicialRust SDK Documentation

**30. [`docs/src/SUMMARY.md`](docs/src/SUMMARY.md)** — Summary
> Summary

**31. [`docs/src/examples/README.md`](docs/src/examples/README.md)** — Readme
> AtipicialRust Examples

**32. [`docs/src/guides/getting-started.md`](docs/src/guides/getting-started.md)** — Getting Started
> Getting Started with AtipicialRust SDK

**33. [`docs/src/guides/installation.md`](docs/src/guides/installation.md)** — Installation
> Installation Guide

**34. [`docs/src/reference/api-overview.md`](docs/src/reference/api-overview.md)** — Api Overview
> API Overview

**35. [`docs/src/reference/configuration.md`](docs/src/reference/configuration.md)** — Configuration
> Configuration

**36. [`docs/src/reference/copyright.md`](docs/src/reference/copyright.md)** — Copyright
> Copyright Information

**37. [`docs/src/reference/error-handling.md`](docs/src/reference/error-handling.md)** — Error Handling
> Error Handling

**38. [`docs/src/tutorials/aep17-tokens.md`](docs/src/tutorials/aep17-tokens.md)** — Aep17 Tokens
> AEP-17 Tokens

**39. [`docs/src/tutorials/atipicial-x.md`](docs/src/tutorials/atipicial-x.md)** — Atipicial X
> Atipicial X Integration

**40. [`docs/src/tutorials/nns.md`](docs/src/tutorials/nns.md)** — Nns
> Atipicial Name Service (NNS)

**41. [`docs/src/tutorials/smart-contracts.md`](docs/src/tutorials/smart-contracts.md)** — Smart Contracts
> Smart Contracts

**42. [`docs/src/tutorials/transactions.md`](docs/src/tutorials/transactions.md)** — Transactions
> Transactions

**43. [`docs/src/tutorials/wallet-management.md`](docs/src/tutorials/wallet-management.md)** — Wallet Management
> Wallet Management

**44. [`docs/superpowers/plans/2026-06-17-audit-refactor-aws-sdk-compliance.md`](docs/superpowers/plans/2026-06-17-audit-refactor-aws-sdk-compliance.md)** — 2026 06 17 Audit Refactor Aws Sdk Compliance
> AtipicialRust 审计 / 重构 / AWS-SDK 规范一致性 实施计划

**45. [`docs/superpowers/plans/2026-06-17-sdk-user-friendliness.md`](docs/superpowers/plans/2026-06-17-sdk-user-friendliness.md)** — 2026 06 17 Sdk User Friendliness
> SDK 用户友好性与专业性提升 实施计划

**46. [`docs/superpowers/plans/2026-07-11-atipicial-rust-sdk-2.1.0-release.md`](docs/superpowers/plans/2026-07-11-atipicial-rust-sdk-2.1.0-release.md)** — 2026 07 11 Atipicial Rust Sdk 2.1.0 Release
> AtipicialRust 2.1.0 Release Plan

**47. [`docs/wallets/README.md`](docs/wallets/README.md)** — Readme
> Atipicial Wallets

**48. [`docs/wallets/message-signing.md`](docs/wallets/message-signing.md)** — Message Signing
> Message Signing in Atipicial


---

## 🤝 Contributing

Read the design laws above. Priorities, in order: deterministic correctness → exact parity → clear boundaries → measured performance → usability. Determinism is not negotiable.

---

<div align="center">

## 👑 FOUNDER

<table>
<tr><td align="center" width="33%">

### **xmoohad**

</td></tr>
<tr><td align="center">

**Founder · Architect · Blockchain Scientist · Computer Programmer**

</td></tr>
<tr><td align="center">

*Atipicial Chain is a sovereign Layer-1 blockchain for smart contracts,
digital assets, and decentralized applications.*

Every crate, every opcode, every line of this repository descends from a
single engineering vision: **build a reliable blockchain platform for
smart contracts, digital assets, and decentralized applications.**

</td></tr>
</table>

`ATC` — Atipicial Coin · `ATD` — AtipicialDollar · addresses begin with **A**

</div>

---

*© Atipicial Chain · Founded by xmoohad · MIT License*
