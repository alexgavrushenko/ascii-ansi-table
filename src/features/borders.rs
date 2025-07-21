use crate::types::BorderConfig;

pub struct BorderRenderer {
    config: BorderConfig,
    column_widths: Vec<usize>,
}

impl BorderRenderer {
    pub fn new(config: BorderConfig, column_widths: Vec<usize>) -> Self {
        Self {
            config,
            column_widths,
        }
    }

    pub fn draw_top_border(&self) -> String {
        self.draw_horizontal_border(
            &self.config.top_left,
            &self.config.top_right,
            &self.config.top_body,
            &self.config.top_join,
        )
    }

    pub fn draw_bottom_border(&self) -> String {
        self.draw_horizontal_border(
            &self.config.bottom_left,
            &self.config.bottom_right,
            &self.config.bottom_body,
            &self.config.bottom_join,
        )
    }

    pub fn draw_separator(&self) -> String {
        self.draw_horizontal_border(
            &self.config.join_left,
            &self.config.join_right,
            &self.config.join_body,
            &self.config.join_join,
        )
    }

    fn draw_horizontal_border(&self, left: &str, right: &str, body: &str, join: &str) -> String {
        let mut result = String::new();

        result.push_str(left);

        for (i, &width) in self.column_widths.iter().enumerate() {
            result.push_str(&body.repeat(width));

            if i < self.column_widths.len() - 1 {
                result.push_str(join);
            }
        }

        result.push_str(right);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::get_border_characters;

    #[test]
    fn test_border_renderer() {
        let border_config = get_border_characters("honeywell").unwrap();
        let column_widths = vec![5, 10, 3];
        let renderer = BorderRenderer::new(border_config, column_widths);

        let top_border = renderer.draw_top_border();
        assert!(top_border.contains("┌"));
        assert!(top_border.contains("┐"));
        assert!(top_border.contains("┬"));
        assert!(top_border.contains("─"));

        let bottom_border = renderer.draw_bottom_border();
        assert!(bottom_border.contains("└"));
        assert!(bottom_border.contains("┘"));
        assert!(bottom_border.contains("┴"));

        let separator = renderer.draw_separator();
        assert!(separator.contains("├"));
        assert!(separator.contains("┤"));
        assert!(separator.contains("┼"));
    }
}
