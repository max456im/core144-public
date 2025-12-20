```markdown
# Валидация когерентности

## Принцип

Когерентность группы вычисляется как:

```
coherence = avg(|projection - background|) / MAX
```

- Значение **> 0.7** → стабильная группа.
- Значение **< 0.4** → распад, искажение, «тень».

## Валидация

Модуль `validation/coherence.rs`:

```rust
fn verify_coherence(group: &OnticGroup) -> ValidationResult {
    if group.coherence.is_coherent() {
        ValidationResult::Pass
    } else {
        ValidationResult::Fail("Low coherence".to_string())
    }
}
```

## Диагностика

В продуктах:

```rust
if cfg!(feature = "diagnostics") {
    validation::coherence::verify_coherence(&group);
}
```
```