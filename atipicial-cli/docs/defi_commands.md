<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# DeFi Commands for Atipicial CLI

This guide explains how to use the Atipicial CLI to interact with various DeFi platforms on the Atipicial blockchain.

## Overview

The Atipicial blockchain ecosystem has several DeFi protocols that provide various financial services. The `atipicial-cli` tool provides direct access to these protocols through the `de-fi` command group, allowing you to:

1. Interact with famous DeFi contracts
2. Get token information
3. Check token balances
4. Transfer tokens
5. Swap tokens
6. Provide/remove liquidity
7. Stake tokens and claim rewards
8. And more

## General DeFi Commands

### Get Token Information

To see detailed information about a token:

```bash
atipicial-cli de-fi token ATC
atipicial-cli de-fi token GAS
atipicial-cli de-fi token 0xf970f4cddcd087ab5d8a5697a32b3cfd32c8b465  # Using script hash
```

### Check Token Balance

To check your balance of a specific token:

```bash
# Check balance for a specific address
atipicial-cli de-fi balance GAS NZKvXidwBhnV8rNXh2eXtpm5bH1rkofaDz
```

### Transfer Tokens

To transfer tokens to another address (requires a signing account; some flows may still be under active development):

```bash
# Transfer 10 GAS
atipicial-cli de-fi transfer GAS NZKvXidwBhnV8rNXh2eXtpm5bH1rkofaDz 10

# Transfer with additional data
atipicial-cli de-fi transfer ATC NZKvXidwBhnV8rNXh2eXtpm5bH1rkofaDz 100 "Payment for services"
```

## Flamingo Finance Commands

Flamingo Finance is a DeFi platform on Atipicial offering trading, earning, and borrowing services.

### Swap Tokens

To swap one token for another:

```bash
# Swap 10 GAS for ATC with default slippage (10%)
atipicial-cli de-fi flamingo swap GAS ATC 10

# Swap with custom minimum return amount
atipicial-cli de-fi flamingo swap FLM GAS 100 5
```

### Add Liquidity

To add liquidity to a trading pair:

```bash
atipicial-cli de-fi flamingo add-liquidity ATC GAS 10 5
```

### Remove Liquidity

To remove liquidity from a trading pair:

```bash
atipicial-cli de-fi flamingo remove-liquidity ATC GAS 10
```

### Stake Tokens

To stake tokens and earn rewards:

```bash
atipicial-cli de-fi flamingo stake FLM 100
```

### Claim Rewards

To claim rewards from staking:

```bash
atipicial-cli de-fi flamingo claim-rewards
```

## AtipicialBurger Commands

AtipicialBurger (bATC) is a wrapped ATC token that allows users to earn GAS while using their ATC in DeFi.

### Wrap ATC to bATC

To wrap your ATC to bATC:

```bash
atipicial-cli de-fi atipicial-burger wrap 100
```

### Unwrap bATC to ATC

To unwrap your bATC back to ATC:

```bash
atipicial-cli de-fi atipicial-burger unwrap 100
```

### Claim GAS

To claim GAS rewards from your bATC:

```bash
atipicial-cli de-fi atipicial-burger claim-gas
```

### Get Exchange Rate

To check the current exchange rate between bATC and ATC:

```bash
atipicial-cli de-fi atipicial-burger get-rate
```

## AtipicialCompound Commands

AtipicialCompound is an automated interest compounding service for Atipicial ecosystem tokens.

### Deposit Tokens

To deposit tokens into AtipicialCompound:

```bash
atipicial-cli de-fi atipicial-compound deposit GAS 50
```

### Withdraw Tokens

To withdraw tokens from AtipicialCompound:

```bash
atipicial-cli de-fi atipicial-compound withdraw GAS 25
```

### Compound Interest

To manually compound your interest:

```bash
atipicial-cli de-fi atipicial-compound compound GAS
```

### Get APY

To check the current APY for a token:

```bash
atipicial-cli de-fi atipicial-compound get-apy GAS
```

## GrandShare Commands

GrandShare is a governance and funding platform for Atipicial ecosystem projects.

### Submit a Proposal

To submit a new proposal:

```bash
atipicial-cli de-fi grand-share submit-proposal "My Project Title" "Project description and details" 1000
```

### Vote on a Proposal

To vote on an existing proposal:

```bash
# Approve a proposal
atipicial-cli de-fi grand-share vote 123 --approve

# Reject a proposal
atipicial-cli de-fi grand-share vote 123
```

### Fund a Project

To fund an approved project:

```bash
atipicial-cli de-fi grand-share fund-project 456 500
```

### Claim Funds

To claim funds for your project (if you're the project owner):

```bash
atipicial-cli de-fi grand-share claim-funds 456
```

## Common Options

Most DeFi commands support the following options:

- `--wallet`: Path to your wallet file
- `--password`: Wallet password

Example:

```bash
atipicial-cli de-fi --wallet path/to/wallet.json --password mypassword flamingo swap ATC GAS 10
```

## Notes

- All amounts are specified in the token's natural units (e.g., 1 GAS = 1 GAS, not 1 * 10^8 GAS)
- Token symbols are case-insensitive
- You can also use script hashes instead of token symbols
- Make sure your wallet has enough GAS to pay for transaction fees

---

> **Atipicial Chain** — sovereign Layer-1 for smart contracts and digital assets.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
