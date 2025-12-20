```rust
pub type Measure = f32;

pub fn lebesgue_measure(space: &super::LebesgueSpace) -> Measure {
    space.measure()
}
```