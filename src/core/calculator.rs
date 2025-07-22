use crate::types::{Row, TableConfig};
use crate::utils::{calculate_cell_height, calculate_maximum_column_widths, wrap_cell};

pub fn calculate_output_column_widths(rows: &[Row], config: &TableConfig) -> Vec<usize> {
    let max_widths = calculate_maximum_column_widths(rows);
    let mut output_widths = Vec::new();

    for (i, &max_width) in max_widths.iter().enumerate() {
        let column_config = config.columns.get(i).unwrap_or(&config.column_default);

        let width = if column_config.width > 0 {
            column_config.width
        } else {
            max_width + column_config.padding_left + column_config.padding_right
        };

        let min_width = column_config.padding_left + column_config.padding_right + 1;
        output_widths.push(width.max(min_width));
    }

    output_widths
}

pub fn calculate_row_heights(rows: &[Row], config: &TableConfig) -> Vec<usize> {
    let column_widths = calculate_output_column_widths(rows, config);
    let mut row_heights = Vec::new();

    for row in rows {
        let mut max_height = 1;

        for (col_idx, cell) in row.iter().enumerate() {
            let column_config = config
                .columns
                .get(col_idx)
                .unwrap_or(&config.column_default);
            let cell_width = column_widths.get(col_idx).unwrap_or(&0);
            let content_width =
                cell_width.saturating_sub(column_config.padding_left + column_config.padding_right);

            let height = calculate_cell_height(cell, content_width, column_config.wrap_word);
            max_height = max_height.max(height);
        }

        row_heights.push(max_height);
    }

    row_heights
}

pub fn map_data_using_row_heights(
    rows: &[Row],
    row_heights: &[usize],
    config: &TableConfig,
) -> Vec<Vec<Row>> {
    let column_widths = calculate_output_column_widths(rows, config);
    let mut result = Vec::new();

    for (row_idx, row) in rows.iter().enumerate() {
        let row_height = row_heights.get(row_idx).unwrap_or(&1);
        let mut mapped_rows = vec![Vec::new(); *row_height];

        for (col_idx, cell) in row.iter().enumerate() {
            let column_config = config
                .columns
                .get(col_idx)
                .unwrap_or(&config.column_default);
            let cell_width = column_widths.get(col_idx).unwrap_or(&0);
            let content_width =
                cell_width.saturating_sub(column_config.padding_left + column_config.padding_right);

            let wrapped_lines = wrap_cell(cell, content_width, column_config.wrap_word);

            for (line_idx, mapped_row) in mapped_rows.iter_mut().enumerate() {
                let line_content = wrapped_lines
                    .get(line_idx)
                    .unwrap_or(&String::new())
                    .clone();
                mapped_row.push(line_content);
            }
        }

        result.push(mapped_rows);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TableConfig;

    #[test]
    fn test_calculate_output_column_widths() {
        let rows = vec![
            vec!["short".to_string(), "longer text".to_string()],
            vec!["a".to_string(), "b".to_string()],
        ];

        let config = TableConfig::default();
        let widths = calculate_output_column_widths(&rows, &config);
        assert_eq!(widths, vec![7, 13]);
    }

    #[test]
    fn test_calculate_row_heights() {
        let rows = vec![
            vec!["line1\nline2".to_string(), "single".to_string()],
            vec!["short".to_string(), "also short".to_string()],
        ];

        let config = TableConfig::default();
        let heights = calculate_row_heights(&rows, &config);

        assert_eq!(heights.len(), 2);
        assert!(heights[0] >= 2);
        assert!(heights[1] >= 1);
    }

    #[test]
    fn test_map_data_using_row_heights() {
        let rows = vec![vec!["line1\nline2".to_string(), "single".to_string()]];

        let config = TableConfig::default();
        let row_heights = vec![2];
        let mapped = map_data_using_row_heights(&rows, &row_heights, &config);

        println!("Mapped: {mapped:?}");

        assert_eq!(mapped.len(), 1);
        assert_eq!(mapped[0].len(), 2);

        assert_eq!(mapped[0][0].len(), 2);
        assert_eq!(mapped[0][1].len(), 2);

        assert!(mapped[0][0].join("").contains("line1"));
        assert!(mapped[0][0].join("").contains("single"));
    }

    #[test]
    fn test_map_data_with_repeated_newlines() {
        let rows = vec![
            vec!["Column 1".to_string(), "Column 2".to_string()],
            vec!["Normal".to_string(), "\n".repeat(10)],
        ];

        let config = TableConfig::default();
        let row_heights = calculate_row_heights(&rows, &config);
        let mapped = map_data_using_row_heights(&rows, &row_heights, &config);

        println!("Row heights: {row_heights:?}");
        println!("Mapped data: {mapped:#?}");

        assert_eq!(row_heights[1], 11, "Second row should have height 11");

        assert_eq!(mapped.len(), 2);
        assert_eq!(mapped[1].len(), 11);

        for (sub_row_idx, sub_row) in mapped[1].iter().enumerate() {
            println!("Sub-row {sub_row_idx}: {sub_row:?}");
            assert_eq!(sub_row.len(), 2, "Each sub-row should have 2 cells");

            if sub_row_idx == 0 {
                assert_eq!(sub_row[0], "Normal");
            } else {
                assert_eq!(sub_row[0], "");
            }

            assert_eq!(sub_row[1], "");
        }
    }

    #[test]
    fn test_map_data_with_realistic_multiline() {
        let rows = vec![
            vec!["ID".to_string(), "Status".to_string()],
            vec![
                "6".to_string(),
                format!("Webcam\n{}\n✓ Active", "\n".repeat(8)),
            ],
        ];

        let config = TableConfig::default();
        let row_heights = calculate_row_heights(&rows, &config);
        let mapped = map_data_using_row_heights(&rows, &row_heights, &config);

        println!("Realistic test - Row heights: {row_heights:?}");
        println!("Realistic test - Mapped data: {mapped:#?}");

        assert_eq!(row_heights[1], 11);

        let second_row = &mapped[1];
        assert_eq!(second_row[0][1], "Webcam");
        assert_eq!(second_row[10][1], "✓ Active");

        for i in 1..10 {
            assert_eq!(second_row[i][1], "");
        }
    }
}
