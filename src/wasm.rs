use wasm_bindgen::prelude::*;
use js_sys::Array;
use web_sys::console;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// WASM-compatible table configuration
#[wasm_bindgen]
pub struct WasmTableConfig {
    border_style: String,
    show_borders: bool,
    show_headers: bool,
    max_width: Option<usize>,
    alignment: String,
}

#[wasm_bindgen]
impl WasmTableConfig {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmTableConfig {
        WasmTableConfig {
            border_style: "default".to_string(),
            show_borders: true,
            show_headers: true,
            max_width: None,
            alignment: "left".to_string(),
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_border_style(&mut self, style: &str) {
        self.border_style = style.to_string();
    }

    #[wasm_bindgen(setter)]
    pub fn set_show_borders(&mut self, show: bool) {
        self.show_borders = show;
    }

    #[wasm_bindgen(setter)]
    pub fn set_show_headers(&mut self, show: bool) {
        self.show_headers = show;
    }

    #[wasm_bindgen(setter)]
    pub fn set_max_width(&mut self, width: Option<usize>) {
        self.max_width = width;
    }

    #[wasm_bindgen(setter)]
    pub fn set_alignment(&mut self, alignment: &str) {
        self.alignment = alignment.to_string();
    }
}

/// WASM-compatible table renderer
#[wasm_bindgen]
pub struct WasmTableRenderer {
    config: WasmTableConfig,
}

#[wasm_bindgen]
impl WasmTableRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(config: WasmTableConfig) -> WasmTableRenderer {
        WasmTableRenderer { config }
    }

    /// Render table from 2D JavaScript array
    #[wasm_bindgen]
    pub fn render_from_array(&self, data: &Array) -> Result<String, JsValue> {
        let mut rows = Vec::new();

        for i in 0..data.length() {
            let row_js = data.get(i);
            
            if let Ok(row_array) = row_js.dyn_into::<Array>() {
                let mut row = Vec::new();
                
                for j in 0..row_array.length() {
                    let cell = row_array.get(j);
                    let cell_str = if let Some(s) = cell.as_string() {
                        s
                    } else {
                        format!("{:?}", cell)
                    };
                    row.push(cell_str);
                }
                
                rows.push(row);
            } else {
                return Err(JsValue::from_str("Invalid row data - expected array"));
            }
        }

        if rows.is_empty() {
            return Err(JsValue::from_str("No data provided"));
        }

        let table_data = crate::TableData::new(rows);
        self.render_table_internal(&table_data)
    }

    /// Render table from CSV string
    #[wasm_bindgen]
    pub fn render_from_csv(&self, csv: &str, separator: Option<String>) -> Result<String, JsValue> {
        let sep = separator.unwrap_or_else(|| ",".to_string());
        let mut rows = Vec::new();

        for line in csv.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let row: Vec<String> = line
                .split(&sep)
                .map(|field| field.trim_matches('"').trim().to_string())
                .collect();

            if !row.is_empty() {
                rows.push(row);
            }
        }

        if rows.is_empty() {
            return Err(JsValue::from_str("No valid data found in CSV"));
        }

        let table_data = crate::TableData::new(rows);
        self.render_table_internal(&table_data)
    }

    /// Render table from JSON string (array of arrays format)
    #[wasm_bindgen]
    pub fn render_from_json(&self, json: &str) -> Result<String, JsValue> {
        // Simple JSON parsing - in production would use serde_json
        let trimmed = json.trim();
        
        if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
            return Err(JsValue::from_str("JSON must be an array of arrays"));
        }

        let mut rows = Vec::new();
        let content = &trimmed[1..trimmed.len()-1].trim();
        
        // Basic parsing for demonstration
        let mut current_row = Vec::new();
        let mut in_string = false;
        let mut current_field = String::new();
        let mut in_array = false;
        
