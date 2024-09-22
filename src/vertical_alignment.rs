#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VerticalAlignment {
    Top,
    Middle, 
    Bottom,
}

impl Default for VerticalAlignment {
    fn default() -> Self {
        VerticalAlignment::Top
    }
}

/// Apply vertical alignment to a column of text lines within a given height
pub fn apply_vertical_alignment(
    lines: Vec<String>,
    target_height: usize,
    alignment: VerticalAlignment,
) -> Vec<String> {
    if lines.len() >= target_height {
        return lines;
    }
    
    let padding_needed = target_height - lines.len();
    let mut result = Vec::with_capacity(target_height);
    
    match alignment {
        VerticalAlignment::Top => {
            result.extend(lines);
            result.extend(vec![String::new(); padding_needed]);
        }
        VerticalAlignment::Bottom => {
            result.extend(vec![String::new(); padding_needed]);
            result.extend(lines);
        }
        VerticalAlignment::Middle => {
            let top_padding = padding_needed / 2;
            let bottom_padding = padding_needed - top_padding;
            
            result.extend(vec![String::new(); top_padding]);
            result.extend(lines);
            result.extend(vec![String::new(); bottom_padding]);
        }
    }
    
    result
}

/// Calculate the centered position for middle alignment
pub fn calculate_middle_position(content_height: usize, total_height: usize) -> usize {
    if content_height >= total_height {
        return 0;
    }
    
    (total_height - content_height) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_alignment() {
        let lines = vec!["Line 1".to_string(), "Line 2".to_string()];
        let result = apply_vertical_alignment(lines, 5, VerticalAlignment::Top);
        
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], "Line 1");
        assert_eq!(result[1], "Line 2");
        assert_eq!(result[2], "");
        assert_eq!(result[3], "");
        assert_eq!(result[4], "");
    }

    #[test]
    fn test_bottom_alignment() {
        let lines = vec!["Line 1".to_string(), "Line 2".to_string()];
        let result = apply_vertical_alignment(lines, 5, VerticalAlignment::Bottom);
        
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], "");
        assert_eq!(result[1], "");
        assert_eq!(result[2], "");
        assert_eq!(result[3], "Line 1");
        assert_eq!(result[4], "Line 2");
    }

    #[test]
    fn test_middle_alignment() {
        let lines = vec!["Line 1".to_string(), "Line 2".to_string()];
        let result = apply_vertical_alignment(lines, 5, VerticalAlignment::Middle);
        
        assert_eq!(result.len(), 5);
        assert_eq!(result[0], "");
        assert_eq!(result[1], "Line 1");
        assert_eq!(result[2], "Line 2");
        assert_eq!(result[3], "");
        assert_eq!(result[4], "");
    }

    #[test]
    fn test_middle_alignment_odd_padding() {
        let lines = vec!["Line 1".to_string()];
        let result = apply_vertical_alignment(lines, 4, VerticalAlignment::Middle);
        
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], "");
        assert_eq!(result[1], "Line 1");
        assert_eq!(result[2], "");
        assert_eq!(result[3], "");
    }

    #[test]
    fn test_no_padding_needed() {
        let lines = vec!["Line 1".to_string(), "Line 2".to_string(), "Line 3".to_string()];
        let result = apply_vertical_alignment(lines.clone(), 3, VerticalAlignment::Middle);
        
        assert_eq!(result, lines);
    }

    #[test]
    fn test_content_taller_than_target() {
        let lines = vec!["Line 1".to_string(), "Line 2".to_string(), "Line 3".to_string()];
        let result = apply_vertical_alignment(lines.clone(), 2, VerticalAlignment::Top);
        
        assert_eq!(result, lines); // Should return original lines unchanged
    }

    #[test]
    fn test_calculate_middle_position() {
        assert_eq!(calculate_middle_position(1, 5), 2);
        assert_eq!(calculate_middle_position(2, 5), 1);
        assert_eq!(calculate_middle_position(3, 5), 1);
        assert_eq!(calculate_middle_position(5, 5), 0);
        assert_eq!(calculate_middle_position(6, 5), 0);
    }

    #[test]
    fn test_empty_lines() {
        let lines = vec![];
        let result = apply_vertical_alignment(lines, 3, VerticalAlignment::Middle);
        
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "");
        assert_eq!(result[1], "");
        assert_eq!(result[2], "");
    }
}