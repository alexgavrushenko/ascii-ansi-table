#[derive(Debug, Clone)]
pub struct HeaderConfig {
    pub has_header: bool,
    pub header_row_count: usize,
    pub header_column_configs: Vec<crate::alignment::ColumnConfig>,
    pub separator_border: Option<crate::border::BorderChars>,
}

impl Default for HeaderConfig {
    fn default() -> Self {
        Self {
            has_header: false,
            header_row_count: 1,
            header_column_configs: Vec::new(),
            separator_border: None,
        }
    }
}

impl HeaderConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_header(mut self) -> Self {
        self.has_header = true;
        self
    }
    
    pub fn with_header_rows(mut self, count: usize) -> Self {
        self.header_row_count = count.max(1);
        self.has_header = true;
        self
    }
    
    pub fn with_header_column_configs(mut self, configs: Vec<crate::alignment::ColumnConfig>) -> Self {
        self.header_column_configs = configs;
        self
    }
    
    pub fn with_separator_border(mut self, border: crate::border::BorderChars) -> Self {
        self.separator_border = Some(border);
        self
    }
}

/// Render table with header support and different styling
pub fn render_table_with_headers(
    data: &crate::TableData,
    border: &crate::BorderChars,
    options: &crate::RenderOptions,
    column_configs: &[crate::alignment::ColumnConfig],
    header_config: &HeaderConfig,
) -> Result<String, String> {
    crate::validate_table_data(data)?;
    
    if data.is_empty() {
        return Ok(String::new());
    }

    // Calculate column widths based on all content
    let mut auto_widths = vec![0; data.column_count()];
    for row in &data.rows {
        for (i, cell) in row.iter().enumerate().take(data.column_count()) {
            auto_widths[i] = auto_widths[i].max(cell.len());
        }
    }
    
    // Calculate final column widths including padding
    let mut column_widths = Vec::new();
    for i in 0..data.column_count() {
        let default_config = crate::alignment::ColumnConfig::default();
        let config = column_configs.get(i).unwrap_or(&default_config);
        let content_width = config.width.unwrap_or(auto_widths[i]);
        let total_width = content_width + config.padding.total();
        column_widths.push(total_width);
    }
    
    let mut result = String::new();
    
    // Top border
    if options.show_top_border {
        result.push_str(&render_border_line(&column_widths, border, true, false));
    }
    
    // Header rows (if configured)
    if header_config.has_header && header_config.header_row_count <= data.row_count() {
        for row_idx in 0..header_config.header_row_count {
            result.push_str(&render_table_row(
                &data.rows[row_idx],
                &column_widths,
                &header_config.header_column_configs,
                column_configs,
                border,
            )?);
        }
        
        // Header separator (if configured)
        if let Some(sep_border) = &header_config.separator_border {
            result.push_str(&render_border_line(&column_widths, sep_border, false, false));
        } else if options.show_row_separators {
            result.push_str(&render_border_line(&column_widths, border, false, false));
        }
        
        // Data rows (excluding header rows)
        for (row_idx, row) in data.rows.iter().enumerate().skip(header_config.header_row_count) {
            result.push_str(&render_table_row(
                row,
                &column_widths,
                &[], // No special header configs for data rows
                column_configs,
                border,
            )?);
            
            // Row separator (optional, not after last row)
            if options.show_row_separators && row_idx < data.rows.len() - 1 {
                result.push_str(&render_border_line(&column_widths, border, false, false));
            }
        }
    } else {
        // No headers, render all rows as data
        for (row_idx, row) in data.rows.iter().enumerate() {
            result.push_str(&render_table_row(
                row,
                &column_widths,
                &[],
                column_configs,
                border,
            )?);
            
            // Row separator (optional, not after last row)
            if options.show_row_separators && row_idx < data.rows.len() - 1 {
                result.push_str(&render_border_line(&column_widths, border, false, false));
            }
        }
    }
    
    // Bottom border
    if options.show_bottom_border {
        result.push_str(&render_border_line(&column_widths, border, false, true));
    }
    
    Ok(result)
}

