```rust
//! IPC-система: клиент и сервер для обмена онтологическими данными.

pub mod client;
pub mod server;

pub use client::IpcClient;
pub use server::IpcServer;
```