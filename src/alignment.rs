#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alignment {
    Left,
    Center,
    Right,
    Justify,
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Left
    }
}

use crate::padding::Padding;
use crate::truncation::TruncationConfig;
use crate::wrapping::WrapConfig;
use crate::vertical_alignment::VerticalAlignment;

#[derive(Debug, Clone)]
pub struct ColumnConfig {
    pub width: Option<usize>,
    pub alignment: Alignment,
    pub vertical_alignment: VerticalAlignment,
    pub padding: Padding,
    pub truncation: TruncationConfig,
    pub wrap_config: Option<WrapConfig>,
}

impl Default for ColumnConfig {
    fn default() -> Self {
        Self {
            width: None,
            alignment: Alignment::default(),
            vertical_alignment: VerticalAlignment::default(),
            padding: Padding::default(),
            truncation: TruncationConfig::default(),
            wrap_config: None,
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

    pub fn with_truncation(mut self, truncation: TruncationConfig) -> Self {
        self.truncation = truncation;
        self
    }

    pub fn with_wrapping(mut self, wrap_config: WrapConfig) -> Self {
        self.wrap_config = Some(wrap_config);
        self
    }

    pub fn with_vertical_alignment(mut self, vertical_alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = vertical_alignment;
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
        Alignment::Justify => justify_text(text, width),
    }
}

pub fn justify_text(text: &str, width: usize) -> String {
    let text_len = text.len();
    
    if text_len >= width {
        return text.to_string();
    }
    
    let words: Vec<&str> = text.split_whitespace().collect();
    
    // If only one word, left-align it
    if words.len() <= 1 {
        return format!("{:<width$}", text, width = width);
    }
    
    // Calculate spaces needed
    let chars_without_spaces: usize = words.iter().map(|word| word.len()).sum();
    let total_spaces_needed = width - chars_without_spaces;
    let gaps = words.len() - 1;
    
    if gaps == 0 {
        return format!("{:<width$}", text, width = width);
    }
    
    let base_spaces = total_spaces_needed / gaps;
    let extra_spaces = total_spaces_needed % gaps;
    
    let mut result = String::new();
    
    for (i, word) in words.iter().enumerate() {
        result.push_str(word);
        
        if i < words.len() - 1 {
            // Add base spaces plus one extra for the first 'extra_spaces' gaps
            let spaces_to_add = if i < extra_spaces {
                base_spaces + 1
            } else {
                base_spaces
            };
            result.push_str(&" ".repeat(spaces_to_add));
        }
    }
    
    result
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

    #[test]
    fn test_justify_alignment() {
        let result = align_text("hello world", 15, Alignment::Justify);
        assert_eq!(result, "hello     world");
        assert_eq!(result.len(), 15);
    }

    #[test]
    fn test_justify_text_multiple_words() {
        let result = justify_text("one two three", 20);
        assert_eq!(result, "one      two   three");
        assert_eq!(result.len(), 20);
    }

    #[test]
    fn test_justify_text_single_word() {
        let result = justify_text("hello", 10);
        assert_eq!(result, "hello     "); // Should left-align single words
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_justify_text_exact_fit() {
        let result = justify_text("exact fit", 9);
        assert_eq!(result, "exact fit");
    }
}