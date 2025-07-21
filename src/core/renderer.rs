use crate::core::calculator::{
    calculate_output_column_widths, calculate_row_heights, map_data_using_row_heights,
};
use crate::core::processor::{
    align_table_data_with_widths, pad_table_data_with_widths, truncate_table_data,
};
use crate::types::{BorderConfig, Row, TableConfig};

pub fn draw_table(rows: &[Row], config: &TableConfig) -> String {
    if rows.is_empty() {
        return String::new();
    }

    let truncated_rows = truncate_table_data(rows, config);

    let column_widths = calculate_output_column_widths(&truncated_rows, config);

    let row_heights = calculate_row_heights(&truncated_rows, config);
    let mapped_data = map_data_using_row_heights(&truncated_rows, &row_heights, config);

    let processed_data = process_mapped_data(&mapped_data, config, &column_widths);

    let has_header = config.header.is_some();
    let header_config = config.header.as_ref().map(|h| h.as_ref()).unwrap_or(config);

    let mut result = String::new();

    if (config.draw_horizontal_line)(0, rows.len()) {
        let border_config = if has_header {
            &header_config.border
        } else {
            &config.border
        };
        result.push_str(&draw_border_line(
            &column_widths,
            border_config,
            BorderType::Top,
        ));
        result.push('\n');
    }

    for (row_idx, row_group) in processed_data.iter().enumerate() {
        let is_header_row = has_header && row_idx == 0;
        let current_config = if is_header_row { header_config } else { config };

        for sub_row in row_group.iter() {
            result.push_str(&draw_row(sub_row, &column_widths, &current_config.border));
            result.push('\n');
        }

        if row_idx < processed_data.len() - 1 {
            if is_header_row {
                result.push_str(&draw_border_line(
                    &column_widths,
                    &header_config.border,
                    BorderType::Header,
                ));
                result.push('\n');
            } else if (config.draw_horizontal_line)(row_idx + 1, rows.len()) {
                result.push_str(&draw_border_line(
                    &column_widths,
                    &config.border,
                    BorderType::Join,
                ));
                result.push('\n');
            }
        }
    }

    if (config.draw_horizontal_line)(rows.len(), rows.len()) {
        result.push_str(&draw_border_line(
            &column_widths,
            &config.border,
            BorderType::Bottom,
        ));
    }

    result
}

fn process_mapped_data(
    mapped_data: &[Vec<Vec<String>>],
    config: &TableConfig,
    column_widths: &[usize],
) -> Vec<Vec<Vec<String>>> {
    let mut result = Vec::new();

    for row_group in mapped_data {
        let mut processed_row_group = Vec::new();

        for sub_row in row_group {
            let aligned_row =
                align_table_data_with_widths(&[sub_row.clone()], config, column_widths);
            let padded_row = pad_table_data_with_widths(&aligned_row, config, column_widths);

            if let Some(processed_sub_row) = padded_row.first() {
                processed_row_group.push(processed_sub_row.clone());
            }
        }

        result.push(processed_row_group);
    }

    result
}

pub fn draw_row(row: &[String], _column_widths: &[usize], border: &BorderConfig) -> String {
    let mut result = String::new();

    result.push_str(&border.body_left);

    for (col_idx, cell) in row.iter().enumerate() {
        result.push_str(cell);

        if col_idx < row.len() - 1 {
            result.push_str(&border.body_join);
        }
    }

    result.push_str(&border.body_right);

    result
}

#[derive(Debug, Clone, Copy)]
pub enum BorderType {
    Top,
    Bottom,
    Join,
    Header,
}

pub fn draw_border_line(
    column_widths: &[usize],
    border: &BorderConfig,
    border_type: BorderType,
) -> String {
    let mut result = String::new();

    let (left, right, body, join) = match border_type {
        BorderType::Top => (
            &border.top_left,
            &border.top_right,
            &border.top_body,
            &border.top_join,
        ),
        BorderType::Bottom => (
            &border.bottom_left,
            &border.bottom_right,
            &border.bottom_body,
            &border.bottom_join,
        ),
        BorderType::Join => (
            &border.join_left,
            &border.join_right,
            &border.join_body,
            &border.join_join,
        ),
        BorderType::Header => (
            &border.join_left,
            &border.join_right,
            &border.header_join,
            &border.join_join,
        ),
    };

    result.push_str(left);

    for (col_idx, &width) in column_widths.iter().enumerate() {
        result.push_str(&body.repeat(width));

        if col_idx < column_widths.len() - 1 {
            result.push_str(join);
        }
    }

    result.push_str(right);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TableConfig;

    #[test]
    fn test_draw_row() {
        let row = vec!["hello".to_string(), "world".to_string()];
        let column_widths = vec![7, 7];
        let border = crate::types::BorderConfig::default();

        let result = draw_row(&row, &column_widths, &border);
        assert!(result.contains("hello"));
        assert!(result.contains("world"));
        assert!(result.contains("│"));
    }

    #[test]
    fn test_draw_border_line() {
        let column_widths = vec![5, 5];
        let border = crate::types::BorderConfig::default();

        let top_border = draw_border_line(&column_widths, &border, BorderType::Top);
        assert!(top_border.contains("┌"));
        assert!(top_border.contains("┐"));
        assert!(top_border.contains("─"));
        assert!(top_border.contains("┬"));

        let bottom_border = draw_border_line(&column_widths, &border, BorderType::Bottom);
        assert!(bottom_border.contains("└"));
        assert!(bottom_border.contains("┘"));
        assert!(bottom_border.contains("┴"));
    }

    #[test]
    fn test_draw_table_basic() {
        let rows = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
        ];

        let config = TableConfig::default();
        let result = draw_table(&rows, &config);

        assert!(result.contains("a"));
        assert!(result.contains("b"));
        assert!(result.contains("c"));
        assert!(result.contains("d"));
        assert!(result.contains("┌"));
        assert!(result.contains("└"));
    }

    #[test]
    fn test_draw_table_empty() {
        let rows: Vec<Vec<String>> = vec![];
        let config = TableConfig::default();
        let result = draw_table(&rows, &config);

        assert_eq!(result, "");
    }
}
