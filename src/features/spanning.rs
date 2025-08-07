use crate::types::{
    CellConfig, CellCoordinates, ColumnConfig, RangeConfig, RangeCoordinate, Row,
    SpanningCellConfig,
};

#[derive(Debug, Clone)]
pub struct SpanningCellManager {
    ranges: Vec<RangeConfig>,
}

impl SpanningCellManager {
    pub fn new(configs: &[SpanningCellConfig], columns: &[ColumnConfig]) -> Self {
        let mut ranges = Vec::new();

        for config in configs {
            let range = create_range_config(config, columns);
            ranges.push(range);
        }

        Self { ranges }
    }

    pub fn get_containing_range(&self, cell: &CellCoordinates) -> Option<&RangeConfig> {
        self.ranges.iter().find(|range| {
            cell.col >= range.top_left.col
                && cell.col <= range.bottom_right.col
                && cell.row >= range.top_left.row
                && cell.row <= range.bottom_right.row
        })
    }

    pub fn in_same_range(&self, cell1: &CellCoordinates, cell2: &CellCoordinates) -> bool {
        if let (Some(range1), Some(range2)) = (
            self.get_containing_range(cell1),
            self.get_containing_range(cell2),
        ) {
            return range1.top_left == range2.top_left
                && range1.bottom_right == range2.bottom_right;
        }
        false
    }

    pub fn get_range_config_table(&self) -> &[RangeConfig] {
        &self.ranges
    }

    pub fn is_cell_in_span(&self, cell: &CellCoordinates) -> bool {
        self.get_containing_range(cell).is_some()
    }

    pub fn get_span_origin(&self, cell: &CellCoordinates) -> Option<CellCoordinates> {
        self.get_containing_range(cell)
            .map(|range| range.top_left.clone())
    }

    pub fn should_render_cell(&self, cell: &CellCoordinates) -> bool {
        if let Some(range) = self.get_containing_range(cell) {
            cell.col == range.top_left.col && cell.row == range.top_left.row
        } else {
            true
        }
    }

    pub fn get_cell_span_info(&self, cell: &CellCoordinates) -> SpanInfo {
        if let Some(range) = self.get_containing_range(cell) {
            SpanInfo {
                is_spanning: true,
                is_origin: cell.col == range.top_left.col && cell.row == range.top_left.row,
                col_span: range.bottom_right.col - range.top_left.col + 1,
                row_span: range.bottom_right.row - range.top_left.row + 1,
                origin: range.top_left.clone(),
            }
        } else {
            SpanInfo {
                is_spanning: false,
                is_origin: true,
                col_span: 1,
                row_span: 1,
                origin: cell.clone(),
            }
        }
    }

    pub fn extract_spanning_cell_content(
        &self,
        cell: &CellCoordinates,
        data: &[Row],
    ) -> Option<String> {
        if let Some(range) = self.get_containing_range(cell)
            && cell.col == range.top_left.col
            && cell.row == range.top_left.row
            && let Some(row) = data.get(range.top_left.row)
            && let Some(cell_content) = row.get(range.top_left.col)
        {
            return Some(cell_content.clone());
        }

        None
    }
}

#[derive(Debug, Clone)]
pub struct SpanInfo {
    pub is_spanning: bool,
    pub is_origin: bool,
    pub col_span: usize,
    pub row_span: usize,
    pub origin: CellCoordinates,
}

fn create_range_config(config: &SpanningCellConfig, columns: &[ColumnConfig]) -> RangeConfig {
    let col_span = config.col_span.unwrap_or(1);
    let row_span = config.row_span.unwrap_or(1);

    let top_left = CellCoordinates {
        col: config.col,
        row: config.row,
    };

    let bottom_right = CellCoordinates {
        col: config.col + col_span - 1,
        row: config.row + row_span - 1,
    };

    let default_column_config = ColumnConfig::default();
    let default_column = columns.get(config.col).unwrap_or(&default_column_config);

    let cell_config = CellConfig {
        alignment: config.alignment.unwrap_or(default_column.alignment),
        vertical_alignment: config
            .vertical_alignment
            .unwrap_or(default_column.vertical_alignment),
        padding_left: config.padding_left.unwrap_or(default_column.padding_left),
        padding_right: config.padding_right.unwrap_or(default_column.padding_right),
        truncate: config.truncate.unwrap_or(default_column.truncate),
        wrap_word: config.wrap_word.unwrap_or(default_column.wrap_word),
    };

    RangeConfig {
        top_left,
        bottom_right,
        config: cell_config,
    }
}

