use crate::{BorderChars, RenderOptions, ColumnConfig};
use crate::alignment::align_text;
use crate::padding::apply_padding;
use crate::truncation::truncate_text;
use crate::spanning::{SpannedTableData, SpannedCell, calculate_spanned_width, should_render_cell};

/// Render a table with cell spanning support
pub fn render_spanned_table(
    data: &SpannedTableData,
    border: &BorderChars,
    options: &RenderOptions,
    column_configs: &[ColumnConfig],
) -> Result<String, String> {
    if data.is_empty() {
        return Ok(String::new());
    }

    // Calculate column widths based on non-spanned content
    let mut auto_widths = vec![0; data.cols];
    for row_idx in 0..data.rows {
        for col_idx in 0..data.cols {
            if let Some(cell) = data.get_cell(row_idx, col_idx) {
                if should_render_cell(cell) && cell.span.col_span == 1 {
                    auto_widths[col_idx] = auto_widths[col_idx].max(cell.content.len());
                }
            }
        }
    }

    // Calculate final column widths including padding
    let mut column_widths = Vec::new();
    for i in 0..data.cols {
        let default_config = ColumnConfig::default();
        let config = column_configs.get(i).unwrap_or(&default_config);
        let content_width = config.width.unwrap_or(auto_widths[i]);
        let total_width = content_width + config.padding.total();
        column_widths.push(total_width);
    }

    let mut result = String::new();

    // Top border
    if options.show_top_border {
        result.push_str(&render_top_border(&column_widths, border, data));
    }

    // Data rows
    for row_idx in 0..data.rows {
        result.push_str(&render_data_row(
            data,
            row_idx,
            &column_widths,
            column_configs,
            border,
        )?);

        // Row separator (optional, not after last row)
        if options.show_row_separators && row_idx < data.rows - 1 {
            result.push_str(&render_row_separator(&column_widths, border, data, row_idx));
        }
    }

    // Bottom border
    if options.show_bottom_border {
        result.push_str(&render_bottom_border(&column_widths, border, data));
    }

    Ok(result)
}

fn render_top_border(column_widths: &[usize], border: &BorderChars, data: &SpannedTableData) -> String {
    let mut result = String::new();
    result.push(border.top_left);
    
    let mut col_idx = 0;
    while col_idx < column_widths.len() {
        // Check if first row has spanning at this position
        let span_width = if let Some(cell) = data.get_cell(0, col_idx) {
            if should_render_cell(cell) && cell.span.col_span > 1 {
                calculate_spanned_width(column_widths, col_idx, cell.span.col_span, 1)
            } else {
                column_widths[col_idx]
            }
        } else {
            column_widths[col_idx]
        };
        
        result.push_str(&border.horizontal.to_string().repeat(span_width));
        
        // Move to next unspanned position
        if let Some(cell) = data.get_cell(0, col_idx) {
            if should_render_cell(cell) && cell.span.col_span > 1 {
                col_idx += cell.span.col_span;
            } else {
                col_idx += 1;
            }
        } else {
            col_idx += 1;
        }
        
        if col_idx < column_widths.len() {
            result.push(border.top_junction);
        }
    }
    
    result.push(border.top_right);
    result.push('\n');
    result
}

