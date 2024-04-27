pub mod border;

pub use border::{BorderChars, get_border_style};
pub type Row = Vec<String>;

#[derive(Debug, Clone)]
pub struct TableData {
    pub rows: Vec<Row>,
}

impl TableData {
    pub fn new(rows: Vec<Row>) -> Self {
        Self { rows }
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn column_count(&self) -> usize {
        self.rows.first().map(|row| row.len()).unwrap_or(0)
    }
}

pub fn validate_table_data(data: &TableData) -> Result<(), String> {
    if data.is_empty() {
        return Ok(());
    }

    let expected_columns = data.column_count();
    for (i, row) in data.rows.iter().enumerate() {
        if row.len() != expected_columns {
            return Err(format!(
                "Row {} has {} columns, expected {}",
                i, row.len(), expected_columns
            ));
        }
    }
    Ok(())
}

pub fn calculate_column_widths(data: &TableData) -> Vec<usize> {
    if data.is_empty() {
        return vec![];
    }
    
    let mut widths = vec![0; data.column_count()];
    
    for row in &data.rows {
        for (i, cell) in row.iter().enumerate() {
            widths[i] = widths[i].max(cell.len());
        }
    }
    
    widths
}

pub fn render_table_with_custom_borders(data: &TableData, border: &BorderChars) -> Result<String, String> {
    validate_table_data(data)?;
    
    if data.is_empty() {
        return Ok(String::new());
    }
    
    let column_widths = calculate_column_widths(data);
    let mut result = String::new();
    
    // Top border
    result.push(border.top_left);
    for (i, width) in column_widths.iter().enumerate() {
        result.push_str(&border.horizontal.to_string().repeat(width + 2));
        if i < column_widths.len() - 1 {
            result.push(border.top_junction);
        }
    }
    result.push(border.top_right);
    result.push('\n');
    
    // Data rows with side borders
    for row in &data.rows {
        result.push(border.vertical);
        for (i, cell) in row.iter().enumerate() {
            let padded_cell = format!(" {:width$} ", cell, width = column_widths[i]);
            result.push_str(&padded_cell);
            result.push(border.vertical);
        }
        result.push('\n');
    }
    
    // Bottom border
    result.push(border.bottom_left);
    for (i, width) in column_widths.iter().enumerate() {
        result.push_str(&border.horizontal.to_string().repeat(width + 2));
        if i < column_widths.len() - 1 {
            result.push(border.bottom_junction);
        }
    }
    result.push(border.bottom_right);
    result.push('\n');
    
    Ok(result)
}

pub fn render_table_with_borders(data: &TableData) -> Result<String, String> {
    render_table_with_custom_borders(data, &BorderChars::default())
}

pub fn render_table_auto_width(data: &TableData) -> Result<String, String> {
    validate_table_data(data)?;
    
    if data.is_empty() {
        return Ok(String::new());
    }
    
    let column_widths = calculate_column_widths(data);
    
    let mut result = String::new();
    
    for row in &data.rows {
        result.push('|');
        for (i, cell) in row.iter().enumerate() {
            let padded_cell = format!("{:width$}", cell, width = column_widths[i]);
            result.push(' ');
            result.push_str(&padded_cell);
            result.push(' ');
            result.push('|');
        }
        result.push('\n');
    }
    
    Ok(result)
}

pub fn render_table(data: &TableData, column_width: usize) -> Result<String, String> {
    validate_table_data(data)?;
    
    if data.is_empty() {
        return Ok(String::new());
    }

    let mut result = String::new();
    
    for row in &data.rows {
        result.push('|');
        for cell in row {
            let padded_cell = format!("{:width$}", cell, width = column_width);
            result.push(' ');
            result.push_str(&padded_cell);
            result.push(' ');
            result.push('|');
        }
        result.push('\n');
    }
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_table() {
        let data = TableData::new(vec![]);
        assert!(data.is_empty());
        assert_eq!(data.row_count(), 0);
        assert_eq!(data.column_count(), 0);
        assert!(validate_table_data(&data).is_ok());
    }

    #[test]
    fn test_valid_table() {
        let data = TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ]);
        assert!(!data.is_empty());
        assert_eq!(data.row_count(), 2);
        assert_eq!(data.column_count(), 2);
        assert!(validate_table_data(&data).is_ok());
    }

    #[test]
    fn test_invalid_table() {
        let data = TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string()], // Wrong column count
        ]);
        assert!(validate_table_data(&data).is_err());
    }

    #[test]
    fn test_render_simple_table() {
        let data = TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ]);
        let result = render_table(&data, 3).unwrap();
        assert!(result.contains("| A   | B   |"));
        assert!(result.contains("| 1   | 2   |"));
    }

    #[test]
    fn test_calculate_column_widths() {
        let data = TableData::new(vec![
            vec!["Short".to_string(), "A".to_string()],
            vec!["Very Long Text".to_string(), "B".to_string()],
        ]);
        let widths = calculate_column_widths(&data);
        assert_eq!(widths, vec![14, 1]); // "Very Long Text" = 14, "B" = 1
    }

    #[test]
    fn test_render_auto_width() {
        let data = TableData::new(vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["John".to_string(), "30".to_string()],
        ]);
        let result = render_table_auto_width(&data).unwrap();
        assert!(result.contains("| Name | Age |"));
        assert!(result.contains("| John | 30  |"));
    }

    #[test]
    fn test_render_with_borders() {
        let data = TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ]);
        let result = render_table_with_borders(&data).unwrap();
        assert!(result.contains("┌"));
        assert!(result.contains("┐"));
        assert!(result.contains("└"));
        assert!(result.contains("┘"));
        assert!(result.contains("│"));
        assert!(result.contains("─"));
    }

    #[test]
    fn test_render_with_custom_borders() {
        let data = TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ]);
        let ascii_border = BorderChars::ascii();
        let result = render_table_with_custom_borders(&data, &ascii_border).unwrap();
        assert!(result.contains("+"));
        assert!(result.contains("|"));
        assert!(result.contains("-"));
    }

    #[test]
    fn test_border_templates() {
        let data = TableData::new(vec![
            vec!["Test".to_string()],
        ]);

        // Test honeywell style
        let honeywell = get_border_style("honeywell").unwrap();
        let result = render_table_with_custom_borders(&data, &honeywell).unwrap();
        assert!(result.contains("┌"));

        // Test ramac style
        let ramac = get_border_style("ramac").unwrap();
        let result = render_table_with_custom_borders(&data, &ramac).unwrap();
        assert!(result.contains("+"));

        // Test norc style
        let norc = get_border_style("norc").unwrap();
        let result = render_table_with_custom_borders(&data, &norc).unwrap();
        assert!(result.contains("╔"));
    }
}