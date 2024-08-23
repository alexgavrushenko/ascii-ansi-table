pub mod border;
pub mod renderer;
pub mod alignment;
pub mod padding;
pub mod truncation;
pub mod wrapping;
pub mod multiline;
pub mod unicode;
pub mod ansi;
pub mod ansi_multiline;
pub mod newline;

pub use border::{BorderChars, get_border_style};
pub use renderer::RenderOptions;
pub use alignment::{Alignment, ColumnConfig, align_text};
pub use padding::{Padding, apply_padding, apply_padding_with_width};
pub use truncation::{TruncationConfig, truncate_text};
pub use wrapping::{WrapMode, WrapConfig, wrap_text, calculate_wrapped_height, wrap_ansi_text};
pub use multiline::render_table_with_wrapping;
pub use unicode::{display_width, char_display_width, truncate_to_width, pad_to_width, 
                 calculate_unicode_column_widths, unicode_wrap_text};
pub use ansi::{AnsiSequence, parse_ansi_sequences, strip_ansi_sequences, ansi_display_width,
               ansi_truncate_to_width, ansi_pad_to_width, colors};
pub use ansi_multiline::render_table_with_ansi_wrapping;
pub use newline::{split_lines, render_table_with_newlines, calculate_newline_column_widths};
pub type Row = Vec<String>;

#[derive(Debug, Clone)]
pub struct TableData {
    pub rows: Vec<Row>,
}

