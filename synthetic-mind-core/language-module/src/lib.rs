```rust
//! Языковой модуль: фазово-семантическая обработка текста.
//! Лицензия: MPL 2.0

mod phase_lexicon;
mod syntax_parser;
mod modality_marker;
mod linguistic_invariants;

pub use phase_lexicon::{Phase, classify_phase};
pub use syntax_parser::{SubjectPredicate, extract_subject_predicate};
pub use modality_marker::{Modality, detect_modality};
pub use linguistic_invariants::{LinguisticInvariant, as_onto8};
```