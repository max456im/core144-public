```rust
/// Удержание незавершённости как «тень».
#[derive(Clone, Debug)]
pub struct UnresolvedTension {
    pub id: u64,
    pub content: u32,
    pub phase_at_creation: u32,
}

impl UnresolvedTension {
    pub fn new(id: u64, content: u32, phase: u32) -> Self {
        Self { id, content, phase_at_creation: phase }
    }
}
```