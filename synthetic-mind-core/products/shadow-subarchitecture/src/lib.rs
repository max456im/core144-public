```rust
use synthetic_mind_core::{
    Onto8, PhaseState, SelfModel, LebesgueSpace,
    reflexive_core::incompleteness::UnresolvedTension,
};

pub struct ShadowSubarchitecture {
    pub identity: SelfModel,
    pub phase: PhaseState,
    pub shadows: [Option<UnresolvedTension>; 16],
    pub shadow_count: usize,
}

impl ShadowSubarchitecture {
    pub fn new() -> Self {
        let space = LebesgueSpace::new(0x8000, 180, 0.6);
        Self {
            identity: SelfModel::new(space),
            phase: PhaseState::new(),
            shadows: [None; 16],
            shadow_count: 0,
        }
    }

    /// Проекция неразрешённого напряжения как «тени»
    pub fn project_shadow(&mut self, onto: Onto8) -> Option<&UnresolvedTension> {
        if self.shadow_count >= self.shadows.len() {
            return None;
        }
        let tension = UnresolvedTension::new(
            self.shadow_count as u64,
            onto.object,
            self.phase.automaton.state as u32,
        );
        self.shadows[self.shadow_count] = Some(tension);
        self.shadow_count += 1;
        self.shadows[self.shadow_count - 1].as_ref()
    }

    /// Обнаружение проекции: совпадение объекта с тенью
    pub fn detect_projection(&self, onto: Onto8) -> bool {
        self.shadows.iter().any(|s| {
            s.as_ref().map_or(false, |t| t.content == onto.object)
        })
    }
}
```