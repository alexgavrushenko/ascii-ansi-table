use std::fmt;

/// ANSI escape sequence types
#[derive(Debug, Clone, PartialEq)]
pub enum AnsiSequence {
    Reset,                    // \x1b[0m
    Color(u8),               // \x1b[31m (foreground colors)
    BackgroundColor(u8),     // \x1b[41m (background colors)
    Bold,                    // \x1b[1m
    Italic,                  // \x1b[3m
    Underline,              // \x1b[4m
    Other(String),          // Other sequences
}

impl fmt::Display for AnsiSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnsiSequence::Reset => write!(f, "\x1b[0m"),
            AnsiSequence::Color(code) => write!(f, "\x1b[{}m", code),
            AnsiSequence::BackgroundColor(code) => write!(f, "\x1b[{}m", code),
            AnsiSequence::Bold => write!(f, "\x1b[1m"),
            AnsiSequence::Italic => write!(f, "\x1b[3m"),
            AnsiSequence::Underline => write!(f, "\x1b[4m"),
            AnsiSequence::Other(seq) => write!(f, "{}", seq),
        }
    }
}

/// Parse ANSI escape sequences from text
pub fn parse_ansi_sequences(text: &str) -> Vec<(usize, AnsiSequence)> {
    let mut sequences = Vec::new();
    let mut chars = text.char_indices().peekable();
    
    while let Some((pos, ch)) = chars.next() {
        if ch == '\x1b' {
            if let Some((_, '[')) = chars.peek() {
                chars.next(); // consume '['
                let start_pos = pos;
                
                // Collect the sequence
                let mut sequence = String::from("\x1b[");
                let mut found_end = false;
                
                while let Some((_, ch)) = chars.next() {
                    sequence.push(ch);
                    if ch.is_alphabetic() {
                        found_end = true;
                        break;
                    }
                }
                
                if found_end {
                    let ansi_seq = parse_sequence(&sequence);
                    sequences.push((start_pos, ansi_seq));
                }
            }
        }
    }
    
    sequences
}

fn parse_sequence(sequence: &str) -> AnsiSequence {
    if sequence == "\x1b[0m" {
        return AnsiSequence::Reset;
    }
    
    if sequence == "\x1b[1m" {
        return AnsiSequence::Bold;
    }
    
    if sequence == "\x1b[3m" {
        return AnsiSequence::Italic;
    }
    
    if sequence == "\x1b[4m" {
        return AnsiSequence::Underline;
    }
    
    // Try to parse color codes
    if let Some(code_str) = sequence.strip_prefix("\x1b[").and_then(|s| s.strip_suffix("m")) {
        if let Ok(code) = code_str.parse::<u8>() {
            if (30..=37).contains(&code) || (90..=97).contains(&code) {
                return AnsiSequence::Color(code);
            } else if (40..=47).contains(&code) || (100..=107).contains(&code) {
                return AnsiSequence::BackgroundColor(code);
            }
        }
    }
    
    AnsiSequence::Other(sequence.to_string())
}

/// Calculate the display width of text, ignoring ANSI escape sequences
pub fn ansi_display_width(text: &str) -> usize {
    strip_ansi_sequences(text).len()
}

/// Remove all ANSI escape sequences from text
pub fn strip_ansi_sequences(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            if let Some(&'[') = chars.peek() {
                chars.next(); // consume '['
                // Skip until we find the end of the sequence (a letter)
                while let Some(ch) = chars.next() {
                    if ch.is_alphabetic() {
                        break;
                    }
                }
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }
    
    result
}

/// Truncate ANSI text to a specific display width while preserving sequences
pub fn ansi_truncate_to_width(text: &str, max_width: usize) -> String {
    let mut result = String::new();
    let mut current_width = 0;
    let mut chars = text.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            if let Some(&'[') = chars.peek() {
                // Add the entire ANSI sequence without counting width
                result.push(ch);
                result.push(chars.next().unwrap()); // '['
                
                while let Some(ch) = chars.next() {
                    result.push(ch);
                    if ch.is_alphabetic() {
                        break;
                    }
                }
            } else {
                // Regular escape character
                if current_width >= max_width {
                    break;
                }
                result.push(ch);
                current_width += 1;
            }
        } else {
            if current_width >= max_width {
                break;
            }
            result.push(ch);
            current_width += 1;
        }
    }
    
    result
}

