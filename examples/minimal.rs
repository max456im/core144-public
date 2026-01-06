// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Maksim Zapevalov. All rights reserved.

//! Minimal example: load a YAML profile, process a stimulus, and generate an onto16 projection.

use core144_public::{Profile, Core144};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Загружаем профиль из встроенных данных (без файловой системы — для WASM-совместимости)
    // В реальном приложении это может быть загружено через API или конфиг.
    let profile_yaml = br#"
name: "TestMind"
temperament: "choleric"
birth_year: 1985
elements:
  - fire
  - metal
traits:
  proactive: true
  reflective: false
"#;

    let profile = Profile::from_yaml(profile_yaml)?;
    println!("✅ Profile loaded: {}", profile.name());

    // 2. Создаём ядро synthetic mind
    let mut core = Core144::new(profile);

    // 3. Подаём стимул (например, текстовый импульс)
    let stimulus = "How should I respond to novelty without prior ontological grounding?";
    
    // 4. Обрабатываем стимул и генерируем onto16-проекцию
    core.process(stimulus)?;
    let projection = core.generate_projection()?;

    // 5. Выводим проекцию (NoemaFast/NoemaSlow-совместимый формат)
    println!("\n🧠 Onto16 Projection:\n{}", serde_json::to_string_pretty(&projection)?);

    Ok(())
}