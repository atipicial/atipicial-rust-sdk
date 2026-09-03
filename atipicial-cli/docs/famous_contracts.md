<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Working with Well-known Contracts and Tokens

Historically, this repository referenced a separate `atipicial-cli famous ...` command group for “well-known” contracts.

The current CLI surfaces these workflows primarily through:

- `atipicial-cli de-fi token` for common token metadata lookups (by symbol or script hash)
- `atipicial-cli de-fi balance` for quick balance checks
- `atipicial-cli contract ...` for generic contract deployment/invocation flows

## Token Shortcuts

```bash
# Resolve common symbols (ATC / GAS / FLM / bATC, etc.) or use a script hash directly
atipicial-cli de-fi token ATC
atipicial-cli de-fi token GAS
atipicial-cli de-fi token 0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5
```

## Balance Checks

```bash
atipicial-cli de-fi balance GAS NZKvXidwBhnV8rNXh2eXtpm5bH1rkofaDz
```

## Generic Contract Workflows

Use `atipicial-cli contract --help` for the full set of contract-related commands supported by your build.

```bash
atipicial-cli contract list-native-contracts
```

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
