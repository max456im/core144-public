```rust
//! Онтологически нейтральные интерфейсы для межмодульного взаимодействия.
//! Используются ядром (Apache), языковым и метаонтологическим (MPL),
//! и GPL-модулями.
//!
//! Лицензия: Apache 2.0
//! Требования:
//! - no_std
//! - без alloc
//! - без внешних зависимостей
//! - только сериализуемые, копируемые структуры

#![no_std]

/// Универсальный 8-байтный инвариант: 4 байта объект + 4 байта фон.
/// Основа всех мультимодальных проекций.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Onto8 {
    pub object: u32,
    pub background: u32,
}

impl Onto8 {
    /// Создаёт инвариант из двух 32-битных компонент.
    pub const fn new(object: u32, background: u32) -> Self {
        Self { object, background }
    }

    /// Хэш-подобное представление для быстрого сравнения.
    pub const fn hash(&self) -> u64 {
        ((self.object as u64) << 32) | (self.background as u64)
    }
}

/// Расширенная онтологическая структура: 2 × Onto8.
/// Используется для представления лингвистико-эмоциональных пар.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Onto16 {
    pub linguistic: Onto8,
    pub emotional: Onto8,
}

impl Onto16 {
    pub const fn new(linguistic: Onto8, emotional: Onto8) -> Self {
        Self { linguistic, emotional }
    }
}

/// Событие фазового перехода.
/// Передаётся между ядром, продуктами и GPL-модулями.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct PhaseEvent {
    /// Уникальный идентификатор фазы (например, хэш фазовой точки в LebesgueSpace)
    pub phase_id: u32,
    /// Уровень когерентности: [-1.0, 1.0]
    pub coherence: f32,
    /// Контент события — минимальный инвариант
    pub content: Onto8,
}

impl PhaseEvent {
    pub const fn new(phase_id: u32, coherence: f32, content: Onto8) -> Self {
        Self {
            phase_id,
            coherence,
            content,
        }
    }
}

/// Тип напряжения в рефлексивной петле «Я–я».
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TensionType {
    IdentityConflict = 0,
    ValueViolation = 1,
    TemporalDissonance = 2,
    UnresolvedProjection = 3,
}

/// Неразрешённое напряжение как «тень».
/// Передаётся в диагностику и GPL-модули для анализа проекций.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct UnresolvedTension {
    pub id: u64,
    pub content: u32,               // хэш или объект из Onto8
    pub phase_at_creation: u32,
    pub tension_type: TensionType,
}

impl UnresolvedTension {
    pub const fn new(id: u64, content: u32, phase: u32, typ: TensionType) -> Self {
        Self {
            id,
            content,
            phase_at_creation: phase,
            tension_type: typ,
        }
    }
}

/// Метка модальности (для языкового модуля → GPL-анализа)
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ModalityMarker {
    Permission = 0,     // можно
    Prohibition = 1,    // нельзя
    Obligation = 2,     // должен
    Impossibility = 3,  // невозможно
    Neutral = 4,
}

/// Обобщённый онтологический триплет: три инварианта для моделирования взаимодействий.
/// Используется в `social-rules-emergence`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct OntoTriplet {
    pub trigger: Onto8,
    pub reaction: Onto8,
    pub expectation: Onto8,
}

impl OntoTriplet {
    pub const fn new(trigger: Onto8, reaction: Onto8, expectation: Onto8) -> Self {
        Self {
            trigger,
            reaction,
            expectation,
        }
    }
}
```