/// Debugging and introspection utilities for table development
use std::fmt::{self, Write};
use std::collections::HashMap;

/// Debug configuration for controlling debug output
#[derive(Debug, Clone)]
pub struct DebugConfig {
    pub show_dimensions: bool,
    pub show_column_widths: bool,
    pub show_cell_contents: bool,
    pub show_border_info: bool,
    pub show_alignment_info: bool,
    pub show_performance_stats: bool,
    pub max_cell_preview: usize,
    pub compact_output: bool,
    pub use_colors: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            show_dimensions: true,
            show_column_widths: true,
            show_cell_contents: true,
            show_border_info: false,
            show_alignment_info: false,
            show_performance_stats: false,
            max_cell_preview: 20,
            compact_output: false,
            use_colors: true,
        }
    }
}

impl DebugConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn verbose() -> Self {
        Self {
            show_dimensions: true,
            show_column_widths: true,
            show_cell_contents: true,
            show_border_info: true,
            show_alignment_info: true,
            show_performance_stats: true,
            max_cell_preview: 50,
            compact_output: false,
            use_colors: true,
        }
    }

    pub fn compact() -> Self {
        Self {
            show_dimensions: true,
            show_column_widths: false,
            show_cell_contents: true,
            show_border_info: false,
            show_alignment_info: false,
            show_performance_stats: false,
            max_cell_preview: 10,
            compact_output: true,
            use_colors: false,
        }
    }

    pub fn minimal() -> Self {
        Self {
            show_dimensions: true,
            show_column_widths: false,
            show_cell_contents: false,
            show_border_info: false,
            show_alignment_info: false,
            show_performance_stats: false,
            max_cell_preview: 0,
            compact_output: true,
            use_colors: false,
        }
    }
}

/// Table debugging and analysis utilities
pub struct TableDebugger {
    config: DebugConfig,
}

impl TableDebugger {
    pub fn new(config: DebugConfig) -> Self {
        Self { config }
    }

    /// Generate comprehensive debug information about a table
    pub fn analyze_table(&self, data: &crate::TableData) -> String {
        let mut output = String::new();
        
        if self.config.use_colors {
            output.push_str("\x1b[1;36m");
        }
        writeln!(output, "=== TABLE DEBUG ANALYSIS ===").unwrap();
        if self.config.use_colors {
            output.push_str("\x1b[0m");
        }

        // Basic dimensions
        if self.config.show_dimensions {
            self.analyze_dimensions(&mut output, data);
        }

        // Column width analysis
        if self.config.show_column_widths {
            self.analyze_column_widths(&mut output, data);
        }

        // Cell contents preview
        if self.config.show_cell_contents {
            self.analyze_cell_contents(&mut output, data);
        }

        // Table structure validation
        self.analyze_structure(&mut output, data);

        output
    }

    /// Analyze table with specific column configurations
    pub fn analyze_with_config(
        &self, 
        data: &crate::TableData, 
        column_configs: &[crate::alignment::ColumnConfig]
    ) -> String {
        let mut output = self.analyze_table(data);

        if self.config.show_alignment_info {
            writeln!(output, "\n--- Column Configuration Analysis ---").unwrap();
            for (i, config) in column_configs.iter().enumerate() {
                writeln!(output, "Column {}: {:?}", i, config).unwrap();
            }
        }

        output
    }

    /// Preview table rendering with debug information
    pub fn preview_rendering(
        &self,
        data: &crate::TableData,
        border: &crate::BorderChars,
        options: &crate::RenderOptions,
    ) -> Result<String, String> {
        let mut output = String::new();

        // Show border configuration
        if self.config.show_border_info {
            writeln!(output, "--- Border Configuration ---").unwrap();
            writeln!(output, "Top Left: '{}' ({})", border.top_left, border.top_left as u32).unwrap();
            writeln!(output, "Top Right: '{}' ({})", border.top_right, border.top_right as u32).unwrap();
            writeln!(output, "Horizontal: '{}' ({})", border.horizontal, border.horizontal as u32).unwrap();
            writeln!(output, "Vertical: '{}' ({})", border.vertical, border.vertical as u32).unwrap();
            writeln!(output, "Options: {:?}", options).unwrap();
            writeln!(output, "").unwrap();
        }

        // Render the actual table
        let table_output = crate::render_table_with_borders(data)?;
        output.push_str(&table_output);

        Ok(output)
    }

