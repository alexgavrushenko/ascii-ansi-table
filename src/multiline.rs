use crate::{TableData, BorderChars, RenderOptions, ColumnConfig};
use crate::alignment::{align_text, Alignment};
use crate::padding::apply_padding;
use crate::truncation::truncate_text;
use crate::wrapping::{wrap_text, WrapConfig, WrapMode};

pub fn render_table_with_wrapping(
    data: &TableData,
    border: &BorderChars,
    options: &RenderOptions,
    column_configs: &[ColumnConfig],
) -> Result<String, String> {
    crate::validate_table_data(data)?;
    
    if data.is_empty() {
        return Ok(String::new());
    }

    let auto_widths = crate::calculate_column_widths(data);
    
    // Process each row to get wrapped lines
    let mut wrapped_rows = Vec::new();
    
    for row in &data.rows {
        let mut row_lines = Vec::new();
        let mut max_height = 1;
        
        // First, wrap all cells and calculate max height
        for (col_idx, cell) in row.iter().enumerate() {
            let config = column_configs.get(col_idx).unwrap_or(&ColumnConfig::default());
            let content_width = config.width.unwrap_or(auto_widths[col_idx]);
            
            // Apply truncation first
            let truncated_cell = truncate_text(cell, &config.truncation);
            
            // Then apply wrapping if configured
            let lines = if let Some(wrap_config) = &config.wrap_config {
                wrap_text(&truncated_cell, wrap_config)
            } else {
                vec![truncated_cell]
            };
            
            max_height = max_height.max(lines.len());
            row_lines.push(lines);
        }
        
        // Normalize all columns to same height by padding with empty strings
        for col_lines in &mut row_lines {
            while col_lines.len() < max_height {
                col_lines.push(String::new());
            }
        }
        
        wrapped_rows.push((row_lines, max_height));
    }
    
    // Calculate final column widths including padding
    let mut column_widths = Vec::new();
    for i in 0..data.column_count() {
        let config = column_configs.get(i).unwrap_or(&ColumnConfig::default());
        let content_width = config.width.unwrap_or(auto_widths[i]);
        let total_width = content_width + config.padding.total();
        column_widths.push(total_width);
    }
    
    let mut result = String::new();
    
    // Top border
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
    
    // Data rows
    for (row_idx, (row_lines, height)) in wrapped_rows.iter().enumerate() {
        // Render each line of this multi-line row
        for line_idx in 0..*height {
            result.push(border.vertical);
            
            for (col_idx, col_lines) in row_lines.iter().enumerate() {
                let config = column_configs.get(col_idx).unwrap_or(&ColumnConfig::default());
                let content_width = config.width.unwrap_or(auto_widths[col_idx]);
                
                let cell_content = col_lines.get(line_idx).unwrap_or(&String::new());
                let aligned_cell = align_text(cell_content, content_width, config.alignment);
                let padded_cell = apply_padding(&aligned_cell, config.padding);
                
                result.push_str(&padded_cell);
                result.push(border.vertical);
            }
            result.push('\n');
        }
        
        // Row separator (optional, not after last row)
        if options.show_row_separators && row_idx < wrapped_rows.len() - 1 {
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
    
    // Bottom border
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wrapping::{WrapConfig, WrapMode};
    use crate::padding::Padding;

    #[test]
    fn test_basic_wrapping() {
        let data = crate::TableData::new(vec![
            vec!["Short".to_string(), "This is a very long text that should wrap".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::default(),
            ColumnConfig::default()
                .with_wrapping(WrapConfig::new(10)),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_wrapping(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Short"));
        assert!(result.contains("This is a"));
        assert!(result.contains("very long"));
        
        // Should have multiple lines due to wrapping
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() > 3); // At least top border, content lines, bottom border
    }
}