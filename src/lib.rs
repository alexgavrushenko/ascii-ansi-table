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
}