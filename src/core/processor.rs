use crate::types::{Row, TableConfig, ColumnConfig, VerticalAlignment};
use crate::utils::ansi::{pad_ansi_string, truncate_ansi_string};

pub fn align_table_data(rows: &[Row], config: &TableConfig) -> Vec<Row> {
    let mut result = Vec::new();
    
    for row in rows {
        let mut aligned_row = Vec::new();
        
        for (col_idx, cell) in row.iter().enumerate() {
            let column_config = config.columns.get(col_idx).unwrap_or(&config.column_default);
            let aligned_cell = align_cell(cell, column_config);
            aligned_row.push(aligned_cell);
        }
        
        result.push(aligned_row);
    }
    
    result
}

pub fn align_table_data_with_widths(rows: &[Row], config: &TableConfig, column_widths: &[usize]) -> Vec<Row> {
    let mut result = Vec::new();
    
    for row in rows {
        let mut aligned_row = Vec::new();
        
        for (col_idx, cell) in row.iter().enumerate() {
            let column_config = config.columns.get(col_idx).unwrap_or(&config.column_default);
            let column_width = column_widths.get(col_idx).unwrap_or(&0);
            let aligned_cell = align_cell_with_width(cell, column_config, *column_width);
            aligned_row.push(aligned_cell);
        }
        
        result.push(aligned_row);
    }
    
    result
}

pub fn pad_table_data(rows: &[Row], config: &TableConfig) -> Vec<Row> {
    let mut result = Vec::new();
    
    for row in rows {
        let mut padded_row = Vec::new();
        
        for (col_idx, cell) in row.iter().enumerate() {
            let column_config = config.columns.get(col_idx).unwrap_or(&config.column_default);
            let padded_cell = pad_cell(cell, column_config);
            padded_row.push(padded_cell);
        }
        
        result.push(padded_row);
    }
    
    result
}

pub fn pad_table_data_with_widths(rows: &[Row], config: &TableConfig, column_widths: &[usize]) -> Vec<Row> {
    let mut result = Vec::new();
    
    for row in rows {
        let mut padded_row = Vec::new();
        
        for (col_idx, cell) in row.iter().enumerate() {
            let column_config = config.columns.get(col_idx).unwrap_or(&config.column_default);
            let column_width = column_widths.get(col_idx).unwrap_or(&0);
            let padded_cell = pad_cell_with_width(cell, column_config, *column_width);
            padded_row.push(padded_cell);
        }
        
        result.push(padded_row);
    }
    
    result
}

pub fn truncate_table_data(rows: &[Row], config: &TableConfig) -> Vec<Row> {
    let mut result = Vec::new();
    
    for row in rows {
        let mut truncated_row = Vec::new();
        
        for (col_idx, cell) in row.iter().enumerate() {
            let column_config = config.columns.get(col_idx).unwrap_or(&config.column_default);
            let truncated_cell = truncate_cell(cell, column_config);
            truncated_row.push(truncated_cell);
        }
        
        result.push(truncated_row);
    }
    
    result
}

fn align_cell(cell: &str, config: &ColumnConfig) -> String {
    
    if config.width > 0 {
        let content_width = config.width.saturating_sub(config.padding_left + config.padding_right);
        let padded = pad_ansi_string(cell, content_width, config.alignment);
        padded.content
    } else {
        cell.to_string()
    }
}

fn align_cell_with_width(cell: &str, config: &ColumnConfig, total_width: usize) -> String {
    
    if total_width > 0 {
        let content_width = total_width.saturating_sub(config.padding_left + config.padding_right);
        let padded = pad_ansi_string(cell, content_width, config.alignment);
        padded.content
    } else {
        cell.to_string()
    }
}

fn pad_cell(cell: &str, config: &ColumnConfig) -> String {
    let left_padding = " ".repeat(config.padding_left);
    let right_padding = " ".repeat(config.padding_right);
    format!("{}{}{}", left_padding, cell, right_padding)
}

