<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Installation

Get started with AtipicialRust SDK by installing it in your development environment.

## System Requirements

- **Rust**: Version 1.91 or later
- **Cargo**: Rust's package manager (included with Rust)
- **Operating System**: Windows, macOS, or Linux

## Install Rust

If you don't have Rust installed, get it from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Create a New Project

```bash
cargo new my-atipicial-app
cd my-atipicial-app
```

## Add AtipicialRust Dependency

Add AtipicialRust to your `Cargo.toml`:

```toml
[dependencies]
atipicial = "3.0.0"
tokio = { version = "1.0", features = ["full"] }
```

## Feature Flags

AtipicialRust provides several optional features:

```toml
[dependencies]
atipicial = { version = "3.0.0", features = ["futures", "ledger"] }
```

### Available Features

- **`futures`** — Async/await support (re-exports the `futures` crate; recommended for most apps)
- **`ledger`** — Ledger hardware wallet types (feature gate is compile-checked; production use should include real-device signing tests in your release environment)
- **`ws`** — Modern WebSocket transport (`tokio-tungstenite`), for real-time blockchain events
- **`legacy-ws`** — Legacy WebSocket compatibility layer (fallback)
- **`ipc`** — IPC transport (Unix domain sockets / Windows named pipes)
- **`mock`** — `MockClient` and in-memory providers for offline tests/CI
- **`yubi`** / **`mock-hsm`** — YubiHSM support, or the YubiHSM mock backend for tests
- **`sgx`** / **`no_std`** — Experimental Intel SGX / specialized `no_std` build gates; not certified as general embedded or `wasm32-unknown-unknown` support

## Verify Installation

Create a simple test to verify everything works:

```rust
// src/main.rs
use atipicial::prelude::*;

fn main() {
    println!("AtipicialRust SDK v3.0.0 is ready!");
    
    // Create a simple account
    let account = Account::create().expect("Failed to create account");
    println!("Generated address: {}", account.get_address());
}
```

Run it:

```bash
cargo run
```

You should see output like:
```
AtipicialRust SDK v3.0.0 is ready!
Generated address: NXXXXxxxXXXxxxXXXxxxXXXxxxXXXxxx
```

## Troubleshooting

### Build Errors

If you encounter build errors, make sure you have the latest stable Rust:

```bash
rustup update stable
```

### Platform-Specific Issues

#### macOS
You may need to install additional tools:
```bash
xcode-select --install
```

#### Windows
Ensure you have the Microsoft C++ Build Tools installed.

#### Linux
Install build essentials:
```bash
# Ubuntu/Debian
sudo apt update && sudo apt install build-essential

# CentOS/RHEL
sudo yum groupinstall "Development Tools"
```

## Next Steps

- [Quick Start Guide](./quick-start.md) - Your first Atipicial application
- [Examples](/examples) - Practical code examples

---

> **Atipicial Chain** — sovereign Layer-1 for smart contracts and digital assets.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
