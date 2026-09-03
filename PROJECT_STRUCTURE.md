<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# AtipicialRust SDK Project Structure

This document outlines the professional organization of the AtipicialRust SDK project, designed for maintainability, scalability, and ease of use.

## 📁 Directory Structure

```
AtipicialRust/
├── .github/                      # GitHub specific files
│   ├── workflows/                # CI/CD workflows
│   │   ├── build-test.yml       # Main CI pipeline
│   │   ├── release.yml          # Release automation
│   ├── ISSUE_TEMPLATE/          # Issue templates
│   ├── PULL_REQUEST_TEMPLATE.md # PR template
│   └── dependabot.yml           # Dependency updates
│
├── src/                          # Main `atipicial` library crate
│   ├── atipicial_builder/              # Transaction & script building
│   ├── atipicial_clients/              # JSON-RPC + transports (HTTP/WS/IPC)
│   ├── atipicial_codec/                # Serialization/deserialization
│   ├── atipicial_config/               # Network + runtime configuration
│   ├── atipicial_contract/             # Smart contract interactions
│   ├── atipicial_crypto/               # Cryptographic operations
│   ├── atipicial_error/                # Error types (legacy + unified)
│   ├── atipicial_fs/                   # AtipicialFs integration
│   ├── atipicial_protocol/             # Protocol/domain types
│   ├── atipicial_sgx/                  # SGX support (optional)
│   ├── atipicial_types/                # Core types and primitives
│   ├── atipicial_utils/                # Shared utilities
│   ├── atipicial_wallets/              # Wallet management
│   ├── atipicial_x/                    # Atipicial X / EVM compatibility layer
│   ├── sdk/                      # High-level ergonomic API (`sdk::Atipicial`)
│   ├── lib.rs                    # Crate root + re-exports
│   └── prelude.rs                # Convenience prelude exports
│
├── examples/                     # Example applications
│   ├── basic/                   # Basic usage examples
│   │   ├── src/
│   │   └── Cargo.toml
│   ├── intermediate/            # Intermediate examples
│   │   ├── src/
│   │   └── Cargo.toml
│   └── advanced/                # Advanced examples
│       ├── src/
│       └── Cargo.toml
│
├── atipicial-cli/                      # Command-line interface (workspace member)
│   ├── src/
│   ├── templates/                # Project generator templates
│   └── Cargo.toml
│
├── docs/                        # Documentation
│   ├── src/                     # mdBook source content
│   ├── book/                    # Generated mdBook output (tracked)
│   └── guides/                  # Additional guides
│
├── tests/                      # Integration tests
│   └── *.rs
│
├── benches/                   # Benchmarks
│   └── *.rs
│
├── .cargo/                    # Cargo configuration
│   └── config.toml
├── vendor/                    # Vendored/patched deps (if any)
├── config/                    # App configuration (dev/prod)
├── assets/                    # Branding assets
├── website/                   # Project website sources
│
├── Cargo.toml                 # Workspace manifest
├── Cargo.lock                 # Lock file
├── README.md                  # Main README
├── LICENSE-MIT               # MIT License
├── LICENSE-APACHE            # Apache License
├── CONTRIBUTING.md           # Contributing guidelines
├── CODE_OF_CONDUCT.md        # Code of conduct
├── CHANGELOG.md              # Change log
├── SECURITY.md               # Security policy
├── rust-toolchain.toml       # Rust toolchain specification
└── deny.toml                 # Cargo deny configuration
```

> Note: A multi-crate split under `crates/` is planned/tracked in `WORKSPACE_REORGANIZATION.md`.

## 🎯 Design Principles

### 1. **Modular Architecture**
- Major components live as modules under `src/` in the `atipicial` crate
- Workspace applications (`atipicial-cli`) are separate crates
- Clear separation of concerns
- Minimal cross-module coupling
- Well-defined public APIs

### 2. **Workspace Organization**
- Single workspace root with multiple member crates
- Shared dependencies managed at workspace level
- Consistent versioning across crates
- Unified build and test commands

### 3. **Professional Naming**
- Consistent naming conventions across modules and crates
- Core SDK modules prefixed with `atipicial_` for clarity (e.g., `atipicial_clients`, `atipicial_builder`)
- Clear, descriptive module names
- No abbreviations in public APIs

### 4. **Documentation First**
- Every crate has its own README
- Comprehensive rustdoc comments
- Example code in documentation
- Architecture decision records

## 📦 Responsibilities

### `atipicial` (library crate)
- Contains the full SDK as internal modules under `src/`
- Exposes a high-level ergonomic API under `src/sdk/`
- Provides a `prelude` for convenient imports

### Workspace applications
- `atipicial-cli/`: CLI for common operations (wallets, contracts, AtipicialFs, etc.)

### Core modules (selected)
- `atipicial_clients`: JSON-RPC client + HTTP/WS/IPC transports
- `atipicial_builder`: transaction/script building + gas estimation
- `atipicial_wallets`: AEP-6 wallets + key management
- `atipicial_contract`: contract invocation + standard contracts

## 🔧 Development Workflow

### Building
```bash
# Build all crates
cargo build --workspace

# Build specific crate
cargo build -p atipicial
cargo build -p atipicial-cli

# Build with all features
cargo build --all-features
```

### Testing
```bash
# Run all tests
cargo test --workspace

# Run specific crate tests
cargo test -p atipicial
cargo test -p atipicial-cli

# Run integration tests
cargo test --tests
```

### Documentation
```bash
# Build and open documentation
cargo doc --open --no-deps

# Build with private items
cargo doc --document-private-items
```

## 📋 Best Practices

### Code Organization
1. Keep modules focused and single-purpose
2. Use `mod.rs` for module organization
3. Separate public API from implementation
4. Group related functionality together

### Error Handling
1. Use custom error types per crate
2. Implement std::error::Error
3. Provide context in error messages
4. Use Result<T, E> consistently

### Testing
1. Unit tests in src files
2. Integration tests in tests/
3. Doc tests for examples
4. Property-based testing where appropriate

### Documentation
1. Document all public items
2. Include examples in docs
3. Link to related items
4. Explain "why" not just "what"

## 🚀 Release Process

1. **Version Bump**
   - Update version in all Cargo.toml files
   - Update CHANGELOG.md
   - Update documentation

2. **Quality Checks**
   - Run full test suite
   - Check documentation builds
   - Run clippy and fmt
   - Security audit

3. **Release**
   - Tag release in git
   - Publish to crates.io
   - Update GitHub release
   - Announce on channels

## 🔐 Security Considerations

- Regular dependency audits
- Security policy in SECURITY.md
- Responsible disclosure process
- Security-focused code reviews
- Fuzzing for critical components

## 📈 Performance Guidelines

- Benchmark critical paths
- Profile before optimizing
- Document performance characteristics
- Consider zero-copy where possible
- Async-first design

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed guidelines on:
- Code style
- Commit messages
- Pull request process
- Issue reporting
- Development setup

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