    fn analyze_dimensions(&self, output: &mut String, data: &crate::TableData) {
        if self.config.use_colors {
            output.push_str("\x1b[1;33m");
        }
        writeln!(output, "--- Table Dimensions ---").unwrap();
        if self.config.use_colors {
            output.push_str("\x1b[0m");
        }

        writeln!(output, "Rows: {}", data.row_count()).unwrap();
        writeln!(output, "Columns: {}", data.column_count()).unwrap();
        writeln!(output, "Total cells: {}", data.row_count() * data.column_count()).unwrap();
        
        if data.is_empty() {
            if self.config.use_colors {
                output.push_str("\x1b[1;31m");
            }
            writeln!(output, "⚠️  Table is empty!").unwrap();
            if self.config.use_colors {
                output.push_str("\x1b[0m");
            }
        }
    }

    fn analyze_column_widths(&self, output: &mut String, data: &crate::TableData) {
        if data.is_empty() {
            return;
        }

        if self.config.use_colors {
            output.push_str("\x1b[1;33m");
        }
        writeln!(output, "\n--- Column Width Analysis ---").unwrap();
        if self.config.use_colors {
            output.push_str("\x1b[0m");
        }

        let widths = crate::calculate_column_widths(data);
        let unicode_widths = crate::unicode::calculate_unicode_column_widths(&data.rows);
        
        for (i, (basic_width, unicode_width)) in widths.iter().zip(unicode_widths.iter()).enumerate() {
            write!(output, "Column {}: {} chars", i, basic_width).unwrap();
            
            if basic_width != unicode_width {
                write!(output, " (Unicode: {} chars)", unicode_width).unwrap();
                if self.config.use_colors {
                    output.push_str("\x1b[1;31m ⚠️\x1b[0m");
                }
            }
            
            writeln!(output).unwrap();
        }

        // Show total width
        let total_width: usize = widths.iter().sum();
        writeln!(output, "Total content width: {} characters", total_width).unwrap();
        
        // Estimate rendered table width (with borders and padding)
        let estimated_rendered_width = total_width + (widths.len() * 3) + 1; // borders + padding
        writeln!(output, "Estimated rendered width: {} characters", estimated_rendered_width).unwrap();
    }

    fn analyze_cell_contents(&self, output: &mut String, data: &crate::TableData) {
        if data.is_empty() || self.config.max_cell_preview == 0 {
            return;
        }

        if self.config.use_colors {
            output.push_str("\x1b[1;33m");
        }
        writeln!(output, "\n--- Cell Contents Preview ---").unwrap();
        if self.config.use_colors {
            output.push_str("\x1b[0m");
        }

        let preview_rows = if self.config.compact_output {
            data.rows.len().min(3)
        } else {
            data.rows.len().min(5)
        };

        for (row_idx, row) in data.rows.iter().take(preview_rows).enumerate() {
            write!(output, "Row {}: [", row_idx).unwrap();
            
            for (col_idx, cell) in row.iter().enumerate() {
                if col_idx > 0 {
                    write!(output, ", ").unwrap();
                }
                
                let preview = if cell.len() > self.config.max_cell_preview {
                    format!("{}...", &cell[..self.config.max_cell_preview])
                } else {
                    cell.clone()
                };
                
                write!(output, "\"{}\"", preview.replace('\n', "\\n")).unwrap();
            }
            
            writeln!(output, "]").unwrap();
        }

        if data.rows.len() > preview_rows {
            writeln!(output, "... ({} more rows)", data.rows.len() - preview_rows).unwrap();
        }
    }