fn pad_cell_with_width(cell: &str, config: &ColumnConfig, total_width: usize) -> String {
    let left_padding = " ".repeat(config.padding_left);
    let right_padding = " ".repeat(config.padding_right);
    
    
    let current_length = left_padding.len() + crate::utils::ansi::calculate_display_width(cell) + right_padding.len();
    let additional_padding = if total_width > current_length {
        total_width - current_length
    } else {
        0
    };
    
    format!("{}{}{}{}", left_padding, cell, right_padding, " ".repeat(additional_padding))
}

fn truncate_cell(cell: &str, config: &ColumnConfig) -> String {
    if config.truncate > 0 {
        let truncated = truncate_ansi_string(cell, config.truncate);
        truncated.content
    } else {
        cell.to_string()
    }
}

pub fn apply_vertical_alignment(rows: &[Vec<Vec<String>>], heights: &[usize], config: &TableConfig) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    
    for (row_idx, row_group) in rows.iter().enumerate() {
        let target_height = heights.get(row_idx).unwrap_or(&1);
        let mut aligned_rows = vec![Vec::new(); *target_height];
        
        
        let column_count = row_group.first().map(|r| r.len()).unwrap_or(0);
        
        for col_idx in 0..column_count {
            let column_config = config.columns.get(col_idx).unwrap_or(&config.column_default);
            
            let column_data: Vec<String> = row_group.iter()
                .map(|r| r.get(col_idx).unwrap_or(&String::new()).clone())
                .collect();
            
            let aligned_column = align_column_vertically(&column_data, *target_height, column_config.vertical_alignment);
            
            for (line_idx, aligned_row) in aligned_rows.iter_mut().enumerate() {
                aligned_row.push(aligned_column.get(line_idx).unwrap_or(&String::new()).clone());
            }
        }
        
        result.extend(aligned_rows);
    }
    
    result
}

fn align_column_vertically(column_data: &[String], target_height: usize, alignment: VerticalAlignment) -> Vec<String> {
    let actual_height = column_data.len();
    
    if actual_height >= target_height {
        return column_data.to_vec();
    }
    
    let empty_lines = target_height - actual_height;
    let mut result = Vec::new();
    
    match alignment {
        VerticalAlignment::Top => {
            result.extend_from_slice(column_data);
            result.resize(target_height, String::new());
        }
        VerticalAlignment::Bottom => {
            result.resize(empty_lines, String::new());
            result.extend_from_slice(column_data);
        }
        VerticalAlignment::Middle => {
            let top_padding = empty_lines / 2;
            let bottom_padding = empty_lines - top_padding;
            
            
            result.resize(top_padding, String::new());
            
            
            result.extend_from_slice(column_data);
            
            
            for _ in 0..bottom_padding {
                result.push(String::new());
            }
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ColumnConfig, Alignment, VerticalAlignment};

    #[test]
    fn test_align_cell() {
        let mut config = ColumnConfig::default();
        config.width = 10;
        config.alignment = Alignment::Center;
        config.padding_left = 1;
        config.padding_right = 1;
        
        let result = align_cell("test", &config);
        assert_eq!(result.len(), 8); 
    }

    #[test]
    fn test_pad_cell() {
        let mut config = ColumnConfig::default();
        config.padding_left = 2;
        config.padding_right = 1;
        
        let result = pad_cell("test", &config);
        assert_eq!(result, "  test ");
    }

    #[test]
    fn test_truncate_cell() {
        let mut config = ColumnConfig::default();
        config.truncate = 5;
        
        let result = truncate_cell("this is a long text", &config);
        assert_eq!(result, "th...");
    }

    #[test]
    fn test_align_column_vertically() {
        let column_data = vec!["line1".to_string(), "line2".to_string()];
        
        let top_aligned = align_column_vertically(&column_data, 4, VerticalAlignment::Top);
        assert_eq!(top_aligned, vec!["line1", "line2", "", ""]);
        
        let bottom_aligned = align_column_vertically(&column_data, 4, VerticalAlignment::Bottom);
        assert_eq!(bottom_aligned, vec!["", "", "line1", "line2"]);
        
        let middle_aligned = align_column_vertically(&column_data, 4, VerticalAlignment::Middle);
        assert_eq!(middle_aligned, vec!["", "line1", "line2", ""]);
    }
}