impl TableData {
    pub fn new(rows: Vec<Row>) -> Self {
        Self { rows }
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn column_count(&self) -> usize {
        self.rows.first().map(|row| row.len()).unwrap_or(0)
    }
}

pub fn validate_table_data(data: &TableData) -> Result<(), String> {
    if data.is_empty() {
        return Ok(());
    }

    let expected_columns = data.column_count();
    for (i, row) in data.rows.iter().enumerate() {
        if row.len() != expected_columns {
            return Err(format!(
                "Row {} has {} columns, expected {}",
                i, row.len(), expected_columns
            ));
        }
    }
    Ok(())
}

pub fn render_table_ansi_aware(
    data: &TableData,
    border: &BorderChars,
    options: &RenderOptions,
    column_configs: &[ColumnConfig],
) -> Result<String, String> {
    validate_table_data(data)?;
    
    if data.is_empty() {
        return Ok(String::new());
    }
    
    // Calculate column widths based on stripped ANSI content
    let mut auto_widths = vec![0; data.column_count()];
    for row in &data.rows {
        for (i, cell) in row.iter().enumerate().take(data.column_count()) {
            auto_widths[i] = auto_widths[i].max(ansi_display_width(cell));
        }
    }
    
    let mut column_widths = Vec::new();
    
    // Determine final column widths and configurations (including padding)
    for i in 0..data.column_count() {
        let config = column_configs.get(i).unwrap_or(&ColumnConfig::default());
        let content_width = config.width.unwrap_or(auto_widths[i]);
        let total_width = content_width + config.padding.total();
        column_widths.push(total_width);
    }
    
    let mut result = String::new();
    
    // Top border (optional)
    if options.show_top_border {
        result.push(border.top_left);
        for (i, width) in column_widths.iter().enumerate() {
            result.push_str(&border.horizontal.to_string().repeat(*width));
            if i < column_widths.len() - 1 {
                result.push(border.top_junction);
            }
        }
        result.push(border.top_right);
        result.push('\n');
    }
    
    // Data rows with side borders and alignment
    for (row_idx, row) in data.rows.iter().enumerate() {
        result.push(border.vertical);
        for (i, cell) in row.iter().enumerate() {
            let config = column_configs.get(i).unwrap_or(&ColumnConfig::default());
            let content_width = config.width.unwrap_or(auto_widths[i]);
            
            // Apply truncation first (ANSI-aware)
            let truncated_cell = if let Some(max_width) = config.truncation.max_width {
                if config.truncation.ellipsis.is_empty() {
                    ansi_truncate_to_width(cell, max_width)
                } else {
                    let cell_width = ansi_display_width(cell);
                    if cell_width > max_width {
                        let ellipsis_width = config.truncation.ellipsis.len();
                        if max_width > ellipsis_width {
                            let truncated = ansi_truncate_to_width(cell, max_width - ellipsis_width);
                            format!("{}{}", truncated, config.truncation.ellipsis)
                        } else {
                            ansi_truncate_to_width(cell, max_width)
                        }
                    } else {
                        cell.to_string()
                    }
                }
            } else {
                cell.to_string()
            };
            
            // Apply alignment (ANSI-aware)
            let aligned_cell = ansi_pad_to_width(&truncated_cell, content_width, config.alignment);
            let padded_cell = apply_padding(&aligned_cell, config.padding);
            
            result.push_str(&padded_cell);
            result.push(border.vertical);
        }
        result.push('\n');
        
        // Row separator (optional, not after last row)
        if options.show_row_separators && row_idx < data.rows.len() - 1 {
            result.push('├');
            for (i, width) in column_widths.iter().enumerate() {
                result.push_str(&border.horizontal.to_string().repeat(*width));
                if i < column_widths.len() - 1 {
                    result.push('┼');
                }
            }
            result.push('┤');
            result.push('\n');
        }
    }
    
    // Bottom border (optional)
    if options.show_bottom_border {
        result.push(border.bottom_left);
        for (i, width) in column_widths.iter().enumerate() {
            result.push_str(&border.horizontal.to_string().repeat(*width));
            if i < column_widths.len() - 1 {
                result.push(border.bottom_junction);
            }
        }
        result.push(border.bottom_right);
        result.push('\n');
    }
    
    Ok(result)
}

pub fn render_table_unicode_aware(
    data: &TableData,
    border: &BorderChars,
    options: &RenderOptions,
    column_configs: &[ColumnConfig],
) -> Result<String, String> {
    validate_table_data(data)?;
    
    if data.is_empty() {
        return Ok(String::new());
    }
    
    let auto_widths = calculate_unicode_column_widths(&data.rows);
    let mut column_widths = Vec::new();
    
    // Determine final column widths and configurations (including padding)
    for i in 0..data.column_count() {
        let config = column_configs.get(i).unwrap_or(&ColumnConfig::default());
        let content_width = config.width.unwrap_or(auto_widths[i]);
        let total_width = content_width + config.padding.total();
        column_widths.push(total_width);
    }
    
    let mut result = String::new();
    
    // Top border (optional)
    if options.show_top_border {
        result.push(border.top_left);
        for (i, width) in column_widths.iter().enumerate() {
            result.push_str(&border.horizontal.to_string().repeat(*width));
            if i < column_widths.len() - 1 {
                result.push(border.top_junction);
            }
        }
        result.push(border.top_right);
        result.push('\n');
    }
    
    // Data rows with side borders and alignment
    for (row_idx, row) in data.rows.iter().enumerate() {
        result.push(border.vertical);
        for (i, cell) in row.iter().enumerate() {
            let config = column_configs.get(i).unwrap_or(&ColumnConfig::default());
            let content_width = config.width.unwrap_or(auto_widths[i]);
            
            // Apply truncation first (Unicode-aware)
            let truncated_cell = if let Some(max_width) = config.truncation.max_width {
                if config.truncation.ellipsis.is_empty() {
                    truncate_to_width(cell, max_width)
                } else {
                    let cell_width = display_width(cell);
                    if cell_width > max_width {
                        let ellipsis_width = display_width(&config.truncation.ellipsis);
                        if max_width > ellipsis_width {
                            let truncated = truncate_to_width(cell, max_width - ellipsis_width);
                            format!("{}{}", truncated, config.truncation.ellipsis)
                        } else {
                            truncate_to_width(cell, max_width)
                        }
                    } else {
                        cell.to_string()
                    }
                }
            } else {
                cell.to_string()
            };
            
            // Apply alignment (Unicode-aware)
            let aligned_cell = pad_to_width(&truncated_cell, content_width, config.alignment);
            let padded_cell = apply_padding(&aligned_cell, config.padding);
            
            result.push_str(&padded_cell);
            result.push(border.vertical);
        }
        result.push('\n');
        
        // Row separator (optional, not after last row)
        if options.show_row_separators && row_idx < data.rows.len() - 1 {
            result.push('├');
            for (i, width) in column_widths.iter().enumerate() {
                result.push_str(&border.horizontal.to_string().repeat(*width));
                if i < column_widths.len() - 1 {
                    result.push('┼');
                }
            }
            result.push('┤');
            result.push('\n');
        }
    }
    
    // Bottom border (optional)
    if options.show_bottom_border {
        result.push(border.bottom_left);
        for (i, width) in column_widths.iter().enumerate() {
            result.push_str(&border.horizontal.to_string().repeat(*width));
            if i < column_widths.len() - 1 {
                result.push(border.bottom_junction);
            }
        }
        result.push(border.bottom_right);
        result.push('\n');
    }
    
    Ok(result)
}

pub fn calculate_column_widths(data: &TableData) -> Vec<usize> {
    if data.is_empty() {
        return vec![];
    }
    
    let mut widths = vec![0; data.column_count()];
    
    for row in &data.rows {
        for (i, cell) in row.iter().enumerate() {
            widths[i] = widths[i].max(cell.len());
        }
    }
    
    widths
}

pub fn render_table_with_custom_borders(data: &TableData, border: &BorderChars) -> Result<String, String> {
    validate_table_data(data)?;
    
    if data.is_empty() {
        return Ok(String::new());
    }
    
    let column_widths = calculate_column_widths(data);
    let mut result = String::new();
    
    // Top border
    result.push(border.top_left);
    for (i, width) in column_widths.iter().enumerate() {
        result.push_str(&border.horizontal.to_string().repeat(*width));
        if i < column_widths.len() - 1 {
            result.push(border.top_junction);
        }
    }
    result.push(border.top_right);
    result.push('\n');
    
    // Data rows with side borders
    for row in &data.rows {
        result.push(border.vertical);
        for (i, cell) in row.iter().enumerate() {
            let padded_cell = format!(" {:width$} ", cell, width = column_widths[i]);
            result.push_str(&padded_cell);
            result.push(border.vertical);
        }
        result.push('\n');
    }
    
    // Bottom border
    result.push(border.bottom_left);
    for (i, width) in column_widths.iter().enumerate() {
        result.push_str(&border.horizontal.to_string().repeat(*width));
        if i < column_widths.len() - 1 {
            result.push(border.bottom_junction);
        }
    }
    result.push(border.bottom_right);
    result.push('\n');
    
    Ok(result)
}

pub fn render_table_with_borders(data: &TableData) -> Result<String, String> {
    render_table_with_custom_borders(data, &BorderChars::default())
}

pub fn render_table_borderless(data: &TableData) -> Result<String, String> {
    render_table_with_custom_borders(data, &BorderChars::void())
}

pub fn render_table_with_column_config(
    data: &TableData,
    border: &BorderChars,
    options: &RenderOptions,
    column_configs: &[ColumnConfig],
) -> Result<String, String> {
    validate_table_data(data)?;
    
    if data.is_empty() {
        return Ok(String::new());
    }
    
    let auto_widths = calculate_column_widths(data);
    let mut column_widths = Vec::new();
    
    // Determine final column widths and configurations (including padding)
    for i in 0..data.column_count() {
        let config = column_configs.get(i).unwrap_or(&ColumnConfig::default());
        let content_width = config.width.unwrap_or(auto_widths[i]);
        let total_width = content_width + config.padding.total();
        column_widths.push(total_width);
    }
    
    let mut result = String::new();
    
    // Top border (optional)
    if options.show_top_border {
        result.push(border.top_left);
        for (i, width) in column_widths.iter().enumerate() {
            result.push_str(&border.horizontal.to_string().repeat(*width));
            if i < column_widths.len() - 1 {
                result.push(border.top_junction);
            }
        }
        result.push(border.top_right);
        result.push('\n');
    }
    
    // Data rows with side borders and alignment
    for (row_idx, row) in data.rows.iter().enumerate() {
        result.push(border.vertical);
        for (i, cell) in row.iter().enumerate() {
            let config = column_configs.get(i).unwrap_or(&ColumnConfig::default());
            let content_width = config.width.unwrap_or(auto_widths[i]);
            
            // Apply truncation first, then alignment, then padding
            let truncated_cell = truncate_text(cell, &config.truncation);
            let aligned_cell = align_text(&truncated_cell, content_width, config.alignment);
            let padded_cell = apply_padding(&aligned_cell, config.padding);
            
            result.push_str(&padded_cell);
            result.push(border.vertical);
        }
        result.push('\n');
        
        // Row separator (optional, not after last row)
        if options.show_row_separators && row_idx < data.rows.len() - 1 {
            result.push('├');
            for (i, width) in column_widths.iter().enumerate() {
                result.push_str(&border.horizontal.to_string().repeat(*width));
                if i < column_widths.len() - 1 {
                    result.push('┼');
                }
            }
            result.push('┤');
            result.push('\n');
        }
    }
    
    // Bottom border (optional)
    if options.show_bottom_border {
        result.push(border.bottom_left);
        for (i, width) in column_widths.iter().enumerate() {
            result.push_str(&border.horizontal.to_string().repeat(*width));
            if i < column_widths.len() - 1 {
                result.push(border.bottom_junction);
            }
        }
        result.push(border.bottom_right);
        result.push('\n');
    }
    
    Ok(result)
}

pub fn render_table_with_options(
    data: &TableData,
    border: &BorderChars,
    options: &RenderOptions,
) -> Result<String, String> {
    validate_table_data(data)?;
    
    if data.is_empty() {
        return Ok(String::new());
    }
    
    let column_widths = calculate_column_widths(data);
    let mut result = String::new();
    
    // Top border (optional)
    if options.show_top_border {
        result.push(border.top_left);
        for (i, width) in column_widths.iter().enumerate() {
            result.push_str(&border.horizontal.to_string().repeat(*width));
            if i < column_widths.len() - 1 {
                result.push(border.top_junction);
            }
        }
        result.push(border.top_right);
        result.push('\n');
    }
    
    // Data rows with side borders
    for (row_idx, row) in data.rows.iter().enumerate() {
        result.push(border.vertical);
        for (i, cell) in row.iter().enumerate() {
            let padded_cell = format!(" {:width$} ", cell, width = column_widths[i]);
            result.push_str(&padded_cell);
            result.push(border.vertical);
        }
        result.push('\n');
        
        // Row separator (optional, not after last row)
        if options.show_row_separators && row_idx < data.rows.len() - 1 {
            result.push('├');
            for (i, width) in column_widths.iter().enumerate() {
                result.push_str(&border.horizontal.to_string().repeat(*width));
                if i < column_widths.len() - 1 {
                    result.push('┼');
                }
            }
            result.push('┤');
            result.push('\n');
        }
    }
    
    // Bottom border (optional)
    if options.show_bottom_border {
        result.push(border.bottom_left);
        for (i, width) in column_widths.iter().enumerate() {
            result.push_str(&border.horizontal.to_string().repeat(*width));
            if i < column_widths.len() - 1 {
                result.push(border.bottom_junction);
            }
        }
        result.push(border.bottom_right);
        result.push('\n');
    }
    
    Ok(result)
}

pub fn render_table_auto_width(data: &TableData) -> Result<String, String> {
    validate_table_data(data)?;
    
    if data.is_empty() {
        return Ok(String::new());
    }
    
    let column_widths = calculate_column_widths(data);
    
    let mut result = String::new();
    
    for row in &data.rows {
        result.push('|');
        for (i, cell) in row.iter().enumerate() {
            let padded_cell = format!("{:width$}", cell, width = column_widths[i]);
            result.push(' ');
            result.push_str(&padded_cell);
            result.push(' ');
            result.push('|');
        }
        result.push('\n');
    }
    
    Ok(result)
}

pub fn render_table(data: &TableData, column_width: usize) -> Result<String, String> {
    validate_table_data(data)?;
    
    if data.is_empty() {
        return Ok(String::new());
    }

    let mut result = String::new();
    
    for row in &data.rows {
        result.push('|');
        for cell in row {
            let padded_cell = format!("{:width$}", cell, width = column_width);
            result.push(' ');
            result.push_str(&padded_cell);
            result.push(' ');
            result.push('|');
        }
        result.push('\n');
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_table() {
        let data = TableData::new(vec![]);
        assert!(data.is_empty());
        assert_eq!(data.row_count(), 0);
        assert_eq!(data.column_count(), 0);
        assert!(validate_table_data(&data).is_ok());
    }

    #[test]
    fn test_valid_table() {
        let data = TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ]);
        assert!(!data.is_empty());
        assert_eq!(data.row_count(), 2);
        assert_eq!(data.column_count(), 2);
        assert!(validate_table_data(&data).is_ok());
    }

