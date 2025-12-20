```rust
use synthetic_mind_core::Onto8;
use shadow_subarchitecture::ShadowSubarchitecture;

fn main() {
    let mut shadow = ShadowSubarchitecture::new();
    let onto = Onto8::new(0xDEADBEEF, 0x00000000);
    shadow.project_shadow(onto);
    println!("Projection detected: {}", shadow.detect_projection(onto));
}
```