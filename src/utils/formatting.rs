use crate::types::{Row, TableError};
use crate::utils::ansi::calculate_display_width;

pub fn normalize_string(input: &str) -> Result<String, TableError> {
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch.is_control() && ch != '\n' && ch != '\t' {
            if ch == '\u{1b}' {
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == '[' {
                        chars.next();
                        for ansi_ch in chars.by_ref() {
                            if ansi_ch.is_ascii_alphabetic() {
                                break;
                            }
                        }
                        continue;
                    }
                }
            }
            return Err(TableError::ControlCharacters);
        }
    }
    Ok(input.to_string())
}

pub fn stringify_table_data(rows: &[Row]) -> Result<Vec<Row>, TableError> {
    let mut result = Vec::new();

    for row in rows {
        let mut string_row = Vec::new();
        for cell in row {
            let normalized = normalize_string(cell)?;
            string_row.push(normalized);
        }
        result.push(string_row);
    }

    Ok(result)
}

pub fn validate_table_data(rows: &[Row]) -> Result<(), TableError> {
    if rows.is_empty() {
        return Ok(());
    }

    let expected_length = rows[0].len();

    for row in rows.iter() {
        if row.len() != expected_length {
            return Err(TableError::InconsistentRowLength);
        }

        for cell in row {
            normalize_string(cell)?;
        }
    }

    Ok(())
}

pub fn calculate_maximum_column_widths(rows: &[Row]) -> Vec<usize> {
    if rows.is_empty() {
        return Vec::new();
    }

    let column_count = rows[0].len();
    let mut max_widths = vec![0; column_count];

    for row in rows {
        for (col_idx, cell) in row.iter().enumerate() {
            let lines = cell.lines().collect::<Vec<_>>();
            let max_line_width = lines
                .iter()
                .map(|line| calculate_display_width(line))
                .max()
                .unwrap_or(0);

            max_widths[col_idx] = max_widths[col_idx].max(max_line_width);
        }
    }

    max_widths
}

pub fn group_by_sizes<T: Clone>(array: &[T], sizes: &[usize]) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    let mut start = 0;

    for &size in sizes {
        let end = (start + size).min(array.len());
        result.push(array[start..end].to_vec());
        start = end;
    }

    result
}

pub fn flatten<T: Clone>(array: &[Vec<T>]) -> Vec<T> {
    array.iter().flat_map(|v| v.iter().cloned()).collect()
}

pub fn sum_array(array: &[usize]) -> usize {
    array.iter().sum()
}

pub fn sequence(start: usize, end: usize) -> Vec<usize> {
    (start..end).collect()
}

pub fn distribute_unevenly(sum: usize, length: usize) -> Vec<usize> {
    if length == 0 {
        return Vec::new();
    }

    let base = sum / length;
    let remainder = sum % length;

    let mut result = vec![base; length];
    for item in result.iter_mut().take(remainder) {
        *item += 1;
    }

    result
}

pub fn count_space_sequence(input: &str) -> usize {
    input.chars().take_while(|&c| c == ' ').count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_string() {
        assert!(normalize_string("hello").is_ok());
        assert!(normalize_string("hello\nworld").is_ok());
        assert!(normalize_string("test\u{0000}").is_err());
    }

    #[test]
    fn test_validate_table_data() {
        let valid_data = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
        ];
        assert!(validate_table_data(&valid_data).is_ok());

        let invalid_data = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string()],
        ];
        assert!(validate_table_data(&invalid_data).is_err());
    }

    #[test]
    fn test_calculate_maximum_column_widths() {
        let data = vec![
            vec!["short".to_string(), "longer text".to_string()],
            vec!["a".to_string(), "b".to_string()],
        ];
        let widths = calculate_maximum_column_widths(&data);
        assert_eq!(widths, vec![5, 11]);
    }

    #[test]
    fn test_group_by_sizes() {
        let data = vec![1, 2, 3, 4, 5];
        let sizes = vec![2, 2, 1];
        let grouped = group_by_sizes(&data, &sizes);
        assert_eq!(grouped, vec![vec![1, 2], vec![3, 4], vec![5]]);
    }

    #[test]
    fn test_distribute_unevenly() {
        assert_eq!(distribute_unevenly(10, 3), vec![4, 3, 3]);
        assert_eq!(distribute_unevenly(7, 3), vec![3, 2, 2]);
        assert_eq!(distribute_unevenly(0, 3), vec![0, 0, 0]);
    }

    #[test]
    fn test_count_space_sequence() {
        assert_eq!(count_space_sequence("   hello"), 3);
        assert_eq!(count_space_sequence("hello"), 0);
        assert_eq!(count_space_sequence(""), 0);
    }
}
