use crate::types::{ColumnConfig, Row, SpanningCellConfig, TableConfig, TableError};
use crate::utils::formatting::validate_table_data;

pub fn validate_config(config: &TableConfig) -> Result<(), TableError> {
    validate_column_configs(&config.columns)?;
    validate_column_config(&config.column_default)?;
    validate_spanning_cell_configs(&config.spanning_cells)?;
    Ok(())
}

pub fn validate_column_configs(columns: &[ColumnConfig]) -> Result<(), TableError> {
    for config in columns {
        validate_column_config(config)?;
    }
    Ok(())
}

pub fn validate_column_config(config: &ColumnConfig) -> Result<(), TableError> {
    if config.width > 0 && config.width < config.padding_left + config.padding_right {
        return Err(TableError::InvalidConfig(
            "Column width must be greater than padding".to_string(),
        ));
    }

    if config.truncate > 0 && config.truncate < 3 {
        return Err(TableError::InvalidConfig(
            "Truncate width must be at least 3 characters".to_string(),
        ));
    }

    Ok(())
}

pub fn validate_spanning_cell_configs(
    spanning_cells: &[SpanningCellConfig],
) -> Result<(), TableError> {
    for config in spanning_cells {
        validate_spanning_cell_config(config)?;
    }

    check_spanning_cell_overlaps(spanning_cells)?;
    Ok(())
}

pub fn validate_spanning_cell_config(config: &SpanningCellConfig) -> Result<(), TableError> {
    if config.col_span.unwrap_or(1) == 0 {
        return Err(TableError::InvalidConfig(
            "Column span must be greater than 0".to_string(),
        ));
    }

    if config.row_span.unwrap_or(1) == 0 {
        return Err(TableError::InvalidConfig(
            "Row span must be greater than 0".to_string(),
        ));
    }

    Ok(())
}

pub fn check_spanning_cell_overlaps(
    spanning_cells: &[SpanningCellConfig],
) -> Result<(), TableError> {
    for (i, cell1) in spanning_cells.iter().enumerate() {
        for cell2 in spanning_cells.iter().skip(i + 1) {
            if spanning_cells_overlap(cell1, cell2) {
                return Err(TableError::InvalidConfig(
                    "Spanning cells cannot overlap".to_string(),
                ));
            }
        }
    }
    Ok(())
}

fn spanning_cells_overlap(cell1: &SpanningCellConfig, cell2: &SpanningCellConfig) -> bool {
    let cell1_end_col = cell1.col + cell1.col_span.unwrap_or(1);
    let cell1_end_row = cell1.row + cell1.row_span.unwrap_or(1);
    let cell2_end_col = cell2.col + cell2.col_span.unwrap_or(1);
    let cell2_end_row = cell2.row + cell2.row_span.unwrap_or(1);

    !(cell1_end_col <= cell2.col
        || cell2_end_col <= cell1.col
        || cell1_end_row <= cell2.row
        || cell2_end_row <= cell1.row)
}

pub fn validate_table_data_with_config(
    rows: &[Row],
    config: &TableConfig,
) -> Result<(), TableError> {
    validate_table_data(rows)?;

    if rows.is_empty() {
        return Ok(());
    }

    let column_count = rows[0].len();

    for spanning_cell in &config.spanning_cells {
        if spanning_cell.col >= column_count {
            return Err(TableError::InvalidConfig(
                "Spanning cell column index out of bounds".to_string(),
            ));
        }

        if spanning_cell.row >= rows.len() {
            return Err(TableError::InvalidConfig(
                "Spanning cell row index out of bounds".to_string(),
            ));
        }

        let end_col = spanning_cell.col + spanning_cell.col_span.unwrap_or(1);
        let end_row = spanning_cell.row + spanning_cell.row_span.unwrap_or(1);

        if end_col > column_count {
            return Err(TableError::InvalidConfig(
                "Spanning cell extends beyond table columns".to_string(),
            ));
        }

        if end_row > rows.len() {
            return Err(TableError::InvalidConfig(
                "Spanning cell extends beyond table rows".to_string(),
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ColumnConfig, SpanningCellConfig};

    #[test]
    fn test_validate_column_config() {
        let mut config = ColumnConfig::default();
        config.width = 10;
        config.padding_left = 2;
        config.padding_right = 2;
        assert!(validate_column_config(&config).is_ok());

        config.width = 3;
        assert!(validate_column_config(&config).is_err());
    }

    #[test]
    fn test_validate_spanning_cell_config() {
        let config = SpanningCellConfig {
            col: 0,
            row: 0,
            col_span: Some(2),
            row_span: Some(2),
            alignment: None,
            vertical_alignment: None,
            padding_left: None,
            padding_right: None,
            truncate: None,
            wrap_word: None,
        };
        assert!(validate_spanning_cell_config(&config).is_ok());

        let invalid_config = SpanningCellConfig {
            col: 0,
            row: 0,
            col_span: Some(0),
            row_span: Some(2),
            alignment: None,
            vertical_alignment: None,
            padding_left: None,
            padding_right: None,
            truncate: None,
            wrap_word: None,
        };
        assert!(validate_spanning_cell_config(&invalid_config).is_err());
    }

    #[test]
    fn test_spanning_cells_overlap() {
        let cell1 = SpanningCellConfig {
            col: 0,
            row: 0,
            col_span: Some(2),
            row_span: Some(2),
            alignment: None,
            vertical_alignment: None,
            padding_left: None,
            padding_right: None,
            truncate: None,
            wrap_word: None,
        };

        let cell2 = SpanningCellConfig {
            col: 1,
            row: 1,
            col_span: Some(2),
            row_span: Some(2),
            alignment: None,
            vertical_alignment: None,
            padding_left: None,
            padding_right: None,
            truncate: None,
            wrap_word: None,
        };

        assert!(spanning_cells_overlap(&cell1, &cell2));

        let cell3 = SpanningCellConfig {
            col: 3,
            row: 3,
            col_span: Some(2),
            row_span: Some(2),
            alignment: None,
            vertical_alignment: None,
            padding_left: None,
            padding_right: None,
            truncate: None,
            wrap_word: None,
        };

        assert!(!spanning_cells_overlap(&cell1, &cell3));
    }
}
