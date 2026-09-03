<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# AtipicialFs Examples

These examples show how to use the AtipicialFs functionality in AtipicialRust.

## Available Examples

1. **Basic Usage** (`basic_usage.rs`): Builds real AtipicialFs container/object payloads, probes the TestNet gateway, and optionally lists containers when `ATCFS_WALLET` is set.
2. **Multipart Upload** (`multipart_upload.rs`): Plans a multipart upload locally (splitting a payload into parts), validates assembly, and optionally exercises the initiate call.

## Running the Examples

> **Note**: AtipicialFs REST/gRPC endpoints may require valid auth/session tokens. The examples
> run in a safe "probe" mode by default and will not panic if the remote returns an error.

To run the examples:

```bash
cargo run --example atipicial_fs_basic_usage

cargo run --example atipicial_fs_multipart_upload

# Optional: supply your wallet address to attempt authenticated calls
ATCFS_WALLET=NdemoAddressHere cargo run --example atipicial_fs_basic_usage
```

## Example Output

With no `ATCFS_WALLET` set, you’ll see a gateway probe and payload previews:

```
🌐 Endpoint: https://rest.testnet.fs.atipicial.com
🔐 Auth: not provided (read-only probe)
🧱 Container request payload:
{ ... }
🔎 Probing AtipicialFs REST gateway...
   ✅ Gateway responded with HTTP 200
ℹ️ Set ATCFS_WALLET to attempt authenticated container listing.
```

## Requirements

- AtipicialRust SDK
- Network connectivity to AtipicialFs TestNet gateway
- Optional `ATCFS_WALLET` for authenticated operations

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
