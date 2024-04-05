#[derive(Debug, Clone)]
pub struct BorderChars {
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
    pub horizontal: char,
    pub vertical: char,
    pub top_junction: char,
    pub bottom_junction: char,
}

impl Default for BorderChars {
    fn default() -> Self {
        Self {
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            horizontal: '─',
            vertical: '│',
            top_junction: '┬',
            bottom_junction: '┴',
        }
    }
}

impl BorderChars {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ascii() -> Self {
        Self {
            top_left: '+',
            top_right: '+',
            bottom_left: '+',
            bottom_right: '+',
            horizontal: '-',
            vertical: '|',
            top_junction: '+',
            bottom_junction: '+',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_border_chars() {
        let border = BorderChars::default();
        assert_eq!(border.top_left, '┌');
        assert_eq!(border.vertical, '│');
    }

    #[test]
    fn test_ascii_border_chars() {
        let border = BorderChars::ascii();
        assert_eq!(border.top_left, '+');
        assert_eq!(border.vertical, '|');
    }
}