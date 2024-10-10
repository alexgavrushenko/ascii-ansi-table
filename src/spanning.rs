#[derive(Debug, Clone, PartialEq)]
pub struct CellSpan {
    pub row_span: usize,
    pub col_span: usize,
}

impl CellSpan {
    pub fn new(row_span: usize, col_span: usize) -> Self {
        Self {
            row_span: row_span.max(1),
            col_span: col_span.max(1),
        }
    }
    
    pub fn single() -> Self {
        Self::new(1, 1)
    }
    
    pub fn horizontal(col_span: usize) -> Self {
        Self::new(1, col_span)
    }
    
    pub fn vertical(row_span: usize) -> Self {
        Self::new(row_span, 1)
    }
    
    pub fn is_spanning(&self) -> bool {
        self.row_span > 1 || self.col_span > 1
    }
}

impl Default for CellSpan {
    fn default() -> Self {
        Self::single()
    }
}

#[derive(Debug, Clone)]
pub struct SpannedTableData {
    pub cells: Vec<Vec<SpannedCell>>,
    pub rows: usize,
    pub cols: usize,
}

#[derive(Debug, Clone)]
pub struct SpannedCell {
    pub content: String,
    pub span: CellSpan,
    pub is_continuation: bool, // True for cells that are part of a span but not the origin
}

impl SpannedCell {
    pub fn new(content: String) -> Self {
        Self {
            content,
            span: CellSpan::single(),
            is_continuation: false,
        }
    }
    
    pub fn with_span(content: String, span: CellSpan) -> Self {
        Self {
            content,
            span,
            is_continuation: false,
        }
    }
    
    pub fn continuation() -> Self {
        Self {
            content: String::new(),
            span: CellSpan::single(),
            is_continuation: true,
        }
    }
}

impl SpannedTableData {
    pub fn new(rows: usize, cols: usize) -> Self {
        let cells = vec![
            vec![SpannedCell::new(String::new()); cols];
            rows
        ];
        
        Self { cells, rows, cols }
    }
    
    pub fn from_regular_table(data: &crate::TableData) -> Self {
        let rows = data.row_count();
        let cols = data.column_count();
        let mut spanned_data = Self::new(rows, cols);
        
        for (row_idx, row) in data.rows.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                if row_idx < rows && col_idx < cols {
                    spanned_data.cells[row_idx][col_idx] = SpannedCell::new(cell.clone());
                }
            }
        }
        
        spanned_data
    }
    
    pub fn set_cell(&mut self, row: usize, col: usize, cell: SpannedCell) -> Result<(), String> {
        if row >= self.rows || col >= self.cols {
            return Err(format!("Cell position ({}, {}) is out of bounds", row, col));
        }
        
        // Validate span doesn't exceed table bounds
        if row + cell.span.row_span > self.rows || col + cell.span.col_span > self.cols {
            return Err("Cell span exceeds table bounds".to_string());
        }
        
        // Set the main cell
        self.cells[row][col] = cell.clone();
        
        // Mark continuation cells if spanning
        if cell.span.is_spanning() {
            for r in row..row + cell.span.row_span {
                for c in col..col + cell.span.col_span {
                    if r != row || c != col {
                        self.cells[r][c] = SpannedCell::continuation();
                    }
                }
            }
        }
        
        Ok(())
    }
    
    pub fn get_cell(&self, row: usize, col: usize) -> Option<&SpannedCell> {
        if row < self.rows && col < self.cols {
            Some(&self.cells[row][col])
        } else {
            None
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.rows == 0 || self.cols == 0
    }
}

/// Calculate the effective width needed for a spanned cell across multiple columns
pub fn calculate_spanned_width(
    column_widths: &[usize],
    start_col: usize,
    col_span: usize,
    border_width: usize,
) -> usize {
    let end_col = (start_col + col_span).min(column_widths.len());
    let total_width: usize = column_widths[start_col..end_col].iter().sum();
    
    // Add border widths between columns (but not at the edges)
    let internal_borders = if col_span > 1 { col_span - 1 } else { 0 };
    total_width + (internal_borders * border_width)
}

