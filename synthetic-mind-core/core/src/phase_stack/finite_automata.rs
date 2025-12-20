```rust
/// Онтологический конечный автомат для локального поведения.
#[derive(Clone, Copy, Debug)]
pub struct OntologicalAutomaton {
    state: u8,
    rules: [u8; 256], // упрощённая таблица переходов
}

impl OntologicalAutomaton {
    pub const fn new() -> Self {
        Self { state: 0, rules: [0; 256] }
    }

    pub fn step(&mut self, input: u8) -> u8 {
        self.state = self.rules[(self.state as usize + input as usize) % 256];
        self.state
    }
}
```