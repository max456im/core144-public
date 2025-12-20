```rust
use synthetic_mind_core::Onto8;
use technical_evangelist::TechnicalEvangelist;

fn main() {
    let mut evang = TechnicalEvangelist::new(Onto8::new(0x100, 0x200));
    evang.observe_interaction(
        Onto8::new(0x100, 0x200),
        Onto8::new(0x300, 0x400),
        Onto8::new(0x500, 0x600),
    );
    if let Some(rule) = evang.infer_rule() {
        println!("Emergent rule: strength = {}", rule.strength);
    }
    println!("Accent weight: {:.2}", evang.get_current_accent().weight);
}
```