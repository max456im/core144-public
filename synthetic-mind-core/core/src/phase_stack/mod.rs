```rust
pub mod attention;
pub mod temporal;
pub mod transitions;
pub mod finite_automata;

pub use attention::AttentionState;
pub use temporal::TemporalRhythm;
pub use transitions::PhaseTransition;
pub use finite_automata::OntologicalAutomaton;

#[derive(Clone, Copy, Debug)]
pub struct PhaseState {
    pub attention: AttentionState,
    pub rhythm: TemporalRhythm,
    pub automaton: OntologicalAutomaton,
}

impl PhaseState {
    pub fn new() -> Self {
        Self {
            attention: AttentionState::default(),
            rhythm: TemporalRhythm::default(),
            automaton: OntologicalAutomaton::new(),
        }
    }
}
```