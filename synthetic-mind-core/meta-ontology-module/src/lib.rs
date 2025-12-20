```rust
//! Модуль метаонтологий: синтез смысла из «пустот» в диалоге.
//! Лицензия: MPL 2.0

mod gap_detector;
mod meta_tree_builder;
mod onto16;
mod emotion_semantic_link;

pub use gap_detector::detect_gaps;
pub use meta_tree_builder::build_meta_tree;
pub use onto16::Onto16;
pub use emotion_semantic_link::link_emotion;
```