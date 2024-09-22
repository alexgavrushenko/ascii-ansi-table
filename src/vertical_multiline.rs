use crate::{TableData, BorderChars, RenderOptions, ColumnConfig};
use crate::alignment::align_text;
use crate::padding::apply_padding;
use crate::truncation::truncate_text;
use crate::wrapping::{wrap_text, WrapConfig};
use crate::vertical_alignment::{apply_vertical_alignment, VerticalAlignment};

/// Render table with support for vertical alignment in multi-line cells
pub fn render_table_with_vertical_alignment(
    data: &TableData,
    border: &BorderChars,
    options: &RenderOptions,
    column_configs: &[ColumnConfig],
) -> Result<String, String> {
    crate::validate_table_data(data)?;
    
    if data.is_empty() {
        return Ok(String::new());
    }

    // Calculate column widths based on content
    let mut auto_widths = vec![0; data.column_count()];
    for row in &data.rows {
        for (i, cell) in row.iter().enumerate().take(data.column_count()) {
            auto_widths[i] = auto_widths[i].max(cell.len());
        }
    }
    
    // Process each row to get multi-line content with vertical alignment
    let mut processed_rows = Vec::new();
    
    for row in &data.rows {
        let mut row_lines = Vec::new();
        let mut max_height = 1;
        
        // Process each cell to handle wrapping and calculate max height
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
        
        // Apply vertical alignment to each column
        for (col_idx, col_lines) in row_lines.iter_mut().enumerate() {
            let config = column_configs.get(col_idx).unwrap_or(&ColumnConfig::default());
            let content_width = config.width.unwrap_or(auto_widths[col_idx]);
            
            // Apply alignment to each line first
            let aligned_lines: Vec<String> = col_lines.iter()
                .map(|line| align_text(line, content_width, config.alignment))
                .collect();
            
            // Then apply vertical alignment
            *col_lines = apply_vertical_alignment(aligned_lines, max_height, config.vertical_alignment);
        }
        
        processed_rows.push((row_lines, max_height));
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
    for (row_idx, (row_lines, height)) in processed_rows.iter().enumerate() {
        // Render each line of this multi-line row
        for line_idx in 0..*height {
            result.push(border.vertical);
            
            for (col_idx, col_lines) in row_lines.iter().enumerate() {
                let config = column_configs.get(col_idx).unwrap_or(&ColumnConfig::default());
                
                let cell_content = col_lines.get(line_idx).unwrap_or(&String::new());
                let padded_cell = apply_padding(cell_content, config.padding);
                
                result.push_str(&padded_cell);
                result.push(border.vertical);
            }
            result.push('\n');
        }
        
        // Row separator (optional, not after last row)
        if options.show_row_separators && row_idx < processed_rows.len() - 1 {
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
    use crate::padding::Padding;
    use crate::wrapping::{WrapConfig, WrapMode};

    #[test]
    fn test_vertical_alignment_top() {
        let data = crate::TableData::new(vec![
            vec!["Short".to_string(), "This is a very long text that will wrap into multiple lines".to_string()],
            vec!["A".to_string(), "B".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::default()
                .with_width(8)
                .with_vertical_alignment(VerticalAlignment::Top),
            ColumnConfig::default()
                .with_width(15)
                .with_wrapping(WrapConfig::new(15))
                .with_vertical_alignment(VerticalAlignment::Top),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_vertical_alignment(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Short"));
        assert!(result.contains("This is a very"));
        assert!(result.contains("long text"));
        
        // Should have multiple lines due to wrapping
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .collect();
        assert!(content_lines.len() >= 4); // Multiple lines for wrapped content
    }

    #[test]
    fn test_vertical_alignment_middle() {
        let data = crate::TableData::new(vec![
            vec!["Short".to_string(), "Line1\nLine2\nLine3\nLine4\nLine5".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::default()
                .with_width(8)
                .with_vertical_alignment(VerticalAlignment::Middle),
            ColumnConfig::default()
                .with_width(15)
                .with_vertical_alignment(VerticalAlignment::Middle),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_vertical_alignment(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Short"));
        
        // First column should have empty lines above and below "Short" due to middle alignment
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .collect();
        
        // Should have multiple lines for vertical alignment
        assert!(content_lines.len() >= 3);
    }

    #[test]
    fn test_vertical_alignment_bottom() {
        let data = crate::TableData::new(vec![
            vec!["Short".to_string(), "Multi\nLine\nContent".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::default()
                .with_width(8)
                .with_vertical_alignment(VerticalAlignment::Bottom),
            ColumnConfig::default()
                .with_width(15)
                .with_vertical_alignment(VerticalAlignment::Bottom),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_vertical_alignment(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Short"));
        assert!(result.contains("Multi"));
        assert!(result.contains("Content"));
        
        // Should align "Short" to bottom of the multi-line cell
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .collect();
        
        assert!(content_lines.len() >= 3); // At least 3 lines for multi-line content
    }

    #[test]
    fn test_mixed_vertical_alignment() {
        let data = crate::TableData::new(vec![
            vec!["Top".to_string(), "Middle".to_string(), "Bottom".to_string()],
            vec!["A\nB\nC\nD".to_string(), "X\nY".to_string(), "P".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::default()
                .with_width(8)
                .with_vertical_alignment(VerticalAlignment::Top),
            ColumnConfig::default()
                .with_width(8)
                .with_vertical_alignment(VerticalAlignment::Middle),
            ColumnConfig::default()
                .with_width(8)
                .with_vertical_alignment(VerticalAlignment::Bottom),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_vertical_alignment(&data, &border, &options, &column_configs).unwrap();
        
        // Should contain all content
        assert!(result.contains("Top"));
        assert!(result.contains("Middle"));
        assert!(result.contains("Bottom"));
        assert!(result.contains("A"));
        assert!(result.contains("X"));
        assert!(result.contains("P"));
        
        // Should handle different vertical alignments properly
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .collect();
        
        assert!(content_lines.len() >= 6); // Header + multi-line content
    }
}