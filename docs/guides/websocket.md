<!-- Atipicial Chain · sovereign Layer-1 for decentralized AGI coordination -->
<!-- 👑 Founded & engineered by xmoohad — Blockchain Scientist · Computer Programmer -->

# AtipicialRust WebSocket Guide

Real-time blockchain events are available through `atipicial::sdk::websocket`. The client handles reconnection and subscription tracking.

Requires the `ws` feature:

```toml
atipicial = { version = "3.0.0", features = ["ws"] }
```

## Quickstart

```rust,no_run
use atipicial::sdk::websocket::{SubscriptionType, WebSocketClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ws = WebSocketClient::new("wss://testnet1.atipicial.com/ws").await?;
    ws.connect().await?;

    // Subscribe to new blocks
    let _handle = ws.subscribe(SubscriptionType::NewBlocks).await?;

    if let Some(mut rx) = ws.take_event_receiver() {
        while let Some((_sub, event)) = rx.recv().await {
            println!("Event: {:?}", event);
        }
    }
    Ok(())
}
```

## Tips
- Use secure endpoints in production (`wss://`).
- Run event processing in a dedicated task to avoid blocking.
- Re-subscribe after connection loss if you manage subscriptions manually.
- If you expect large responses, adjust the JSON-RPC message cap with `ATIPICIAL_MAX_RPC_MESSAGE_SIZE` (bytes). The default is 16MiB; increasing it increases memory usage and DoS exposure.
- To avoid hanging connects/requests on unresponsive nodes, set `ATIPICIAL_RPC_TIMEOUT_SECS` to a positive integer (used as a per-request timeout and WS connect/reconnect timeout).

---

> **Atipicial Chain** — sovereign Layer-1 for decentralized AGI coordination.
> 👑 Founded & engineered by **xmoohad** — Blockchain Scientist · Computer Programmer.
> `ATC` Atipicial Coin · `ATD` AtipicialDollar · addresses begin with **A**
