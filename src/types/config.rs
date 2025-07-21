use super::{Alignment, BorderConfig, BorderUserConfig, VerticalAlignment};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellConfig {
    pub alignment: Alignment,
    pub vertical_alignment: VerticalAlignment,
    pub padding_left: usize,
    pub padding_right: usize,
    pub truncate: usize,
    pub wrap_word: bool,
}

impl Default for CellConfig {
    fn default() -> Self {
        Self {
            alignment: Alignment::Left,
            vertical_alignment: VerticalAlignment::Top,
            padding_left: 1,
            padding_right: 1,
            truncate: 0,
            wrap_word: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellUserConfig {
    pub alignment: Option<Alignment>,
    pub vertical_alignment: Option<VerticalAlignment>,
    pub padding_left: Option<usize>,
    pub padding_right: Option<usize>,
    pub truncate: Option<usize>,
    pub wrap_word: Option<bool>,
}

impl CellUserConfig {
    pub fn merge_with_default(self, default: &CellConfig) -> CellConfig {
        CellConfig {
            alignment: self.alignment.unwrap_or(default.alignment),
            vertical_alignment: self
                .vertical_alignment
                .unwrap_or(default.vertical_alignment),
            padding_left: self.padding_left.unwrap_or(default.padding_left),
            padding_right: self.padding_right.unwrap_or(default.padding_right),
            truncate: self.truncate.unwrap_or(default.truncate),
            wrap_word: self.wrap_word.unwrap_or(default.wrap_word),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColumnConfig {
    pub alignment: Alignment,
    pub vertical_alignment: VerticalAlignment,
    pub padding_left: usize,
    pub padding_right: usize,
    pub truncate: usize,
    pub wrap_word: bool,
    pub width: usize,
}

impl Default for ColumnConfig {
    fn default() -> Self {
        Self {
            alignment: Alignment::Left,
            vertical_alignment: VerticalAlignment::Top,
            padding_left: 1,
            padding_right: 1,
            truncate: 0,
            wrap_word: false,
            width: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ColumnUserConfig {
    pub alignment: Option<Alignment>,
    pub vertical_alignment: Option<VerticalAlignment>,
    pub padding_left: Option<usize>,
    pub padding_right: Option<usize>,
    pub truncate: Option<usize>,
    pub wrap_word: Option<bool>,
    pub width: Option<usize>,
}

impl ColumnUserConfig {
    pub fn merge_with_default(self, default: &ColumnConfig) -> ColumnConfig {
        ColumnConfig {
            alignment: self.alignment.unwrap_or(default.alignment),
            vertical_alignment: self
                .vertical_alignment
                .unwrap_or(default.vertical_alignment),
            padding_left: self.padding_left.unwrap_or(default.padding_left),
            padding_right: self.padding_right.unwrap_or(default.padding_right),
            truncate: self.truncate.unwrap_or(default.truncate),
            wrap_word: self.wrap_word.unwrap_or(default.wrap_word),
            width: self.width.unwrap_or(default.width),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellCoordinates {
    pub col: usize,
    pub row: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpanningCellConfig {
    pub col: usize,
    pub row: usize,
    pub col_span: Option<usize>,
    pub row_span: Option<usize>,
    pub alignment: Option<Alignment>,
    pub vertical_alignment: Option<VerticalAlignment>,
    pub padding_left: Option<usize>,
    pub padding_right: Option<usize>,
    pub truncate: Option<usize>,
    pub wrap_word: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RangeCoordinate {
    pub top_left: CellCoordinates,
    pub bottom_right: CellCoordinates,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RangeConfig {
    pub top_left: CellCoordinates,
    pub bottom_right: CellCoordinates,
    pub config: CellConfig,
}

pub type DrawVerticalLine = fn(line_index: usize, column_count: usize) -> bool;
pub type DrawHorizontalLine = fn(line_index: usize, row_count: usize) -> bool;

pub fn default_draw_vertical_line(_line_index: usize, _column_count: usize) -> bool {
    true
}

pub fn default_draw_horizontal_line(_line_index: usize, _row_count: usize) -> bool {
    true
}

#[derive(Debug, Clone)]
pub struct TableConfig {
    pub border: BorderConfig,
    pub columns: Vec<ColumnConfig>,
    pub column_default: ColumnConfig,
    pub draw_vertical_line: DrawVerticalLine,
    pub draw_horizontal_line: DrawHorizontalLine,
    pub single_line: bool,
    pub spanning_cells: Vec<SpanningCellConfig>,
    pub header: Option<Box<TableConfig>>,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            border: BorderConfig::default(),
            columns: Vec::new(),
            column_default: ColumnConfig::default(),
            draw_vertical_line: default_draw_vertical_line,
            draw_horizontal_line: default_draw_horizontal_line,
            single_line: false,
            spanning_cells: Vec::new(),
            header: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TableUserConfig {
    pub border: Option<BorderUserConfig>,
    pub columns: Option<Vec<ColumnUserConfig>>,
    pub column_default: Option<ColumnUserConfig>,
    pub single_line: Option<bool>,
    pub spanning_cells: Option<Vec<SpanningCellConfig>>,
    pub header: Option<Box<TableUserConfig>>,
}

impl TableUserConfig {
    pub fn merge_with_default(self, default: &TableConfig) -> TableConfig {
        let border = self
            .border
            .map(|b| b.merge_with_default(&default.border))
            .unwrap_or_else(|| default.border.clone());

        let column_default = self
            .column_default
            .map(|c| c.merge_with_default(&default.column_default))
            .unwrap_or_else(|| default.column_default.clone());

        let columns = self
            .columns
            .map(|cols| {
                cols.into_iter()
                    .map(|c| c.merge_with_default(&column_default))
                    .collect()
            })
            .unwrap_or_else(|| default.columns.clone());

        let header = self
            .header
            .map(|h| Box::new(h.merge_with_default(&TableConfig::default())));

        TableConfig {
            border,
            columns,
            column_default,
            draw_vertical_line: default.draw_vertical_line,
            draw_horizontal_line: default.draw_horizontal_line,
            single_line: self.single_line.unwrap_or(default.single_line),
            spanning_cells: self
                .spanning_cells
                .unwrap_or_else(|| default.spanning_cells.clone()),
            header,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StreamConfig {
    pub border: BorderConfig,
    pub columns: Vec<ColumnConfig>,
    pub column_default: ColumnConfig,
    pub draw_vertical_line: DrawVerticalLine,
    pub draw_horizontal_line: DrawHorizontalLine,
    pub single_line: bool,
}

impl Default for StreamConfig {
    fn default() -> Self {
        Self {
            border: BorderConfig::default(),
            columns: Vec::new(),
            column_default: ColumnConfig::default(),
            draw_vertical_line: default_draw_vertical_line,
            draw_horizontal_line: default_draw_horizontal_line,
            single_line: false,
        }
    }
}

impl From<StreamConfig> for TableConfig {
    fn from(stream_config: StreamConfig) -> Self {
        Self {
            border: stream_config.border,
            columns: stream_config.columns,
            column_default: stream_config.column_default,
            draw_vertical_line: stream_config.draw_vertical_line,
            draw_horizontal_line: stream_config.draw_horizontal_line,
            single_line: stream_config.single_line,
            spanning_cells: Vec::new(),
            header: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamUserConfig {
    pub border: Option<BorderUserConfig>,
    pub columns: Option<Vec<ColumnUserConfig>>,
    pub column_default: Option<ColumnUserConfig>,
    pub single_line: Option<bool>,
}
