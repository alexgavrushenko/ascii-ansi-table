#[derive(Debug, Clone)]
pub struct RenderOptions {
    pub show_top_border: bool,
    pub show_bottom_border: bool,
    pub show_row_separators: bool,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            show_top_border: true,
            show_bottom_border: true,
            show_row_separators: false,
        }
    }
}

impl RenderOptions {
    pub fn no_horizontal_lines() -> Self {
        Self {
            show_top_border: false,
            show_bottom_border: false,
            show_row_separators: false,
        }
    }

    pub fn with_row_separators() -> Self {
        Self {
            show_top_border: true,
            show_bottom_border: true,
            show_row_separators: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_render_options() {
        let options = RenderOptions::default();
        assert!(options.show_top_border);
        assert!(options.show_bottom_border);
        assert!(!options.show_row_separators);
    }

    #[test]
    fn test_no_horizontal_lines() {
        let options = RenderOptions::no_horizontal_lines();
        assert!(!options.show_top_border);
        assert!(!options.show_bottom_border);
        assert!(!options.show_row_separators);
    }
}