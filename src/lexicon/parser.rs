//! Простой текстовый парсер для лексикона в формате:
//!
//!   token = "category:subcategory" [weight: f32]
//!
//! Пример:
//!   love = "affect:positive" weight: 0.85
//!   fire = "element:active"
//!
//! Вес по умолчанию — 1.0.
//!
//! Цель: создать легковесное отображение токен → (категория, вес),
//! пригодное для онтологической проекции в onto16.

use std::collections::HashMap;

/// Представление записи лексикона.
#[derive(Debug, Clone)]
pub struct LexiconEntry {
    pub category: String,
    pub weight: f32,
}

/// Парсит байтовый срез как текстовый лексикон.
///
/// Игнорирует пустые строки и строки, начинающиеся с `#`.
/// Возвращает `HashMap<token, LexiconEntry>`.
///
/// Не использует регулярные выражения — только простые split/trim.
pub fn parse_lexicon(data: &[u8]) -> Result<HashMap<String, LexiconEntry>, LexiconParseError> {
    let input = std::str::from_utf8(data)
        .map_err(|e| LexiconParseError::Encoding(e.to_string()))?;

    let mut lexicon = HashMap::new();

    for (line_no, line) in input.lines().enumerate() {
        let line = line.trim();

        // Пропуск комментариев и пустых строк
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Ожидаем: token = "category" [weight: N]
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err(LexiconParseError::Syntax(line_no + 1, line.to_string()));
        }

        let token = parts[0].trim().to_string();
        let mut rhs = parts[1].trim();

        // Извлекаем категорию в кавычках
        if !rhs.starts_with('"') {
            return Err(LexiconParseError::Syntax(line_no + 1, line.to_string()));
        }
        let end_quote = rhs[1..].find('"')
            .ok_or_else(|| LexiconParseError::Syntax(line_no + 1, line.to_string()))? + 1;
        let category = rhs[1..end_quote].to_string();
        rhs = rhs[end_quote + 1..].trim();

        // Извлекаем опциональный вес
        let weight = if rhs.starts_with("weight:") {
            let weight_str = rhs["weight:".len()..].trim();
            weight_str.parse::<f32>()
                .map_err(|_| LexiconParseError::InvalidWeight(line_no + 1, weight_str.to_string()))?
        } else {
            1.0
        };

        lexicon.insert(token, LexiconEntry { category, weight });
    }

    Ok(lexicon)
}

/// Ошибки парсинга лексикона.
#[derive(Debug)]
pub enum LexiconParseError {
    Encoding(String),
    Syntax(usize, String), // line number, line content
    InvalidWeight(usize, String),
}

impl std::fmt::Display for LexiconParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexiconParseError::Encoding(e) => write!(f, "UTF-8 decoding failed: {}", e),
            LexiconParseError::Syntax(ln, line) => write!(f, "Syntax error on line {}: `{}`", ln, line),
            LexiconParseError::InvalidWeight(ln, w) => write!(f, "Invalid weight on line {}: `{}`", ln, w),
        }
    }
}

impl std::error::Error for LexiconParseError {}