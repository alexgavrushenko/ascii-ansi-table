#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WrapMode {
    Word,      // Wrap at word boundaries
    Character, // Wrap at any character
    None,      // No wrapping
}

impl Default for WrapMode {
    fn default() -> Self {
        WrapMode::Word
    }
}

#[derive(Debug, Clone)]
pub struct WrapConfig {
    pub mode: WrapMode,
    pub width: usize,
    pub preserve_whitespace: bool,
}

impl WrapConfig {
    pub fn new(width: usize) -> Self {
        Self {
            mode: WrapMode::default(),
            width,
            preserve_whitespace: false,
        }
    }

    pub fn with_mode(mut self, mode: WrapMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn with_preserve_whitespace(mut self) -> Self {
        self.preserve_whitespace = true;
        self
    }
}

pub fn wrap_text(text: &str, config: &WrapConfig) -> Vec<String> {
    if config.mode == WrapMode::None || config.width == 0 {
        return vec![text.to_string()];
    }

    match config.mode {
        WrapMode::Word => wrap_word_mode(text, config),
        WrapMode::Character => wrap_character_mode(text, config),
        WrapMode::None => vec![text.to_string()],
    }
}

fn wrap_word_mode(text: &str, config: &WrapConfig) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_length = 0;

    for word in text.split_whitespace() {
        let word_len = word.len();
        
        // If adding this word would exceed width, start new line
        if current_length + word_len + (if current_line.is_empty() { 0 } else { 1 }) > config.width {
            if !current_line.is_empty() {
                lines.push(current_line.trim_end().to_string());
                current_line.clear();
                current_length = 0;
            }
        }
        
        // If single word is longer than width, handle it
        if word_len > config.width {
            if !current_line.is_empty() {
                lines.push(current_line.trim_end().to_string());
                current_line.clear();
                current_length = 0;
            }
            // Split long word by characters
            let char_wrapped = wrap_character_mode(word, &WrapConfig::new(config.width).with_mode(WrapMode::Character));
            lines.extend(char_wrapped);
        } else {
            // Add word to current line
            if !current_line.is_empty() {
                current_line.push(' ');
                current_length += 1;
            }
            current_line.push_str(word);
            current_length += word_len;
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

fn wrap_character_mode(text: &str, config: &WrapConfig) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    
    for ch in text.chars() {
        if current_line.len() >= config.width && !current_line.is_empty() {
            lines.push(current_line);
            current_line = String::new();
        }
        current_line.push(ch);
    }
    
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    if lines.is_empty() {
        lines.push(String::new());
    }
    
    lines
}

pub fn calculate_wrapped_height(text: &str, config: &WrapConfig) -> usize {
    wrap_text(text, config).len()
}

/// Wrap text that contains ANSI escape sequences while preserving them
pub fn wrap_ansi_text(text: &str, config: &WrapConfig) -> Vec<String> {
    if config.mode == WrapMode::None || config.width == 0 {
        return vec![text.to_string()];
    }

    match config.mode {
        WrapMode::Word => wrap_ansi_word_mode(text, config),
        WrapMode::Character => wrap_ansi_character_mode(text, config),
        WrapMode::None => vec![text.to_string()],
    }
}

fn wrap_ansi_word_mode(text: &str, config: &WrapConfig) -> Vec<String> {
    use crate::ansi::{ansi_display_width, strip_ansi_sequences};
    
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;
    
    // Split by whitespace but preserve ANSI sequences
    let words = split_ansi_words(text);
    
    for word in words {
        let word_display_width = ansi_display_width(&word);
        let space_width = if current_width == 0 { 0 } else { 1 };
        
        // If adding this word would exceed width, start new line
        if current_width + space_width + word_display_width > config.width && current_width > 0 {
            lines.push(current_line);
            current_line = String::new();
            current_width = 0;
        }
        
        // If single word is longer than width, handle it
        if word_display_width > config.width {
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = String::new();
                current_width = 0;
            }
            // Split long word by characters (ANSI-aware)
            let char_wrapped = wrap_ansi_character_mode(&word, &WrapConfig::new(config.width).with_mode(WrapMode::Character));
            lines.extend(char_wrapped);
        } else {
            // Add word to current line
            if current_width > 0 {
                current_line.push(' ');
                current_width += 1;
            }
            current_line.push_str(&word);
            current_width += word_display_width;
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

fn wrap_ansi_character_mode(text: &str, config: &WrapConfig) -> Vec<String> {
    use crate::ansi::ansi_display_width;
    
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;
    
    let mut chars = text.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Handle ANSI escape sequence
            if let Some(&'[') = chars.peek() {
                current_line.push(ch);
                current_line.push(chars.next().unwrap()); // consume '['
                
                // Continue until we find the end letter
                while let Some(ch) = chars.next() {
                    current_line.push(ch);
                    if ch.is_alphabetic() {
                        break;
                    }
                }
                // ANSI sequences don't contribute to width
            } else {
                // Regular escape character - count as 1 width
                if current_width >= config.width && !current_line.is_empty() {
                    lines.push(current_line);
                    current_line = String::new();
                    current_width = 0;
                }
                current_line.push(ch);
                current_width += 1;
            }
        } else {
            // Regular character
            if current_width >= config.width && !current_line.is_empty() {
                lines.push(current_line);
                current_line = String::new();
                current_width = 0;
            }
            current_line.push(ch);
            current_width += 1;
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

/// Split text into words while keeping ANSI sequences attached to words
fn split_ansi_words(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current_word = String::new();
    let mut chars = text.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch.is_whitespace() {
            if !current_word.is_empty() {
                words.push(current_word);
                current_word = String::new();
            }
            // Skip consecutive whitespace
            while let Some(&next_ch) = chars.peek() {
                if next_ch.is_whitespace() {
                    chars.next();
                } else {
                    break;
                }
            }
        } else if ch == '\x1b' {
            // Handle ANSI escape sequence - keep it with current word
            if let Some(&'[') = chars.peek() {
                current_word.push(ch);
                current_word.push(chars.next().unwrap()); // consume '['
                
                // Continue until we find the end letter
                while let Some(ch) = chars.next() {
                    current_word.push(ch);
                    if ch.is_alphabetic() {
                        break;
                    }
                }
            } else {
                current_word.push(ch);
            }
        } else {
            current_word.push(ch);
        }
    }
    
    if !current_word.is_empty() {
        words.push(current_word);
    }
    
    words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_wrapping_needed() {
        let config = WrapConfig::new(20);
        let result = wrap_text("short text", &config);
        assert_eq!(result, vec!["short text"]);
    }

    #[test]
    fn test_word_wrapping() {
        let config = WrapConfig::new(10);
        let result = wrap_text("this is a very long text that needs wrapping", &config);
        assert_eq!(result, vec!["this is a", "very long", "text that", "needs", "wrapping"]);
    }

    #[test]
    fn test_character_wrapping() {
        let config = WrapConfig::new(5).with_mode(WrapMode::Character);
        let result = wrap_text("verylongword", &config);
        assert_eq!(result, vec!["veryl", "ongwo", "rd"]);
    }

    #[test]
    fn test_long_word_in_word_mode() {
        let config = WrapConfig::new(8);
        let result = wrap_text("short verylongword end", &config);
        assert_eq!(result, vec!["short", "verylong", "word", "end"]);
    }

    #[test]
    fn test_no_wrap_mode() {
        let config = WrapConfig::new(5).with_mode(WrapMode::None);
        let result = wrap_text("this is very long text", &config);
        assert_eq!(result, vec!["this is very long text"]);
    }

    #[test]
    fn test_empty_text() {
        let config = WrapConfig::new(10);
        let result = wrap_text("", &config);
        assert_eq!(result, vec![""]);
    }

    #[test]
    fn test_calculate_wrapped_height() {
        let config = WrapConfig::new(10);
        let height = calculate_wrapped_height("this is a very long text", &config);
        assert_eq!(height, 3); // "this is a", "very long", "text"
    }

    #[test]
    fn test_single_character_per_line() {
        let config = WrapConfig::new(1).with_mode(WrapMode::Character);
        let result = wrap_text("abc", &config);
        assert_eq!(result, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_wrap_ansi_word_mode() {
        let config = WrapConfig::new(10);
        let text = "\x1b[31mhello\x1b[0m \x1b[32mworld\x1b[0m \x1b[34mvery long text\x1b[0m";
        let result = wrap_ansi_text(text, &config);
        
        // Should wrap based on display width, not including ANSI codes
        assert!(result.len() >= 2);
        assert!(result[0].contains("\x1b[31m"));
        assert!(result[0].contains("hello"));
    }

    #[test]
    fn test_wrap_ansi_character_mode() {
        let config = WrapConfig::new(5).with_mode(WrapMode::Character);
        let text = "\x1b[31mhello world\x1b[0m";
        let result = wrap_ansi_text(text, &config);
        
        // Should wrap while preserving ANSI codes
        assert!(result.len() >= 2);
        assert!(result[0].contains("\x1b[31m"));
        assert!(result.join("").contains("\x1b[0m"));
    }

    #[test]
    fn test_split_ansi_words() {
        let text = "\x1b[31mred\x1b[0m word \x1b[32mgreen\x1b[0m";
        let words = split_ansi_words(text);
        
        assert_eq!(words.len(), 3);
        assert_eq!(words[0], "\x1b[31mred\x1b[0m");
        assert_eq!(words[1], "word");
        assert_eq!(words[2], "\x1b[32mgreen\x1b[0m");
    }

    #[test]
    fn test_ansi_wrapping_preserves_sequences() {
        let config = WrapConfig::new(8);
        let text = "\x1b[1;31mBold Red Text That Is Long\x1b[0m";
        let result = wrap_ansi_text(text, &config);
        
        let full_text = result.join("");
        assert!(full_text.contains("\x1b[1;31m"));
        assert!(full_text.contains("\x1b[0m"));
        assert!(full_text.contains("Bold"));
        assert!(full_text.contains("Long"));
    }
}