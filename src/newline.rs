use crate::{TableData, BorderChars, RenderOptions, ColumnConfig};
use crate::alignment::align_text;
use crate::padding::apply_padding;
use crate::truncation::truncate_text;

/// Split text into lines based on explicit newline characters
pub fn split_lines(text: &str) -> Vec<String> {
    if text.is_empty() {
        return vec![String::new()];
    }
    
    text.lines().map(|line| line.to_string()).collect()
}

/// Render table with support for newline characters in cells
pub fn render_table_with_newlines(
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
            // Calculate width considering all lines in multi-line cells
            let lines = split_lines(cell);
            let max_line_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
            auto_widths[i] = auto_widths[i].max(max_line_width);
        }
    }
    
    // Process each row to get multi-line content
    let mut processed_rows = Vec::new();
    
    for row in &data.rows {
        let mut row_lines = Vec::new();
        let mut max_height = 1;
        
        // Process each cell to handle newlines
        for (col_idx, cell) in row.iter().enumerate() {
            let default_config = ColumnConfig::default();
            let config = column_configs.get(col_idx).unwrap_or(&default_config);
            let content_width = config.width.unwrap_or(auto_widths[col_idx]);
            
            // Split cell content by newlines
            let lines = split_lines(cell);
            
            // Apply truncation and alignment to each line
            let processed_lines: Vec<String> = lines.into_iter().map(|line| {
                let truncated_line = truncate_text(&line, &config.truncation);
                align_text(&truncated_line, content_width, config.alignment)
            }).collect();
            
            max_height = max_height.max(processed_lines.len());
            row_lines.push(processed_lines);
        }
        
        // Normalize all columns to same height by padding with empty strings
        for col_lines in &mut row_lines {
            while col_lines.len() < max_height {
                let default_config = ColumnConfig::default();
                let config = column_configs.get(row_lines.iter().position(|x| std::ptr::eq(x, col_lines)).unwrap_or(0))
                    .unwrap_or(&default_config);
                let content_width = config.width.unwrap_or(auto_widths[col_lines.len()]);
                col_lines.push(" ".repeat(content_width));
            }
        }
        
        processed_rows.push((row_lines, max_height));
    }
    
    // Calculate final column widths including padding
    let mut column_widths = Vec::new();
    for i in 0..data.column_count() {
        let default_config = ColumnConfig::default();
        let config = column_configs.get(i).unwrap_or(&default_config);
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
                let default_config = ColumnConfig::default();
                let config = column_configs.get(col_idx).unwrap_or(&default_config);
                
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

/// Process text with newlines for column width calculation
pub fn calculate_newline_column_widths(rows: &[Vec<String>]) -> Vec<usize> {
    let mut widths = vec![0; rows.first().map(|row| row.len()).unwrap_or(0)];
    
    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            let lines = split_lines(cell);
            let max_line_width = lines.iter().map(|line| line.len()).max().unwrap_or(0);
            widths[i] = widths[i].max(max_line_width);
        }
    }
    
    widths
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::padding::Padding;

    #[test]
    fn test_split_lines() {
        assert_eq!(split_lines("hello"), vec!["hello"]);
        assert_eq!(split_lines("hello\nworld"), vec!["hello", "world"]);
        assert_eq!(split_lines("line1\nline2\nline3"), vec!["line1", "line2", "line3"]);
        assert_eq!(split_lines(""), vec![""]);
    }

    #[test]
    fn test_render_with_newlines() {
        let data = crate::TableData::new(vec![
            vec!["Name".to_string(), "Description".to_string()],
            vec!["Alice".to_string(), "First line\nSecond line".to_string()],
            vec!["Bob".to_string(), "Single line".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::default().with_width(8),
            ColumnConfig::default().with_width(12),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_newlines(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Name"));
        assert!(result.contains("Alice"));
        assert!(result.contains("First line"));
        assert!(result.contains("Second line"));
        assert!(result.contains("Bob"));
        assert!(result.contains("Single line"));
        
        // Should have multiple lines due to newlines in cells
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() > 6); // More than just basic table structure
    }

    #[test]
    fn test_multi_line_cell_alignment() {
        let data = crate::TableData::new(vec![
            vec!["Multi\nLine\nCell".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::default()
                .with_width(10)
                .with_alignment(crate::alignment::Alignment::Center),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_newlines(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Multi"));
        assert!(result.contains("Line"));
        assert!(result.contains("Cell"));
        
        // Check that lines are properly centered
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .collect();
        assert_eq!(content_lines.len(), 3); // Three lines for the multi-line cell
    }

    #[test]
    fn test_empty_lines_in_cells() {
        let data = crate::TableData::new(vec![
            vec!["Line 1\n\nLine 3".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::default().with_width(8),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_newlines(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("Line 1"));
        assert!(result.contains("Line 3"));
        
        // Should handle empty line in the middle
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .collect();
        assert_eq!(content_lines.len(), 3); // Three lines including empty middle line
    }

    #[test]
    fn test_calculate_newline_column_widths() {
        let rows = vec![
            vec!["Short".to_string(), "Normal".to_string()],
            vec!["Very\nLong\nMulti-line".to_string(), "Single".to_string()],
        ];
        
        let widths = calculate_newline_column_widths(&rows);
        assert_eq!(widths, vec![10, 6]); // "Multi-line" = 10, "Normal" = 6
    }

    #[test]
    fn test_padding_with_newlines() {
        let data = crate::TableData::new(vec![
            vec!["A\nB".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::default()
                .with_width(4)
                .with_padding(Padding::new(2, 1)),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_newlines(&data, &border, &options, &column_configs).unwrap();
        
        assert!(result.contains("A"));
        assert!(result.contains("B"));
        
        // Check that padding is applied correctly to each line
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .collect();
        assert_eq!(content_lines.len(), 2);
    }
}