use unicode_width::UnicodeWidthChar;

pub fn convert_ansi_to_html(text: &str) -> String {
    let html = ansi_to_html::convert(text).unwrap_or(text.to_string());
    format!(
        "<pre style=\"margin: 0; font-family: monospace; white-space: pre;\">{}</pre>",
        wrap_emojis_with_fixed_width(&html)
    )
}

fn wrap_emojis_with_fixed_width(text: &str) -> String {
    let mut result = String::new();

    for ch in text.chars() {
        // Use unicode-width crate to determine actual character width
        let width = ch.width().unwrap_or(1);

        if width == 2 {
            // Wrap 2-width characters in fixed-width spans
            result.push_str(&format!(
                r#"<span style="display: inline-block; width: 2ch; text-align: center;">{ch}</span>"#
            ));
        } else {
            // Leave 1-width characters unwrapped
            result.push(ch);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_color_conversion() {
        let input = "\x1b[31mRed text\x1b[0m";
        let result = convert_ansi_to_html(input);
        assert!(result.contains(r#"<span style='color:var(--red,#a00)'>Red text</span>"#));
    }

    #[test]
    fn test_bold_formatting() {
        let input = "\x1b[1mBold text\x1b[0m";
        let result = convert_ansi_to_html(input);
        assert!(result.contains(r#"<b>Bold text</b>"#));
        assert!(result.starts_with("<pre"));
        assert!(result.ends_with("</pre>"));
    }

    #[test]
    fn test_rgb_color() {
        let input = "\x1b[38;2;255;128;0mOrange text\x1b[0m";
        let result = convert_ansi_to_html(input);
        assert!(result.contains(r#"<span style='color:#ff8000'>Orange text</span>"#));
        assert!(result.starts_with("<pre"));
        assert!(result.ends_with("</pre>"));
    }

    #[test]
    fn test_256_color() {
        let input = "\x1b[38;5;196mBright red\x1b[0m";
        let result = convert_ansi_to_html(input);
        assert!(result.contains("color:"));
        assert!(result.contains("Bright red"));
    }

    #[test]
    fn test_newline_and_spacing_preservation() {
        let input = "Line 1\nLine 2  with spaces\n  Indented line";
        let result = convert_ansi_to_html(input);

        assert!(result.starts_with("<pre"));
        assert!(result.ends_with("</pre>"));

        assert!(result.contains("Line 1\nLine 2  with spaces\n  Indented line"));

        assert!(result.contains("font-family: monospace"));
        assert!(result.contains("white-space: pre"));
    }

    #[test]
    fn test_table_like_output() {
        let input = "┌─────┬─────┐\n│ A   │ B   │\n└─────┴─────┘";
        let result = convert_ansi_to_html(input);

        assert!(result.contains("┌─────┬─────┐"));
        assert!(result.contains("│ A   │ B   │"));
        assert!(result.contains("└─────┴─────┘"));

        assert!(result.starts_with("<pre"));
        assert!(result.ends_with("</pre>"));
    }

    #[test]
    fn test_emoji_fixed_width() {
        // Test with mix of 1-width symbols and 2-width emojis
        let input = "Status: ✅ Success ✓ Done 🚀 Launch ⚠ Warning ❌ Error";
        let result = convert_ansi_to_html(input);

        // Should wrap 2-width emojis in fixed-width spans
        assert!(result.contains(
            r#"<span style="display: inline-block; width: 2ch; text-align: center;">✅</span>"#
        ));
        assert!(result.contains(
            r#"<span style="display: inline-block; width: 2ch; text-align: center;">🚀</span>"#
        ));
        assert!(result.contains(
            r#"<span style="display: inline-block; width: 2ch; text-align: center;">❌</span>"#
        ));

        // Should NOT wrap 1-width symbols
        assert!(!result.contains(
            r#"<span style="display: inline-block; width: 2ch; text-align: center;">✓</span>"#
        ));
        assert!(!result.contains(
            r#"<span style="display: inline-block; width: 2ch; text-align: center;">⚠</span>"#
        ));

        // Should still contain the unwrapped 1-width symbols
        assert!(result.contains("✓"));
        assert!(result.contains("⚠"));

        // Basic text should be preserved
        assert!(result.contains("Status:"));
        assert!(result.contains("Success"));
        assert!(result.contains("Launch"));
        assert!(result.contains("Warning"));
        assert!(result.contains("Error"));
    }
}
