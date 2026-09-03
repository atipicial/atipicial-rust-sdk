<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Wallet Management

Learn how to create and manage wallets with AtipicialRust SDK.

## Creating a New Wallet

```rust
use atipicial::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wallet = Wallet::new();
    wallet.set_name("MyWallet".to_string());
    
    let account = Account::create()?;
    wallet.add_account(account);
    
    println!("Wallet created with {} accounts", wallet.get_accounts().len());
    Ok(())
}
```

## Loading from AEP-6

```rust
use atipicial::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wallet_json = std::fs::read_to_string("wallet.json")?;
    let aep6_wallet: Aep6Wallet = serde_json::from_str(&wallet_json)?;
    
    let password = "your_password";
    let wallet = Wallet::from_aep6(&aep6_wallet, password)?;
    
    println!("Loaded wallet: {}", wallet.get_name());
    Ok(())
}
```

## Account Management

- Create new accounts
- Import existing accounts
- Manage multiple accounts
- Set default account

## Security

- Password protection
- Secure key storage
- Hardware wallet integration

---

> **Atipicial Chain** — sovereign Layer-1 for smart contracts and digital assets.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
