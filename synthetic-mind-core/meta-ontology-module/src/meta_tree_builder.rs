```rust
use crate::{Onto16, detect_gaps, link_emotion};
use language_module::{extract_subject_predicate, as_onto8};

#[derive(Clone, Debug)]
pub struct MetaTreeNode {
    pub onto16: Onto16,
    pub children: Vec<MetaTreeNode>,
}

pub fn build_meta_tree(dialogue: &str) -> Vec<MetaTreeNode> {
    let gaps = detect_gaps(dialogue);
    let sentences: Vec<&str> = dialogue.split('.').collect();
    let mut tree = Vec::new();

    for (i, sent) in sentences.iter().enumerate() {
        if gaps.contains(&i) {
            // Пропускаем «пустоту», но сохраняем как узел с эмоцией
            let emotion = link_emotion(sent);
            let onto16 = Onto16::new(Onto8::new(0, 0), emotion);
            tree.push(MetaTreeNode {
                onto16,
                children: Vec::new(),
            });
        } else {
            let sp = extract_subject_predicate(sent);
            let ling = as_onto8(&language_module::LinguisticInvariant::from_text(sent));
            let emotion = link_emotion(sent);
            let onto16 = Onto16::new(ling, emotion);
            tree.push(MetaTreeNode {
                onto16,
                children: Vec::new(),
            });
        }
    }
    tree
}
```