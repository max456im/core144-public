```rust
use crate::{ontology::LebesgueSpace, phase_stack::PhaseState, SignificanceMetric};

/// Интеграция внутреннего времени во фазовый стек.
pub fn integrate_time(
    metric: &mut SignificanceMetric,
    phase: &PhaseState,
    space: &LebesgueSpace,
) {
    let delta = space.measure() * match phase.attention {
        crate::phase_stack::AttentionState::Focused(x) => x,
        crate::phase_stack::AttentionState::Scattered(x) => 1.0 - x,
    };
    metric.update(delta);
}
```