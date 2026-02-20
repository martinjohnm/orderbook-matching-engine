<!-- 
rust_orderbook/
│
├── Cargo.toml
├── src/
│   ├── main.rs                  # Entry point, launches engine + HTTP/WebSocket server
│   ├── lib.rs                   # Optional, core modules
│   │
│   ├── models/
│   │   ├── order.rs             # Order struct + Side enum
│   │   └── trade.rs             # Trade struct
│   │
│   ├── orderbook/
│   │   ├── mod.rs               # Re-exports add_order, match_orders
│   │   └── orderbook.rs         # OrderBook implementation
│   │
│   ├── engine/
│   │   ├── mod.rs
│   │   └── market.rs            # Market thread, event loop, matching logic
│   │
│   ├── network/
│   │   ├── mod.rs
│   │   ├── http.rs              # HTTP endpoints (POST order, GET book)
│   │   └── ws.rs                # WebSocket server for real-time orders/trades
│   │
│   └── broadcaster/
│       ├── mod.rs
│       └── broadcast.rs         # Async broadcasting logic
│
└── tests/
    ├── unit_tests.rs            # Core orderbook unit tests
    └── integration_tests.rs     # HTTP/WebSocket + engine integration tests -->


## Modules 
### models → Data structures (Order, Trade)

### orderbook → Core matching logic, price-time priority

### engine → Single-threaded event loop per market, handles incoming orders

### network → Async layer: HTTP + WebSocket, receives client requests

### broadcaster → Sends executed trades to clients asynchronously