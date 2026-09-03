<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Installation Guide

## Prerequisites

- Rust 1.91 or later with Cargo
- Optional: Ledger hardware device (for ledger features)
- Optional: YubiHSM device (for hardware security module features)

## Installation

Add AtipicialRust to your `Cargo.toml`:

```toml
[dependencies]
atipicial = "2.1.0"
```

The crate is published and imported as `atipicial`:

```rust,no_run
use atipicial::prelude::*;
```

## Features

AtipicialRust provides several features to customize functionality:

- `futures`: Enables async/futures support (recommended)
- `ws`: Enables the modern WebSocket transport
- `ipc`: Enables IPC (Unix domain sockets / Windows named pipes) transport
- `ledger`: Enables hardware wallet support via Ledger devices
- `yubi` / `mock-hsm`: YubiHSM support and its mock for testing
- `legacy-ws`: Compatibility WebSocket transport (fallback)

Hardware wallet feature gates are compile-checked, but production releases should include real-device tests. The `sgx` and `no_std` flags are experimental specialized build gates and are not certified as general embedded or WASM support.

Example of enabling specific features:

```toml
[dependencies]
atipicial = { version = "3.0.0", features = ["futures", "ws", "ledger"] }
```

You can disable default features with:

```toml
[dependencies]
atipicial = { version = "3.0.0", default-features = false, features = ["futures"] }
```

## Verifying Installation

To verify that the SDK is installed correctly, create a simple test program:

```rust,no_run
use atipicial::prelude::*;

fn main() {
    println!("AtipicialRust SDK installed successfully!");
}
```

Compile and run the program:

```bash
cargo run
```

If the program compiles and runs without errors, the SDK is installed correctly.

<!-- toc -->

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
