```rust
//! Runtime для безопасного взаимодействия с GPL-компонентами.
//! Использует IPC для изоляции лицензий.
//! 
//! Лицензия: Apache 2.0

pub mod ipc;
pub mod plugin;

pub use ipc::{IpcClient, IpcServer};
pub use plugin::GplPluginLoader;
```