    fn analyze_structure(&self, output: &mut String, data: &crate::TableData) {
        if data.is_empty() {
            return;
        }

        if self.config.use_colors {
            output.push_str("\x1b[1;33m");
        }
        writeln!(output, "\n--- Structure Analysis ---").unwrap();
        if self.config.use_colors {
            output.push_str("\x1b[0m");
        }

        // Check for consistent row lengths
        let expected_cols = data.column_count();
        let mut inconsistent_rows = Vec::new();
        
        for (i, row) in data.rows.iter().enumerate() {
            if row.len() != expected_cols {
                inconsistent_rows.push((i, row.len()));
            }
        }

        if inconsistent_rows.is_empty() {
            if self.config.use_colors {
                output.push_str("\x1b[1;32m");
            }
            writeln!(output, "✅ All rows have consistent column count").unwrap();
            if self.config.use_colors {
                output.push_str("\x1b[0m");
            }
        } else {
            if self.config.use_colors {
                output.push_str("\x1b[1;31m");
            }
            writeln!(output, "❌ Found {} rows with inconsistent column count:", inconsistent_rows.len()).unwrap();
            if self.config.use_colors {
                output.push_str("\x1b[0m");
            }
            
            for (row_idx, actual_cols) in inconsistent_rows.iter().take(5) {
                writeln!(output, "  Row {}: {} columns (expected {})", row_idx, actual_cols, expected_cols).unwrap();
            }
            
            if inconsistent_rows.len() > 5 {
                writeln!(output, "  ... and {} more", inconsistent_rows.len() - 5).unwrap();
            }
        }

        // Analyze data types in each column
        self.analyze_data_types(output, data);

        // Check for special characters
        self.analyze_special_characters(output, data);
    }

    fn analyze_data_types(&self, output: &mut String, data: &crate::TableData) {
        writeln!(output, "\nData type analysis per column:").unwrap();
        
        for col_idx in 0..data.column_count() {
            let mut type_counts = HashMap::new();
            
            for row in &data.rows {
                if let Some(cell) = row.get(col_idx) {
                    let data_type = self.classify_data_type(cell);
                    *type_counts.entry(data_type).or_insert(0) += 1;
                }
            }
            
            write!(output, "  Column {}: ", col_idx).unwrap();
            let types: Vec<String> = type_counts.iter()
                .map(|(t, count)| format!("{} ({})", t, count))
                .collect();
            writeln!(output, "{}", types.join(", ")).unwrap();
        }
    }

    fn analyze_special_characters(&self, output: &mut String, data: &crate::TableData) {
        let mut has_newlines = false;
        let mut has_tabs = false;
        let mut has_ansi = false;
        let mut has_unicode = false;
        let mut has_emojis = false;

        for row in &data.rows {
            for cell in row {
                if cell.contains('\n') { has_newlines = true; }
                if cell.contains('\t') { has_tabs = true; }
                if cell.contains('\x1b') { has_ansi = true; }
                
                for ch in cell.chars() {
                    if !ch.is_ascii() { has_unicode = true; }
                    if self.is_emoji(ch) { has_emojis = true; }
                }
            }
        }

        let mut special_chars = Vec::new();
        if has_newlines { special_chars.push("newlines"); }
        if has_tabs { special_chars.push("tabs"); }
        if has_ansi { special_chars.push("ANSI sequences"); }
        if has_unicode { special_chars.push("Unicode"); }
        if has_emojis { special_chars.push("emojis"); }

        if !special_chars.is_empty() {
            writeln!(output, "\nSpecial characters detected: {}", special_chars.join(", ")).unwrap();
            
            if has_ansi {
                writeln!(output, "  💡 Consider using render_table_ansi_aware() for ANSI sequences").unwrap();
            }
            if has_unicode || has_emojis {
                writeln!(output, "  💡 Consider using render_table_unicode_aware() or emoji-aware functions").unwrap();
            }
            if has_newlines {
                writeln!(output, "  💡 Consider using render_table_with_newlines() for multi-line cells").unwrap();
            }
        }
    }

