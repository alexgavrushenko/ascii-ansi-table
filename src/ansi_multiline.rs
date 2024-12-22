use crate::{TableData, BorderChars, RenderOptions, ColumnConfig};
use crate::alignment::align_text;
use crate::padding::apply_padding;
use crate::truncation::truncate_text;
use crate::wrapping::{wrap_ansi_text, WrapConfig};
use crate::ansi::{ansi_display_width, ansi_pad_to_width, ansi_truncate_to_width};

pub fn render_table_with_ansi_wrapping(
    data: &TableData,
    border: &BorderChars,
    options: &RenderOptions,
    column_configs: &[ColumnConfig],
) -> Result<String, String> {
    crate::validate_table_data(data)?;
    
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
    
    // Process each row to get wrapped lines
    let mut wrapped_rows = Vec::new();
    
    for row in &data.rows {
        let mut row_lines = Vec::new();
        let mut max_height = 1;
        
        // First, wrap all cells and calculate max height
        for (col_idx, cell) in row.iter().enumerate() {
            let default_config = ColumnConfig::default();
            let config = column_configs.get(col_idx).unwrap_or(&default_config);
            let content_width = config.width.unwrap_or(auto_widths[col_idx]);
            
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
            
            // Then apply wrapping if configured (ANSI-aware)
            let lines = if let Some(wrap_config) = &config.wrap_config {
                wrap_ansi_text(&truncated_cell, wrap_config)
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
    for (row_idx, (row_lines, height)) in wrapped_rows.iter().enumerate() {
        // Render each line of this multi-line row
        for line_idx in 0..*height {
            result.push(border.vertical);
            
            for (col_idx, col_lines) in row_lines.iter().enumerate() {
                let default_config = ColumnConfig::default();
                let config = column_configs.get(col_idx).unwrap_or(&default_config);
                let content_width = config.width.unwrap_or(auto_widths[col_idx]);
                
                let cell_content = col_lines.get(line_idx).unwrap_or(&String::new());
                
                // Apply alignment (ANSI-aware)
                let aligned_cell = ansi_pad_to_width(cell_content, content_width, config.alignment);
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
    use crate::ansi::colors;

    #[test]
    fn test_ansi_wrapping_integration() {
        let data = crate::TableData::new(vec![
            vec!["Name".to_string(), format!("{}Status with very long text{}", colors::GREEN, colors::RESET)],
            vec!["Alice".to_string(), format!("{}Active and working{}", colors::BLUE, colors::RESET)],
        ]);
        
        let column_configs = vec![
            ColumnConfig::default().with_width(8),
            ColumnConfig::default()
                .with_width(12)
                .with_wrapping(WrapConfig::new(12)),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_ansi_wrapping(&data, &border, &options, &column_configs).unwrap();
        
        // Should contain ANSI color codes
        assert!(result.contains(colors::GREEN));
        assert!(result.contains(colors::BLUE));
        assert!(result.contains(colors::RESET));
        
        // Should contain the actual content
        assert!(result.contains("Name"));
        assert!(result.contains("Alice"));
        assert!(result.contains("Status"));
        assert!(result.contains("Active"));
        
        // Should have multiple lines due to wrapping
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() > 4); // More than just borders due to wrapped content
    }

    #[test]
    fn test_ansi_wrapping_preserves_colors() {
        let data = crate::TableData::new(vec![
            vec![format!("{}Multi-line colored text that should wrap nicely{}", colors::BOLD, colors::RESET)],
        ]);
        
        let column_configs = vec![
            ColumnConfig::default()
                .with_width(15)
                .with_wrapping(WrapConfig::new(15)),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default();
        let result = render_table_with_ansi_wrapping(&data, &border, &options, &column_configs).unwrap();
        
        // Should preserve ANSI sequences across wrapped lines
        assert!(result.contains(colors::BOLD));
        assert!(result.contains(colors::RESET));
        assert!(result.contains("Multi-line"));
        assert!(result.contains("colored"));
        assert!(result.contains("wrap"));
        
        // Should have wrapped the long text
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .collect();
        assert!(content_lines.len() >= 2); // Multiple lines due to wrapping
    }
}