<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Installation

Get started with AtipicialRust SDK v3.0.0 by installing it in your Rust project.

## Prerequisites

- Rust 1.91 or later
- Cargo package manager

## Adding to Your Project

Add AtipicialRust to your `Cargo.toml`:

```toml
[dependencies]
atipicial = "3.0.0"
```

For specific features, use:

```toml
[dependencies]
atipicial = { version = "3.0.0", features = ["futures", "ledger"] }
```

## Available Features

- `futures` - Async/await support (recommended)
- `ledger` - Hardware wallet support
- `ws` - WebSocket client support

## Verification

Verify your installation:

```rust
use atipicial::prelude::*;

fn main() {
    println!("AtipicialRust SDK v3.0.0 is ready!");
}
```

## Next Steps

- [Quick Start Guide](./quick-start.md)
- [Examples](./examples.md)

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**

---

## About Atipicial Chain

Atipicial is a sovereign Layer-1 blockchain engineered specifically for the
decentralized coordination of Artificial General Intelligence. It makes
**deploying an intelligent Agent** the universal primitive: an entity with
identity, memory, permissions, a wallet, and pricing that can call AI
capabilities and transact with other Agents on-chain.

| | |
|---|---|
| **ATC** — Atipicial Coin | Governance & staking · 1,000,000,000 total · locked, non-spent |
| **ATD** — AtipicialDollar | Settlement & fees · 500,000,000 genesis |
| **Addresses** | Begin with capital **`A`** (version byte `0x09`) |
| **Genesis** | 2026-07-20 00:00:00 UTC |
| **Standards** | AEP-17 (fungible) · AEP-11 (NFT) · AEP-6 (wallets) · AEP-2 (keys) |
| **Format** | AEF — Atipicial Executable Format |
| **Consensus** | dBFT 2.0 — single-block finality |
| **Seeds** | `seed1-5.atipicial.com:10333` (P2P) · `seed1-5.atipicial.com:10332` (RPC) |

> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
