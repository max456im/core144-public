// SPDX-License-Identifier: MPL-2.0
use super::projection::{Projection, NoemaMode};

/// Основной синтетический движок core144.
/// Инкапсулирует состояние проекции и управляет обработкой стимулов.
pub struct Core144 {
    projection: Projection,
}

impl Core144 {
    /// Создаёт новый экземпляр движка с пустой onto16-проекцией.
    pub fn new() -> Self {
        Self {
            projection: Projection::new(),
        }
    }

    /// Обрабатывает входной стимул и обновляет внутреннюю проекцию.
    /// Возвращает `true`, если проекция изменилась (активация произошла).
    pub fn process(&mut self, stimulus: &[u8]) -> bool {
        // В упрощённой реализации стимул трактуется как сырые байты,
        // которые могут влиять на энергетическое состояние проекции.
        // В полной версии здесь была бы интерпретация через лексикон и онтологию.
        let changed = self.projection.absorb(stimulus);
        if changed {
            self.projection.stabilize(NoemaMode::NoemaFast);
        }
        changed
    }

    /// Генерирует текущую onto16-проекцию как сериализуемый вектор байтов.
    /// Может использоваться для передачи состояния, логирования или отладки.
    pub fn generate_projection(&self) -> Vec<u8> {
        // В реальной системе это был бы сериализованный onto16-формат.
        // Здесь — заглушка для демонстрации.
        format!("{:?}", self.projection).into_bytes()
    }
}