fn render_table_row(
    row: &[String],
    column_widths: &[usize],
    header_column_configs: &[crate::alignment::ColumnConfig],
    data_column_configs: &[crate::alignment::ColumnConfig],
    border: &crate::BorderChars,
) -> Result<String, String> {
    let mut result = String::new();
    result.push(border.vertical);
    
    for (i, cell) in row.iter().enumerate() {
        // Use header config if available, otherwise fall back to data config
        let default_config = crate::alignment::ColumnConfig::default();
        let config = if !header_column_configs.is_empty() {
            header_column_configs.get(i).unwrap_or(
                data_column_configs.get(i).unwrap_or(&default_config)
            )
        } else {
            data_column_configs.get(i).unwrap_or(&default_config)
        };
        
        let content_width = config.width.unwrap_or(column_widths[i] - config.padding.total());
        
        // Apply truncation, alignment, and padding
        let truncated_cell = crate::truncation::truncate_text(cell, &config.truncation);
        let aligned_cell = crate::alignment::align_text(&truncated_cell, content_width, config.alignment);
        let padded_cell = crate::padding::apply_padding(&aligned_cell, config.padding);
        
        result.push_str(&padded_cell);
        result.push(border.vertical);
    }
    result.push('\n');
    Ok(result)
}

fn render_border_line(
    column_widths: &[usize], 
    border: &crate::BorderChars, 
    is_top: bool, 
    is_bottom: bool
) -> String {
    let mut result = String::new();
    
    if is_top {
        result.push(border.top_left);
        for (i, width) in column_widths.iter().enumerate() {
            result.push_str(&border.horizontal.to_string().repeat(*width));
            if i < column_widths.len() - 1 {
                result.push(border.top_junction);
            }
        }
        result.push(border.top_right);
    } else if is_bottom {
        result.push(border.bottom_left);
        for (i, width) in column_widths.iter().enumerate() {
            result.push_str(&border.horizontal.to_string().repeat(*width));
            if i < column_widths.len() - 1 {
                result.push(border.bottom_junction);
            }
        }
        result.push(border.bottom_right);
    } else {
        // Middle separator line
        result.push('├');
        for (i, width) in column_widths.iter().enumerate() {
            result.push_str(&border.horizontal.to_string().repeat(*width));
            if i < column_widths.len() - 1 {
                result.push('┼');
            }
        }
        result.push('┤');
    }
    result.push('\n');
    result
}

