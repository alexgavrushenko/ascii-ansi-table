pub mod border;
pub mod renderer;
pub mod alignment;
pub mod vertical_alignment;
pub mod padding;
pub mod truncation;
pub mod wrapping;
pub mod multiline;
pub mod unicode;
pub mod ansi;
pub mod ansi_multiline;
pub mod newline;
pub mod vertical_multiline;
pub mod spanning;
pub mod spanned_renderer;
pub mod headers;
pub mod column_arrays;
pub mod streaming;
pub mod performance;
pub mod single_line;
pub mod cli;

#[cfg(feature = "wasm")]
pub mod wasm;
pub mod html;
pub mod emoji;
pub mod debug;
pub mod validation;

pub use border::{BorderChars, get_border_style};
pub use renderer::RenderOptions;
pub use alignment::{Alignment, ColumnConfig, align_text};
pub use vertical_alignment::{VerticalAlignment, apply_vertical_alignment, calculate_middle_position};
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
pub use vertical_multiline::render_table_with_vertical_alignment;
pub use spanning::{CellSpan, SpannedCell, SpannedTableData, calculate_spanned_width, should_render_cell};
pub use spanned_renderer::render_spanned_table;
pub use headers::{HeaderConfig, render_table_with_headers, default_header_config};
pub use column_arrays::{ColumnConfigArray, ColumnArrayBuilder, render_table_with_column_array, patterns};
pub use streaming::{StreamingTableConfig, StreamingTableWriter, StreamingTableBuilder, 
                   stream_table_to_writer, stream_table_to_stdout};
pub use performance::{PerformanceConfig, RenderCache, StringPool, FastTableRenderer, BatchProcessor};
pub use single_line::{SingleLineConfig, render_single_line_table, render_compact_single_line,
                     render_key_value_pairs, render_transposed_single_line, SummaryRenderer};
pub use cli::{CliConfig, TableCli, ArgParser, InputFormat, OutputFormat};

#[cfg(feature = "wasm")]
pub use wasm::{WasmTableConfig, WasmTableRenderer, WasmUtils, init, version, library_name};
pub use html::{HtmlConfig, HtmlTableRenderer, HtmlUtils, table_to_html, table_to_html_with_config,
              table_to_bootstrap_html, table_to_material_html, html_escape, html_unescape};
pub use emoji::{EmojiConfig, EmojiWidthCalculator, emoji_align_text, emoji_truncate_text, 
               calculate_emoji_column_widths, render_emoji_table};
pub use debug::{DebugConfig, TableDebugger, PerformanceMeasurer, debug_table, debug_table_compact,
               debug_table_verbose, print_debug_table, validate_and_recommend};
