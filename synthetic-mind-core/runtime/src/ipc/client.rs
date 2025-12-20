```rust
use std::process::Command;
use synthetic_mind_interfaces::{
    Onto8, Onto16, PhaseEvent, UnresolvedTension, OntoTriplet, ModalityMarker,
};
use serde::{Deserialize, Serialize};

/// Запрос к GPL-сервису
#[derive(Serialize, Deserialize, Debug)]
pub struct IpcRequest {
    pub command: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onto8: Option<Onto8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onto16: Option<Onto16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_event: Option<PhaseEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tension: Option<UnresolvedTension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triplet: Option<OntoTriplet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modality: Option<ModalityMarker>,
}

/// Ответ от GPL-сервиса
#[derive(Deserialize, Serialize, Debug)]
pub struct IpcResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onto8: Option<Onto8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onto16: Option<Onto16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_f32: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_u32: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_bool: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_string: Option<String>,
}

impl Default for IpcResponse {
    fn default() -> Self {
        Self {
            success: false,
            message: String::new(),
            onto8: None,
            onto16: None,
            result_f32: None,
            result_u32: None,
            result_bool: None,
            result_string: None,
        }
    }
}

impl IpcResponse {
    pub fn error(msg: &str) -> Self {
        Self {
            success: false,
            message: msg.to_string(),
            ..Default::default()
        }
    }
}

/// Клиент для отправки запросов GPL-CLI
pub struct IpcClient {
    pub executable_path: String,
}

impl IpcClient {
    pub fn new(executable_path: &str) -> Self {
        Self {
            executable_path: executable_path.to_string(),
        }
    }

    pub fn send_request(&self, request: &IpcRequest) -> Result<IpcResponse, Box<dyn std::error::Error>> {
        let json = serde_json::to_string(request)?;
        let output = Command::new(&self.executable_path)
            .arg("--request")
            .arg(json)
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("GPL process failed: {}", stderr).into());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let response: IpcResponse = serde_json::from_str(&stdout)?;
        Ok(response)
    }
}
```