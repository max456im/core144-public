```rust
pub mod builder;
pub mod coherence;
pub mod decay;

pub use builder::OnticGroupBuilder;
pub use coherence::CoherenceLevel;
pub use decay::DecayMechanism;

#[derive(Clone, Debug)]
pub struct OnticGroup {
    pub projections: [u32; 4], // до 4 проекций объекта
    pub background: u32,
    pub coherence: CoherenceLevel,
}

impl OnticGroup {
    pub fn new(projections: [u32; 4], background: u32) -> Self {
        let coherence = coherence::compute(&projections, background);
        Self { projections, background, coherence }
    }
}
```