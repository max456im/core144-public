// Copyright (c) 2026 Maksim Zapevalov
// Licensed under the MPL-2.0 License. See LICENSE for details.

use crate::profile;

/// Ошибка при работе с профилем.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileError(String);

impl std::fmt::Display for ProfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Profile error: {}", self.0)
    }
}

impl std::error::Error for ProfileError {}

/// Структура профиля синтетического разума.
///
/// Не содержит внутреннего состояния ядра — только идентификационные,
/// темпераментные и онтологические параметры.
#[derive(Debug, Clone)]
pub struct Profile(profile::Profile);

impl Profile {
    /// Загружает профиль из байтового представления YAML.
    ///
    /// Возвращает `Err` при синтаксической или семантической ошибке.
    pub fn from_yaml(data: &[u8]) -> Result<Self, ProfileError> {
        profile::Profile::from_yaml(data)
            .map(Self)
            .map_err(|e| ProfileError(e.to_string()))
    }

    /// Возвращает идентификатор профиля.
    pub fn id(&self) -> &str {
        self.0.id()
    }

    /// Возвращает темперамент (например, "choleric").
    pub fn temperament(&self) -> &str {
        self.0.temperament()
    }
}