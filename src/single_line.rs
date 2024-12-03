#[derive(Debug, Clone)]
pub struct SingleLineConfig {
    pub separator: String,
    pub field_separator: String,
    pub quote_fields: bool,
    pub quote_character: char,
    pub escape_quotes: bool,
    pub include_headers: bool,
    pub max_field_width: Option<usize>,
}

impl Default for SingleLineConfig {
    fn default() -> Self {
        Self {
            separator: " | ".to_string(),
            field_separator: " ".to_string(),
            quote_fields: false,
            quote_character: '"',
            escape_quotes: true,
            include_headers: true,
            max_field_width: None,
        }
    }
}

impl SingleLineConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_separator(mut self, separator: &str) -> Self {
        self.separator = separator.to_string();
        self
    }
    
    pub fn with_field_separator(mut self, separator: &str) -> Self {
        self.field_separator = separator.to_string();
        self
    }
    
    pub fn with_quotes(mut self, quote_char: char) -> Self {
        self.quote_fields = true;
        self.quote_character = quote_char;
        self
    }
    
    pub fn with_escape_quotes(mut self, escape: bool) -> Self {
        self.escape_quotes = escape;
        self
    }
    
    pub fn without_headers(mut self) -> Self {
        self.include_headers = false;
        self
    }
    
    pub fn with_max_field_width(mut self, width: usize) -> Self {
        self.max_field_width = Some(width);
        self
    }
    
    /// CSV-style configuration
    pub fn csv() -> Self {
        Self::new()
            .with_separator(",")
            .with_quotes('"')
            .with_field_separator("")
    }
    
    /// TSV-style configuration
    pub fn tsv() -> Self {
        Self::new()
            .with_separator("\t")
            .with_field_separator("")
            .without_headers()
    }
    
    /// Pipe-separated values
    pub fn psv() -> Self {
        Self::new()
            .with_separator(" | ")
            .with_field_separator(" ")
    }
    
    /// Space-separated compact format
    pub fn compact() -> Self {
        Self::new()
            .with_separator(" ")
            .with_field_separator("")
            .with_max_field_width(15)
    }
    
    /// JSON-like array format
    pub fn json_array() -> Self {
        Self::new()
            .with_separator(", ")
            .with_quotes('"')
            .with_field_separator("")
    }
}

/// Render table in single line format (horizontal layout)
pub fn render_single_line_table(
    data: &crate::TableData,
    config: &SingleLineConfig,
) -> Result<String, String> {
    crate::validate_table_data(data)?;
    
    if data.is_empty() {
        return Ok(String::new());
    }
    
    let mut result = String::new();
    
    // Process each row
    for (row_idx, row) in data.rows.iter().enumerate() {
        // Skip header if not included and this is first row
        if !config.include_headers && row_idx == 0 {
            continue;
        }
        
        let processed_row = process_row(row, config);
        result.push_str(&processed_row);
        
        // Add newline except for last row
        if row_idx < data.rows.len() - 1 {
            result.push('\n');
        }
    }
    
    Ok(result)
}

/// Process a single row according to configuration
fn process_row(row: &[String], config: &SingleLineConfig) -> String {
    let processed_fields: Vec<String> = row.iter()
        .map(|field| process_field(field, config))
        .collect();
    
    processed_fields.join(&config.separator)
}

/// Process a single field according to configuration
fn process_field(field: &str, config: &SingleLineConfig) -> String {
    let mut processed = field.to_string();
    
    // Apply field width limit
    if let Some(max_width) = config.max_field_width {
        if processed.len() > max_width {
            processed.truncate(max_width.saturating_sub(3));
            processed.push_str("...");
        }
    }
    
    // Handle quotes
    if config.quote_fields {
        if config.escape_quotes {
            processed = processed.replace(config.quote_character, &format!("\\{}", config.quote_character));
        }
        processed = format!("{}{}{}", config.quote_character, processed, config.quote_character);
    }
    
    processed
}

/// Render table as compact single line (all data in one line)
pub fn render_compact_single_line(
    data: &crate::TableData,
    config: &SingleLineConfig,
) -> Result<String, String> {
    crate::validate_table_data(data)?;
    
    if data.is_empty() {
        return Ok(String::new());
    }
    
    let mut result = String::new();
    let mut first_row = true;
    
    for row in &data.rows {
        if !first_row {
            result.push_str(&config.field_separator);
        }
        first_row = false;
        
        let processed_row = process_row(row, config);
        result.push_str(&processed_row);
    }
    
    Ok(result)
}

