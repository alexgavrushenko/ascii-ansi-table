#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Padding {
    pub left: usize,
    pub right: usize,
}

impl Default for Padding {
    fn default() -> Self {
        Self { left: 1, right: 1 }
    }
}

impl Padding {
    pub fn new(left: usize, right: usize) -> Self {
        Self { left, right }
    }

    pub fn symmetric(padding: usize) -> Self {
        Self { left: padding, right: padding }
    }

    pub fn none() -> Self {
        Self { left: 0, right: 0 }
    }

    pub fn total(&self) -> usize {
        self.left + self.right
    }
}

pub fn apply_padding(text: &str, padding: Padding) -> String {
    format!("{}{}{}", 
        " ".repeat(padding.left), 
        text, 
        " ".repeat(padding.right)
    )
}

pub fn apply_padding_with_width(text: &str, total_width: usize, padding: Padding) -> String {
    let content_width = total_width.saturating_sub(padding.total());
    let truncated = if text.len() > content_width {
        &text[..content_width]
    } else {
        text
    };
    
    let padded_content = format!("{:<width$}", truncated, width = content_width);
    apply_padding(&padded_content, padding)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding_default() {
        let padding = Padding::default();
        assert_eq!(padding.left, 1);
        assert_eq!(padding.right, 1);
        assert_eq!(padding.total(), 2);
    }

    #[test]
    fn test_padding_creation() {
        let padding = Padding::new(2, 3);
        assert_eq!(padding.left, 2);
        assert_eq!(padding.right, 3);
        assert_eq!(padding.total(), 5);

        let sym_padding = Padding::symmetric(2);
        assert_eq!(sym_padding.left, 2);
        assert_eq!(sym_padding.right, 2);

        let no_padding = Padding::none();
        assert_eq!(no_padding.total(), 0);
    }

    #[test]
    fn test_apply_padding() {
        let padding = Padding::new(2, 1);
        let result = apply_padding("test", padding);
        assert_eq!(result, "  test ");
        assert_eq!(result.len(), 7); // 2 + 4 + 1
    }

    #[test]
    fn test_apply_padding_with_width() {
        let padding = Padding::new(1, 1);
        let result = apply_padding_with_width("test", 8, padding);
        assert_eq!(result, " test   ");
        assert_eq!(result.len(), 8);

        // Test truncation
        let result = apply_padding_with_width("verylongtext", 8, padding);
        assert_eq!(result, " verylo ");
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_no_padding() {
        let padding = Padding::none();
        let result = apply_padding("test", padding);
        assert_eq!(result, "test");
    }
}