pub fn calculate_range_coordinate(spanning_cell_config: &SpanningCellConfig) -> RangeCoordinate {
    let col_span = spanning_cell_config.col_span.unwrap_or(1);
    let row_span = spanning_cell_config.row_span.unwrap_or(1);

    RangeCoordinate {
        top_left: CellCoordinates {
            col: spanning_cell_config.col,
            row: spanning_cell_config.row,
        },
        bottom_right: CellCoordinates {
            col: spanning_cell_config.col + col_span - 1,
            row: spanning_cell_config.row + row_span - 1,
        },
    }
}

pub fn are_cells_equal(cell1: &CellCoordinates, cell2: &CellCoordinates) -> bool {
    cell1.col == cell2.col && cell1.row == cell2.row
}

pub fn is_cell_in_range(cell: &CellCoordinates, range: &RangeCoordinate) -> bool {
    cell.col >= range.top_left.col
        && cell.col <= range.bottom_right.col
        && cell.row >= range.top_left.row
        && cell.row <= range.bottom_right.row
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ColumnConfig, SpanningCellConfig};

    #[test]
    fn test_spanning_cell_manager_creation() {
        let configs = vec![SpanningCellConfig {
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
        }];

        let columns = vec![ColumnConfig::default(); 3];
        let manager = SpanningCellManager::new(&configs, &columns);

        assert_eq!(manager.ranges.len(), 1);
        assert_eq!(manager.ranges[0].top_left.col, 0);
        assert_eq!(manager.ranges[0].top_left.row, 0);
        assert_eq!(manager.ranges[0].bottom_right.col, 1);
        assert_eq!(manager.ranges[0].bottom_right.row, 1);
    }

    #[test]
    fn test_get_containing_range() {
        let configs = vec![SpanningCellConfig {
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
        }];

        let columns = vec![ColumnConfig::default(); 3];
        let manager = SpanningCellManager::new(&configs, &columns);

        let cell_in_range = CellCoordinates { col: 1, row: 1 };
        let cell_out_of_range = CellCoordinates { col: 2, row: 2 };

        assert!(manager.get_containing_range(&cell_in_range).is_some());
        assert!(manager.get_containing_range(&cell_out_of_range).is_none());
    }

    #[test]
    fn test_in_same_range() {
        let configs = vec![SpanningCellConfig {
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
        }];

        let columns = vec![ColumnConfig::default(); 3];
        let manager = SpanningCellManager::new(&configs, &columns);

        let cell1 = CellCoordinates { col: 0, row: 0 };
        let cell2 = CellCoordinates { col: 1, row: 1 };
        let cell3 = CellCoordinates { col: 2, row: 2 };

        assert!(manager.in_same_range(&cell1, &cell2));
        assert!(!manager.in_same_range(&cell1, &cell3));
    }

    #[test]
    fn test_should_render_cell() {
        let configs = vec![SpanningCellConfig {
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
        }];

        let columns = vec![ColumnConfig::default(); 3];
        let manager = SpanningCellManager::new(&configs, &columns);

        let origin_cell = CellCoordinates { col: 0, row: 0 };
        let spanned_cell = CellCoordinates { col: 1, row: 1 };
        let normal_cell = CellCoordinates { col: 2, row: 2 };

        assert!(manager.should_render_cell(&origin_cell));
        assert!(!manager.should_render_cell(&spanned_cell));
        assert!(manager.should_render_cell(&normal_cell));
    }

    #[test]
    fn test_calculate_range_coordinate() {
        let config = SpanningCellConfig {
            col: 1,
            row: 2,
            col_span: Some(3),
            row_span: Some(2),
            alignment: None,
            vertical_alignment: None,
            padding_left: None,
            padding_right: None,
            truncate: None,
            wrap_word: None,
        };

        let range = calculate_range_coordinate(&config);

        assert_eq!(range.top_left.col, 1);
        assert_eq!(range.top_left.row, 2);
        assert_eq!(range.bottom_right.col, 3);
        assert_eq!(range.bottom_right.row, 3);
    }

    #[test]
    fn test_are_cells_equal() {
        let cell1 = CellCoordinates { col: 1, row: 2 };
        let cell2 = CellCoordinates { col: 1, row: 2 };
        let cell3 = CellCoordinates { col: 2, row: 1 };

        assert!(are_cells_equal(&cell1, &cell2));
        assert!(!are_cells_equal(&cell1, &cell3));
    }

    #[test]
    fn test_is_cell_in_range() {
        let range = RangeCoordinate {
            top_left: CellCoordinates { col: 1, row: 1 },
            bottom_right: CellCoordinates { col: 3, row: 3 },
        };

        let cell_in_range = CellCoordinates { col: 2, row: 2 };
        let cell_out_of_range = CellCoordinates { col: 0, row: 0 };

        assert!(is_cell_in_range(&cell_in_range, &range));
        assert!(!is_cell_in_range(&cell_out_of_range, &range));
    }
}