/// Convert table to key-value pairs (first column as keys, rest as values)
pub fn render_key_value_pairs(
    data: &crate::TableData,
    config: &SingleLineConfig,
) -> Result<String, String> {
    crate::validate_table_data(data)?;
    
    if data.is_empty() {
        return Ok(String::new());
    }
    
    if data.column_count() < 2 {
        return Err("Key-value format requires at least 2 columns".to_string());
    }
    
    let mut result = String::new();
    
    // Skip header row if configured
    let start_idx = if config.include_headers { 0 } else { 1 };
    
    for (row_idx, row) in data.rows.iter().enumerate().skip(start_idx) {
        if row.is_empty() {
            continue;
        }
        
        let key = process_field(&row[0], config);
        let values: Vec<String> = row.iter().skip(1)
            .map(|value| process_field(value, config))
            .collect();
        
        result.push_str(&format!("{}: {}", key, values.join(&config.separator)));
        
        if row_idx < data.rows.len() - 1 {
            result.push('\n');
        }
    }
    
    Ok(result)
}

/// Transpose table and render as single line
pub fn render_transposed_single_line(
    data: &crate::TableData,
    config: &SingleLineConfig,
) -> Result<String, String> {
    crate::validate_table_data(data)?;
    
    if data.is_empty() {
        return Ok(String::new());
    }
    
    // Create transposed data
    let mut transposed_rows = Vec::new();
    
    for col_idx in 0..data.column_count() {
        let mut transposed_row = Vec::new();
        
        for row in &data.rows {
            if col_idx < row.len() {
                transposed_row.push(row[col_idx].clone());
            } else {
                transposed_row.push(String::new());
            }
        }
        
        transposed_rows.push(transposed_row);
    }
    
    // Render transposed data
    let transposed_data = crate::TableData::new(transposed_rows);
    render_single_line_table(&transposed_data, config)
}

/// Summary statistics renderer for single line output
pub struct SummaryRenderer;

impl SummaryRenderer {
    /// Render basic table statistics in single line
    pub fn render_stats(data: &crate::TableData) -> String {
        if data.is_empty() {
            return "Empty table (0 rows, 0 columns)".to_string();
        }
        
        let row_count = data.row_count();
        let col_count = data.column_count();
        
        // Calculate basic statistics
        let total_cells = row_count * col_count;
        let empty_cells = data.rows.iter()
            .flat_map(|row| row.iter())
            .filter(|cell| cell.is_empty())
            .count();
        
        let avg_cell_length = data.rows.iter()
            .flat_map(|row| row.iter())
            .map(|cell| cell.len())
            .sum::<usize>() as f64 / total_cells as f64;
        
        format!(
            "Table: {} rows × {} columns, {} total cells, {} empty, avg length: {:.1}",
            row_count, col_count, total_cells, empty_cells, avg_cell_length
        )
    }
    
    /// Render column statistics
    pub fn render_column_stats(data: &crate::TableData) -> Result<String, String> {
        if data.is_empty() {
            return Ok("No columns".to_string());
        }
        
        let mut stats = Vec::new();
        
        for col_idx in 0..data.column_count() {
            let column_data: Vec<&String> = data.rows.iter()
                .filter_map(|row| row.get(col_idx))
                .collect();
            
            let max_len = column_data.iter().map(|cell| cell.len()).max().unwrap_or(0);
            let min_len = column_data.iter().map(|cell| cell.len()).min().unwrap_or(0);
            let avg_len = column_data.iter().map(|cell| cell.len()).sum::<usize>() as f64 / column_data.len() as f64;
            let empty_count = column_data.iter().filter(|cell| cell.is_empty()).count();
            
            stats.push(format!(
                "Col{}: max={}, min={}, avg={:.1}, empty={}",
                col_idx, max_len, min_len, avg_len, empty_count
            ));
        }
        
        Ok(stats.join(" | "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_config() {
        let config = SingleLineConfig::new()
            .with_separator(" | ")
            .with_quotes('"')
            .with_max_field_width(10);
        
        assert_eq!(config.separator, " | ");
        assert!(config.quote_fields);
        assert_eq!(config.quote_character, '"');
        assert_eq!(config.max_field_width, Some(10));
    }

    #[test]
    fn test_render_single_line_table() {
        let data = crate::TableData::new(vec![
            vec!["Name".to_string(), "Age".to_string(), "City".to_string()],
            vec!["Alice".to_string(), "30".to_string(), "New York".to_string()],
            vec!["Bob".to_string(), "25".to_string(), "London".to_string()],
        ]);
        
        let config = SingleLineConfig::new();
        let result = render_single_line_table(&data, &config).unwrap();
        
        assert!(result.contains("Name | Age | City"));
        assert!(result.contains("Alice | 30 | New York"));
        assert!(result.contains("Bob | 25 | London"));
        
        // Should have multiple lines
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 3);
    }

    #[test]
    fn test_csv_format() {
        let data = crate::TableData::new(vec![
            vec!["Product".to_string(), "Price".to_string()],
            vec!["Widget".to_string(), "$19.99".to_string()],
            vec!["Gadget".to_string(), "$29.99".to_string()],
        ]);
        
        let config = SingleLineConfig::csv();
        let result = render_single_line_table(&data, &config).unwrap();
        
        assert!(result.contains("\"Product\",\"Price\""));
        assert!(result.contains("\"Widget\",\"$19.99\""));
        assert!(result.contains("\"Gadget\",\"$29.99\""));
    }

    #[test]
    fn test_tsv_format() {
        let data = crate::TableData::new(vec![
            vec!["Header1".to_string(), "Header2".to_string()],
            vec!["Data1".to_string(), "Data2".to_string()],
        ]);
        
        let config = SingleLineConfig::tsv();
        let result = render_single_line_table(&data, &config).unwrap();
        
        // TSV should skip headers
        assert!(!result.contains("Header1"));
        assert!(result.contains("Data1\tData2"));
    }

    #[test]
    fn test_compact_format() {
        let data = crate::TableData::new(vec![
            vec!["Very Long Field Name That Exceeds Limit".to_string(), "Short".to_string()],
            vec!["Another Long Field".to_string(), "B".to_string()],
        ]);
        
        let config = SingleLineConfig::compact();
        let result = render_single_line_table(&data, &config).unwrap();
        
        // Should truncate long fields
        assert!(result.contains("Very Long Fi..."));
        assert!(result.contains("Another Lon..."));
        assert!(result.contains("Short"));
    }

    #[test]
    fn test_render_compact_single_line() {
        let data = crate::TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string(), "2".to_string()],
            vec!["3".to_string(), "4".to_string()],
        ]);
        
