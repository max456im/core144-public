```rust
use super::Onto8;

#[derive(Clone, Copy, Debug)]
pub struct VisualInvariant {
    pub onto8: Onto8,
    pub phase: u8, // фазовая метка
}

impl From<Onto8> for VisualInvariant {
    fn from(onto8: Onto8) -> Self {
        Self { onto8, phase: 0 }
    }
}
```