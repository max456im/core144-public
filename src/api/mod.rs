// Copyright (c) 2026 Maksim Zapevalov
// Licensed under the MPL-2.0 License. See LICENSE for details.

//! Публичный API библиотеки core144-public.
//!
//! Этот модуль предоставляет стабильные, документированные интерфейсы
//! для интеграции синтетического ядра в внешние системы,
//! включая WASM и нативные окружения.
//!
//! Основные компоненты:
//! - `Profile` — загрузка и валидация профиля из YAML
//! - `Core144` — инициализация ядра и обработка стимулов
//! - `Projection` — результат проецирования onto16 (NoemaFast/NoemaSlow)

mod profile;
mod core;

pub use profile::{Profile, ProfileError};
pub use core::{Core144, Projection, CoreError};