        for ch in content.chars() {
            match ch {
                '[' if !in_string => {
                    in_array = true;
                    current_row.clear();
                }
                ']' if !in_string => {
                    if !current_field.is_empty() {
                        current_row.push(current_field.trim_matches('"').to_string());
                        current_field.clear();
                    }
                    if !current_row.is_empty() {
                        rows.push(current_row.clone());
                    }
                    in_array = false;
                }
                '"' => in_string = !in_string,
                ',' if !in_string => {
                    if in_array {
                        current_row.push(current_field.trim_matches('"').to_string());
                        current_field.clear();
                    }
                }
                _ if in_string || (!ch.is_whitespace() || in_array) => {
                    current_field.push(ch);
                }
                _ => {}
            }
        }

        if rows.is_empty() {
            return Err(JsValue::from_str("No valid data found in JSON"));
        }

        let table_data = crate::TableData::new(rows);
        self.render_table_internal(&table_data)
    }

    /// Get supported border styles
    #[wasm_bindgen]
    pub fn get_border_styles() -> Array {
        let styles = Array::new();
        styles.push(&JsValue::from_str("default"));
        styles.push(&JsValue::from_str("ascii"));
        styles.push(&JsValue::from_str("honeywell"));
        styles.push(&JsValue::from_str("ramac"));
        styles.push(&JsValue::from_str("norc"));
        styles.push(&JsValue::from_str("void"));
        styles
    }

    /// Get supported alignments
    #[wasm_bindgen]
    pub fn get_alignments() -> Array {
        let alignments = Array::new();
        alignments.push(&JsValue::from_str("left"));
        alignments.push(&JsValue::from_str("center"));
        alignments.push(&JsValue::from_str("right"));
        alignments.push(&JsValue::from_str("justify"));
        alignments
    }

    fn render_table_internal(&self, data: &crate::TableData) -> Result<String, JsValue> {
        let border = crate::get_border_style(&self.config.border_style)
            .map_err(|e| JsValue::from_str(&e))?;

        let options = crate::RenderOptions {
            show_top_border: self.config.show_borders,
            show_bottom_border: self.config.show_borders,
            show_row_separators: false,
        };

        let alignment = match self.config.alignment.as_str() {
            "center" => crate::alignment::Alignment::Center,
            "right" => crate::alignment::Alignment::Right,
            "justify" => crate::alignment::Alignment::Justify,
            _ => crate::alignment::Alignment::Left,
        };

        // Create column configs with specified alignment
        let column_configs: Vec<crate::alignment::ColumnConfig> = (0..data.column_count())
            .map(|_| {
                let mut config = crate::alignment::ColumnConfig::new()
                    .with_alignment(alignment);
                
                if let Some(max_width) = self.config.max_width {
                    config = config.with_width(max_width.min(50)); // Reasonable max for web
                }
                
                config
            })
            .collect();

        crate::render_table_with_column_config(data, &border, &options, &column_configs)
            .map_err(|e| JsValue::from_str(&e))
    }
}

/// Utility functions for WASM integration
#[wasm_bindgen]
pub struct WasmUtils;

#[wasm_bindgen]
impl WasmUtils {
    /// Log message to browser console
    #[wasm_bindgen]
    pub fn log(message: &str) {
        console::log_1(&JsValue::from_str(message));
    }

    /// Validate CSV format
    #[wasm_bindgen]
    pub fn validate_csv(csv: &str, separator: Option<String>) -> bool {
        let sep = separator.unwrap_or_else(|| ",".to_string());
        let lines: Vec<&str> = csv.lines().filter(|line| !line.trim().is_empty()).collect();
        
        if lines.is_empty() {
            return false;
        }

        let expected_cols = lines[0].split(&sep).count();
        
        lines.iter().all(|line| line.split(&sep).count() == expected_cols)
    }