fn render_data_row(
    data: &SpannedTableData,
    row_idx: usize,
    column_widths: &[usize],
    column_configs: &[ColumnConfig],
    border: &BorderChars,
) -> Result<String, String> {
    let mut result = String::new();
    result.push(border.vertical);

    let mut col_idx = 0;
    while col_idx < data.cols {
        if let Some(cell) = data.get_cell(row_idx, col_idx) {
            if should_render_cell(cell) {
                let default_config = ColumnConfig::default();
                let config = column_configs.get(col_idx).unwrap_or(&default_config);
                
                // Calculate available width for this cell
                let available_width = if cell.span.col_span > 1 {
                    // For spanned cells, calculate width across all spanned columns
                    let mut total_width = 0;
                    for i in col_idx..col_idx + cell.span.col_span {
                        if i < column_widths.len() {
                            let default_config = ColumnConfig::default();
                            let col_config = column_configs.get(i).unwrap_or(&default_config);
                            let content_width = col_config.width.unwrap_or(column_widths[i] - col_config.padding.total());
                            total_width += content_width;
                            if i > col_idx {
                                total_width += 1; // Add space for border between columns
                            }
                        }
                    }
                    total_width
                } else {
                    config.width.unwrap_or(column_widths[col_idx] - config.padding.total())
                };
                
                // Process cell content
                let truncated_cell = truncate_text(&cell.content, &config.truncation);
                let aligned_cell = align_text(&truncated_cell, available_width, config.alignment);
                let padded_cell = apply_padding(&aligned_cell, config.padding);
                
                result.push_str(&padded_cell);
                col_idx += cell.span.col_span;
            } else {
                // Skip continuation cells
                col_idx += 1;
                continue;
            }
        } else {
            col_idx += 1;
            continue;
        }

        if col_idx < data.cols {
            result.push(border.vertical);
        }
    }

    result.push(border.vertical);
    result.push('\n');
    Ok(result)
}

fn render_row_separator(
    column_widths: &[usize], 
    border: &BorderChars, 
    data: &SpannedTableData,
    row_idx: usize,
) -> String {
    let mut result = String::new();
    result.push('├');
    
    let mut col_idx = 0;
    while col_idx < column_widths.len() {
        // Check spans for current and next row
        let current_span = data.get_cell(row_idx, col_idx)
            .map(|c| if should_render_cell(c) { c.span.col_span } else { 1 })
            .unwrap_or(1);
        
        let next_span = data.get_cell(row_idx + 1, col_idx)
            .map(|c| if should_render_cell(c) { c.span.col_span } else { 1 })
            .unwrap_or(1);
            
        let span_width = current_span.max(next_span);
        let width = if span_width > 1 {
            calculate_spanned_width(column_widths, col_idx, span_width, 1)
        } else {
            column_widths[col_idx]
        };
        
        result.push_str(&border.horizontal.to_string().repeat(width));
        
        col_idx += span_width;
        if col_idx < column_widths.len() {
            result.push('┼');
        }
    }
    
    result.push('┤');
    result.push('\n');
    result
}

