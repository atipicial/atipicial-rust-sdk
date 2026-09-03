<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Contract Examples

These examples demonstrate Atipicial contract manifests, script construction, method invocation,
events, deployment concepts, and the SDK's typed native and token contract wrappers.

Atipicial contracts use a manifest ABI and AtipicialVM parameters rather than an Ethereum Solidity ABI.
Start with these SDK surfaces:

- `SmartContractTrait` for low-level read and invoke helpers.
- `FungibleTokenContract` and `NonFungibleTokenContract` for AEP-17 and AEP-11 contracts.
- `ContractParameter` and `ScriptBuilder` for explicit AtipicialVM calls.
- `ContractManagement` for native deployment and update operations.

Run an example from the workspace root:

```bash
cargo run -p examples-contracts --example atipicial_contract_interaction
cargo run -p examples-contracts --example methods
```

For Atipicial X EVM contracts, use the Alloy-backed APIs in `atipicial::atipicial_x`; the bridge binding in
`src/atipicial_x/bridge/evm_bridge.rs` is a compact `alloy::sol!` example.

---

> **Atipicial Chain** — sovereign Layer-1 for smart contracts and digital assets.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
