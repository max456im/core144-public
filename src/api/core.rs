// Copyright (c) 2026 Maksim Zapevalov
// Licensed under the MPL-2.0 License. See LICENSE for details.

use crate::{core, profile};

use super::Profile;

/// Ошибка при работе с ядром.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreError(String);

impl std::fmt::Display for CoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Core error: {}", self.0)
    }
}

impl std::error::Error for CoreError {}

/// Результат проецирования onto16.
///
/// Содержит как быструю (NoemaFast), так и медленную (NoemaSlow) компоненты,
/// представляющие внешнюю и внутреннюю онтологическую реакцию.
#[derive(Debug, Clone)]
pub struct Projection {
    /// Внешняя проекция (NoemaFast): реакция на стимул через социальный интерфейс.
    pub noema_fast: String,
    /// Внутренняя проекция (NoemaSlow): переработка через внутренние модели.
    pub noema_slow: String,
}

/// Основное ядро синтетического разума (Core144).
///
/// Не зависит от пользователя: идентичность задаётся через `Profile`.
pub struct Core144(core::Core144);

impl Core144 {
    /// Создаёт новое ядро с заданным профилем.
    pub fn new(profile: Profile) -> Self {
        Self(core::Core144::new(profile.0))
    }

    /// Обрабатывает стимул и возвращает онтологическую проекцию onto16.
    ///
    /// Стимул — это UTF-8 строка, представляющая внешнее воздействие
    /// (сообщение, событие, запрос и т.п.).
    pub fn process(&mut self, stimulus: &str) -> Result<Projection, CoreError> {
        self.0
            .process(stimulus)
            .map(|p| Projection {
                noema_fast: p.noema_fast,
                noema_slow: p.noema_slow,
            })
            .map_err(|e| CoreError(e.to_string()))
    }

    /// Генерирует пустую проекцию (например, при инициализации).
    pub fn generate_projection(&self) -> Projection {
        let p = self.0.generate_projection();
        Projection {
            noema_fast: p.noema_fast,
            noema_slow: p.noema_slow,
        }
    }
}