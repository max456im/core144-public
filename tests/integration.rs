// SPDX-License-Identifier: MPL-2.0
// Copyright (c) 2026 Maksim Zapevalov. All rights reserved.

use core144_public::{
    core::{Core144, Projection},
    profile::Profile,
};

const MOCK_PROFILE_YAML: &[u8] = br#"
name: "TestAgent"
temperament: "choleric"
birth_year: 1985
ontotype: "onto16"
"#;

const MOCK_STIMULUS: &str = "User initiated query about licensing compatibility.";

#[test]
fn test_profile_loading() {
    let profile = Profile::from_yaml(MOCK_PROFILE_YAML).expect("Failed to parse mock profile");
    assert_eq!(profile.name, "TestAgent");
    assert_eq!(profile.temperament, "choleric");
    assert_eq!(profile.birth_year, 1985);
}

#[test]
fn test_core_initialization() {
    let profile = Profile::from_yaml(MOCK_PROFILE_YAML).expect("Profile load failed");
    let core = Core144::new(profile);
    assert!(core.is_ready());
}

#[test]
fn test_projection_generation() {
    let profile = Profile::from_yaml(MOCK_PROFILE_YAML).expect("Profile load failed");
    let mut core = Core144::new(profile);
    let projection = core.process(MOCK_STIMULUS).expect("Processing failed");

    // Проверка структуры onto16-проекции
    match projection {
        Projection::NoemaFast(ref fast) => {
            assert!(fast.appearance.is_some());
            assert!(fast.reality.is_none()); // NoemaFast — prospective only
        }
        Projection::NoemaSlow(ref slow) => {
            assert!(slow.reality.is_some());
            assert!(slow.appearance.is_some());
        }
    }

    // Генерация проекции не должна паниковать
    let _ = core.generate_projection();
}

#[test]
fn test_idempotent_processing() {
    let profile = Profile::from_yaml(MOCK_PROFILE_YAML).expect("Profile load failed");
    let mut core1 = Core144::new(profile.clone());
    let mut core2 = Core144::new(profile);

    let proj1 = core1.process(MOCK_STIMULUS).expect("First processing failed");
    let proj2 = core2.process(MOCK_STIMULUS).expect("Second processing failed");

    // Для детерминированных стимулов и профилей — одинаковый результат
    assert_eq!(proj1, proj2);
}