    /// Count CSV rows and columns
    #[wasm_bindgen]
    pub fn csv_dimensions(csv: &str, separator: Option<String>) -> Array {
        let sep = separator.unwrap_or_else(|| ",".to_string());
        let lines: Vec<&str> = csv.lines().filter(|line| !line.trim().is_empty()).collect();
        
        let result = Array::new();
        result.push(&JsValue::from_f64(lines.len() as f64));
        
        if !lines.is_empty() {
            result.push(&JsValue::from_f64(lines[0].split(&sep).count() as f64));
        } else {
            result.push(&JsValue::from_f64(0.0));
        }
        
        result
    }

    /// Detect CSV separator
    #[wasm_bindgen]
    pub fn detect_csv_separator(csv: &str) -> String {
        let first_line = csv.lines().next().unwrap_or("");
        
        let comma_count = first_line.matches(',').count();
        let semicolon_count = first_line.matches(';').count();
        let tab_count = first_line.matches('\t').count();
        let pipe_count = first_line.matches('|').count();
        
        if tab_count > 0 && tab_count >= comma_count {
            "\t".to_string()
        } else if semicolon_count > comma_count {
            ";".to_string()
        } else if pipe_count > comma_count {
            "|".to_string()
        } else {
            ",".to_string()
        }
    }

    /// Convert table to CSV format
    #[wasm_bindgen]
    pub fn table_to_csv(data: &Array, separator: Option<String>) -> Result<String, JsValue> {
        let sep = separator.unwrap_or_else(|| ",".to_string());
        let mut csv = String::new();

        for i in 0..data.length() {
            let row_js = data.get(i);
            
            if let Ok(row_array) = row_js.dyn_into::<Array>() {
                let mut row_parts = Vec::new();
                
                for j in 0..row_array.length() {
                    let cell = row_array.get(j);
                    let cell_str = if let Some(s) = cell.as_string() {
                        // Quote fields containing separator, quotes, or newlines
                        if s.contains(&sep) || s.contains('"') || s.contains('\n') {
                            format!("\"{}\"", s.replace('"', "\"\""))
                        } else {
                            s
                        }
                    } else {
                        format!("{:?}", cell)
                    };
                    row_parts.push(cell_str);
                }
                
                csv.push_str(&row_parts.join(&sep));
                if i < data.length() - 1 {
                    csv.push('\n');
                }
            } else {
                return Err(JsValue::from_str("Invalid row data"));
            }
        }

        Ok(csv)
    }
}

/// Initialize panic hook for better error reporting in WASM
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Export version information
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Export library name
#[wasm_bindgen]
pub fn library_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_wasm_table_config() {
        let mut config = WasmTableConfig::new();
        config.set_border_style("ascii");
        config.set_show_borders(false);
        config.set_alignment("center");
        
        assert_eq!(config.border_style, "ascii");
        assert!(!config.show_borders);
        assert_eq!(config.alignment, "center");
    }

    #[wasm_bindgen_test]
    fn test_csv_validation() {
        assert!(WasmUtils::validate_csv("a,b,c\n1,2,3", None));
        assert!(!WasmUtils::validate_csv("a,b,c\n1,2", None));
        assert!(WasmUtils::validate_csv("a;b;c\n1;2;3", Some(";".to_string())));
    }

    #[wasm_bindgen_test]
    fn test_csv_dimensions() {
        let dims = WasmUtils::csv_dimensions("a,b,c\n1,2,3\nx,y,z", None);
        assert_eq!(dims.length(), 2);
    }

    #[wasm_bindgen_test]
    fn test_separator_detection() {
        assert_eq!(WasmUtils::detect_csv_separator("a,b,c"), ",");
        assert_eq!(WasmUtils::detect_csv_separator("a;b;c"), ";");
        assert_eq!(WasmUtils::detect_csv_separator("a\tb\tc"), "\t");
    }

    #[wasm_bindgen_test]
    fn test_version_info() {
        let ver = version();
        let name = library_name();
        
        assert!(!ver.is_empty());
        assert!(!name.is_empty());
        assert_eq!(name, "ascii-ansi-table");
    }
}