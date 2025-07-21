use regex::Regex;
use std::sync::OnceLock;

static ANSI_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_ansi_regex() -> &'static Regex {
    ANSI_REGEX.get_or_init(|| {
        Regex::new(r"\x1b\[[0-9;]*m").unwrap()
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnsiString {
    pub content: String,
    pub display_width: usize,
}

impl AnsiString {
    pub fn new(content: String) -> Self {
        let display_width = calculate_display_width(&content);
        Self { content, display_width }
    }

    pub fn slice(&self, start: usize, end: usize) -> AnsiString {
        slice_ansi_string(&self.content, start, end)
    }

    pub fn pad_to_width(&self, width: usize, alignment: crate::types::Alignment) -> AnsiString {
        pad_ansi_string(&self.content, width, alignment)
    }

    pub fn truncate(&self, max_width: usize) -> AnsiString {
        truncate_ansi_string(&self.content, max_width)
    }
}

impl From<String> for AnsiString {
    fn from(content: String) -> Self {
        Self::new(content)
    }
}

impl From<&str> for AnsiString {
    fn from(content: &str) -> Self {
        Self::new(content.to_string())
    }
}

pub fn calculate_display_width(text: &str) -> usize {
    let clean_text = strip_ansi_sequences(text);
    unicode_width::UnicodeWidthStr::width(clean_text.as_str())
}

pub fn strip_ansi_sequences(text: &str) -> String {
    get_ansi_regex().replace_all(text, "").to_string()
}

pub fn split_ansi_string(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    let mut result = Vec::new();
    let mut current = String::new();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '\u{1b}' {
            if !current.is_empty() {
                result.push(current.clone());
                current.clear();
            }
            
            let ansi_start = i;
            i = skip_ansi_sequence(&chars, i);
            let ansi_seq: String = chars[ansi_start..i].iter().collect();
            result.push(ansi_seq);
        } else {
            current.push(chars[i]);
            i += 1;
        }
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}

pub fn skip_ansi_sequence(chars: &[char], mut pos: usize) -> usize {
    if pos >= chars.len() || chars[pos] != '\u{1b}' {
        return pos;
    }
    
    pos += 1;
    if pos >= chars.len() {
        return pos;
    }
    
    if chars[pos] == '[' {
        pos += 1;
        while pos < chars.len() && chars[pos] >= '\u{30}' && chars[pos] <= '\u{3F}' {
            pos += 1;
        }
        while pos < chars.len() && chars[pos] >= '\u{20}' && chars[pos] <= '\u{2F}' {
            pos += 1;
        }
        if pos < chars.len() && chars[pos] >= '\u{40}' && chars[pos] <= '\u{7E}' {
            pos += 1;
        }
    } else {
        pos += 1;
    }
    
    pos
}

pub fn slice_ansi_string(text: &str, start: usize, end: usize) -> AnsiString {
    if start >= end {
        return AnsiString::new(String::new());
    }

    let chars: Vec<char> = text.chars().collect();
    let mut result = String::new();
    let mut display_pos = 0;
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '\u{1b}' {
            if display_pos >= start && display_pos < end {
                let ansi_start = i;
                i = skip_ansi_sequence(&chars, i);
                let ansi_seq: String = chars[ansi_start..i].iter().collect();
                result.push_str(&ansi_seq);
            } else {
                i = skip_ansi_sequence(&chars, i);
            }
        } else {
            if display_pos >= start && display_pos < end {
                result.push(chars[i]);
            }
            display_pos += unicode_width::UnicodeWidthChar::width(chars[i]).unwrap_or(0);
            i += 1;
        }
    }

    AnsiString::new(result)
}

pub fn pad_ansi_string(text: &str, width: usize, alignment: crate::types::Alignment) -> AnsiString {
    let display_width = calculate_display_width(text);
    
    if display_width >= width {
        return AnsiString::new(text.to_string());
    }

    let padding = width - display_width;
    let result = match alignment {
        crate::types::Alignment::Left => format!("{}{}", text, " ".repeat(padding)),
        crate::types::Alignment::Right => format!("{}{}", " ".repeat(padding), text),
        crate::types::Alignment::Center => {
            let left_padding = padding / 2;
            let right_padding = padding - left_padding;
            format!("{}{}{}", " ".repeat(left_padding), text, " ".repeat(right_padding))
        }
        crate::types::Alignment::Justify => {
            justify_text(text, width)
        }
    };

    AnsiString::new(result)
}

pub fn truncate_ansi_string(text: &str, max_width: usize) -> AnsiString {
    let display_width = calculate_display_width(text);
    
    if display_width <= max_width {
        return AnsiString::new(text.to_string());
    }

    if max_width <= 3 {
        return slice_ansi_string(text, 0, max_width);
    }

    let truncated = slice_ansi_string(text, 0, max_width - 3);
    AnsiString::new(format!("{}...", truncated.content))
}

fn justify_text(text: &str, width: usize) -> String {
    let clean_text = strip_ansi_sequences(text);
    let words: Vec<&str> = clean_text.split_whitespace().collect();
    
    if words.len() <= 1 {
        return text.to_string();
    }

    let total_chars: usize = words.iter().map(|w| w.len()).sum();
    let total_spaces = width - total_chars;
    let gaps = words.len() - 1;
    
    if gaps == 0 {
        return text.to_string();
    }

    let spaces_per_gap = total_spaces / gaps;
    let extra_spaces = total_spaces % gaps;

    let mut result = String::new();
    let ansi_segments = split_ansi_string(text);
    let mut segment_idx = 0;

    for (i, _word) in words.iter().enumerate() {
        while segment_idx < ansi_segments.len() {
            let segment = &ansi_segments[segment_idx];
            if segment.starts_with('\u{1b}') {
                result.push_str(segment);
                segment_idx += 1;
            } else {
                result.push_str(segment);
                segment_idx += 1;
                break;
            }
        }

        if i < words.len() - 1 {
            let spaces = spaces_per_gap + if i < extra_spaces { 1 } else { 0 };
            result.push_str(&" ".repeat(spaces));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_display_width() {
        assert_eq!(calculate_display_width("hello"), 5);
        assert_eq!(calculate_display_width("\u{1b}[31mhello\u{1b}[39m"), 5);
        assert_eq!(calculate_display_width("测试"), 4);
    }

    #[test]
    fn test_strip_ansi_sequences() {
        assert_eq!(strip_ansi_sequences("hello"), "hello");
        assert_eq!(strip_ansi_sequences("\u{1b}[31mhello\u{1b}[39m"), "hello");
        assert_eq!(strip_ansi_sequences("\u{1b}[31m\u{1b}[1mhello\u{1b}[22m\u{1b}[39m"), "hello");
    }

    #[test]
    fn test_ansi_string_slice() {
        let text = "\u{1b}[31mhello world\u{1b}[39m";
        let ansi_str = AnsiString::new(text.to_string());
        let sliced = ansi_str.slice(0, 5);
        assert_eq!(calculate_display_width(&sliced.content), 5);
    }

    #[test]
    fn test_pad_ansi_string() {
        let text = "\u{1b}[31mhello\u{1b}[39m";
        let padded = pad_ansi_string(text, 10, crate::types::Alignment::Center);
        assert_eq!(calculate_display_width(&padded.content), 10);
    }
}