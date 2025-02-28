/// Advanced table validation and error reporting system
use std::fmt;
use std::collections::HashMap;

/// Validation error types
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    EmptyTable,
    InconsistentRowLength { row_index: usize, expected: usize, actual: usize },
    ExcessiveColumnCount { count: usize, max_recommended: usize },
    ExcessiveRowCount { count: usize, max_recommended: usize },
    InvalidCellContent { row: usize, col: usize, issue: String },
    PerformanceWarning { issue: String, recommendation: String },
    ConfigurationIssue { issue: String },
    DataIntegrityIssue { issue: String },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::EmptyTable => {
                write!(f, "Table is empty - no rows or columns to render")
            }
            ValidationError::InconsistentRowLength { row_index, expected, actual } => {
                write!(f, "Row {} has {} columns, expected {} columns", row_index, actual, expected)
            }
            ValidationError::ExcessiveColumnCount { count, max_recommended } => {
                write!(f, "Table has {} columns, which exceeds recommended maximum of {}", count, max_recommended)
            }
            ValidationError::ExcessiveRowCount { count, max_recommended } => {
                write!(f, "Table has {} rows, which exceeds recommended maximum of {}", count, max_recommended)
            }
            ValidationError::InvalidCellContent { row, col, issue } => {
                write!(f, "Cell at row {}, column {} has invalid content: {}", row, col, issue)
            }
            ValidationError::PerformanceWarning { issue, recommendation } => {
                write!(f, "Performance warning: {}. Recommendation: {}", issue, recommendation)
            }
            ValidationError::ConfigurationIssue { issue } => {
                write!(f, "Configuration issue: {}", issue)
            }
            ValidationError::DataIntegrityIssue { issue } => {
                write!(f, "Data integrity issue: {}", issue)
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Validation severity levels
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Info => write!(f, "INFO"),
            Severity::Warning => write!(f, "WARNING"),
            Severity::Error => write!(f, "ERROR"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// Validation result with error details
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationError>,
    pub info: Vec<ValidationError>,
    pub performance_score: u32, // 0-100, higher is better
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            info: Vec::new(),
            performance_score: 100,
        }
    }

    pub fn add_error(&mut self, error: ValidationError) {
        self.is_valid = false;
        self.errors.push(error);
        self.performance_score = self.performance_score.saturating_sub(20);
    }

    pub fn add_warning(&mut self, warning: ValidationError) {
        self.warnings.push(warning);
        self.performance_score = self.performance_score.saturating_sub(5);
    }

    pub fn add_info(&mut self, info: ValidationError) {
        self.info.push(info);
    }

    pub fn has_issues(&self) -> bool {
        !self.errors.is_empty() || !self.warnings.is_empty()
    }

    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }

    /// Get a formatted report of all validation issues
    pub fn report(&self) -> String {
        let mut report = String::new();
        
        if self.is_valid && !self.has_issues() {
            report.push_str("✅ Table validation passed with no issues\n");
            report.push_str(&format!("Performance score: {}/100\n", self.performance_score));
            return report;
        }

        report.push_str("📋 Table Validation Report\n");
        report.push_str("==========================\n");

        if !self.errors.is_empty() {
            report.push_str("\n❌ ERRORS:\n");
            for (i, error) in self.errors.iter().enumerate() {
                report.push_str(&format!("  {}: {}\n", i + 1, error));
            }
        }

        if !self.warnings.is_empty() {
            report.push_str("\n⚠️  WARNINGS:\n");
            for (i, warning) in self.warnings.iter().enumerate() {
                report.push_str(&format!("  {}: {}\n", i + 1, warning));
            }
        }

        if !self.info.is_empty() {
            report.push_str("\n💡 INFO:\n");
            for (i, info) in self.info.iter().enumerate() {
                report.push_str(&format!("  {}: {}\n", i + 1, info));
            }
        }

        report.push_str(&format!("\nPerformance score: {}/100\n", self.performance_score));
        
        if !self.is_valid {
            report.push_str("\n❌ Validation failed - please fix errors before rendering\n");
        }

        report
    }
}

/// Configuration for table validation
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    pub max_columns: Option<usize>,
    pub max_rows: Option<usize>,
    pub max_cell_length: Option<usize>,
    pub allow_empty_cells: bool,
    pub allow_inconsistent_rows: bool,
    pub check_unicode_issues: bool,
    pub check_ansi_sequences: bool,
    pub check_whitespace_issues: bool,
    pub performance_thresholds: PerformanceThresholds,
}

