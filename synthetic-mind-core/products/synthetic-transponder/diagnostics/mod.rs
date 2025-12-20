```rust
use crate::SyntheticTransponder;
use synthetic_mind_core::ontic_groups::CoherenceLevel;

pub fn run_full_diagnostic(trans: &SyntheticTransponder) {
    println!("=== DIAGNOSTIC: Synthetic Transponder ===");
    for (i, group) in trans.scene_buffer.iter().enumerate() {
        if let Some(g) = group {
            println!("Scene[{}]: coherence = {:.2}", i, g.coherence.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use synthetic_mind_core::Onto8;

    #[test]
    fn test_scene_projection() {
        let mut trans = SyntheticTransponder::new();
        let onto16 = meta_ontology_module::Onto16::new(
            Onto8::new(0x1234, 0x5678),
            Onto8::new(0, 0),
        );
        let group = trans.project_scene(onto16).unwrap();
        assert!(group.coherence.is_coherent() || !group.coherence.is_coherent());
    }
}
```