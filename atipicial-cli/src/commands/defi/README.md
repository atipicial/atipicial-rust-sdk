<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Atipicial CLI DeFi Module

The DeFi module exposes production-backed AEP-17 token operations that use Atipicial RPC calls and signed transactions.

## Commands

```text
atipicial-cli de-fi token <CONTRACT>
atipicial-cli de-fi balance <CONTRACT> <ADDRESS>
atipicial-cli de-fi transfer <TOKEN> <TO> <AMOUNT> [DATA]
```

`<CONTRACT>` and `<TOKEN>` accept known symbols such as `ATC` and `GAS`, or a contract script hash.

## Requirements

- A reachable Atipicial RPC endpoint.
- A wallet for state-changing transfers.
- Enough GAS to pay transaction fees.

Protocol-specific DeFi workflows such as swaps, staking, governance, and liquidity management should use `atipicial-cli contract call` or `atipicial-cli contract invoke` with verified contract hashes and method parameters. Dedicated adapters should only be added when the target protocol contract addresses, method signatures, slippage and fee rules, and signing flow are covered by tests.

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
