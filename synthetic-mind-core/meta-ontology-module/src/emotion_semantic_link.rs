```rust
use synthetic_mind_core::multimodal_invariants::Onto8;

// Упрощённый эмоциональный корпус: слово → эмоциональный хэш
const EMOTIONAL_CORPUS: &[(&str, u32)] = &[
    ("страх", 0x1A2B3C4D),
    ("радость", 0x5E6F7A8B),
    ("гнев", 0x9C0D1E2F),
    ("печаль", 0x3A4B5C6D),
];

pub fn link_emotion(text: &str) -> Onto8 {
    let lower = text.to_lowercase();
    for &(word, hash) in EMOTIONAL_CORPUS {
        if lower.contains(word) {
            return Onto8::new(hash, 0xFFFFFFFF); // фон — «максимальная эмоция»
        }
    }
    Onto8::new(0, 0) // нейтрально
}
```