/// Check if a cell should be rendered (not a continuation cell)
pub fn should_render_cell(cell: &SpannedCell) -> bool {
    !cell.is_continuation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_span_creation() {
        let span = CellSpan::new(2, 3);
        assert_eq!(span.row_span, 2);
        assert_eq!(span.col_span, 3);
        assert!(span.is_spanning());
        
        let single = CellSpan::single();
        assert_eq!(single.row_span, 1);
        assert_eq!(single.col_span, 1);
        assert!(!single.is_spanning());
    }

    #[test]
    fn test_span_helpers() {
        let h_span = CellSpan::horizontal(3);
        assert_eq!(h_span.row_span, 1);
        assert_eq!(h_span.col_span, 3);
        
        let v_span = CellSpan::vertical(2);
        assert_eq!(v_span.row_span, 2);
        assert_eq!(v_span.col_span, 1);
    }

    #[test]
    fn test_spanned_cell_creation() {
        let cell = SpannedCell::new("Test".to_string());
        assert_eq!(cell.content, "Test");
        assert!(!cell.is_continuation);
        
        let continuation = SpannedCell::continuation();
        assert_eq!(continuation.content, "");
        assert!(continuation.is_continuation);
    }

    #[test]
    fn test_spanned_table_data_creation() {
        let mut table = SpannedTableData::new(3, 4);
        assert_eq!(table.rows, 3);
        assert_eq!(table.cols, 4);
        assert!(!table.is_empty());
        
        // Test setting a cell
        let cell = SpannedCell::with_span("Spanning".to_string(), CellSpan::new(2, 2));
        assert!(table.set_cell(0, 0, cell).is_ok());
        
        // Check that continuation cells were set
        assert!(table.get_cell(0, 1).unwrap().is_continuation);
        assert!(table.get_cell(1, 0).unwrap().is_continuation);
        assert!(table.get_cell(1, 1).unwrap().is_continuation);
    }

    #[test]
    fn test_from_regular_table() {
        let data = crate::TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["C".to_string(), "D".to_string()],
        ]);
        
        let spanned = SpannedTableData::from_regular_table(&data);
        assert_eq!(spanned.rows, 2);
        assert_eq!(spanned.cols, 2);
        assert_eq!(spanned.get_cell(0, 0).unwrap().content, "A");
        assert_eq!(spanned.get_cell(1, 1).unwrap().content, "D");
    }

    #[test]
    fn test_span_bounds_checking() {
        let mut table = SpannedTableData::new(2, 2);
        
        // Test out of bounds
        let cell = SpannedCell::new("Test".to_string());
        assert!(table.set_cell(3, 0, cell).is_err());
        
        // Test span exceeding bounds
        let spanning_cell = SpannedCell::with_span("Span".to_string(), CellSpan::new(2, 3));
        assert!(table.set_cell(0, 0, spanning_cell).is_err());
    }

    #[test]
    fn test_calculate_spanned_width() {
        let column_widths = vec![10, 15, 12, 8];
        
        // Single column
        let width = calculate_spanned_width(&column_widths, 0, 1, 1);
        assert_eq!(width, 10);
        
        // Two columns with border
        let width = calculate_spanned_width(&column_widths, 0, 2, 1);
        assert_eq!(width, 26); // 10 + 15 + 1 border
        
        // Three columns
        let width = calculate_spanned_width(&column_widths, 1, 3, 1);
        assert_eq!(width, 37); // 15 + 12 + 8 + 2 borders
    }

    #[test]
    fn test_should_render_cell() {
        let regular_cell = SpannedCell::new("Test".to_string());
        assert!(should_render_cell(&regular_cell));
        
        let continuation_cell = SpannedCell::continuation();
        assert!(!should_render_cell(&continuation_cell));
    }

    #[test]
    fn test_zero_span_protection() {
        let span = CellSpan::new(0, 0);
        assert_eq!(span.row_span, 1);
        assert_eq!(span.col_span, 1);
        assert!(!span.is_spanning());
    }
}