/// Pad ANSI text to a specific display width while preserving sequences
pub fn ansi_pad_to_width(text: &str, target_width: usize, alignment: crate::alignment::Alignment) -> String {
    let display_width = ansi_display_width(text);
    
    if display_width >= target_width {
        return text.to_string();
    }
    
    let padding_needed = target_width - display_width;
    
    match alignment {
        crate::alignment::Alignment::Left => {
            format!("{}{}", text, " ".repeat(padding_needed))
        }
        crate::alignment::Alignment::Right => {
            format!("{}{}", " ".repeat(padding_needed), text)
        }
        crate::alignment::Alignment::Center => {
            let left_pad = padding_needed / 2;
            let right_pad = padding_needed - left_pad;
            format!("{}{}{}", " ".repeat(left_pad), text, " ".repeat(right_pad))
        }
        crate::alignment::Alignment::Justify => {
            // For ANSI text justify, fall back to left alignment for now
            format!("{}{}", text, " ".repeat(padding_needed))
        }
    }
}

/// Common ANSI color constants for convenience
pub mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const ITALIC: &str = "\x1b[3m";
    pub const UNDERLINE: &str = "\x1b[4m";
    
    // Foreground colors
    pub const BLACK: &str = "\x1b[30m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";
    
    // Bright foreground colors
    pub const BRIGHT_BLACK: &str = "\x1b[90m";
    pub const BRIGHT_RED: &str = "\x1b[91m";
    pub const BRIGHT_GREEN: &str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &str = "\x1b[93m";
    pub const BRIGHT_BLUE: &str = "\x1b[94m";
    pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
    pub const BRIGHT_CYAN: &str = "\x1b[96m";
    pub const BRIGHT_WHITE: &str = "\x1b[97m";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alignment::Alignment;

    #[test]
    fn test_strip_ansi_sequences() {
        assert_eq!(strip_ansi_sequences("hello world"), "hello world");
        assert_eq!(strip_ansi_sequences("\x1b[31mred\x1b[0m"), "red");
        assert_eq!(strip_ansi_sequences("\x1b[1;32mbold green\x1b[0m"), "bold green");
        assert_eq!(strip_ansi_sequences("normal \x1b[31mred\x1b[0m normal"), "normal red normal");
    }

    #[test]
    fn test_ansi_display_width() {
        assert_eq!(ansi_display_width("hello"), 5);
        assert_eq!(ansi_display_width("\x1b[31mred\x1b[0m"), 3);
        assert_eq!(ansi_display_width("\x1b[1;32mbold green\x1b[0m"), 10);
    }

    #[test]
    fn test_ansi_truncate_to_width() {
        let text = "\x1b[31mhello world\x1b[0m";
        let result = ansi_truncate_to_width(text, 5);
        assert_eq!(result, "\x1b[31mhello\x1b[0m");
        
        let short_text = "\x1b[32mhi\x1b[0m";
        let result = ansi_truncate_to_width(short_text, 10);
        assert_eq!(result, short_text);
    }

    #[test]
    fn test_ansi_pad_to_width() {
        let text = "\x1b[31mhi\x1b[0m";
        
        let result = ansi_pad_to_width(text, 5, Alignment::Left);
        assert_eq!(result, "\x1b[31mhi\x1b[0m   ");
        assert_eq!(ansi_display_width(&result), 5);
        
        let result = ansi_pad_to_width(text, 5, Alignment::Right);
        assert_eq!(result, "   \x1b[31mhi\x1b[0m");
        assert_eq!(ansi_display_width(&result), 5);
        
        let result = ansi_pad_to_width(text, 6, Alignment::Center);
        assert_eq!(result, "  \x1b[31mhi\x1b[0m  ");
        assert_eq!(ansi_display_width(&result), 6);
    }

    #[test]
    fn test_parse_ansi_sequences() {
        let sequences = parse_ansi_sequences("\x1b[31mred\x1b[0m");
        assert_eq!(sequences.len(), 2);
        assert_eq!(sequences[0].1, AnsiSequence::Color(31));
        assert_eq!(sequences[1].1, AnsiSequence::Reset);
    }

    #[test]
    fn test_ansi_sequence_display() {
        assert_eq!(AnsiSequence::Reset.to_string(), "\x1b[0m");
        assert_eq!(AnsiSequence::Color(31).to_string(), "\x1b[31m");
        assert_eq!(AnsiSequence::Bold.to_string(), "\x1b[1m");
    }

    #[test]
    fn test_colors_constants() {
        assert_eq!(colors::RED, "\x1b[31m");
        assert_eq!(colors::RESET, "\x1b[0m");
        assert_eq!(colors::BOLD, "\x1b[1m");
    }
}