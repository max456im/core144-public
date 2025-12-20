```rust
#[derive(Clone, Copy, Debug)]
pub enum Modality {
    Permission,     // можно
    Prohibition,    // нельзя
    Obligation,     // должен
    Impossibility,  // невозможно
    Neutral,
}

pub fn detect_modality(text: &str) -> Option<Modality> {
    let lower = text.to_lowercase();
    if lower.contains("нельзя") || lower.contains("запрещено") {
        Some(Modality::Prohibition)
    } else if lower.contains("должен") || lower.contains("обязан") {
        Some(Modality::Obligation)
    } else if lower.contains("можно") || lower.contains("разрешено") {
        Some(Modality::Permission)
    } else if lower.contains("невозможно") {
        Some(Modality::Impossibility)
    } else {
        None
    }
}
```