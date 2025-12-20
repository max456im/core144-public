```rust
//! Контекстные акценты и метаконтексты
//! Лицензия: GPL v3

use interfaces::Onto8;

#[derive(Clone, Debug)]
pub struct ContextAccent {
    pub onto: Onto8,
    pub weight: f32,      // [-1.0, 1.0]
    pub meta_level: u8,   // 0 = контекст, 1 = метаконтекст, 2 = мета-мета...
}

pub fn compute_accent(onto: Onto8, depth: u8) -> ContextAccent {
    let weight = (onto.object as f32 / u32::MAX as f32) * 2.0 - 1.0;
    ContextAccent { onto, weight, meta_level: depth }
}

pub fn elevate_to_meta(accent: ContextAccent) -> ContextAccent {
    ContextAccent {
        meta_level: accent.meta_level + 1,
        ..accent
    }
}
```