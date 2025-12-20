```rust
//! Расширенные инварианты: проекция, реконструкция, искажение
//! Лицензия: GPL v3

use interfaces::{Onto8, PhaseEvent};

#[derive(Clone, Debug)]
pub struct DistortedInvariant {
    pub original: Onto8,
    pub projected: Onto8,
    pub distortion_type: DistortionType,
}

#[derive(Clone, Copy, Debug)]
pub enum DistortionType {
    Projection,
    AllInOne,
    TemporalCollapse,
}

/// Генерация искажённого инварианта на основе фазового напряжения
pub fn distort_invariant(event: &PhaseEvent) -> DistortedInvariant {
    let projected = if event.coherence < 0.4 {
        Onto8::new(event.content.object, event.content.object) // фон = объект → "всё-в-одном"
    } else {
        event.content
    };
    DistortedInvariant {
        original: event.content,
        projected,
        distortion_type: if event.coherence < 0.4 {
            DistortionType::AllInOne
        } else {
            DistortionType::Projection
        },
    }
}
```