```rust
use std::env;
use synthetic_mind_interfaces::{
    Onto8, Onto16, PhaseEvent, UnresolvedTension, OntoTriplet, ModalityMarker,
};
use serde::{Deserialize, Serialize};

/// Структура запроса — дублируется для изоляции (не зависит от client)
#[derive(Deserialize, Serialize)]
pub struct IpcRequest {
    pub command: String,
    pub onto8: Option<Onto8>,
    pub onto16: Option<Onto16>,
    pub phase_event: Option<PhaseEvent>,
    pub tension: Option<UnresolvedTension>,
    pub triplet: Option<OntoTriplet>,
    pub modality: Option<ModalityMarker>,
}

/// Структура ответа — дублируется
#[derive(Deserialize, Serialize)]
pub struct IpcResponse {
    pub success: bool,
    pub message: String,
    pub onto8: Option<Onto8>,
    pub onto16: Option<Onto16>,
    pub result_f32: Option<f32>,
    pub result_u32: Option<u32>,
    pub result_bool: Option<bool>,
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

/// Запускает серверную часть — обрабатывает один запрос и завершается.
/// Используется в CLI-обёртках GPL-модулей.
pub fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "--request" {
        eprintln!("Usage: {} --request '<json>'", args[0]);
        std::process::exit(1);
    }

    let json_request = &args[2];
    let request: IpcRequest = serde_json::from_str(json_request)?;

    let response = handle_request(request);
    let json_response = serde_json::to_string(&response)?;
    println!("{}", json_response);

    Ok(())
}

/// Обрабатывает запрос — здесь можно подключать реальную GPL-логику.
fn handle_request(req: IpcRequest) -> IpcResponse {
    match req.command.as_str() {
        // invariant-extended
        "distort_invariant" => {
            if let Some(event) = req.phase_event {
                let distorted = event.coherence < 0.4 || event.content.object == event.content.background;
                IpcResponse {
                    success: true,
                    message: "Distortion analyzed".to_string(),
                    result_bool: Some(distorted),
                    ..Default::default()
                }
            } else {
                IpcResponse::error("Missing phase_event for distort_invariant")
            }
        }

        // context-accents
        "compute_accent" => {
            if let Some(onto) = req.onto8 {
                let weight = (onto.object as f32 / u32::MAX as f32) * 2.0 - 1.0;
                IpcResponse {
                    success: true,
                    message: "Accent computed".to_string(),
                    result_f32: Some(weight),
                    ..Default::default()
                }
            } else {
                IpcResponse::error("Missing onto8 for compute_accent")
            }
        }

        // specific-certainty
        "assess_certainty" => {
            if let Some(onto) = req.onto8 {
                let specific = (onto.object as f32 / u32::MAX as f32).clamp(0.0, 1.0);
                IpcResponse {
                    success: true,
                    message: "Certainty assessed".to_string(),
                    result_f32: Some(specific),
                    ..Default::default()
                }
            } else {
                IpcResponse::error("Missing onto8 for assess_certainty")
            }
        }

        // shadow-subarchitecture (вызывается из Apache-продукта!)
        "detect_projection" => {
            if let (Some(tension), Some(onto)) = (req.tension, req.onto8) {
                let detected = tension.content == onto.object;
                IpcResponse {
                    success: true,
                    message: "Projection detection complete".to_string(),
                    result_bool: Some(detected),
                    ..Default::default()
                }
            } else {
                IpcResponse::error("Missing tension or onto8 for detect_projection")
            }
        }

        // social-rules-emergence
        "extract_social_rule" => {
            if let Some(triplet) = req.triplet {
                // Простая эвристика: правило сильнее, если фон совпадает
                let strength = if triplet.trigger.background == triplet.reaction.background {
                    255u32
                } else {
                    128u32
                };
                IpcResponse {
                    success: true,
                    message: "Social rule extracted".to_string(),
                    result_u32: Some(strength),
                    ..Default::default()
                }
            } else {
                IpcResponse::error("Missing triplet for extract_social_rule")
            }
        }

        _ => IpcResponse::error(&format!("Unknown command: {}", req.command)),
    }
}
```