#[derive(Debug, Clone)]
pub struct PerformanceThresholds {
    pub large_table_rows: usize,
    pub large_table_columns: usize,
    pub very_large_cell_chars: usize,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_columns: Some(50),
            max_rows: Some(10000),
            max_cell_length: Some(1000),
            allow_empty_cells: true,
            allow_inconsistent_rows: false,
            check_unicode_issues: true,
            check_ansi_sequences: true,
            check_whitespace_issues: true,
            performance_thresholds: PerformanceThresholds {
                large_table_rows: 1000,
                large_table_columns: 20,
                very_large_cell_chars: 500,
            },
        }
    }
}

impl ValidationConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn strict() -> Self {
        Self {
            max_columns: Some(20),
            max_rows: Some(1000),
            max_cell_length: Some(200),
            allow_empty_cells: false,
            allow_inconsistent_rows: false,
            check_unicode_issues: true,
            check_ansi_sequences: true,
            check_whitespace_issues: true,
            performance_thresholds: PerformanceThresholds {
                large_table_rows: 500,
                large_table_columns: 10,
                very_large_cell_chars: 100,
            },
        }
    }

    pub fn permissive() -> Self {
        Self {
            max_columns: None,
            max_rows: None,
            max_cell_length: None,
            allow_empty_cells: true,
            allow_inconsistent_rows: true,
            check_unicode_issues: false,
            check_ansi_sequences: false,
            check_whitespace_issues: false,
            performance_thresholds: PerformanceThresholds {
                large_table_rows: 10000,
                large_table_columns: 100,
                very_large_cell_chars: 2000,
            },
        }
    }

    pub fn web_optimized() -> Self {
        Self {
            max_columns: Some(10),
            max_rows: Some(500),
            max_cell_length: Some(300),
            allow_empty_cells: true,
            allow_inconsistent_rows: false,
            check_unicode_issues: true,
            check_ansi_sequences: false, // ANSI not relevant for web
            check_whitespace_issues: true,
            performance_thresholds: PerformanceThresholds {
                large_table_rows: 100,
                large_table_columns: 8,
                very_large_cell_chars: 200,
            },
        }
    }
}

/// Comprehensive table validator
pub struct TableValidator {
    config: ValidationConfig,
}

impl TableValidator {
    pub fn new(config: ValidationConfig) -> Self {
        Self { config }
    }

    /// Validate table data comprehensively
    pub fn validate(&self, data: &crate::TableData) -> ValidationResult {
        let mut result = ValidationResult::new();

        // Basic structure validation
        self.validate_structure(data, &mut result);

        // Content validation
        self.validate_content(data, &mut result);

        // Performance analysis
        self.analyze_performance(data, &mut result);

        // Data integrity checks
        self.check_data_integrity(data, &mut result);

        result
    }

    /// Validate with column configurations
    pub fn validate_with_config(
        &self, 
        data: &crate::TableData, 
        column_configs: &[crate::alignment::ColumnConfig]
    ) -> ValidationResult {
        let mut result = self.validate(data);

        // Configuration-specific validation
        if !data.is_empty() && column_configs.len() != data.column_count() {
            result.add_warning(ValidationError::ConfigurationIssue {
                issue: format!(
                    "Column configuration count ({}) doesn't match table columns ({})",
                    column_configs.len(),
                    data.column_count()
                ),
            });
        }

        // Check for potential width issues
        for (i, config) in column_configs.iter().enumerate() {
            if let Some(width) = config.width {
                if width > 100 {
                    result.add_warning(ValidationError::ConfigurationIssue {
                        issue: format!("Column {} has very large width ({}), may cause rendering issues", i, width),
                    });
                }
            }
        }

        result
    }

    fn validate_structure(&self, data: &crate::TableData, result: &mut ValidationResult) {
        // Check for empty table
        if data.is_empty() {
            result.add_error(ValidationError::EmptyTable);
            return;
        }

        // Check row consistency
        let expected_columns = data.column_count();
        for (i, row) in data.rows.iter().enumerate() {
            if row.len() != expected_columns {
                if self.config.allow_inconsistent_rows {
                    result.add_warning(ValidationError::InconsistentRowLength {
                        row_index: i,
                        expected: expected_columns,
                        actual: row.len(),
                    });
                } else {
                    result.add_error(ValidationError::InconsistentRowLength {
                        row_index: i,
                        expected: expected_columns,
                        actual: row.len(),
                    });
                }
            }
        }

        // Check size limits
        if let Some(max_cols) = self.config.max_columns {
            if expected_columns > max_cols {
                result.add_error(ValidationError::ExcessiveColumnCount {
                    count: expected_columns,
                    max_recommended: max_cols,
                });
            }
        }

        if let Some(max_rows) = self.config.max_rows {
            if data.row_count() > max_rows {
                result.add_error(ValidationError::ExcessiveRowCount {
                    count: data.row_count(),
                    max_recommended: max_rows,
                });
            }
        }
    }

