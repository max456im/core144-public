```rust
use std::collections::HashMap;

use crate::ipc::{IpcClient, IpcRequest, IpcResponse};

/// Загрузчик и менеджер GPL-плагинов (CLI-процессов)
pub struct GplPluginLoader {
    plugins: HashMap<String, IpcClient>,
}

impl GplPluginLoader {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }

    /// Регистрирует плагин по имени и пути к исполняемому файлу
    pub fn register_plugin(&mut self, name: &str, executable_path: &str) {
        self.plugins.insert(name.to_string(), IpcClient::new(executable_path));
    }

    /// Вызывает зарегистрированный плагин с запросом
    pub fn call_plugin(
        &self,
        plugin_name: &str,
        request: &IpcRequest,
    ) -> Result<IpcResponse, Box<dyn std::error::Error>> {
        self.plugins
            .get(plugin_name)
            .ok_or_else(|| format!("Plugin '{}' not registered", plugin_name))?
            .send_request(request)
    }

    /// Проверяет, зарегистрирован ли плагин
    pub fn is_plugin_available(&self, name: &str) -> bool {
        self.plugins.contains_key(name)
    }

    /// Получает список всех зарегистрированных плагинов
    pub fn list_plugins(&self) -> Vec<&String> {
        self.plugins.keys().collect()
    }
}
```