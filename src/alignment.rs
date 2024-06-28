#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Left
    }
}

use crate::padding::Padding;

#[derive(Debug, Clone)]
pub struct ColumnConfig {
    pub width: Option<usize>,
    pub alignment: Alignment,
    pub padding: Padding,
}

impl Default for ColumnConfig {
    fn default() -> Self {
        Self {
            width: None,
            alignment: Alignment::default(),
            padding: Padding::default(),
        }
    }
}

impl ColumnConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_width(mut self, width: usize) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }
}

pub fn align_text(text: &str, width: usize, alignment: Alignment) -> String {
    let text_len = text.len();
    
    if text_len >= width {
        return text.to_string();
    }
    
    match alignment {
        Alignment::Left => format!("{:<width$}", text, width = width),
        Alignment::Right => format!("{:>width$}", text, width = width),
        Alignment::Center => {
            let padding = width - text_len;
            let left_pad = padding / 2;
            let right_pad = padding - left_pad;
            format!("{}{}{}", " ".repeat(left_pad), text, " ".repeat(right_pad))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alignment_enum() {
        assert_eq!(Alignment::default(), Alignment::Left);
    }

    #[test]
    fn test_align_text_left() {
        let result = align_text("test", 10, Alignment::Left);
        assert_eq!(result, "test      ");
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_align_text_right() {
        let result = align_text("test", 10, Alignment::Right);
        assert_eq!(result, "      test");
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_align_text_center() {
        let result = align_text("test", 10, Alignment::Center);
        assert_eq!(result, "   test   ");
        assert_eq!(result.len(), 10);
        
        // Test odd padding
        let result = align_text("test", 9, Alignment::Center);
        assert_eq!(result, "  test   ");
        assert_eq!(result.len(), 9);
    }

    #[test]
    fn test_align_text_no_padding_needed() {
        let result = align_text("exact", 5, Alignment::Center);
        assert_eq!(result, "exact");
        
        let result = align_text("toolong", 5, Alignment::Left);
        assert_eq!(result, "toolong");
    }

    #[test]
    fn test_column_config_builder() {
        let config = ColumnConfig::new()
            .with_width(15)
            .with_alignment(Alignment::Center);
        
        assert_eq!(config.width, Some(15));
        assert_eq!(config.alignment, Alignment::Center);
    }
}