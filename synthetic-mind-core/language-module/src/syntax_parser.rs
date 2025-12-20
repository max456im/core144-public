```rust
#[derive(Clone, Debug)]
pub struct SubjectPredicate {
    pub subject: String,   // подлежащее (тема)
    pub predicate: String, // сказуемое (рема)
}

pub fn extract_subject_predicate(sentence: &str) -> SubjectPredicate {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    if words.len() < 2 {
        return SubjectPredicate {
            subject: sentence.to_string(),
            predicate: "".to_string(),
        };
    }
    // Простая эвристика: первое слово — подлежащее
    SubjectPredicate {
        subject: words[0].to_string(),
        predicate: words[1..].join(" "),
    }
}
```