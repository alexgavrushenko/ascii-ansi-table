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

    // Unicode box-drawing (honeywell style)
    pub fn honeywell() -> Self {
        Self::default()
    }

    // ASCII characters (ramac style)
    pub fn ramac() -> Self {
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

    // Double-line Unicode (norc style)
    pub fn norc() -> Self {
        Self {
            top_left: '╔',
            top_right: '╗',
            bottom_left: '╚',
            bottom_right: '╝',
            horizontal: '═',
            vertical: '║',
            top_junction: '╦',
            bottom_junction: '╩',
        }
    }

    // No borders (void style)
    pub fn void() -> Self {
        Self {
            top_left: ' ',
            top_right: ' ',
            bottom_left: ' ',
            bottom_right: ' ',
            horizontal: ' ',
            vertical: ' ',
            top_junction: ' ',
            bottom_junction: ' ',
        }
    }

    // Legacy alias
    pub fn ascii() -> Self {
        Self::ramac()
    }
}

pub fn get_border_style(name: &str) -> Result<BorderChars, String> {
    match name.to_lowercase().as_str() {
        "honeywell" => Ok(BorderChars::honeywell()),
        "ramac" => Ok(BorderChars::ramac()),
        "norc" => Ok(BorderChars::norc()),
        "void" => Ok(BorderChars::void()),
        "ascii" => Ok(BorderChars::ascii()),
        _ => Err(format!("Unknown border style: {}", name)),
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

    #[test]
    fn test_honeywell_border() {
        let border = BorderChars::honeywell();
        assert_eq!(border.top_left, '┌');
        assert_eq!(border.vertical, '│');
    }

    #[test]
    fn test_norc_border() {
        let border = BorderChars::norc();
        assert_eq!(border.top_left, '╔');
        assert_eq!(border.vertical, '║');
    }

    #[test]
    fn test_void_border() {
        let border = BorderChars::void();
        assert_eq!(border.top_left, ' ');
        assert_eq!(border.vertical, ' ');
        assert_eq!(border.horizontal, ' ');
    }

    #[test]
    fn test_get_border_style() {
        assert!(get_border_style("honeywell").is_ok());
        assert!(get_border_style("ramac").is_ok());
        assert!(get_border_style("norc").is_ok());
        assert!(get_border_style("void").is_ok());
        assert!(get_border_style("invalid").is_err());
    }
}