    fn validate_content(&self, data: &crate::TableData, result: &mut ValidationResult) {
        for (row_idx, row) in data.rows.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                self.validate_cell(cell, row_idx, col_idx, result);
            }
        }
    }

    fn validate_cell(&self, cell: &str, row: usize, col: usize, result: &mut ValidationResult) {
        // Check for empty cells
        if cell.is_empty() && !self.config.allow_empty_cells {
            result.add_warning(ValidationError::InvalidCellContent {
                row,
                col,
                issue: "Cell is empty".to_string(),
            });
        }

        // Check cell length
        if let Some(max_len) = self.config.max_cell_length {
            if cell.len() > max_len {
                result.add_warning(ValidationError::InvalidCellContent {
                    row,
                    col,
                    issue: format!("Cell content is {} characters (max: {})", cell.len(), max_len),
                });
            }
        }

        // Check for Unicode issues
        if self.config.check_unicode_issues {
            self.check_unicode_issues(cell, row, col, result);
        }

        // Check for ANSI sequences
        if self.config.check_ansi_sequences && cell.contains('\x1b') {
            result.add_info(ValidationError::InvalidCellContent {
                row,
                col,
                issue: "Cell contains ANSI escape sequences - consider using ANSI-aware rendering".to_string(),
            });
        }

        // Check for whitespace issues
        if self.config.check_whitespace_issues {
            self.check_whitespace_issues(cell, row, col, result);
        }
    }

    fn check_unicode_issues(&self, cell: &str, row: usize, col: usize, result: &mut ValidationResult) {
        let mut has_wide_chars = false;
        let mut has_emojis = false;
        let mut has_combining_chars = false;

        for ch in cell.chars() {
            let width = crate::unicode::char_display_width(ch);
            if width > 1 {
                has_wide_chars = true;
            }
            if self.is_emoji(ch) {
                has_emojis = true;
            }
            if self.is_combining_character(ch) {
                has_combining_chars = true;
            }
        }

        if has_wide_chars {
            result.add_info(ValidationError::InvalidCellContent {
                row,
                col,
                issue: "Cell contains wide Unicode characters - consider using Unicode-aware rendering".to_string(),
            });
        }

        if has_emojis {
            result.add_info(ValidationError::InvalidCellContent {
                row,
                col,
                issue: "Cell contains emoji characters - consider using emoji-aware rendering".to_string(),
            });
        }

        if has_combining_chars {
            result.add_info(ValidationError::InvalidCellContent {
                row,
                col,
                issue: "Cell contains combining characters - width calculation may be affected".to_string(),
            });
        }
    }

    fn check_whitespace_issues(&self, cell: &str, row: usize, col: usize, result: &mut ValidationResult) {
        if cell.starts_with(' ') || cell.ends_with(' ') {
            result.add_info(ValidationError::InvalidCellContent {
                row,
                col,
                issue: "Cell has leading or trailing whitespace".to_string(),
            });
        }

        if cell.contains('\t') {
            result.add_info(ValidationError::InvalidCellContent {
                row,
                col,
                issue: "Cell contains tab characters - may affect alignment".to_string(),
            });
        }

        if cell.contains('\n') {
            result.add_info(ValidationError::InvalidCellContent {
                row,
                col,
                issue: "Cell contains newlines - consider using newline-aware rendering".to_string(),
            });
        }
    }

    fn analyze_performance(&self, data: &crate::TableData, result: &mut ValidationResult) {
        let thresholds = &self.config.performance_thresholds;

        // Check table size
        if data.row_count() > thresholds.large_table_rows {
            result.add_warning(ValidationError::PerformanceWarning {
                issue: format!("Large table with {} rows", data.row_count()),
                recommendation: "Consider using streaming API or pagination".to_string(),
            });
        }

        if data.column_count() > thresholds.large_table_columns {
            result.add_warning(ValidationError::PerformanceWarning {
                issue: format!("Wide table with {} columns", data.column_count()),
                recommendation: "Consider splitting into multiple tables or using horizontal scrolling".to_string(),
            });
        }

        // Check for very large cells
        let mut large_cell_count = 0;
        for row in &data.rows {
            for cell in row {
                if cell.len() > thresholds.very_large_cell_chars {
                    large_cell_count += 1;
                }
            }
        }

        if large_cell_count > 0 {
            result.add_warning(ValidationError::PerformanceWarning {
                issue: format!("{} cells with more than {} characters", large_cell_count, thresholds.very_large_cell_chars),
                recommendation: "Consider truncating long content or using wrapping".to_string(),
            });
        }

        // Estimate memory usage
        let estimated_memory = self.estimate_memory_usage(data);
        if estimated_memory > 10_000_000 { // 10MB
            result.add_warning(ValidationError::PerformanceWarning {
                issue: format!("Estimated memory usage: {} bytes", estimated_memory),
                recommendation: "Consider processing in chunks or using streaming".to_string(),
            });
        }
    }

    fn check_data_integrity(&self, data: &crate::TableData, result: &mut ValidationResult) {
        // Check for duplicate rows
        let mut row_hashes = HashMap::new();
        for (i, row) in data.rows.iter().enumerate() {
            let row_hash = self.hash_row(row);
            if let Some(&first_occurrence) = row_hashes.get(&row_hash) {
                result.add_info(ValidationError::DataIntegrityIssue {
                    issue: format!("Row {} appears to be duplicate of row {}", i, first_occurrence),
                });
            } else {
                row_hashes.insert(row_hash, i);
            }
        }

        // Check for suspiciously uniform data
        if data.row_count() > 5 {
            let mut uniform_columns = 0;
            for col_idx in 0..data.column_count() {
                let first_cell = data.rows.get(0).and_then(|row| row.get(col_idx));
                let all_same = data.rows.iter().skip(1).all(|row| {
                    row.get(col_idx) == first_cell
                });
                if all_same {
                    uniform_columns += 1;
                }
            }

            if uniform_columns > data.column_count() / 2 {
                result.add_info(ValidationError::DataIntegrityIssue {
                    issue: "Many columns have identical values in all rows".to_string(),
                });
            }
        }
    }

    fn is_emoji(&self, ch: char) -> bool {
        matches!(ch as u32, 
            0x1F600..=0x1F64F |  // Emoticons
            0x1F300..=0x1F5FF |  // Miscellaneous Symbols and Pictographs
            0x1F680..=0x1F6FF |  // Transport and Map Symbols
            0x1F1E6..=0x1F1FF    // Regional Indicator Symbols
        )
    }

    fn is_combining_character(&self, ch: char) -> bool {
        matches!(ch as u32,
            0x0300..=0x036F |  // Combining Diacritical Marks
            0x1AB0..=0x1AFF |  // Combining Diacritical Marks Extended
            0x1DC0..=0x1DFF |  // Combining Diacritical Marks Supplement
            0x20D0..=0x20FF |  // Combining Diacritical Marks for Symbols
            0xFE20..=0xFE2F    // Combining Half Marks
        )
    }

    fn estimate_memory_usage(&self, data: &crate::TableData) -> usize {
        let content_size: usize = data.rows.iter()
            .map(|row| row.iter().map(|cell| cell.len()).sum::<usize>())
            .sum();
        
        // Rough estimate including overhead
        content_size * 2 + (data.row_count() * data.column_count() * 64)
    }

    fn hash_row(&self, row: &[String]) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        row.hash(&mut hasher);
        hasher.finish()
    }
}