    fn classify_data_type(&self, cell: &str) -> &'static str {
        if cell.is_empty() {
            "empty"
        } else if cell.chars().all(|c| c.is_ascii_digit()) {
            "integer"
        } else if cell.parse::<f64>().is_ok() {
            "number"
        } else if cell.len() == 1 {
            "char"
        } else if cell.chars().all(|c| c.is_ascii_alphabetic() || c.is_ascii_whitespace()) {
            "text"
        } else if cell.contains('\n') {
            "multiline"
        } else if cell.contains('\x1b') {
            "ansi"
        } else if !cell.is_ascii() {
            "unicode"
        } else {
            "mixed"
        }
    }

    fn is_emoji(&self, ch: char) -> bool {
        matches!(ch as u32, 0x1F600..=0x1F64F | 0x1F300..=0x1F5FF | 0x1F680..=0x1F6FF)
    }
}

/// Performance measurement utilities
pub struct PerformanceMeasurer {
    measurements: HashMap<String, Vec<std::time::Duration>>,
}

impl PerformanceMeasurer {
    pub fn new() -> Self {
        Self {
            measurements: HashMap::new(),
        }
    }

    /// Time a table rendering operation
    pub fn measure_render<F>(&mut self, operation_name: &str, f: F) -> Result<String, String>
    where
        F: FnOnce() -> Result<String, String>,
    {
        let start = std::time::Instant::now();
        let result = f();
        let duration = start.elapsed();
        
        self.measurements.entry(operation_name.to_string())
            .or_insert_with(Vec::new)
            .push(duration);
        
        result
    }

    /// Get performance report
    pub fn report(&self) -> String {
        let mut output = String::new();
        writeln!(output, "=== Performance Report ===").unwrap();
        
        for (operation, measurements) in &self.measurements {
            if measurements.is_empty() {
                continue;
            }
            
            let total: std::time::Duration = measurements.iter().sum();
            let avg = total / measurements.len() as u32;
            let min = measurements.iter().min().copied().unwrap_or_default();
            let max = measurements.iter().max().copied().unwrap_or_default();
            
            writeln!(output, "{}:", operation).unwrap();
            writeln!(output, "  Runs: {}", measurements.len()).unwrap();
            writeln!(output, "  Average: {:?}", avg).unwrap();
            writeln!(output, "  Min: {:?}", min).unwrap();
            writeln!(output, "  Max: {:?}", max).unwrap();
            writeln!(output, "  Total: {:?}", total).unwrap();
            writeln!(output, "").unwrap();
        }
        
        output
    }

    pub fn clear(&mut self) {
        self.measurements.clear();
    }
}

/// Quick debug functions for interactive use
pub fn debug_table(data: &crate::TableData) -> String {
    let debugger = TableDebugger::new(DebugConfig::default());
    debugger.analyze_table(data)
}

pub fn debug_table_compact(data: &crate::TableData) -> String {
    let debugger = TableDebugger::new(DebugConfig::compact());
    debugger.analyze_table(data)
}

pub fn debug_table_verbose(data: &crate::TableData) -> String {
    let debugger = TableDebugger::new(DebugConfig::verbose());
    debugger.analyze_table(data)
}

/// Print debug information to console
pub fn print_debug_table(data: &crate::TableData) {
    println!("{}", debug_table(data));
}

