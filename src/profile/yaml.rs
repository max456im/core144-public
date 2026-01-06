// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{Element, Profile, ProfileError, ProfileErrorKind, Temperament};
use serde::{Deserialize, Deserializer};

// Вспомогательная структура только для десериализации
#[derive(Deserialize)]
struct YamlProfile {
    id: String,
    #[serde(deserialize_with = "parse_temperament")]
    temperament: Temperament,
    birth_year: Option<i32>,
    #[serde(default, deserialize_with = "parse_elements")]
    elements: Vec<Element>,
}

fn parse_temperament<'de, D>(deserializer: D) -> Result<Temperament, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?.to_lowercase();
    match s.as_str() {
        "choleric" => Ok(Temperament::Choleric),
        "sanguine" => Ok(Temperament::Sanguine),
        "phlegmatic" => Ok(Temperament::Phlegmatic),
        "melancholic" => Ok(Temperament::Melancholic),
        _ => Err(serde::de::Error::custom(format!(
            "unknown temperament: {}",
            s
        ))),
    }
}

fn parse_elements<'de, D>(deserializer: D) -> Result<Vec<Element>, D::Error>
where
    D: Deserializer<'de>,
{
    let list: Vec<String> = Deserialize::deserialize(deserializer)?;
    list.into_iter()
        .map(|s| match s.to_lowercase().as_str() {
            "wood" => Ok(Element::Wood),
            "fire" => Ok(Element::Fire),
            "earth" => Ok(Element::Earth),
            "metal" => Ok(Element::Metal),
            "water" => Ok(Element::Water),
            _ => Err(serde::de::Error::custom(format!(
                "unknown element: {}",
                s
            ))),
        })
        .collect()
}

pub(super) fn parse_yaml_profile(data: &[u8]) -> Result<Profile, ProfileError> {
    let parsed: YamlProfile = serde_yaml::from_slice(data)
        .map_err(|e| ProfileError {
            kind: ProfileErrorKind::Parse,
            message: e.to_string(),
        })?;

    // Можно добавить валидацию (например, непустой id)
    if parsed.id.is_empty() {
        return Err(ProfileError {
            kind: ProfileErrorKind::Validation,
            message: "profile id must not be empty".into(),
        });
    }

    Ok(Profile {
        id: parsed.id,
        temperament: parsed.temperament,
        birth_year: parsed.birth_year,
        elements: parsed.elements,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_profile() {
        let yaml = r#"
id: "test-001"
temperament: choleric
birth_year: 1985
elements: [Fire, Earth]
        "#;

        let profile = Profile::from_yaml(yaml.as_bytes()).unwrap();
        assert_eq!(profile.id, "test-001");
        assert_eq!(profile.temperament, Temperament::Choleric);
        assert_eq!(profile.birth_year, Some(1985));
        assert_eq!(profile.elements, vec![Element::Fire, Element::Earth]);
    }

    #[test]
    fn test_invalid_temperament() {
        let yaml = r#"
id: "bad"
temperament: unknown_type
        "#;
        let err = Profile::from_yaml(yaml.as_bytes()).unwrap_err();
        assert_eq!(err.kind, ProfileErrorKind::Parse);
    }
}