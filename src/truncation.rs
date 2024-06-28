#[derive(Debug, Clone)]
pub struct TruncationConfig {
    pub max_width: Option<usize>,
    pub ellipsis: String,
    pub truncate_start: bool, // If true, truncate from start; if false, from end
}

impl Default for TruncationConfig {
    fn default() -> Self {
        Self {
            max_width: None,
            ellipsis: "...".to_string(),
            truncate_start: false,
        }
    }
}

impl TruncationConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_max_width(mut self, max_width: usize) -> Self {
        self.max_width = Some(max_width);
        self
    }

    pub fn with_ellipsis(mut self, ellipsis: &str) -> Self {
        self.ellipsis = ellipsis.to_string();
        self
    }

    pub fn with_start_truncation(mut self) -> Self {
        self.truncate_start = true;
        self
    }

    pub fn none() -> Self {
        Self {
            max_width: None,
            ellipsis: String::new(),
            truncate_start: false,
        }
    }
}

pub fn truncate_text(text: &str, config: &TruncationConfig) -> String {
    let max_width = match config.max_width {
        Some(width) => width,
        None => return text.to_string(),
    };

    if text.len() <= max_width {
        return text.to_string();
    }

    let ellipsis_len = config.ellipsis.len();
    
    if max_width <= ellipsis_len {
        // If max_width is too small for ellipsis, just truncate without ellipsis
        return text.chars().take(max_width).collect();
    }

    let content_len = max_width - ellipsis_len;
    
    if config.truncate_start {
        // Truncate from start: "...ending"
        let chars: Vec<char> = text.chars().collect();
        let start_pos = chars.len() - content_len;
        format!("{}{}", config.ellipsis, chars[start_pos..].iter().collect::<String>())
    } else {
        // Truncate from end: "beginning..."
        let truncated: String = text.chars().take(content_len).collect();
        format!("{}{}", truncated, config.ellipsis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_truncation_needed() {
        let config = TruncationConfig::new().with_max_width(10);
        let result = truncate_text("short", &config);
        assert_eq!(result, "short");
    }

    #[test]
    fn test_truncation_from_end() {
        let config = TruncationConfig::new().with_max_width(10);
        let result = truncate_text("this is a very long text", &config);
        assert_eq!(result, "this is...");
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_truncation_from_start() {
        let config = TruncationConfig::new()
            .with_max_width(10)
            .with_start_truncation();
        let result = truncate_text("this is a very long text", &config);
        assert_eq!(result, "...ng text");
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_custom_ellipsis() {
        let config = TruncationConfig::new()
            .with_max_width(10)
            .with_ellipsis(" [+]");
        let result = truncate_text("this is a very long text", &config);
        assert_eq!(result, "this  [+]");
        assert_eq!(result.len(), 9); // "this " + " [+]" = 9 chars
    }

    #[test]
    fn test_max_width_too_small() {
        let config = TruncationConfig::new()
            .with_max_width(2)
            .with_ellipsis("...");
        let result = truncate_text("hello", &config);
        assert_eq!(result, "he");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_no_max_width() {
        let config = TruncationConfig::none();
        let result = truncate_text("this text should not be truncated", &config);
        assert_eq!(result, "this text should not be truncated");
    }

    #[test]
    fn test_unicode_truncation() {
        let config = TruncationConfig::new().with_max_width(8);
        let result = truncate_text("héllo wörld", &config);
        assert_eq!(result, "héllo...");
    }
}