<!-- Atipicial Chain · sovereign Layer-1 for smart contracts and digital assets -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# Transaction Simulation Guide

Use `atipicial::sdk::transaction_simulator::TransactionSimulator` to preview gas, VM state, and effects before broadcasting.

## Example

```rust,no_run
use atipicial::{
    atipicial_builder::ScriptBuilder,
    atipicial_clients::{HttpProvider, RpcClient},
    sdk::transaction_simulator::TransactionSimulator,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = HttpProvider::new("https://testnet1.atipicial.com:443")?;
    let client = RpcClient::new(provider);
    let mut simulator = TransactionSimulator::new(client);

    let script = ScriptBuilder::new().push_data(vec![0x01, 0x02, 0x03]).to_bytes();
    let result = simulator.simulate_script(&script, vec![]).await?;

    println!("Gas: {}", result.gas_consumed);
    println!("VM State: {:?}", result.vm_state);
    println!("Success: {}", result.success);
    Ok(())
}
```

## Recommendations
- Simulate before every user-facing transaction in production.
- Fail fast on `FAULT` VM states and surface the exception message.
- Add a safety margin to gas estimates for congested periods.

---

> **Atipicial Chain** — sovereign Layer-1 for smart contracts and digital assets.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
