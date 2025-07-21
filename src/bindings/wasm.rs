#![cfg(feature = "wasm")]

use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

use crate::types::{Row, TableUserConfig};
use crate::{get_border_characters, table as table_fn};

#[wasm_bindgen]
pub struct WasmTable {
    data: Vec<Row>,
    config: Option<TableUserConfig>,
}

#[wasm_bindgen]
impl WasmTable {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmTable {
        WasmTable {
            data: Vec::new(),
            config: None,
        }
    }

    #[wasm_bindgen(js_name = setData)]
    pub fn set_data(&mut self, data: &JsValue) -> Result<(), JsValue> {
        let data: Vec<Row> = serde_wasm_bindgen::from_value(data.clone())?;
        self.data = data;
        Ok(())
    }

    #[wasm_bindgen(js_name = setConfig)]
    pub fn set_config(&mut self, config: &JsValue) -> Result<(), JsValue> {
        let config: TableUserConfig = serde_wasm_bindgen::from_value(config.clone())?;
        self.config = Some(config);
        Ok(())
    }

    #[wasm_bindgen(js_name = render)]
    pub fn render(&self) -> Result<String, JsValue> {
        table_fn(&self.data, self.config.as_ref()).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen(js_name = clear)]
    pub fn clear(&mut self) {
        self.data.clear();
        self.config = None;
    }
}

#[wasm_bindgen(js_name = table)]
pub fn wasm_table(data: &JsValue, config: &JsValue) -> Result<String, JsValue> {
    let data: Vec<Row> = serde_wasm_bindgen::from_value(data.clone())?;
    let config: Option<TableUserConfig> = if config.is_undefined() || config.is_null() {
        None
    } else {
        Some(serde_wasm_bindgen::from_value(config.clone())?)
    };

    table_fn(&data, config.as_ref()).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(js_name = getBorderCharacters)]
pub fn wasm_get_border_characters(name: &str) -> Result<JsValue, JsValue> {
    let border_config =
        get_border_characters(name).map_err(|e| JsValue::from_str(&e.to_string()))?;

    serde_wasm_bindgen::to_value(&border_config).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(js_name = validateTableData)]
pub fn wasm_validate_table_data(data: &JsValue) -> Result<bool, JsValue> {
    let data: Vec<Row> = serde_wasm_bindgen::from_value(data.clone())?;

    match crate::utils::formatting::validate_table_data(&data) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[wasm_bindgen(js_name = calculateColumnWidths)]
pub fn wasm_calculate_column_widths(data: &JsValue) -> Result<JsValue, JsValue> {
    let data: Vec<Row> = serde_wasm_bindgen::from_value(data.clone())?;
    let widths = crate::utils::formatting::calculate_maximum_column_widths(&data);

    serde_wasm_bindgen::to_value(&widths).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(js_name = wrapText)]
pub fn wasm_wrap_text(text: &str, width: u32, word_wrap: bool) -> Result<JsValue, JsValue> {
    let wrapped = crate::utils::wrapping::wrap_text(text, width as usize, word_wrap);

    serde_wasm_bindgen::to_value(&wrapped).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(js_name = calculateDisplayWidth)]
pub fn wasm_calculate_display_width(text: &str) -> u32 {
    crate::utils::ansi::calculate_display_width(text) as u32
}

#[wasm_bindgen(js_name = stripAnsi)]
pub fn wasm_strip_ansi(text: &str) -> String {
    crate::utils::ansi::strip_ansi_sequences(text)
}

#[wasm_bindgen(js_name = convertAnsiToHtml)]
pub fn wasm_convert_ansi_to_html(text: &str) -> String {
    crate::utils::html::convert_ansi_to_html(text)
}

#[wasm_bindgen(start)]
pub fn wasm_init() {}

pub mod js_exports {
    pub use super::wasm_calculate_column_widths as calculateColumnWidths;
    pub use super::wasm_calculate_display_width as calculateDisplayWidth;
    pub use super::wasm_convert_ansi_to_html as convertAnsiToHtml;
    pub use super::wasm_get_border_characters as getBorderCharacters;
    pub use super::wasm_strip_ansi as stripAnsi;
    pub use super::wasm_table as table;
    pub use super::wasm_validate_table_data as validateTableData;
    pub use super::wasm_wrap_text as wrapText;

    pub use super::WasmTable as Table;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_table_creation() {
        let mut table = WasmTable::new();
        assert!(table.data.is_empty());
        assert!(table.config.is_none());

        table.clear();
        assert!(table.data.is_empty());
        assert!(table.config.is_none());
    }

    #[test]
    fn test_wasm_display_width() {
        assert_eq!(wasm_calculate_display_width("hello"), 5);
        assert_eq!(wasm_calculate_display_width("测试"), 4);
    }

    #[test]
    fn test_wasm_strip_ansi() {
        let text = "\u{1b}[31mhello\u{1b}[39m";
        assert_eq!(wasm_strip_ansi(text), "hello");
    }
}
