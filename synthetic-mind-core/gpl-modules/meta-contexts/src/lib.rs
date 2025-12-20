```rust
//! Полный цикл: от контекста к метаконтексту и обратно
//! Лицензия: GPL v3

use interfaces::Onto8;
use context_accents::{ContextAccent, elevate_to_meta};

pub struct MetaContextSystem {
    pub current: ContextAccent,
    pub history: [ContextAccent; 8],
    pub index: usize,
}

impl MetaContextSystem {
    pub fn new(onto: Onto8) -> Self {
        let accent = context_accents::compute_accent(onto, 0);
        Self {
            current: accent,
            history: [accent; 8],
            index: 0,
        }
    }

    pub fn step(&mut self, onto: Onto8) {
        self.current = context_accents::compute_accent(onto, self.current.meta_level);
        if self.current.weight.abs() > 0.8 {
            self.current = elevate_to_meta(self.current);
        }
        self.history[self.index] = self.current;
        self.index = (self.index + 1) % 8;
    }
}
```