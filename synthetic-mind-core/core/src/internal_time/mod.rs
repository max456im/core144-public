```rust
pub mod metric;
pub mod integration;

pub use metric::SignificanceMetric;
```

### `core/src/internal_time/metric.rs`
```rust
/// Непрерывная метрика значимости как внутреннее время.
#[derive(Clone, Copy, Debug)]
pub struct SignificanceMetric {
    pub value: f32, // [0.0, ∞)
}

impl SignificanceMetric {
    pub fn new() -> Self {
        Self { value: 0.0 }
    }

    pub fn update(&mut self, delta: f32) {
        self.value = (self.value + delta).max(0.0);
    }
}
```
