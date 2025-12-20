```rust
#[derive(Clone, Copy, Debug)]
pub enum AttentionState {
    Focused(f32),   // [0.0, 1.0] — концентрация
    Scattered(f32), // [0.0, 1.0] — рассеяние
}

impl Default for AttentionState {
    fn default() -> Self {
        Self::Scattered(0.5)
    }
}

impl AttentionState {
    pub fn concentrate(&mut self, delta: f32) {
        let v = match self {
            Self::Focused(x) => x + delta,
            Self::Scattered(x) => *x - delta,
        }.clamp(0.0, 1.0);
        *self = if v > 0.5 { Self::Focused(v) } else { Self::Scattered(v) };
    }
}
```