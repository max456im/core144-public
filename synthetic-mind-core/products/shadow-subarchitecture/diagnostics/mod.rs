```rust
use crate::ShadowSubarchitecture;

pub fn run_full_diagnostic(shadow: &ShadowSubarchitecture) {
    println!("=== DIAGNOSTIC: Shadow Subarchitecture ===");
    println!("Active shadows: {}", shadow.shadow_count);
    for (i, s) in shadow.shadows.iter().enumerate() {
        if let Some(t) = s {
            println!("  Shadow[{}]: content=0x{:08X}, phase={}", i, t.content, t.phase_at_creation);
        }
    }
}
```