/// Quick validation functions
pub fn validate_table(data: &crate::TableData) -> ValidationResult {
    let validator = TableValidator::new(ValidationConfig::default());
    validator.validate(data)
}

pub fn validate_table_strict(data: &crate::TableData) -> ValidationResult {
    let validator = TableValidator::new(ValidationConfig::strict());
    validator.validate(data)
}

pub fn validate_table_for_web(data: &crate::TableData) -> ValidationResult {
    let validator = TableValidator::new(ValidationConfig::web_optimized());
    validator.validate(data)
}

/// Check if table is valid for rendering
pub fn is_table_valid(data: &crate::TableData) -> bool {
    let result = validate_table(data);
    result.is_valid
}

/// Get quick validation summary
pub fn validation_summary(data: &crate::TableData) -> String {
    let result = validate_table(data);
    
    if result.is_valid && !result.has_issues() {
        format!("✅ Valid table ({} rows, {} cols, score: {}/100)",
                data.row_count(), data.column_count(), result.performance_score)
    } else {
        format!("⚠️  Issues found: {} errors, {} warnings (score: {}/100)",
                result.error_count(), result.warning_count(), result.performance_score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_display() {
        let error = ValidationError::EmptyTable;
        assert!(error.to_string().contains("empty"));

        let inconsistent = ValidationError::InconsistentRowLength {
            row_index: 1,
            expected: 3,
            actual: 2,
        };
        assert!(inconsistent.to_string().contains("Row 1"));
    }

    #[test]
    fn test_validation_result() {
        let mut result = ValidationResult::new();
        assert!(result.is_valid);
        assert_eq!(result.performance_score, 100);

        result.add_error(ValidationError::EmptyTable);
        assert!(!result.is_valid);
        assert!(result.performance_score < 100);

        result.add_warning(ValidationError::PerformanceWarning {
            issue: "test".to_string(),
            recommendation: "test".to_string(),
        });
        assert!(result.has_issues());
    }

    #[test]
    fn test_validation_config_presets() {
        let default = ValidationConfig::default();
        assert!(default.max_columns.is_some());
        assert!(!default.allow_inconsistent_rows);

        let strict = ValidationConfig::strict();
        assert!(strict.max_columns.unwrap() < default.max_columns.unwrap());

        let permissive = ValidationConfig::permissive();
        assert!(permissive.max_columns.is_none());
        assert!(permissive.allow_inconsistent_rows);
    }

    #[test]
    fn test_valid_table_validation() {
        let data = crate::TableData::new(vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "25".to_string()],
        ]);

        let validator = TableValidator::new(ValidationConfig::default());
        let result = validator.validate(&data);

        assert!(result.is_valid);
        assert_eq!(result.errors.len(), 0);
    }

    #[test]
    fn test_empty_table_validation() {
        let data = crate::TableData::new(vec![]);
        let validator = TableValidator::new(ValidationConfig::default());
        let result = validator.validate(&data);

        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| matches!(e, ValidationError::EmptyTable)));
    }

    #[test]
    fn test_inconsistent_row_validation() {
        let data = crate::TableData::new(vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["1".to_string(), "2".to_string()], // Missing column
        ]);

        let validator = TableValidator::new(ValidationConfig::default());
        let result = validator.validate(&data);

        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| matches!(e, ValidationError::InconsistentRowLength { .. })));
    }

    #[test]
    fn test_unicode_content_detection() {
        let data = crate::TableData::new(vec![
            vec!["Name".to_string(), "Location".to_string()],
            vec!["Alice".to_string(), "北京".to_string()], // Chinese characters
            vec!["Bob".to_string(), "Café".to_string()],   // Accented character
        ]);

        let validator = TableValidator::new(ValidationConfig::default());
        let result = validator.validate(&data);

        assert!(result.is_valid); // Unicode content should not be an error
        assert!(result.info.iter().any(|i| matches!(i, ValidationError::InvalidCellContent { .. })));
    }

    #[test]
    fn test_performance_warnings() {
        // Create a large table
        let rows: Vec<Vec<String>> = (0..2000)
            .map(|i| vec![format!("Row {}", i), "Data".to_string()])
            .collect();
        let data = crate::TableData::new(rows);

        let validator = TableValidator::new(ValidationConfig::default());
        let result = validator.validate(&data);

        assert!(result.warnings.iter().any(|w| matches!(w, ValidationError::PerformanceWarning { .. })));
    }

    #[test]
    fn test_quick_validation_functions() {
        let valid_data = crate::TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ]);

        assert!(is_table_valid(&valid_data));

        let summary = validation_summary(&valid_data);
        assert!(summary.contains("✅"));

        let empty_data = crate::TableData::new(vec![]);
        assert!(!is_table_valid(&empty_data));

        let empty_summary = validation_summary(&empty_data);
        assert!(empty_summary.contains("Issues found"));
    }

    #[test]
    fn test_validation_report() {
        let data = crate::TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string()], // Inconsistent
        ]);

        let result = validate_table(&data);
        let report = result.report();

        assert!(report.contains("Validation Report"));
        assert!(report.contains("ERRORS"));
        assert!(report.contains("Performance score"));
    }

    #[test]
    fn test_column_config_validation() {
        let data = crate::TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ]);

        // Wrong number of column configs
        let configs = vec![crate::alignment::ColumnConfig::new()]; // Only 1 config for 2 columns
        
        let validator = TableValidator::new(ValidationConfig::default());
        let result = validator.validate_with_config(&data, &configs);

        assert!(result.warnings.iter().any(|w| matches!(w, ValidationError::ConfigurationIssue { .. })));
    }
}