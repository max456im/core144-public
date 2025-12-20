```rust
#[derive(Clone, Copy, Debug)]
pub struct CoherenceLevel(pub f32); // [0.0, 1.0]

impl CoherenceLevel {
    pub fn is_coherent(&self) -> bool {
        self.0 > 0.7
    }
}

pub fn compute(projections: &[u32; 4], background: u32) -> CoherenceLevel {
    let mut sum = 0u64;
    for &p in projections {
        sum += (p as u64).wrapping_sub(background as u64);
    }
    let avg = sum / 4;
    CoherenceLevel((avg as f32 / u32::MAX as f32).clamp(0.0, 1.0))
}
```