        let config = SingleLineConfig::new();
        let result = render_compact_single_line(&data, &config).unwrap();
        
        // Should be all on one line
        assert!(!result.contains('\n'));
        assert!(result.contains("A | B"));
        assert!(result.contains("1 | 2"));
        assert!(result.contains("3 | 4"));
    }

    #[test]
    fn test_key_value_pairs() {
        let data = crate::TableData::new(vec![
            vec!["Key".to_string(), "Value1".to_string(), "Value2".to_string()],
            vec!["Name".to_string(), "Alice".to_string(), "Engineer".to_string()],
            vec!["Age".to_string(), "30".to_string(), "Years".to_string()],
        ]);
        
        let config = SingleLineConfig::new().without_headers();
        let result = render_key_value_pairs(&data, &config).unwrap();
        
        assert!(result.contains("Name: Alice | Engineer"));
        assert!(result.contains("Age: 30 | Years"));
    }

    #[test]
    fn test_transposed_single_line() {
        let data = crate::TableData::new(vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["X".to_string(), "Y".to_string(), "Z".to_string()],
        ]);
        
        let config = SingleLineConfig::new();
        let result = render_transposed_single_line(&data, &config).unwrap();
        
        // Should show columns as rows
        assert!(result.contains("A | 1 | X"));
        assert!(result.contains("B | 2 | Y"));
        assert!(result.contains("C | 3 | Z"));
        
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 3); // Three columns become three rows
    }

    #[test]
    fn test_quote_escaping() {
        let data = crate::TableData::new(vec![
            vec!["Field with \"quotes\"".to_string(), "Normal".to_string()],
        ]);
        
        let config = SingleLineConfig::new().with_quotes('"');
        let result = render_single_line_table(&data, &config).unwrap();
        
        assert!(result.contains("\"Field with \\\"quotes\\\"\""));
        assert!(result.contains("\"Normal\""));
    }

    #[test]
    fn test_summary_stats() {
        let data = crate::TableData::new(vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "".to_string()], // Empty cell
            vec!["Carol".to_string(), "35".to_string()],
        ]);
        
        let stats = SummaryRenderer::render_stats(&data);
        
        assert!(stats.contains("4 rows"));
        assert!(stats.contains("2 columns"));
        assert!(stats.contains("8 total cells"));
        assert!(stats.contains("1 empty"));
    }

    #[test]
    fn test_column_stats() {
        let data = crate::TableData::new(vec![
            vec!["Short".to_string(), "Very Long Text".to_string()],
            vec!["A".to_string(), "Medium".to_string()],
            vec!["".to_string(), "B".to_string()], // Empty in first column
        ]);
        
        let stats = SummaryRenderer::render_column_stats(&data).unwrap();
        
        assert!(stats.contains("Col0:"));
        assert!(stats.contains("Col1:"));
        assert!(stats.contains("empty=1")); // First column has one empty cell
    }

    #[test]
    fn test_empty_table_handling() {
        let data = crate::TableData::new(vec![]);
        
        let config = SingleLineConfig::new();
        let result = render_single_line_table(&data, &config).unwrap();
        assert_eq!(result, "");
        
        let stats = SummaryRenderer::render_stats(&data);
        assert_eq!(stats, "Empty table (0 rows, 0 columns)");
    }

    #[test]
    fn test_max_field_width() {
        let data = crate::TableData::new(vec![
            vec!["This is a very long field that should be truncated".to_string()],
        ]);
        
        let config = SingleLineConfig::new().with_max_field_width(10);
        let result = render_single_line_table(&data, &config).unwrap();
        
        assert!(result.contains("This is..."));
        assert!(!result.contains("truncated"));
    }
}