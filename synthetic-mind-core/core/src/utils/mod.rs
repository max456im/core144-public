```rust
// Только истинные утилиты: битовые операции, хэши и т.д.
pub const fn rotate_left(val: u32, shift: u32) -> u32 {
    val.rotate_left(shift)
}
```