/// Validate table structure and provide recommendations
pub fn validate_and_recommend(data: &crate::TableData) -> String {
    let mut output = String::new();
    
    writeln!(output, "=== Table Validation & Recommendations ===").unwrap();
    
    if data.is_empty() {
        writeln!(output, "❌ Table is empty").unwrap();
        writeln!(output, "💡 Recommendation: Add data before rendering").unwrap();
        return output;
    }
    
    // Check row consistency
    let expected_cols = data.column_count();
    let mut inconsistent_count = 0;
    
    for row in &data.rows {
        if row.len() != expected_cols {
            inconsistent_count += 1;
        }
    }
    
    if inconsistent_count == 0 {
        writeln!(output, "✅ All rows have consistent column count").unwrap();
    } else {
        writeln!(output, "❌ {} rows have inconsistent column count", inconsistent_count).unwrap();
        writeln!(output, "💡 Recommendation: Use validate_table_data() before rendering").unwrap();
    }
    
    // Performance recommendations based on size
    let total_cells = data.row_count() * data.column_count();
    if total_cells > 10000 {
        writeln!(output, "🔶 Large table detected ({} cells)", total_cells).unwrap();
        writeln!(output, "💡 Recommendation: Consider using streaming API or performance optimizations").unwrap();
    }
    
    // Unicode content detection
    let has_unicode = data.rows.iter()
        .any(|row| row.iter().any(|cell| !cell.is_ascii()));
    
    if has_unicode {
        writeln!(output, "🌐 Unicode content detected").unwrap();
        writeln!(output, "💡 Recommendation: Use render_table_unicode_aware() for proper width calculation").unwrap();
    }
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_config_presets() {
        let default = DebugConfig::default();
        assert!(default.show_dimensions);
        assert!(default.show_column_widths);

        let verbose = DebugConfig::verbose();
        assert!(verbose.show_performance_stats);
        assert!(verbose.show_border_info);

        let compact = DebugConfig::compact();
        assert!(compact.compact_output);
        assert!(!compact.use_colors);
    }

    #[test]
    fn test_table_debugger() {
        let data = crate::TableData::new(vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "25".to_string()],
        ]);

        let debugger = TableDebugger::new(DebugConfig::default());
        let analysis = debugger.analyze_table(&data);

        assert!(analysis.contains("Rows: 3"));
        assert!(analysis.contains("Columns: 2"));
        assert!(analysis.contains("Column 0"));
        assert!(analysis.contains("Column 1"));
    }

    #[test]
    fn test_performance_measurer() {
        let mut measurer = PerformanceMeasurer::new();

        let result = measurer.measure_render("test_operation", || {
            std::thread::sleep(std::time::Duration::from_millis(1));
            Ok("test result".to_string())
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test result");

        let report = measurer.report();
        assert!(report.contains("test_operation"));
        assert!(report.contains("Runs: 1"));
    }

    #[test]
    fn test_data_type_classification() {
        let debugger = TableDebugger::new(DebugConfig::default());

        assert_eq!(debugger.classify_data_type(""), "empty");
        assert_eq!(debugger.classify_data_type("123"), "integer");
        assert_eq!(debugger.classify_data_type("12.34"), "number");
        assert_eq!(debugger.classify_data_type("hello"), "text");
        assert_eq!(debugger.classify_data_type("line1\nline2"), "multiline");
        assert_eq!(debugger.classify_data_type("café"), "unicode");
    }

    #[test]
    fn test_validation_and_recommendations() {
        let valid_data = crate::TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ]);

        let recommendations = validate_and_recommend(&valid_data);
        assert!(recommendations.contains("consistent column count"));

        let empty_data = crate::TableData::new(vec![]);
        let empty_recommendations = validate_and_recommend(&empty_data);
        assert!(empty_recommendations.contains("Table is empty"));
    }

    #[test]
    fn test_quick_debug_functions() {
        let data = crate::TableData::new(vec![
            vec!["Test".to_string()],
        ]);

        let debug_output = debug_table(&data);
        assert!(debug_output.contains("TABLE DEBUG ANALYSIS"));

        let compact_output = debug_table_compact(&data);
        assert!(compact_output.contains("Rows: 1"));

        let verbose_output = debug_table_verbose(&data);
        assert!(verbose_output.contains("Structure Analysis"));
    }

    #[test]
    fn test_inconsistent_table_detection() {
        let inconsistent_data = crate::TableData::new(vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["1".to_string(), "2".to_string()], // Missing column
        ]);

        let debugger = TableDebugger::new(DebugConfig::verbose());
        let analysis = debugger.analyze_table(&inconsistent_data);
        
        assert!(analysis.contains("inconsistent column count"));
    }

    #[test]
    fn test_special_character_detection() {
        let special_data = crate::TableData::new(vec![
            vec!["Hello\nWorld".to_string(), "Café".to_string(), "😀".to_string()],
        ]);

        let debugger = TableDebugger::new(DebugConfig::verbose());
        let analysis = debugger.analyze_table(&special_data);
        
        assert!(analysis.contains("Special characters detected"));
    }
}