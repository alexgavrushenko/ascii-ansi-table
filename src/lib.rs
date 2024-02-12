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
}