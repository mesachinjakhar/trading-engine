# 🚀 Async Trading Engine in Rust (Tokio-based)

A **deterministic, event-driven trading engine** written in Rust, implementing a real **price–time priority (FIFO) limit order matching engine** with partial fills, async order ingestion, structured logging, and graceful shutdown.

This project demonstrates **production-grade trading system fundamentals**, not a toy example.

---

## ✨ Features

- ✅ Limit order matching engine  
- ✅ Price priority (highest buy / lowest sell)  
- ✅ Time priority (FIFO) for same-price orders  
- ✅ Partial fills with remaining quantity tracking  
- ✅ Strict order lifecycle (state machine)  
- ✅ Deterministic, single-threaded matching core  
- ✅ Async event-driven wrapper using Tokio  
- ✅ Structured logging with `tracing`  
- ✅ Graceful shutdown (SIGINT / CTRL+C)  
- ✅ Unit-tested core matching logic  

---

## 🧠 Design Philosophy

### 1. Deterministic Core (Single-Threaded)

The matching engine itself is **strictly synchronous and single-threaded**.

**Why this design?**
- Guarantees determinism (same input → same output)
- Avoids locks (`Mutex`, `RwLock`)
- Prevents race conditions
- Simplifies testing and reasoning

> Concurrency is handled **around** the engine, not **inside** it.

This is how real exchanges design their matching engines.

---

### 2. Async Event-Driven Architecture

Async is used only for **event delivery**, not for matching logic.

```text
Producers (clients / feeds / APIs)
        |
        |  Tokio mpsc channel
        v
+------------------------------+
| Engine Task (single task)    |
| - receives events            |
| - updates OrderBook          |
| - runs matching logic        |
+------------------------------+
```

- Many async producers
- One authoritative engine task
- No shared mutable state
- Natural backpressure via channels

---

## 📦 Core Components

### Order
Represents a limit order with:
- Side (Buy / Sell)
- Price
- Quantity & remaining quantity
- Order state (Created → Accepted → PartiallyFilled → Filled)
- Engine-assigned sequence number (FIFO fairness)

### OrderBook
- Maintains buy & sell books
- Selects best buy and best sell
- Applies price–time priority
- Executes full or partial matches
- Cleans up filled orders

### Engine
- Owns the OrderBook
- Assigns sequence numbers (FIFO)
- Processes events:
  - NewOrder
  - CancelOrder
  - Shutdown
- Runs inside a single Tokio task

---

## 🧪 Testing Strategy

Core correctness is validated using unit tests on the matching engine:

- ✔ Price priority test
- ✔ FIFO (time priority) test
- ✔ Partial fill behavior test

Async code is intentionally kept thin and untested; all correctness lives in the deterministic core.

Run tests:

```bash
cargo test
```

---

## 🪵 Logging & Observability

Uses `tracing` for structured, async-aware logging.

Logs include:
- Engine startup & shutdown
- New orders received
- Trades executed
- Cancel requests

Example output:

```text
INFO  Engine started
INFO  New order received order_id=1 side=Buy price=100 qty=10
INFO  New order received order_id=2 side=Sell price=90 qty=5
INFO  Trade executed buy=1 sell=2 price=90 qty=5
INFO  Engine shutdown signal received
INFO  Engine stopped
```

---

## 🔒 Graceful Shutdown

- Listens for SIGINT (CTRL+C)
- Sends Shutdown event to engine
- Engine completes current work
- Clean, deterministic exit

This is critical for production trading systems.

---

## 🛠 Tech Stack

- **Rust**
- **Tokio** (async runtime)
- **tokio::mpsc** (event delivery)
- **tracing / tracing-subscriber** (logging)

**No databases.**  
**No locks.**  
**No shared mutable state.**

---

## ⚠️ Trade-offs & Future Work

### Intentional trade-offs
- HashMap-based order storage (clarity over micro-optimizations)
- Single-threaded core (determinism over raw throughput)

### Possible extensions
- Price-level order queues
- Market orders
- Persistent snapshots
- REST / WebSocket gateway
- Performance benchmarks
- Multi-instrument support

---

## 🎯 Purpose of This Project

This project exists to demonstrate:

- Real market microstructure rules
- Correct price–time priority matching
- Safe async Rust architecture
- Production-grade system design discipline

It is intentionally **not** a CRUD app or tutorial toy.

---

## 📄 License

MIT