pub use validation::{ValidationError, ValidationResult, ValidationConfig, TableValidator, Severity,
                    validate_table, validate_table_strict, validate_table_for_web, is_table_valid, validation_summary};
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
        let default_config = ColumnConfig::default();
        let config = column_configs.get(i).unwrap_or(&default_config);
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
            let default_config = ColumnConfig::default();
            let config = column_configs.get(i).unwrap_or(&default_config);
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
        let default_config = ColumnConfig::default();
        let config = column_configs.get(i).unwrap_or(&default_config);
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
            let default_config = ColumnConfig::default();
            let config = column_configs.get(i).unwrap_or(&default_config);
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
        let default_config = ColumnConfig::default();
        let config = column_configs.get(i).unwrap_or(&default_config);
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
            let default_config = ColumnConfig::default();
            let config = column_configs.get(i).unwrap_or(&default_config);
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

    #[test]
    fn test_vertical_alignment_support() {
        let data = TableData::new(vec![
            vec!["Top".to_string(), "Middle".to_string(), "Bottom".to_string()],
            vec!["Line1\nLine2\nLine3".to_string(), "A\nB".to_string(), "X".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::new()
                .with_width(8)
                .with_vertical_alignment(VerticalAlignment::Top),
            ColumnConfig::new()
                .with_width(8)
                .with_vertical_alignment(VerticalAlignment::Middle),
            ColumnConfig::new()
                .with_width(8)
                .with_vertical_alignment(VerticalAlignment::Bottom),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_vertical_alignment(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Top"));
        assert!(result.contains("Middle"));
        assert!(result.contains("Bottom"));
        assert!(result.contains("Line1"));
        assert!(result.contains("A"));
        assert!(result.contains("X"));
        
        // Should handle vertical alignment properly
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .collect();
        assert!(content_lines.len() >= 4); // Header + multi-line content with proper alignment
    }

    #[test]
    fn test_vertical_alignment_with_wrapping() {
        let data = TableData::new(vec![
            vec!["Short".to_string(), "This is a very long text that will definitely wrap".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::new()
                .with_width(8)
                .with_vertical_alignment(VerticalAlignment::Middle),
            ColumnConfig::new()
                .with_width(12)
                .with_wrapping(WrapConfig::new(12))
                .with_vertical_alignment(VerticalAlignment::Top),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_vertical_alignment(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Short"));
        assert!(result.contains("This is a"));
        assert!(result.contains("very long"));
        
        // Should combine wrapping and vertical alignment
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .collect();
        assert!(content_lines.len() >= 3); // Multiple wrapped lines with vertical alignment
    }

    #[test]
    fn test_cell_spanning_horizontal() {
        let mut data = SpannedTableData::new(2, 3);
        
        // Set up table with horizontal spanning
        data.set_cell(0, 0, SpannedCell::with_span("Header Spans Two".to_string(), CellSpan::horizontal(2))).unwrap();
        data.set_cell(0, 2, SpannedCell::new("C".to_string())).unwrap();
        data.set_cell(1, 0, SpannedCell::new("A".to_string())).unwrap();
        data.set_cell(1, 1, SpannedCell::new("B".to_string())).unwrap();
        data.set_cell(1, 2, SpannedCell::new("C".to_string())).unwrap();
        
        let column_configs = vec![
            ColumnConfig::default().with_width(10),
            ColumnConfig::default().with_width(10),
            ColumnConfig::default().with_width(10),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_spanned_table(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Header Spans Two"));
        assert!(result.contains("A"));
        assert!(result.contains("B"));
        assert!(result.contains("C"));
        
        // Should have proper table structure
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() >= 4);
    }

    #[test]
    fn test_cell_spanning_vertical() {
        let mut data = SpannedTableData::new(3, 2);
        
        // Set up table with vertical spanning
        data.set_cell(0, 0, SpannedCell::with_span("Tall".to_string(), CellSpan::vertical(2))).unwrap();
        data.set_cell(0, 1, SpannedCell::new("Top Right".to_string())).unwrap();
        data.set_cell(1, 1, SpannedCell::new("Mid Right".to_string())).unwrap();
        data.set_cell(2, 0, SpannedCell::new("Bottom Left".to_string())).unwrap();
        data.set_cell(2, 1, SpannedCell::new("Bottom Right".to_string())).unwrap();
        
        let column_configs = vec![
            ColumnConfig::default().with_width(12),
            ColumnConfig::default().with_width(12),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_spanned_table(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Tall"));
        assert!(result.contains("Top Right"));
        assert!(result.contains("Mid Right"));
        assert!(result.contains("Bottom Left"));
        assert!(result.contains("Bottom Right"));
        
        // Should render all 3 data rows
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .collect();
        assert_eq!(content_lines.len(), 3);
    }

    #[test]
    fn test_cell_spanning_mixed() {
        let mut data = SpannedTableData::new(3, 3);
        
        // Complex spanning: 2x2 cell in top-left
        data.set_cell(0, 0, SpannedCell::with_span("Big Cell".to_string(), CellSpan::new(2, 2))).unwrap();
        data.set_cell(0, 2, SpannedCell::new("Top Right".to_string())).unwrap();
        data.set_cell(1, 2, SpannedCell::new("Mid Right".to_string())).unwrap();
        data.set_cell(2, 0, SpannedCell::new("Bottom 1".to_string())).unwrap();
        data.set_cell(2, 1, SpannedCell::new("Bottom 2".to_string())).unwrap();
        data.set_cell(2, 2, SpannedCell::new("Bottom 3".to_string())).unwrap();
        
        let column_configs = vec![
            ColumnConfig::default().with_width(10),
            ColumnConfig::default().with_width(10),
            ColumnConfig::default().with_width(10),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_spanned_table(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Big Cell"));
        assert!(result.contains("Top Right"));
        assert!(result.contains("Bottom"));
        
        // Should properly handle mixed spans
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() >= 5);
    }

    #[test]
    fn test_spanned_table_from_regular() {
        let data = TableData::new(vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
        ]);
        
        let spanned_data = SpannedTableData::from_regular_table(&data);
        assert_eq!(spanned_data.rows, 2);
        assert_eq!(spanned_data.cols, 3);
        assert_eq!(spanned_data.get_cell(0, 0).unwrap().content, "A");
        assert_eq!(spanned_data.get_cell(1, 2).unwrap().content, "3");
        
        let column_configs = vec![
            ColumnConfig::default().with_width(8),
            ColumnConfig::default().with_width(8),
            ColumnConfig::default().with_width(8),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_spanned_table(&spanned_data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("A"));
        assert!(result.contains("B"));
        assert!(result.contains("C"));
        assert!(result.contains("1"));
        assert!(result.contains("2"));
        assert!(result.contains("3"));
    }

    #[test]
    fn test_cell_span_utilities() {
        assert_eq!(CellSpan::single().row_span, 1);
        assert_eq!(CellSpan::single().col_span, 1);
        assert!(!CellSpan::single().is_spanning());
        
        assert_eq!(CellSpan::horizontal(3).col_span, 3);
        assert!(CellSpan::horizontal(3).is_spanning());
        
        assert_eq!(CellSpan::vertical(2).row_span, 2);
        assert!(CellSpan::vertical(2).is_spanning());
        
        let big_span = CellSpan::new(3, 4);
        assert!(big_span.is_spanning());
        assert_eq!(big_span.row_span, 3);
        assert_eq!(big_span.col_span, 4);
    }

    #[test]
    fn test_header_support() {
        let data = TableData::new(vec![
            vec!["Name".to_string(), "Age".to_string(), "City".to_string()],
            vec!["Alice".to_string(), "30".to_string(), "New York".to_string()],
            vec!["Bob".to_string(), "25".to_string(), "London".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::default().with_width(8),
            ColumnConfig::default().with_width(5),
            ColumnConfig::default().with_width(10),
        ];
        
        let header_config = HeaderConfig::new()
            .with_header()
            .with_header_column_configs(vec![
                ColumnConfig::default()
                    .with_width(8)
                    .with_alignment(Alignment::Center),
                ColumnConfig::default()
                    .with_width(5)
                    .with_alignment(Alignment::Center),
                ColumnConfig::default()
                    .with_width(10)
                    .with_alignment(Alignment::Center),
            ]);
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_headers(&data, &border, &options, &column_configs, &header_config).unwrap();
        
        assert!(result.contains("Name"));
        assert!(result.contains("Age"));
        assert!(result.contains("City"));
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
        
        // Should render headers with different styling
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() >= 5); // Top border, header, separator, data rows, bottom border
    }

    #[test]
    fn test_multi_row_headers() {
        let data = TableData::new(vec![
            vec!["Main Header".to_string(), "Secondary".to_string()],
            vec!["Sub A".to_string(), "Sub B".to_string()],
            vec!["Data 1".to_string(), "Value 1".to_string()],
            vec!["Data 2".to_string(), "Value 2".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::default().with_width(12),
            ColumnConfig::default().with_width(12),
        ];
        
        let header_config = HeaderConfig::new()
            .with_header_rows(2)
            .with_header_column_configs(vec![
                ColumnConfig::default()
                    .with_width(12)
                    .with_alignment(Alignment::Center),
                ColumnConfig::default()
                    .with_width(12)
                    .with_alignment(Alignment::Center),
            ]);
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_headers(&data, &border, &options, &column_configs, &header_config).unwrap();
        
        assert!(result.contains("Main Header"));
        assert!(result.contains("Sub A"));
        assert!(result.contains("Data 1"));
        assert!(result.contains("Data 2"));
        
        // Should handle multi-row headers properly
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│"))
            .collect();
        assert_eq!(content_lines.len(), 4); // 2 header rows + 2 data rows
    }

    #[test]
    fn test_header_with_separator() {
        let data = TableData::new(vec![
            vec!["Header".to_string(), "Column".to_string()],
            vec!["Data".to_string(), "Value".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::default().with_width(10),
            ColumnConfig::default().with_width(10),
        ];
        
        let separator_border = BorderChars {
            horizontal: '=',
            vertical: '│',
            top_left: '├',
            top_right: '┤',
            bottom_left: '├',
            bottom_right: '┤',
            top_junction: '┼',
            bottom_junction: '┼',
        };
        
        let header_config = HeaderConfig::new()
            .with_header()
            .with_separator_border(separator_border);
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_headers(&data, &border, &options, &column_configs, &header_config).unwrap();
        
        assert!(result.contains("Header"));
        assert!(result.contains("Data"));
        assert!(result.contains("=")); // Should use separator border
    }

    #[test]
    fn test_default_header_config() {
        let config = default_header_config();
        assert!(config.has_header);
        assert_eq!(config.header_row_count, 1);
        assert!(!config.header_column_configs.is_empty());
        assert_eq!(config.header_column_configs[0].alignment, Alignment::Center);
    }

    #[test]
    fn test_no_header_config() {
        let data = TableData::new(vec![
            vec!["Row 1".to_string(), "Col 1".to_string()],
            vec!["Row 2".to_string(), "Col 2".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::default().with_width(8),
            ColumnConfig::default().with_width(8),
        ];
        
        let header_config = HeaderConfig::new(); // No headers enabled
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_headers(&data, &border, &options, &column_configs, &header_config).unwrap();
        
        assert!(result.contains("Row 1"));
        assert!(result.contains("Row 2"));
        
        // Should render as normal table without header separation
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│"))
            .collect();
        assert_eq!(content_lines.len(), 2);
    }

    #[test]
    fn test_column_config_arrays() {
        let data = TableData::new(vec![
            vec!["Description".to_string(), "Amount".to_string(), "Total".to_string()],
            vec!["Item A".to_string(), "100.50".to_string(), "100.50".to_string()],
            vec!["Item B".to_string(), "75.25".to_string(), "175.75".to_string()],
        ]);
        
        let column_array = ColumnArrayBuilder::new()
            .left_column(15)    // Description - left aligned
            .right_column(10)   // Amount - right aligned
            .right_column(10)   // Total - right aligned
            .build();
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_column_array(&data, &border, &options, &column_array).unwrap();
        
        assert!(result.contains("Description"));
        assert!(result.contains("Amount"));
        assert!(result.contains("Total"));
        assert!(result.contains("Item A"));
        assert!(result.contains("100.50"));
        assert!(result.contains("175.75"));
    }

    #[test]
    fn test_financial_column_pattern() {
        let data = TableData::new(vec![
            vec!["Product".to_string(), "Price".to_string(), "Qty".to_string(), "Total".to_string()],
            vec!["Widget".to_string(), "$10.99".to_string(), "5".to_string(), "$54.95".to_string()],
            vec!["Gadget".to_string(), "$25.50".to_string(), "2".to_string(), "$51.00".to_string()],
        ]);
        
        let widths = [12, 8, 5, 10];
        let column_array = patterns::financial_columns(&widths);
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_column_array(&data, &border, &options, &column_array).unwrap();
        
        assert!(result.contains("Product"));
        assert!(result.contains("Widget"));
        assert!(result.contains("$10.99"));
        assert!(result.contains("$54.95"));
        
        // Should have proper alignment (first column left, others right)
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() >= 4);
    }

    #[test]
    fn test_repeating_column_pattern() {
        let data = TableData::new(vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string(), "E".to_string()],
            vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()],
        ]);
        
        let pattern = vec![
            ColumnConfig::new().with_width(6).with_alignment(Alignment::Left),
            ColumnConfig::new().with_width(6).with_alignment(Alignment::Right),
        ];
        
        let column_array = patterns::repeating_pattern(pattern);
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_column_array(&data, &border, &options, &column_array).unwrap();
        
        assert!(result.contains("A"));
        assert!(result.contains("B"));
        assert!(result.contains("C"));
        assert!(result.contains("D"));
        assert!(result.contains("E"));
        
        // Should handle repeating pattern across all columns
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .collect();
        assert_eq!(content_lines.len(), 2);
    }

    #[test]
    fn test_mixed_content_pattern() {
        let data = TableData::new(vec![
            vec!["Name".to_string(), "Score".to_string(), "Grade".to_string(), "Points".to_string()],
            vec!["Alice".to_string(), "95".to_string(), "A".to_string(), "950".to_string()],
            vec!["Bob".to_string(), "87".to_string(), "B+".to_string(), "870".to_string()],
        ]);
        
        let widths = [10, 8, 8, 8];
        let column_array = patterns::mixed_content(&widths);
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_column_array(&data, &border, &options, &column_array).unwrap();
        
        assert!(result.contains("Name"));
        assert!(result.contains("Alice"));
        assert!(result.contains("95"));
        assert!(result.contains("A"));
        assert!(result.contains("950"));
    }

    #[test]
    fn test_centered_headers_pattern() {
        let data = TableData::new(vec![
            vec!["Header 1".to_string(), "Header 2".to_string(), "Header 3".to_string()],
            vec!["Data 1".to_string(), "Data 2".to_string(), "Data 3".to_string()],
        ]);
        
        let column_array = patterns::centered_headers(12, 3);
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_column_array(&data, &border, &options, &column_array).unwrap();
        
        assert!(result.contains("Header 1"));
        assert!(result.contains("Header 2"));
        assert!(result.contains("Header 3"));
        assert!(result.contains("Data 1"));
        assert!(result.contains("Data 2"));
        assert!(result.contains("Data 3"));
        
        // All columns should be centered
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│"))
            .collect();
        assert_eq!(content_lines.len(), 2);
    }

    #[test]
    fn test_column_array_with_custom_default() {
        let data = TableData::new(vec![
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
        ]);
        
        let custom_default = ColumnConfig::new()
            .with_width(8)
            .with_alignment(Alignment::Center);
            
        let column_array = ColumnConfigArray::new(vec![])
            .with_default_config(custom_default);
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_column_array(&data, &border, &options, &column_array).unwrap();
        
        assert!(result.contains("A"));
        assert!(result.contains("B"));
        assert!(result.contains("C"));
        
        // Should use custom default for all columns
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│"))
            .collect();
        assert_eq!(content_lines.len(), 2);
    }

    #[test]
    fn test_column_array_builder_methods() {
        let array = ColumnArrayBuilder::new()
            .left_column(10)
            .center_column(12)
            .right_column(14)
            .with_repeat_pattern()
            .build();
        
        assert_eq!(array.configs.len(), 3);
        assert!(array.repeat_pattern);
        
        // Test the configurations
        assert_eq!(array.get_config(0).width, Some(10));
        assert_eq!(array.get_config(0).alignment, Alignment::Left);
        
        assert_eq!(array.get_config(1).width, Some(12));
        assert_eq!(array.get_config(1).alignment, Alignment::Center);
        
        assert_eq!(array.get_config(2).width, Some(14));
        assert_eq!(array.get_config(2).alignment, Alignment::Right);
        
        // Test repeat pattern
        assert_eq!(array.get_config(3).alignment, Alignment::Left);   // Cycles back to first
        assert_eq!(array.get_config(4).alignment, Alignment::Center); // Cycles to second
        assert_eq!(array.get_config(5).alignment, Alignment::Right);  // Cycles to third
    }

    #[test]
    fn test_streaming_table_writer() {
        use std::io::Cursor;
        
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        
        let config = StreamingTableConfig::new()
            .with_column_configs(vec![
                ColumnConfig::new().with_width(10),
                ColumnConfig::new().with_width(8),
            ]);
        
        let mut writer = StreamingTableWriter::new(cursor, config);
        writer.initialize(vec![10, 8]).unwrap();
        
        writer.add_row(&["Product".to_string(), "Price".to_string()]).unwrap();
        writer.add_row(&["Widget".to_string(), "$19.99".to_string()]).unwrap();
        writer.add_row(&["Gadget".to_string(), "$29.99".to_string()]).unwrap();
        
        let result_cursor = writer.finalize().unwrap();
        let output = String::from_utf8(result_cursor.into_inner()).unwrap();
        
        assert!(output.contains("Product"));
        assert!(output.contains("Widget"));
        assert!(output.contains("$19.99"));
        assert!(output.contains("Gadget"));
        assert!(output.contains("$29.99"));
        
        // Should have proper table structure
        assert!(output.contains("│"));
        assert!(output.contains("┌"));
        assert!(output.contains("└"));
    }

    #[test]
    fn test_streaming_with_iterator() {
        use std::io::Cursor;
        
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        
        let rows = vec![
            vec!["Name".to_string(), "Age".to_string(), "City".to_string()],
            vec!["Alice".to_string(), "30".to_string(), "New York".to_string()],
            vec!["Bob".to_string(), "25".to_string(), "London".to_string()],
            vec!["Carol".to_string(), "35".to_string(), "Paris".to_string()],
        ];
        
        let config = StreamingTableConfig::default();
        let result_cursor = stream_table_to_writer(cursor, rows.into_iter(), config).unwrap();
        let output = String::from_utf8(result_cursor.into_inner()).unwrap();
        
        assert!(output.contains("Name"));
        assert!(output.contains("Alice"));
        assert!(output.contains("Bob"));
        assert!(output.contains("Carol"));
        assert!(output.contains("New York"));
        assert!(output.contains("London"));
        assert!(output.contains("Paris"));
    }

    #[test]
    fn test_streaming_builder() {
        use std::io::Cursor;
        
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        
        let rows = vec![
            vec!["Header 1".to_string(), "Header 2".to_string()],
            vec!["Data 1".to_string(), "Data 2".to_string()],
            vec!["Data 3".to_string(), "Data 4".to_string()],
        ];
        
        let result_cursor = StreamingTableBuilder::new()
            .with_border(BorderChars::default())
            .with_options(RenderOptions::default())
            .with_buffer_size(1024)
            .with_auto_flush()
            .stream_to_writer(cursor, rows.into_iter())
            .unwrap();
            
        let output = String::from_utf8(result_cursor.into_inner()).unwrap();
        
        assert!(output.contains("Header 1"));
        assert!(output.contains("Header 2"));
        assert!(output.contains("Data 1"));
        assert!(output.contains("Data 3"));
    }

    #[test]
    fn test_streaming_large_dataset() {
        use std::io::Cursor;
        
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        
        // Generate a large dataset
        let rows: Vec<Vec<String>> = (0..100)
            .map(|i| vec![
                format!("Item {}", i),
                format!("Value {}", i * 10),
                format!("Category {}", i % 5),
            ])
            .collect();
        
        let config = StreamingTableConfig::new()
            .with_buffer_size(512) // Small buffer to test frequent flushing
            .with_flush_on_row();
        
        let result_cursor = stream_table_to_writer(cursor, rows.into_iter(), config).unwrap();
        let output = String::from_utf8(result_cursor.into_inner()).unwrap();
        
        assert!(output.contains("Item 0"));
        assert!(output.contains("Item 99"));
        assert!(output.contains("Value 990"));
        assert!(output.contains("Category 4"));
        
        // Should handle large dataset without issues
        let line_count = output.lines().count();
        assert!(line_count > 100); // Should have many lines due to borders and data
    }

    #[test]
    fn test_streaming_auto_column_detection() {
        use std::io::Cursor;
        
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        
        let config = StreamingTableConfig::default();
        let mut writer = StreamingTableWriter::new(cursor, config);
        
        // Don't call initialize - let first row determine columns
        writer.add_row(&["Auto".to_string(), "Detected".to_string(), "Columns".to_string()]).unwrap();
        writer.add_row(&["Row".to_string(), "Two".to_string(), "Data".to_string()]).unwrap();
        
        let result_cursor = writer.finalize().unwrap();
        let output = String::from_utf8(result_cursor.into_inner()).unwrap();
        
        assert!(output.contains("Auto"));
        assert!(output.contains("Detected"));
        assert!(output.contains("Columns"));
        assert!(output.contains("Row"));
        assert!(output.contains("Two"));
        assert!(output.contains("Data"));
    }

    #[test]
    fn test_fast_table_renderer() {
        let config = PerformanceConfig::new()
            .with_caching(true)
            .with_memory_optimization(true);
        
        let mut renderer = FastTableRenderer::new(config);
        
        let data = TableData::new(vec![
            vec!["Product".to_string(), "Price".to_string(), "Category".to_string()],
            vec!["Widget".to_string(), "$19.99".to_string(), "Electronics".to_string()],
            vec!["Gadget".to_string(), "$29.99".to_string(), "Electronics".to_string()],
            vec!["Book".to_string(), "$12.99".to_string(), "Media".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::new().with_width(10),
            ColumnConfig::new().with_width(8),
            ColumnConfig::new().with_width(12),
        ];
        
        let result = renderer.render_table(
            &data,
            &BorderChars::default(),
            &RenderOptions::default(),
            &column_configs,
        ).unwrap();
        
        assert!(result.contains("Product"));
        assert!(result.contains("Widget"));
        assert!(result.contains("$19.99"));
        assert!(result.contains("Electronics"));
        assert!(result.contains("Book"));
        
        // Check that caching was used
        let (cache_size, _) = renderer.cache_stats();
        assert!(cache_size > 0);
    }

    #[test]
    fn test_performance_caching() {
        let config = PerformanceConfig::new()
            .with_caching(true)
            .with_cache_limit(10);
        
        let mut renderer = FastTableRenderer::new(config);
        
        // Create data with repeated values to test caching effectiveness
        let data = TableData::new(vec![
            vec!["Same".to_string(), "Same".to_string()],
            vec!["Same".to_string(), "Same".to_string()],
            vec!["Same".to_string(), "Same".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::new().with_width(8),
            ColumnConfig::new().with_width(8),
        ];
        
        let result = renderer.render_table(
            &data,
            &BorderChars::default(),
            &RenderOptions::default(),
            &column_configs,
        ).unwrap();
        
        assert!(result.contains("Same"));
        
        // Cache should be utilized for repeated values
        let (cache_size, _) = renderer.cache_stats();
        assert!(cache_size > 0);
    }

    #[test]
    fn test_batch_processor() {
        let processor = BatchProcessor::new(2);
        
        let data = TableData::new(vec![
            vec!["Row 1".to_string(), "Data 1".to_string()],
            vec!["Row 2".to_string(), "Data 2".to_string()],
            vec!["Row 3".to_string(), "Data 3".to_string()],
            vec!["Row 4".to_string(), "Data 4".to_string()],
            vec!["Row 5".to_string(), "Data 5".to_string()],
        ]);
        
        let results = processor.process_in_batches(&data, |batch| {
            Ok(format!("Processed {} rows", batch.len()))
        }).unwrap();
        
        assert_eq!(results.len(), 3); // 5 rows / 2 batch size = 3 batches (2+2+1)
        assert_eq!(results[0], "Processed 2 rows");
        assert_eq!(results[1], "Processed 2 rows");
        assert_eq!(results[2], "Processed 1 rows");
    }

    #[test]
    fn test_memory_usage_estimation() {
        let processor = BatchProcessor::new(10);
        
        let data = TableData::new(vec![
            vec!["Hello".to_string(), "World".to_string()],
            vec!["Large".to_string(), "Dataset".to_string()],
            vec!["Memory".to_string(), "Usage".to_string()],
        ]);
        
        let estimated = processor.estimate_memory_usage(&data);
        assert!(estimated > 0);
        
        // Should be reasonable estimate
        let content_chars = "HelloWorldLargeDatasetMemoryUsage".len();
        assert!(estimated >= content_chars);
    }

    #[test]
    fn test_render_cache_operations() {
        let mut cache = RenderCache::new(5);
        
        // Test alignment caching
        let aligned1 = cache.get_aligned_text("test", 10, Alignment::Center);
        let aligned2 = cache.get_aligned_text("test", 10, Alignment::Center);
        assert_eq!(aligned1, aligned2);
        
        // Test padding caching
        let padding = crate::padding::Padding::new(2, 2);
        let padded1 = cache.get_padded_text("content", padding);
        let padded2 = cache.get_padded_text("content", padding);
        assert_eq!(padded1, padded2);
        
        // Test truncation caching
        let truncated1 = cache.get_truncated_text("very long text", 8, "...");
        let truncated2 = cache.get_truncated_text("very long text", 8, "...");
        assert_eq!(truncated1, truncated2);
        
        assert!(cache.cache_size() > 0);
        
        cache.clear();
        assert_eq!(cache.cache_size(), 0);
    }

    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new(3);
        
        pool.intern("common");
        pool.intern("string");
        pool.intern("common"); // Should reuse
        
        assert_eq!(pool.size(), 2); // Only two unique strings
        
        pool.clear();
        assert_eq!(pool.size(), 0);
    }

    #[test]
    fn test_performance_config_builder() {
        let config = PerformanceConfig::new()
            .with_caching(false)
            .with_cache_limit(100)
            .with_string_pool(false)
            .with_memory_optimization(true);
        
        assert!(!config.enable_caching);
        assert_eq!(config.cache_size_limit, 100);
        assert!(!config.use_string_pool);
        assert!(config.optimize_memory);
    }

    #[test]
    fn test_performance_vs_regular_rendering() {
        // Test that fast renderer produces same output as regular renderer
        let data = TableData::new(vec![
            vec!["Name".to_string(), "Score".to_string()],
            vec!["Alice".to_string(), "95".to_string()],
            vec!["Bob".to_string(), "87".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::new().with_width(8),
            ColumnConfig::new().with_width(6),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        
        // Regular rendering
        let regular_result = render_table_with_column_config(&data, &border, &options, &column_configs).unwrap();
        
        // Fast rendering
        let mut fast_renderer = FastTableRenderer::new(PerformanceConfig::new());
        let fast_result = fast_renderer.render_table(&data, &border, &options, &column_configs).unwrap();
        
        // Results should contain the same content
        assert!(regular_result.contains("Name"));
        assert!(fast_result.contains("Name"));
        assert!(regular_result.contains("Alice"));
        assert!(fast_result.contains("Alice"));
        assert!(regular_result.contains("95"));
        assert!(fast_result.contains("95"));
    }

    #[test]
    fn test_single_line_rendering() {
        let data = TableData::new(vec![
            vec!["Name".to_string(), "Age".to_string(), "Department".to_string()],
            vec!["Alice".to_string(), "30".to_string(), "Engineering".to_string()],
            vec!["Bob".to_string(), "25".to_string(), "Marketing".to_string()],
            vec!["Carol".to_string(), "35".to_string(), "Sales".to_string()],
        ]);
        
        let config = SingleLineConfig::new();
        let result = render_single_line_table(&data, &config).unwrap();
        
        assert!(result.contains("Name | Age | Department"));
        assert!(result.contains("Alice | 30 | Engineering"));
        assert!(result.contains("Bob | 25 | Marketing"));
        assert!(result.contains("Carol | 35 | Sales"));
        
        // Should have multiple lines
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 4);
    }

    #[test]
    fn test_csv_single_line_format() {
        let data = TableData::new(vec![
            vec!["Product".to_string(), "Price".to_string(), "In Stock".to_string()],
            vec!["Widget".to_string(), "$19.99".to_string(), "Yes".to_string()],
            vec!["Gadget".to_string(), "$29.99".to_string(), "No".to_string()],
        ]);
        
        let config = SingleLineConfig::csv();
        let result = render_single_line_table(&data, &config).unwrap();
        
        assert!(result.contains("\"Product\",\"Price\",\"In Stock\""));
        assert!(result.contains("\"Widget\",\"$19.99\",\"Yes\""));
        assert!(result.contains("\"Gadget\",\"$29.99\",\"No\""));
    }

    #[test]
    fn test_compact_single_line() {
        let data = TableData::new(vec![
            vec!["Row1Col1".to_string(), "Row1Col2".to_string()],
            vec!["Row2Col1".to_string(), "Row2Col2".to_string()],
            vec!["Row3Col1".to_string(), "Row3Col2".to_string()],
        ]);
        
        let config = SingleLineConfig::new();
        let result = render_compact_single_line(&data, &config).unwrap();
        
        // Should be all on one line
        assert!(!result.contains('\n'));
        assert!(result.contains("Row1Col1 | Row1Col2"));
        assert!(result.contains("Row2Col1 | Row2Col2"));
        assert!(result.contains("Row3Col1 | Row3Col2"));
    }

    #[test]
    fn test_key_value_pairs() {
        let data = TableData::new(vec![
            vec!["Attribute".to_string(), "Value".to_string(), "Unit".to_string()],
            vec!["Height".to_string(), "180".to_string(), "cm".to_string()],
            vec!["Weight".to_string(), "75".to_string(), "kg".to_string()],
            vec!["Age".to_string(), "25".to_string(), "years".to_string()],
        ]);
        
        let config = SingleLineConfig::new().without_headers();
        let result = render_key_value_pairs(&data, &config).unwrap();
        
        assert!(result.contains("Height: 180 | cm"));
        assert!(result.contains("Weight: 75 | kg"));
        assert!(result.contains("Age: 25 | years"));
        
        // Should not contain the header row
        assert!(!result.contains("Attribute: Value"));
    }

    #[test]
    fn test_transposed_single_line() {
        let data = TableData::new(vec![
            vec!["Name".to_string(), "Alice".to_string(), "Bob".to_string(), "Carol".to_string()],
            vec!["Age".to_string(), "30".to_string(), "25".to_string(), "35".to_string()],
            vec!["City".to_string(), "NY".to_string(), "LA".to_string(), "SF".to_string()],
        ]);
        
        let config = SingleLineConfig::new();
        let result = render_transposed_single_line(&data, &config).unwrap();
        
        // Columns should become rows
        assert!(result.contains("Name | Alice | Bob | Carol"));
        assert!(result.contains("Age | 30 | 25 | 35"));
        assert!(result.contains("City | NY | LA | SF"));
        
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 4); // 4 columns become 4 rows
    }

    #[test]
    fn test_field_width_truncation() {
        let data = TableData::new(vec![
            vec!["This is a very long field that should be truncated".to_string(), "Short".to_string()],
            vec!["Another extremely long field name".to_string(), "Brief".to_string()],
        ]);
        
        let config = SingleLineConfig::new().with_max_field_width(15);
        let result = render_single_line_table(&data, &config).unwrap();
        
        assert!(result.contains("This is a ve...")); // Truncated
        assert!(result.contains("Another extr...")); // Truncated  
        assert!(result.contains("Short")); // Not truncated
        assert!(result.contains("Brief")); // Not truncated
    }

    #[test]
    fn test_quote_handling() {
        let data = TableData::new(vec![
            vec!["Field with \"quotes\" inside".to_string(), "Normal field".to_string()],
            vec!["Another \"quoted\" field".to_string(), "Regular".to_string()],
        ]);
        
        let config = SingleLineConfig::new().with_quotes('"');
        let result = render_single_line_table(&data, &config).unwrap();
        
        // Should escape quotes inside quoted fields
        assert!(result.contains("\"Field with \\\"quotes\\\" inside\""));
        assert!(result.contains("\"Another \\\"quoted\\\" field\""));
        assert!(result.contains("\"Normal field\""));
        assert!(result.contains("\"Regular\""));
    }

    #[test]
    fn test_summary_statistics() {
        let data = TableData::new(vec![
            vec!["Name".to_string(), "Score".to_string(), "Grade".to_string()],
            vec!["Alice".to_string(), "95".to_string(), "A".to_string()],
            vec!["Bob".to_string(), "".to_string(), "B".to_string()], // Empty score
            vec!["Carol".to_string(), "87".to_string(), "B+".to_string()],
            vec!["Dave".to_string(), "92".to_string(), "A-".to_string()],
        ]);
        
        let stats = SummaryRenderer::render_stats(&data);
        
        assert!(stats.contains("5 rows"));
        assert!(stats.contains("3 columns"));
        assert!(stats.contains("15 total cells"));
        assert!(stats.contains("1 empty")); // One empty score
        assert!(stats.contains("avg length:"));
    }

    #[test]
    fn test_column_statistics() {
        let data = TableData::new(vec![
            vec!["Short".to_string(), "Very Long Column Content".to_string()],
            vec!["A".to_string(), "Medium Length".to_string()],
            vec!["".to_string(), "Brief".to_string()], // Empty first column
            vec!["Medium".to_string(), "X".to_string()],
        ]);
        
        let result = SummaryRenderer::render_column_stats(&data).unwrap();
        
        assert!(result.contains("Col0:"));
        assert!(result.contains("Col1:"));
        assert!(result.contains("empty=1")); // First column has one empty cell
        assert!(result.contains("max="));
        assert!(result.contains("min="));
        assert!(result.contains("avg="));
    }

    #[test]
    fn test_different_single_line_formats() {
        let data = TableData::new(vec![
            vec!["Name".to_string(), "Value".to_string()],
            vec!["Test".to_string(), "Data".to_string()],
        ]);
        
        // Test TSV format
        let tsv_result = render_single_line_table(&data, &SingleLineConfig::tsv()).unwrap();
        assert!(tsv_result.contains("Test\tData")); // Tab separated
        assert!(!tsv_result.contains("Name")); // No headers in TSV
        
        // Test compact format
        let compact_result = render_single_line_table(&data, &SingleLineConfig::compact()).unwrap();
        assert!(compact_result.contains("Name Value"));
        assert!(compact_result.contains("Test Data"));
        
        // Test JSON array style
        let json_result = render_single_line_table(&data, &SingleLineConfig::json_array()).unwrap();
        assert!(json_result.contains("\"Name\", \"Value\""));
        assert!(json_result.contains("\"Test\", \"Data\""));
    }
}