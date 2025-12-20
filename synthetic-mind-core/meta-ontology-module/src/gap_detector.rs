```rust
/// Обнаружение «пустот» — мест, где логика прерывается, но смысл продолжается.
pub fn detect_gaps(dialogue: &str) -> Vec<usize> {
    let mut gaps = Vec::new();
    let sentences: Vec<&str> = dialogue.split('.').collect();
    for (i, s) in sentences.iter().enumerate() {
        if s.trim().ends_with("но") || s.trim().ends_with("однако") || s.is_empty() {
            gaps.push(i);
        }
    }
    gaps
}
```