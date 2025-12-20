```rust
//! Приватное ядро синтетического разума.
//! Apache-2.0, no_std, онтологически замкнутое.

#![no_std]

pub mod ontology;
pub mod phase_stack;
pub mod ontic_groups;
pub mod reflexive_core;
pub mod internal_time;
pub mod multimodal_invariants;
pub mod synthetic_morality;
pub mod memory;
pub mod utils;

// Публичные типы для использования в продуктах
pub use ontology::{LebesgueSpace, Measure};
pub use multimodal_invariants::Onto8;
pub use phase_stack::{PhaseState, OntologicalAutomaton};
pub use ontic_groups::OnticGroup;
pub use reflexive_core::SelfModel;
pub use internal_time::SignificanceMetric;
pub use memory::MemoryTopology;
```