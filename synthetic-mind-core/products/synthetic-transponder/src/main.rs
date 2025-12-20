```rust
use synthetic_transponder::SyntheticTransponder;

fn main() {
    let mut transponder = SyntheticTransponder::new();
    let input = "Он сказал, что уйдёт. Но не ушёл.";
    let meta_tree = transponder.ingest_dialogue(input);
    for onto16 in meta_tree {
        transponder.project_scene(onto16);
    }
    #[cfg(feature = "diagnostics")]
    diagnostics::run_full_diagnostic(&transponder);
}
```