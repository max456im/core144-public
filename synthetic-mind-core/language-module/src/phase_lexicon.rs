```rust
use synthetic_mind_core::ontology::LebesgueSpace;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Phase {
    Beginning,   // начало
    Process,     // ход
    Event,       // событие
    Phenomenon,  // явление
    End,         // конец
    Undefined,
}

pub fn classify_phase(word: &str) -> Phase {
    match word.to_lowercase().as_str() {
        w if w.contains("нача") || w.contains("возник") => Phase::Beginning,
        w if w.contains("идёт") || w.contains("развива") => Phase::Process,
        w if w.contains("взрыв") || w.contains("выбор") => Phase::Event,
        w if w.contains("чудо") || w.contains("кризис") => Phase::Phenomenon,
        w if w.contains("конец") || w.contains("умер") => Phase::End,
        _ => Phase::Undefined,
    }
}

impl Phase {
    pub fn to_lebesgue(&self) -> LebesgueSpace {
        use Phase::*;
        let (phase_point, intensity, coherence) = match self {
            Beginning => (0x0001, 200, 0.8),
            Process   => (0x0002, 180, 0.7),
            Event     => (0x0003, 255, 0.9),
            Phenomenon=> (0x0004, 220, 0.85),
            End       => (0x0005, 190, 0.75),
            Undefined => (0x0000, 100, 0.3),
        };
        LebesgueSpace::new(phase_point, intensity, coherence)
    }
}
```