```rust
pub mod vision;
pub mod audio;
pub mod text;
pub mod sync;

pub use vision::VisualInvariant;

/// Универсальный 8-байтный инвариант: 4 байта объект, 4 байта фон
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Onto8 {
    pub object: u32,
    pub background: u32,
}

impl Onto8 {
    pub const fn new(object: u32, background: u32) -> Self {
        Self { object, background }
    }
}
```