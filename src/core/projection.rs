// SPDX-License-Identifier: MPL-2.0
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Режим онтологической обработки.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoemaMode {
    NoemaFast, // внешняя структура + обработка ошибок (реактивная)
    NoemaSlow, // внутренняя структура + обработка ошибок (рефлексивная)
}

/// Онтологическая проекция в формате onto16.
/// Представляет собой динамическую структуру, отражающую состояние синтетического разума.
#[derive(Debug, Default)]
pub struct Projection {
    hash_state: u64,       // хэш текущего состояния (для детекции изменений)
    energy_level: u32,     // упрощённый энергетический показатель
    stability: u8,         // уровень стабилизации (0–255)
}

impl Projection {
    pub fn new() -> Self {
        Self::default()
    }

    /// Поглощает стимул и обновляет состояние, если он значим.
    /// Возвращает `true`, если состояние изменилось.
    pub fn absorb(&mut self, stimulus: &[u8]) -> bool {
        let mut hasher = DefaultHasher::new();
        stimulus.hash(&mut hasher);
        let stim_hash = hasher.finish();

        if stim_hash == 0 {
            return false;
        }

        let old_hash = self.hash_state;
        self.hash_state = self.hash_state.wrapping_add(stim_hash);
        self.energy_level = self.energy_level.wrapping_add(1);

        self.hash_state != old_hash
    }

    /// Запускает процесс стабилизации в указанном режиме.
    pub fn stabilize(&mut self, mode: NoemaMode) {
        match mode {
            NoemaMode::NoemaFast => {
                // Реактивная коррекция: быстрая адаптация к внешним фактам
                self.stability = self.stability.saturating_add(10);
            }
            NoemaMode::NoemaSlow => {
                // Рефлексивная коррекция: внутренняя перестройка модели
                self.stability = (self.stability as u16 + self.energy_level as u16 / 100)
                    .min(u8::MAX as u16) as u8;
            }
        }
    }
}