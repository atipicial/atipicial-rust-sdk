<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Choosing the Right API Layer

AtipicialRust ships **two complementary layers**. Most applications use the
high-level layer for everything and only reach into the low-level modules for a
handful of advanced cases. This guide tells you which to reach for, and when to
switch.

> **TL;DR** — Start with `Atipicial::testnet().await?` from the high-level layer.
> Drop down only when the table below tells you to.

---

## Quick decision table

| You want to… | Use | Returns |
|---|---|---|
| Connect in one line, query block height / balance | `sdk::Atipicial` | `Balance`, `u32`, … |
| Send or transfer AEP-17 tokens with retry + signing handled | `sdk::Atipicial::transfer` | `sdk::TxHash` |
| Read a contract method without sending a tx | `sdk::Atipicial::invoke_read` | `serde_json::Value` |
| Write/call a contract method (broadcasts a tx) | `sdk::Atipicial::invoke_write` | `sdk::TxHash` |
| Deploy a contract | `sdk::Atipicial::deploy_contract` | `atipicial_types::ScriptHash` |
| Wait for a tx to confirm | `sdk::Atipicial::wait_for_confirmation` | `()` |
| HD / BIP-39 wallet generation | `sdk::hd_wallet::HDWallet` | `Account` |
| Preview a tx's fees/state-changes before sending | `sdk::transaction_simulator` | `SimulationResult` |
| Stream blocks / txs over WebSocket | `sdk::websocket::WebSocketClient` | events |
| Call a **specific** RPC method by hand (e.g. `getproof`, `getstateheight`) | `providers::RpcClient` | typed RPC response |
| Build a raw script / sign manually / control every byte | `builder::TransactionBuilder` + `builder::ScriptBuilder` | `Transaction` |
| Work with Atipicial X (EVM sidechain) | `atipicial_x::AtipicialXWallet`, `sdk::unified::EcosystemClient` | EVM types |
| Store/retrieve on AtipicialFs | `atipicial_fs::client` | AtipicialFs objects |

The high-level `sdk::TxHash` is a hex-encoded `String`. It is distinct from
`atipicial_types::TxHash`, the low-level alias for `H256`.

---

## Layer 1 — High-level: `sdk::Atipicial`

The opinionated, batteries-included client. **Start here.**

```no_run
use atipicial::prelude::*; // brings Atipicial, AtipicialBuilder, Network, AtipicialError, … into scope

# #[tokio::main]
# async fn main() -> Result<(), AtipicialError> {
let atipicial = Atipicial::testnet().await?;           // or Atipicial::from_env() / Atipicial::connect(url)
let height = atipicial.get_block_height().await?;
let balance = atipicial.get_balance("NbTiM6h8r99kpRtb428XcsUk1TzKed2gTc").await?;
println!("tip={height}, ATC={}", balance.atipicial);
# Ok(())
# }
```

What you get for free:
- **Automatic retry** with bounded budget (`SdkConfig::retries`).
- **Caching** of idempotent lookups (`SdkConfig::cache`).
- **Unified errors** — every call returns `AtipicialError` with `kind()`,
  `is_retryable()`, and human-readable recovery hints (mirrors the AWS SDK
  `ProvideErrorMetadata` pattern).
- **Configurability** via `Atipicial::builder().network(..).timeout(..).retries(..)`.

All errors are the single `atipicial::prelude::AtipicialError` type (= `unified::AtipicialError`),
so `fn() -> Result<T, AtipicialError>` composes with `?` across the whole SDK.

---

## Layer 2 — Low-level modules

Reach here when the high-level layer does not expose exactly what you need.

### `providers::RpcClient` — raw JSON-RPC

Covers the full Atipicial RPC surface (91 methods), including the state-root /
proof / iterator family that the high-level layer does not wrap:

```no_run
use atipicial::atipicial_clients::{HttpProvider, RpcClient, APITrait};

# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
let client = RpcClient::new(HttpProvider::new("https://testnet1.atipicial.com:443")?);
let count = client.get_block_count().await?;
let state_height = client.get_state_height().await?;
# Ok(())
# }
```

Low-level calls return their own domain error (`ProviderError`, `ContractError`,
`BuilderError`, …), each of which converts into the unified `AtipicialError` via `?`
if you need a single error boundary.

### `builder::TransactionBuilder` — byte-level tx construction

When you need exact control over the script, signers, attributes, or fees:

```no_run
use atipicial::atipicial_builder::{AccountSigner, ScriptBuilder, TransactionBuilder};
// build script, attach signers, set valid_until_block, then .sign().await?
```

### Domain modules

- `crypto` — key generation, signing, hashing, AEP-2.
- `wallets` — AEP-6 wallets, account import/export, encryption.
- `contract` — typed wrappers over native contracts (`AtipicialCoin`,
  `GasToken`, `PolicyContract`, `RoleManagement`).
- `codec` — Atipicial VM binary serialization.

---

## When to drop down from high-level to low-level

Switch layers when **any** of these is true:

1. **You need an RPC method `Atipicial` does not wrap.** Rare — `Atipicial` covers the
   common ones (balance, transfer, invoke, deploy, confirm). For state proofs,
   session iterators, or mempool details, use `RpcClient` directly.
2. **You need byte-exact transaction control** — custom attributes, multiple
   signers with specific scopes, or hand-tuned network fees. Use
   `TransactionBuilder`.
3. **You want a different retry/cache policy** than `SdkConfig` allows, or want
   none at all. Use `RpcClient` + your own orchestration.
4. **You are building infrastructure** (an indexer, an exchange backend, a
   block explorer) where the overhead of the high-level layer is not justified.

The two layers interoperate: `Atipicial::client()` hands you the underlying
`RpcClient` if you need to make a one-off low-level call without reconnecting.

---

## See also

- Crate-level docs (`cargo doc -p atipicial --open`) — the `## Choosing an API
  layer` section is the short version of this guide.
- `examples/standalone/src/bin/high_level_sdk.rs` — high-level end-to-end.
- `examples/basic/`, `examples/atipicial_transactions/`, `examples/contracts/` —
  low-level patterns by topic.

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