fn render_bottom_border(column_widths: &[usize], border: &BorderChars, data: &SpannedTableData) -> String {
    let mut result = String::new();
    result.push(border.bottom_left);
    
    let last_row = data.rows - 1;
    let mut col_idx = 0;
    while col_idx < column_widths.len() {
        // Check if last row has spanning at this position
        let span_width = if let Some(cell) = data.get_cell(last_row, col_idx) {
            if should_render_cell(cell) && cell.span.col_span > 1 {
                calculate_spanned_width(column_widths, col_idx, cell.span.col_span, 1)
            } else {
                column_widths[col_idx]
            }
        } else {
            column_widths[col_idx]
        };
        
        result.push_str(&border.horizontal.to_string().repeat(span_width));
        
        // Move to next unspanned position
        if let Some(cell) = data.get_cell(last_row, col_idx) {
            if should_render_cell(cell) && cell.span.col_span > 1 {
                col_idx += cell.span.col_span;
            } else {
                col_idx += 1;
            }
        } else {
            col_idx += 1;
        }
        
        if col_idx < column_widths.len() {
            result.push(border.bottom_junction);
        }
    }
    
    result.push(border.bottom_right);
    result.push('\n');
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spanning::{CellSpan, SpannedCell};
    use crate::padding::Padding;

    #[test]
    fn test_render_simple_spanned_table() {
        let mut data = SpannedTableData::new(2, 3);
        
        // Set up cells: first cell spans 2 columns
        data.set_cell(0, 0, SpannedCell::with_span("Spanning Header".to_string(), CellSpan::horizontal(2))).unwrap();
        data.set_cell(0, 2, SpannedCell::new("C".to_string())).unwrap();
        data.set_cell(1, 0, SpannedCell::new("A".to_string())).unwrap();
        data.set_cell(1, 1, SpannedCell::new("B".to_string())).unwrap();
        data.set_cell(1, 2, SpannedCell::new("C".to_string())).unwrap();
        
        let column_configs = vec![
            ColumnConfig::default().with_width(8),
            ColumnConfig::default().with_width(8),
            ColumnConfig::default().with_width(8),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_spanned_table(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Spanning Header"));
        assert!(result.contains("A"));
        assert!(result.contains("B"));
        assert!(result.contains("C"));
        
        // Should have proper table structure with spanning
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() >= 4); // Top border, header row, data row, bottom border
    }

    #[test]
    fn test_vertical_span() {
        let mut data = SpannedTableData::new(3, 2);
        
        // First cell spans 2 rows
        data.set_cell(0, 0, SpannedCell::with_span("Tall Cell".to_string(), CellSpan::vertical(2))).unwrap();
        data.set_cell(0, 1, SpannedCell::new("Header".to_string())).unwrap();
        data.set_cell(1, 1, SpannedCell::new("Row 2".to_string())).unwrap();
        data.set_cell(2, 0, SpannedCell::new("Bottom".to_string())).unwrap();
        data.set_cell(2, 1, SpannedCell::new("Row 3".to_string())).unwrap();
        
        let column_configs = vec![
            ColumnConfig::default().with_width(10),
            ColumnConfig::default().with_width(10),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_spanned_table(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Tall Cell"));
        assert!(result.contains("Header"));
        assert!(result.contains("Row 2"));
        assert!(result.contains("Bottom"));
        
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() >= 5); // Top border + 3 data rows + bottom border
    }

    #[test]
    fn test_mixed_spans() {
        let mut data = SpannedTableData::new(3, 3);
        
        // Complex spanning pattern
        data.set_cell(0, 0, SpannedCell::with_span("Big Cell".to_string(), CellSpan::new(2, 2))).unwrap();
        data.set_cell(0, 2, SpannedCell::new("Corner".to_string())).unwrap();
        data.set_cell(1, 2, SpannedCell::new("Middle".to_string())).unwrap();
        data.set_cell(2, 0, SpannedCell::new("Bottom L".to_string())).unwrap();
        data.set_cell(2, 1, SpannedCell::new("Bottom M".to_string())).unwrap();
        data.set_cell(2, 2, SpannedCell::new("Bottom R".to_string())).unwrap();
        
        let column_configs = vec![
            ColumnConfig::default().with_width(8),
            ColumnConfig::default().with_width(8),
            ColumnConfig::default().with_width(8),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_spanned_table(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Big Cell"));
        assert!(result.contains("Corner"));
        assert!(result.contains("Bottom"));
        
        // Should handle complex spanning correctly
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() >= 5);
    }

    #[test]
    fn test_spanned_cell_alignment() {
        let mut data = SpannedTableData::new(2, 3);
        
        data.set_cell(0, 0, SpannedCell::with_span("Center This Text".to_string(), CellSpan::horizontal(3))).unwrap();
        data.set_cell(1, 0, SpannedCell::new("A".to_string())).unwrap();
        data.set_cell(1, 1, SpannedCell::new("B".to_string())).unwrap();
        data.set_cell(1, 2, SpannedCell::new("C".to_string())).unwrap();
        
        let column_configs = vec![
            ColumnConfig::default().with_width(10).with_alignment(crate::alignment::Alignment::Center),
            ColumnConfig::default().with_width(10),
            ColumnConfig::default().with_width(10),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_spanned_table(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Center This Text"));
        assert!(result.contains("A"));
        assert!(result.contains("B"));
        assert!(result.contains("C"));
    }

    #[test]
    fn test_empty_spanned_table() {
        let data = SpannedTableData::new(0, 0);
        let result = render_spanned_table(&data, &BorderChars::default(), &RenderOptions::default(), &[]).unwrap();
        assert_eq!(result, "");
    }
}