/// Create a default header configuration with centered, bold-style alignment
pub fn default_header_config() -> HeaderConfig {
    HeaderConfig::new()
        .with_header()
        .with_header_column_configs(vec![
            crate::alignment::ColumnConfig::new()
                .with_alignment(crate::alignment::Alignment::Center)
        ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::padding::Padding;

    #[test]
    fn test_header_config_creation() {
        let config = HeaderConfig::new();
        assert!(!config.has_header);
        assert_eq!(config.header_row_count, 1);
        
        let with_header = HeaderConfig::new().with_header();
        assert!(with_header.has_header);
        
        let multi_header = HeaderConfig::new().with_header_rows(3);
        assert!(multi_header.has_header);
        assert_eq!(multi_header.header_row_count, 3);
    }

    #[test]
    fn test_render_with_headers() {
        let data = crate::TableData::new(vec![
            vec!["Name".to_string(), "Age".to_string(), "City".to_string()],
            vec!["Alice".to_string(), "30".to_string(), "New York".to_string()],
            vec!["Bob".to_string(), "25".to_string(), "London".to_string()],
        ]);
        
        let column_configs = vec![
            crate::alignment::ColumnConfig::default().with_width(8),
            crate::alignment::ColumnConfig::default().with_width(5),
            crate::alignment::ColumnConfig::default().with_width(10),
        ];
        
        let header_config = HeaderConfig::new()
            .with_header()
            .with_header_column_configs(vec![
                crate::alignment::ColumnConfig::default()
                    .with_width(8)
                    .with_alignment(crate::alignment::Alignment::Center),
                crate::alignment::ColumnConfig::default()
                    .with_width(5)
                    .with_alignment(crate::alignment::Alignment::Center),
                crate::alignment::ColumnConfig::default()
                    .with_width(10)
                    .with_alignment(crate::alignment::Alignment::Center),
            ]);
        
        let border = crate::BorderChars::default();
        let options = crate::RenderOptions::default();
        let result = render_table_with_headers(&data, &border, &options, &column_configs, &header_config).unwrap();
        
        assert!(result.contains("Name"));
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
        
        // Should have proper table structure with headers
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() >= 4); // Top border, header, data rows, bottom border
    }

    #[test]
    fn test_multi_row_headers() {
        let data = crate::TableData::new(vec![
            vec!["Main Header".to_string(), "".to_string()],
            vec!["Sub A".to_string(), "Sub B".to_string()],
            vec!["Data 1".to_string(), "Data 2".to_string()],
            vec!["Data 3".to_string(), "Data 4".to_string()],
        ]);
        
        let column_configs = vec![
            crate::alignment::ColumnConfig::default().with_width(12),
            crate::alignment::ColumnConfig::default().with_width(12),
        ];
        
        let header_config = HeaderConfig::new()
            .with_header_rows(2) // First two rows are headers
            .with_header_column_configs(vec![
                crate::alignment::ColumnConfig::default()
                    .with_width(12)
                    .with_alignment(crate::alignment::Alignment::Center),
                crate::alignment::ColumnConfig::default()
                    .with_width(12)
                    .with_alignment(crate::alignment::Alignment::Center),
            ]);
        
        let border = crate::BorderChars::default();
        let options = crate::RenderOptions::default();
        let result = render_table_with_headers(&data, &border, &options, &column_configs, &header_config).unwrap();
        
        assert!(result.contains("Main Header"));
        assert!(result.contains("Sub A"));
        assert!(result.contains("Sub B"));
        assert!(result.contains("Data 1"));
        assert!(result.contains("Data 3"));
        
        // Should separate headers from data
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│"))
            .collect();
        assert!(content_lines.len() >= 4); // 2 header rows + 2 data rows
    }

    #[test]
    fn test_header_with_separator_border() {
        let data = crate::TableData::new(vec![
            vec!["Header 1".to_string(), "Header 2".to_string()],
            vec!["Data A".to_string(), "Data B".to_string()],
        ]);
        
        let column_configs = vec![
            crate::alignment::ColumnConfig::default().with_width(10),
            crate::alignment::ColumnConfig::default().with_width(10),
        ];
        
        let separator_border = crate::BorderChars {
            horizontal: '=',
            vertical: '│',
            top_left: '╞',
            top_right: '╡',
            bottom_left: '└',
            bottom_right: '┘',
            top_junction: '╤',
            bottom_junction: '┴',
        };
        
        let header_config = HeaderConfig::new()
            .with_header()
            .with_separator_border(separator_border);
        
        let border = crate::BorderChars::default();
        let options = crate::RenderOptions::default();
        let result = render_table_with_headers(&data, &border, &options, &column_configs, &header_config).unwrap();
        
        assert!(result.contains("Header 1"));
        assert!(result.contains("Data A"));
        assert!(result.contains("═")); // Should use separator border
        
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() >= 4);
    }

    #[test]
    fn test_no_headers() {
        let data = crate::TableData::new(vec![
            vec!["Row 1".to_string(), "Col 1".to_string()],
            vec!["Row 2".to_string(), "Col 2".to_string()],
        ]);
        
        let column_configs = vec![
            crate::alignment::ColumnConfig::default().with_width(8),
            crate::alignment::ColumnConfig::default().with_width(8),
        ];
        
        let header_config = HeaderConfig::new(); // No header
        
        let border = crate::BorderChars::default();
        let options = crate::RenderOptions::default();
        let result = render_table_with_headers(&data, &border, &options, &column_configs, &header_config).unwrap();
        
        assert!(result.contains("Row 1"));
        assert!(result.contains("Row 2"));
        
        // Should render as regular table
        let content_lines: Vec<&str> = result
            .lines()
            .filter(|line| line.starts_with("│"))
            .collect();
        assert_eq!(content_lines.len(), 2);
    }

    #[test]
    fn test_default_header_config() {
        let config = default_header_config();
        assert!(config.has_header);
        assert_eq!(config.header_row_count, 1);
        
        // Should have at least one column config with center alignment
        assert!(!config.header_column_configs.is_empty());
        assert_eq!(config.header_column_configs[0].alignment, crate::alignment::Alignment::Center);
    }

    #[test]
    fn test_header_with_padding() {
        let data = crate::TableData::new(vec![
            vec!["H1".to_string(), "H2".to_string()],
            vec!["D1".to_string(), "D2".to_string()],
        ]);
        
        let column_configs = vec![
            crate::alignment::ColumnConfig::default().with_width(8),
            crate::alignment::ColumnConfig::default().with_width(8),
        ];
        
        let header_config = HeaderConfig::new()
            .with_header()
            .with_header_column_configs(vec![
                crate::alignment::ColumnConfig::default()
                    .with_width(8)
                    .with_padding(Padding::new(3, 3))
                    .with_alignment(crate::alignment::Alignment::Center),
                crate::alignment::ColumnConfig::default()
                    .with_width(8)
                    .with_padding(Padding::new(2, 2))
                    .with_alignment(crate::alignment::Alignment::Center),
            ]);
        
        let border = crate::BorderChars::default();
        let options = crate::RenderOptions::default();
        let result = render_table_with_headers(&data, &border, &options, &column_configs, &header_config).unwrap();
        
        assert!(result.contains("H1"));
        assert!(result.contains("H2"));
        assert!(result.contains("D1"));
        assert!(result.contains("D2"));
    }
}