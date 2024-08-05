use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Calculate the display width of a string, accounting for Unicode character widths.
/// This is different from string length as some Unicode characters take up more or less
/// than one terminal column (e.g., CJK characters are often 2 columns wide).
pub fn display_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

/// Calculate the display width of a single character.
pub fn char_display_width(c: char) -> usize {
    UnicodeWidthChar::width(c).unwrap_or(0)
}

/// Truncate a string to fit within a specific display width, accounting for Unicode.
pub fn truncate_to_width(s: &str, max_width: usize) -> String {
    if display_width(s) <= max_width {
        return s.to_string();
    }
    
    let mut result = String::new();
    let mut current_width = 0;
    
    for c in s.chars() {
        let char_width = char_display_width(c);
        if current_width + char_width > max_width {
            break;
        }
        result.push(c);
        current_width += char_width;
    }
    
    result
}

/// Pad a string to a specific display width with spaces, accounting for Unicode.
pub fn pad_to_width(s: &str, target_width: usize, alignment: crate::alignment::Alignment) -> String {
    let current_width = display_width(s);
    
    if current_width >= target_width {
        return s.to_string();
    }
    
    let padding_needed = target_width - current_width;
    
    match alignment {
        crate::alignment::Alignment::Left => {
            format!("{}{}", s, " ".repeat(padding_needed))
        }
        crate::alignment::Alignment::Right => {
            format!("{}{}", " ".repeat(padding_needed), s)
        }
        crate::alignment::Alignment::Center => {
            let left_pad = padding_needed / 2;
            let right_pad = padding_needed - left_pad;
            format!("{}{}{}", " ".repeat(left_pad), s, " ".repeat(right_pad))
        }
        crate::alignment::Alignment::Justify => {
            // For justify, if it's a single "word" (no spaces), left-align it
            if !s.contains(' ') {
                format!("{}{}", s, " ".repeat(padding_needed))
            } else {
                // This is more complex - for now, fall back to left alignment
                format!("{}{}", s, " ".repeat(padding_needed))
            }
        }
    }
}

/// Calculate column widths for table data, using Unicode-aware width calculation.
pub fn calculate_unicode_column_widths(data: &[crate::Row]) -> Vec<usize> {
    if data.is_empty() {
        return vec![];
    }
    
    let column_count = data[0].len();
    let mut widths = vec![0; column_count];
    
    for row in data {
        for (i, cell) in row.iter().enumerate().take(column_count) {
            widths[i] = widths[i].max(display_width(cell));
        }
    }
    
    widths
}

/// Wrap text accounting for Unicode character widths.
pub fn unicode_wrap_text(text: &str, max_width: usize, word_wrap: bool) -> Vec<String> {
    if max_width == 0 {
        return vec![text.to_string()];
    }
    
    if !word_wrap {
        return unicode_character_wrap(text, max_width);
    }
    
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;
    
    for word in text.split_whitespace() {
        let word_width = display_width(word);
        let space_width = if current_line.is_empty() { 0 } else { 1 };
        
        if current_width + space_width + word_width > max_width && !current_line.is_empty() {
            lines.push(current_line);
            current_line = String::new();
            current_width = 0;
        }
        
        // If single word is still too long, character wrap it
        if word_width > max_width {
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = String::new();
                current_width = 0;
            }
            let char_wrapped = unicode_character_wrap(word, max_width);
            lines.extend(char_wrapped);
        } else {
            if !current_line.is_empty() {
                current_line.push(' ');
                current_width += 1;
            }
            current_line.push_str(word);
            current_width += word_width;
        }
    }
    
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    if lines.is_empty() {
        lines.push(String::new());
    }
    
    lines
}

fn unicode_character_wrap(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;
    
    for c in text.chars() {
        let char_width = char_display_width(c);
        
        if current_width + char_width > max_width && !current_line.is_empty() {
            lines.push(current_line);
            current_line = String::new();
            current_width = 0;
        }
        
        current_line.push(c);
        current_width += char_width;
    }
    
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    if lines.is_empty() {
        lines.push(String::new());
    }
    
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alignment::Alignment;

    #[test]
    fn test_ascii_display_width() {
        assert_eq!(display_width("hello"), 5);
        assert_eq!(display_width(""), 0);
        assert_eq!(display_width("a"), 1);
    }

    #[test]
    fn test_unicode_display_width() {
        // CJK characters are typically 2 columns wide
        assert_eq!(display_width("你好"), 4); // 2 chars × 2 width each
        assert_eq!(display_width("hello世界"), 9); // 5 + 4
        
        // Emoji handling may vary
        assert!(display_width("😀") >= 1);
    }

    #[test]
    fn test_truncate_to_width() {
        assert_eq!(truncate_to_width("hello world", 5), "hello");
        assert_eq!(truncate_to_width("你好世界", 4), "你好");
        assert_eq!(truncate_to_width("short", 10), "short");
    }

    #[test]
    fn test_pad_to_width() {
        assert_eq!(pad_to_width("hi", 5, Alignment::Left), "hi   ");
        assert_eq!(pad_to_width("hi", 5, Alignment::Right), "   hi");
        assert_eq!(pad_to_width("hi", 5, Alignment::Center), " hi  ");
        
        // Unicode padding
        assert_eq!(pad_to_width("你", 4, Alignment::Left), "你  ");
        assert_eq!(pad_to_width("你", 4, Alignment::Right), "  你");
    }

    #[test]
    fn test_unicode_column_widths() {
        let data = vec![
            vec!["Name".to_string(), "City".to_string()],
            vec!["John".to_string(), "东京".to_string()], // Tokyo in Chinese
        ];
        
        let widths = calculate_unicode_column_widths(&data);
        assert_eq!(widths, vec![4, 4]); // "John".len()=4, "东京".width=4
    }

    #[test]
    fn test_unicode_wrap_text() {
        let result = unicode_wrap_text("你好世界测试", 4, false);
        assert_eq!(result, vec!["你好", "世界", "测试"]);
        
        let result = unicode_wrap_text("hello 世界", 7, true);
        assert_eq!(result, vec!["hello", "世界"]);
    }

    #[test]
    fn test_mixed_unicode_ascii() {
        let text = "Hello 世界 Test";
        let result = unicode_wrap_text(text, 8, true);
        // Should wrap considering Unicode widths
        assert!(result.len() >= 2);
    }
}