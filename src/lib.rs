// core144-public/src/lib.rs
// SPDX-License-Identifier: MPL-2.0
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! `core144-public` — библиотека ядра синтетического разума, реализующая
//! onto16-проекцию через движок Core144 с поддержкой внешних профилей и
//! интеграции в GPL-совместимые системы.
//!
//! Основные компоненты:
//! - [`Profile`] — онтологический профиль (YAML → onto16)
//! - [`Core144`] — движок обработки стимулов и генерации проекций
//! - Публичный API: [`api`]

// Стабильность и совместимость
#![no_std]
#![deny(missing_docs, unsafe_code, clippy::pedantic)]
// Включение std только при необходимости (WASM/native)
#[cfg(feature = "std")]
extern crate std;

// Модули
pub mod profile;
pub mod core;
pub mod lexicon;
pub mod api;

// Публичные переэкспорты для удобства использования
pub use profile::Profile;
pub use core::Core144;