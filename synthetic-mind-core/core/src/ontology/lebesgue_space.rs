```rust
/// Пространство Лебега–Гаммерштейна как среда порождения сущностей.
/// Онтологическая основа всех фаз и инвариантов.

#[derive(Clone, Copy, Debug)]
pub struct LebesgueSpace {
    pub phase_point: u32,
    pub intensity: u8,
    pub coherence: f32, // [-1.0, 1.0]
}

impl LebesgueSpace {
    pub const fn new(phase_point: u32, intensity: u8, coherence: f32) -> Self {
        Self { phase_point, intensity, coherence }
    }

    /// Возвращает меру значимости
    pub fn measure(&self) -> f32 {
        (self.intensity as f32 / 255.0) * self.coherence.abs()
    }
}
```