```rust
//! Возникновение социальных правил из повторяющихся взаимодействий
//! Лицензия: GPL v3

use interfaces::Onto8;
use core::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct SocialRule {
    pub pattern: [Onto8; 3], // триггер → реакция → ожидание
    pub strength: u8,        // [0..255] — сила правила
}

impl Hash for SocialRule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for o in &self.pattern {
            o.object.hash(state);
            o.background.hash(state);
        }
    }
}

pub fn extract_rule(history: &[[Onto8; 3]]) -> Option<SocialRule> {
    if history.is_empty() { return None; }
    let pattern = history[0];
    let strength = history.len().min(255) as u8;
    Some(SocialRule { pattern, strength })
}
```
