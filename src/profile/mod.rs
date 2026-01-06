// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod yaml;

pub use yaml::Profile;

/// Основной тип профиля синтетического разума.
/// Представляет идентичность, темперамент и онтологические параметры.
#[derive(Debug, Clone, PartialEq)]
pub struct Profile {
    pub id: String,
    pub temperament: Temperament,
    pub birth_year: Option<i32>,
    pub elements: Vec<Element>,
    // Дополнительные поля могут быть добавлены без нарушения совместимости
}

#[derive(Debug, Clone, PartialEq)]
pub enum Temperament {
    Choleric,
    Sanguine,
    Phlegmatic,
    Melancholic,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Wood,
    Fire,
    Earth,
    Metal,
    Water,
}

impl Profile {
    /// Создаёт профиль из YAML-данных в виде байтового среза.
    ///
    /// Возвращает `Err` при синтаксической ошибке или отсутствии обязательных полей.
    pub fn from_yaml(data: &[u8]) -> Result<Self, ProfileError> {
        yaml::parse_yaml_profile(data)
    }
}

/// Обобщённая ошибка при загрузке профиля.
#[derive(Debug)]
pub struct ProfileError {
    pub kind: ProfileErrorKind,
    pub message: String,
}

#[derive(Debug, PartialEq)]
pub enum ProfileErrorKind {
    Parse,
    Validation,
    Io,
}

impl std::fmt::Display for ProfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.kind, self.message)
    }
}

impl std::error::Error for ProfileError {}

impl std::fmt::Display for ProfileErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfileErrorKind::Parse => write!(f, "ParseError"),
            ProfileErrorKind::Validation => write!(f, "ValidationError"),
            ProfileErrorKind::Io => write!(f, "IoError"),
        }
    }
}