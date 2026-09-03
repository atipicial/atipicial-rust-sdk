<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Atipicial Cryptography

## Overview

The cryptography module in the AtipicialRust SDK provides comprehensive cryptographic utilities for working with the Atipicial blockchain. It implements the core security primitives necessary for blockchain operations, including key management, hashing, signature creation and verification, and encrypted key storage.

## Key Features

- **Key Pair Management**: Creation and management of ECDSA key pairs on the secp256r1 curve
- **Hashing Functions**: SHA-256, RIPEMD-160, and other cryptographic hash functions
- **Digital Signatures**: Creation and verification of cryptographic signatures
- **AEP-2 Standard**: Private key encryption and decryption according to the AEP-2 standard
- **Base58 Encoding/Decoding**: Utilities for Base58 and Base58Check encoding
- **WIF (Wallet Import Format)**: Import and export of private keys in WIF format
- **Secure Random Number Generation**: Cryptographically secure random number generation

## Documentation Sections

- [AEP-2 Standard](AEP2.md): Implementation of the AEP-2 encrypted key format

## Implementation Components

The cryptography system in AtipicialRust includes several key components:

1. **KeyPair**: Manages ECDSA key pairs for cryptographic operations
2. **Hash**: Cryptographic hash functions and utilities
3. **Base58Helper**: Encoding and decoding utilities
4. **AEP2**: Implementation of the AEP-2 standard for encrypted private keys
5. **WIF**: Utilities for Wallet Import Format handling

## Example Usage

```rust,no_run
use atipicial::prelude::*;

// Create a new random key pair
let key_pair = KeyPair::new_random();
println!("Public key: {}", key_pair.public_key());
// SECURITY: Avoid logging private keys. Use `private_key_bytes()?` only when you
// explicitly need to export key material.

// Sign and verify data
let data = b"Hello, Atipicial!";
let signature = key_pair.sign(data)?;
let is_valid = key_pair.verify_signature(data, &signature)?;
assert!(is_valid);

// Work with AEP-2 encrypted keys
let encrypted = AEP2::encrypt("my-secure-password", &key_pair)?;
let decrypted_key_pair = AEP2::decrypt("my-secure-password", &encrypted)?;
```

## Security Considerations

- The AEP-2 standard uses scrypt as a key derivation function to resist brute force attacks
- Proper handling of private keys is critical; they should never be exposed or stored insecurely
- Random number generation is a cornerstone of cryptographic security
- Signature verification is essential before accepting any signed message or transaction

## Related Modules

- [atipicial_wallets](../wallets/README.md): Wallet management built on cryptography primitives
- [atipicial_protocol](../protocol/README.md): Core protocol implementations
- [atipicial_types](../types/README.md): Type definitions for Atipicial

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
