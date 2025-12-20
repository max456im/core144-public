```rust
use synthetic_mind_core::multimodal_invariants::Onto8;

#[derive(Clone, Debug)]
pub struct LinguisticInvariant {
    pub subject_hash: u32,
    pub predicate_hash: u32,
}

impl LinguisticInvariant {
    pub fn from_text(text: &str) -> Self {
        let words: Vec<u32> = text
            .split_whitespace()
            .map(|w| {
                let mut hash = 0u32;
                for b in w.bytes() {
                    hash = hash.wrapping_mul(31).wrapping_add(b as u32);
                }
                hash
            })
            .collect();
        let subject = words.first().copied().unwrap_or(0);
        let predicate = words.get(1..).unwrap_or(&[]).iter().fold(0u32, |a, &b| a ^ b);
        Self {
            subject_hash: subject,
            predicate_hash: predicate,
        }
    }
}

pub fn as_onto8(inv: &LinguisticInvariant) -> Onto8 {
    Onto8::new(inv.subject_hash, inv.predicate_hash)
}
```