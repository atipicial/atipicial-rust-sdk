<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# DeFi Commands in Atipicial CLI

The `de-fi` command group provides production-backed AEP-17 token operations.

## Prerequisites

- A reachable Atipicial RPC endpoint.
- A wallet for state-changing transfers.
- Enough GAS for transaction fees.

Use `atipicial-cli de-fi --help` and subcommand help for the exact arguments supported by your build.

## Token Information

```bash
atipicial-cli de-fi token ATC
atipicial-cli de-fi token GAS
atipicial-cli de-fi token 0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5
```

## Balance

```bash
atipicial-cli de-fi balance GAS NZKvXidwBhnV8rNXh2eXtpm5bH1rkofaDz
```

## Transfer

```bash
atipicial-cli de-fi transfer GAS NZKvXidwBhnV8rNXh2eXtpm5bH1rkofaDz 10
atipicial-cli de-fi transfer ATC NZKvXidwBhnV8rNXh2eXtpm5bH1rkofaDz 100 "optional data"
```

For protocol-specific swaps, staking, liquidity, or governance workflows, use `atipicial-cli contract call` or `atipicial-cli contract invoke` with verified contract hashes and method parameters.

---

> **Atipicial Chain** — sovereign Layer-1 for smart contracts and digital assets.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