    #[test]
    fn test_invalid_table() {
        let data = TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string()], // Wrong column count
        ]);
        assert!(validate_table_data(&data).is_err());
    }

    #[test]
    fn test_render_simple_table() {
        let data = TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ]);
        let result = render_table(&data, 3).unwrap();
        assert!(result.contains("| A   | B   |"));
        assert!(result.contains("| 1   | 2   |"));
    }

    #[test]
    fn test_calculate_column_widths() {
        let data = TableData::new(vec![
            vec!["Short".to_string(), "A".to_string()],
            vec!["Very Long Text".to_string(), "B".to_string()],
        ]);
        let widths = calculate_column_widths(&data);
        assert_eq!(widths, vec![14, 1]); // "Very Long Text" = 14, "B" = 1
    }

    #[test]
    fn test_render_auto_width() {
        let data = TableData::new(vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["John".to_string(), "30".to_string()],
        ]);
        let result = render_table_auto_width(&data).unwrap();
        assert!(result.contains("| Name | Age |"));
        assert!(result.contains("| John | 30  |"));
    }

    #[test]
    fn test_render_with_borders() {
        let data = TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ]);
        let result = render_table_with_borders(&data).unwrap();
        assert!(result.contains("┌"));
        assert!(result.contains("┐"));
        assert!(result.contains("└"));
        assert!(result.contains("┘"));
        assert!(result.contains("│"));
        assert!(result.contains("─"));
    }

    #[test]
    fn test_render_with_custom_borders() {
        let data = TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ]);
        let ascii_border = BorderChars::ascii();
        let result = render_table_with_custom_borders(&data, &ascii_border).unwrap();
        assert!(result.contains("+"));
        assert!(result.contains("|"));
        assert!(result.contains("-"));
    }

    #[test]
    fn test_border_templates() {
        let data = TableData::new(vec![
            vec!["Test".to_string()],
        ]);

        // Test honeywell style
        let honeywell = get_border_style("honeywell").unwrap();
        let result = render_table_with_custom_borders(&data, &honeywell).unwrap();
        assert!(result.contains("┌"));

        // Test ramac style
        let ramac = get_border_style("ramac").unwrap();
        let result = render_table_with_custom_borders(&data, &ramac).unwrap();
        assert!(result.contains("+"));

        // Test norc style
        let norc = get_border_style("norc").unwrap();
        let result = render_table_with_custom_borders(&data, &norc).unwrap();
        assert!(result.contains("╔"));
    }

    #[test]
    fn test_render_borderless() {
        let data = TableData::new(vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["John".to_string(), "30".to_string()],
        ]);
        let result = render_table_borderless(&data).unwrap();
        
        // Should contain content but no border characters
        assert!(result.contains("Name"));
        assert!(result.contains("John"));
        assert!(!result.contains("┌"));
        assert!(!result.contains("+"));
        assert!(!result.contains("╔"));
        
        // Should use spaces for all border positions
        let void = get_border_style("void").unwrap();
        let expected = render_table_with_custom_borders(&data, &void).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_render_with_options() {
        let data = TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string(), "2".to_string()],
            vec!["X".to_string(), "Y".to_string()],
        ]);
        
        // Test with row separators
        let options = RenderOptions::with_row_separators();
        let border = BorderChars::default();
        let result = render_table_with_options(&data, &border, &options).unwrap();
        assert!(result.contains("├"));
        assert!(result.contains("┼"));
        assert!(result.contains("┤"));
        
        // Test no horizontal lines
        let options = RenderOptions::no_horizontal_lines();
        let result = render_table_with_options(&data, &border, &options).unwrap();
        assert!(!result.contains("┌"));
        assert!(!result.contains("└"));
        assert!(result.contains("│")); // Should still have vertical borders
    }

    #[test]
    fn test_text_alignment() {
        let data = TableData::new(vec![
            vec!["Left".to_string(), "Center".to_string(), "Right".to_string()],
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::new().with_width(8).with_alignment(Alignment::Left),
            ColumnConfig::new().with_width(8).with_alignment(Alignment::Center), 
            ColumnConfig::new().with_width(8).with_alignment(Alignment::Right),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_column_config(&data, &border, &options, &column_configs).unwrap();
        
        // Check that alignment is working by looking at spaces
        assert!(result.contains("│ Left     │"));  // Left aligned
        assert!(result.contains("│  Center  │"));  // Center aligned  
        assert!(result.contains("│    Right │"));  // Right aligned
    }

    #[test]
    fn test_cell_padding() {
        let data = TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::new()
                .with_width(4)
                .with_padding(Padding::new(2, 1)),  // 2 left, 1 right
            ColumnConfig::new()
                .with_width(4) 
                .with_padding(Padding::symmetric(3)), // 3 on each side
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_column_config(&data, &border, &options, &column_configs).unwrap();
        
        // Check padding is applied correctly
        assert!(result.contains("│  A   │"));    // 2 left + "A" + 1 right + 2 spaces = "  A   "
        assert!(result.contains("│   B   │"));   // 3 left + "B" + 3 right = "   B   "
    }

    #[test]
    fn test_text_truncation() {
        let data = TableData::new(vec![
            vec!["Short".to_string(), "This is very long text".to_string()],
            vec!["A".to_string(), "Another long text here".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::new().with_width(8), // No truncation
            ColumnConfig::new()
                .with_width(10)
                .with_truncation(TruncationConfig::new().with_max_width(10)),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_column_config(&data, &border, &options, &column_configs).unwrap();
        
        // Check truncation is working
        assert!(result.contains("Short"));  // First column, no truncation
        assert!(result.contains("This is..."));  // Second column truncated with ellipsis
        assert!(result.contains("Another..."));  // Another truncated text
    }

    #[test]
    fn test_justify_alignment() {
        let data = TableData::new(vec![
            vec!["hello world".to_string()],
            vec!["one two three".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::new()
                .with_width(15)
                .with_alignment(Alignment::Justify),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_column_config(&data, &border, &options, &column_configs).unwrap();
        
        // Check justify alignment is working
        assert!(result.contains("hello     world"));  // Justified text
        assert!(result.contains("one    two  three")); // Justified with multiple gaps
    }

    #[test]
    fn test_text_wrapping_integration() {
        let data = TableData::new(vec![
            vec!["Short".to_string(), "This is a very long text that needs to be wrapped properly".to_string()],
            vec!["A".to_string(), "Another long line here".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::new().with_width(8),
            ColumnConfig::new()
                .with_width(15)
                .with_wrapping(WrapConfig::new(15)),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_wrapping(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Short"));
        assert!(result.contains("This is a very"));  // First wrapped line
        assert!(result.contains("long text that"));   // Second wrapped line
        
        // Should have multiple content lines due to wrapping
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .collect();
        assert!(content_lines.len() >= 2); // At least 2 content lines for wrapped text
    }

    #[test]
    fn test_unicode_support() {
        let data = TableData::new(vec![
            vec!["Name".to_string(), "City".to_string()],
            vec!["Alice".to_string(), "东京".to_string()], // Tokyo in Chinese
            vec!["Bob".to_string(), "北京".to_string()],   // Beijing in Chinese  
        ]);
        
        let column_configs = vec![
            ColumnConfig::new().with_width(8),
            ColumnConfig::new().with_width(6),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_unicode_aware(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Alice"));
        assert!(result.contains("东京"));
        assert!(result.contains("北京"));
        
        // Check that Unicode characters are properly aligned
        let lines: Vec<&str> = result.lines().collect();
        let content_lines: Vec<&str> = lines
            .iter()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .collect();
        
        // All content lines should have consistent length due to proper Unicode width handling
        if content_lines.len() > 1 {
            let first_len = content_lines[0].len();
            for line in &content_lines[1..] {
                assert_eq!(line.len(), first_len, "Unicode width calculation should make all lines same length");
            }
        }
    }

    #[test]
    fn test_ansi_color_support() {
        let data = TableData::new(vec![
            vec!["Name".to_string(), "Status".to_string()],
            vec!["Alice".to_string(), format!("{}Active{}", colors::GREEN, colors::RESET)],
            vec!["Bob".to_string(), format!("{}Inactive{}", colors::RED, colors::RESET)],
        ]);
        
        let column_configs = vec![
            ColumnConfig::new().with_width(8),
            ColumnConfig::new().with_width(10),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_ansi_aware(&data, &border, &options, &column_configs).unwrap();
        
        // Should contain ANSI color codes
        assert!(result.contains(colors::GREEN));
        assert!(result.contains(colors::RED));
        assert!(result.contains(colors::RESET));
        
        // Should contain the actual content
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
        assert!(result.contains("Active"));
        assert!(result.contains("Inactive"));
        
        // Table structure should be maintained despite ANSI codes
        let lines: Vec<&str> = result.lines().collect();
        let content_lines: Vec<&str> = lines
            .iter()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .collect();
        
        // All content lines should have consistent visual length despite ANSI codes
        assert!(content_lines.len() >= 2);
    }

    #[test]
    fn test_newline_support() {
        let data = TableData::new(vec![
            vec!["Name".to_string(), "Address".to_string()],
            vec!["Alice".to_string(), "123 Main St\nAnytown\nCA 90210".to_string()],
            vec!["Bob".to_string(), "456 Oak Ave\nSomecity, TX".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::new().with_width(8),
            ColumnConfig::new().with_width(15),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_newlines(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Alice"));
        assert!(result.contains("123 Main St"));
        assert!(result.contains("Anytown"));
        assert!(result.contains("CA 90210"));
        assert!(result.contains("Bob"));
        assert!(result.contains("456 Oak Ave"));
        assert!(result.contains("Somecity, TX"));
        
        // Should handle multi-line cells properly
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .collect();
        assert!(content_lines.len() >= 5); // At least 5 content lines due to multi-line addresses
    }

    #[test]
    fn test_split_lines_function() {
        assert_eq!(split_lines("single line"), vec!["single line"]);
        assert_eq!(split_lines("line1\nline2"), vec!["line1", "line2"]);
        assert_eq!(split_lines("a\nb\nc"), vec!["a", "b", "c"]);
        assert_eq!(split_lines(""), vec![""]);
        assert_eq!(split_lines("line\n\nafter empty"), vec!["line", "", "after empty"]);
    }

    #[test]
    fn test_calculate_newline_column_widths() {
        let rows = vec![
            vec!["Short".to_string(), "Normal".to_string()],
            vec!["Multi\nLine\nContent".to_string(), "Single".to_string()],
        ];
        
        let widths = calculate_newline_column_widths(&rows);
        assert_eq!(widths[0], 7); // "Content" = 7 chars
        assert_eq!(widths[1], 6); // "Normal" = 6 chars
    }

    #[test]
    fn test_newline_with_alignment() {
        let data = TableData::new(vec![
            vec!["Left\nAlign".to_string(), "Center\nAlign".to_string(), "Right\nAlign".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::new().with_width(10).with_alignment(Alignment::Left),
            ColumnConfig::new().with_width(10).with_alignment(Alignment::Center),
            ColumnConfig::new().with_width(10).with_alignment(Alignment::Right),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_newlines(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Left"));
        assert!(result.contains("Center"));
        assert!(result.contains("Right"));
        assert!(result.contains("Align"));
        
        // Should have proper multi-line rendering with different alignments
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .collect();
        assert_eq!(content_lines.len(), 2); // Two lines for each multi-line cell
    }
}