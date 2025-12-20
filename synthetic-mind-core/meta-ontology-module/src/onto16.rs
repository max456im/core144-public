```rust
use synthetic_mind_core::multimodal_invariants::Onto8;

/// Онто16: расширенная структура — 2 × Onto8 (язык + эмоция)
#[derive(Clone, Debug)]
pub struct Onto16 {
    pub linguistic: Onto8,
    pub emotional: Onto8,
}

impl Onto16 {
    pub fn new(linguistic: Onto8, emotional: Onto8) -> Self {
        Self { linguistic, emotional }
    }
}
```