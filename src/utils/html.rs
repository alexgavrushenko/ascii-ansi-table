use regex::Regex;
use unicode_width::UnicodeWidthChar;

pub fn convert_ansi_to_html(text: &str) -> String {
    let mut result = text.to_string();

    let fg_colors = [
        (30, "#000000"),
        (31, "#ff0000"),
        (32, "#00ff00"),
        (33, "#ffff00"),
        (34, "#0000ff"),
        (35, "#ff00ff"),
        (36, "#00ffff"),
        (37, "#ffffff"),
    ];

    let bg_colors = [
        (40, "#000000"),
        (41, "#ff0000"),
        (42, "#00ff00"),
        (43, "#ffff00"),
        (44, "#0000ff"),
        (45, "#ff00ff"),
        (46, "#00ffff"),
        (47, "#ffffff"),
    ];

    // Process basic foreground colors
    for (code, color) in fg_colors {
        let pattern = format!("\x1b[{}m", code);
        let replacement = format!(r#"<span style="color: {};">"#, color);
        result = result.replace(&pattern, &replacement);
    }

    // Process basic background colors
    for (code, color) in bg_colors {
        let pattern = format!("\x1b[{}m", code);
        let replacement = format!(r#"<span style="background-color: {};">"#, color);
        result = result.replace(&pattern, &replacement);
    }

    // Process formatting codes
    result = result.replace("\x1b[1m", r#"<span style="font-weight: bold;">"#);
    result = result.replace("\x1b[4m", r#"<span style="text-decoration: underline;">"#);
    result = result.replace("\x1b[3m", r#"<span style="font-style: italic;">"#);
    result = result.replace(
        "\x1b[9m",
        r#"<span style="text-decoration: line-through;">"#,
    );
    result = result.replace("\x1b[2m", r#"<span style="opacity: 0.6;">"#);
    result = result.replace("\x1b[7m", r#"<span style="filter: invert(1);">"#);

    // Process RGB and 256-color codes
    let re_256_fg = Regex::new(r"\x1b\[38;5;(\d+)m").unwrap();
    result = re_256_fg
        .replace_all(&result, |caps: &regex::Captures| {
            let color_num: u8 = caps[1].parse().unwrap_or(15);
            let color = ansi_256_to_rgb(color_num);
            format!(r#"<span style="color: {};">"#, color)
        })
        .to_string();

    let re_256_bg = Regex::new(r"\x1b\[48;5;(\d+)m").unwrap();
    result = re_256_bg
        .replace_all(&result, |caps: &regex::Captures| {
            let color_num: u8 = caps[1].parse().unwrap_or(0);
            let color = ansi_256_to_rgb(color_num);
            format!(r#"<span style="background-color: {};">"#, color)
        })
        .to_string();

    let re_rgb_fg = Regex::new(r"\x1b\[38;2;(\d+);(\d+);(\d+)m").unwrap();
    result = re_rgb_fg
        .replace_all(&result, |caps: &regex::Captures| {
            let r: u8 = caps[1].parse().unwrap_or(255);
            let g: u8 = caps[2].parse().unwrap_or(255);
            let b: u8 = caps[3].parse().unwrap_or(255);
            format!(r#"<span style="color: rgb({}, {}, {});">"#, r, g, b)
        })
        .to_string();

    let re_rgb_bg = Regex::new(r"\x1b\[48;2;(\d+);(\d+);(\d+)m").unwrap();
    result = re_rgb_bg
        .replace_all(&result, |caps: &regex::Captures| {
            let r: u8 = caps[1].parse().unwrap_or(0);
            let g: u8 = caps[2].parse().unwrap_or(0);
            let b: u8 = caps[3].parse().unwrap_or(0);
            format!(
                r#"<span style="background-color: rgb({}, {}, {});">"#,
                r, g, b
            )
        })
        .to_string();

    // Handle ANSI reset - close ALL open spans
    while result.contains("\x1b[0m") {
        let reset_pos = result.find("\x1b[0m").unwrap();
        let mut replacement = String::new();
        
        // Count how many spans are currently open at this position
        let before_reset = &result[..reset_pos];
        let span_opens = before_reset.matches("<span").count();
        let span_closes = before_reset.matches("</span>").count();
        let spans_to_close = span_opens.saturating_sub(span_closes);
        
        // Close all open spans
        for _ in 0..spans_to_close {
            replacement.push_str("</span>");
        }
        
        result = result.replacen("\x1b[0m", &replacement, 1);
    }

    // Handle specific color resets (close only one span each)
    result = result.replace("\x1b[39m", "</span>");
    result = result.replace("\x1b[49m", "</span>");

    let re_cleanup = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    result = re_cleanup.replace_all(&result, "").to_string();

    // Wrap emojis in spans with fixed width to match Rust's 2-character calculation
    result = wrap_emojis_with_fixed_width(&result);

    result = format!(
        "<pre style=\"margin: 0; font-family: monospace; white-space: pre;\">{}</pre>",
        result
    );

    result
}

fn wrap_emojis_with_fixed_width(text: &str) -> String {
    let mut result = String::new();
    
    for ch in text.chars() {
        // Use unicode-width crate to determine actual character width
        let width = ch.width().unwrap_or(1);
        
        if width == 2 {
            // Wrap 2-width characters in fixed-width spans
            result.push_str(&format!(
                r#"<span style="display: inline-block; width: 2ch; text-align: center;">{}</span>"#,
                ch
            ));
        } else {
            // Leave 1-width characters unwrapped
            result.push(ch);
        }
    }
    
    result
}

fn ansi_256_to_rgb(color_num: u8) -> String {
    match color_num {
        0 => "#000000".to_string(),
        1 => "#800000".to_string(),
        2 => "#008000".to_string(),
        3 => "#808000".to_string(),
        4 => "#000080".to_string(),
        5 => "#800080".to_string(),
        6 => "#008080".to_string(),
        7 => "#c0c0c0".to_string(),
        8 => "#808080".to_string(),
        9 => "#ff0000".to_string(),
        10 => "#00ff00".to_string(),
        11 => "#ffff00".to_string(),
        12 => "#0000ff".to_string(),
        13 => "#ff00ff".to_string(),
        14 => "#00ffff".to_string(),
        15 => "#ffffff".to_string(),

        16..=231 => {
            let n = color_num - 16;
            let r = (n / 36) * 51;
            let g = ((n % 36) / 6) * 51;
            let b = (n % 6) * 51;
            format!("#{:02x}{:02x}{:02x}", r, g, b)
        }

        232..=255 => {
            let gray = 8 + (color_num - 232) * 10;
            format!("#{:02x}{:02x}{:02x}", gray, gray, gray)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_color_conversion() {
        let input = "\x1b[31mRed text\x1b[0m";
        let result = convert_ansi_to_html(input);
        assert!(result.contains(r#"<span style="color: #ff0000;">Red text</span>"#));
        assert!(result.starts_with("<pre"));
        assert!(result.ends_with("</pre>"));
    }

    #[test]
    fn test_bold_formatting() {
        let input = "\x1b[1mBold text\x1b[0m";
        let result = convert_ansi_to_html(input);
        assert!(result.contains(r#"<span style="font-weight: bold;">Bold text</span>"#));
        assert!(result.starts_with("<pre"));
        assert!(result.ends_with("</pre>"));
    }

    #[test]
    fn test_rgb_color() {
        let input = "\x1b[38;2;255;128;0mOrange text\x1b[0m";
        let result = convert_ansi_to_html(input);
        assert!(result.contains(r#"<span style="color: rgb(255, 128, 0);">Orange text</span>"#));
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
        assert!(result.contains(r#"<span style="display: inline-block; width: 2ch; text-align: center;">✅</span>"#));
        assert!(result.contains(r#"<span style="display: inline-block; width: 2ch; text-align: center;">🚀</span>"#));
        assert!(result.contains(r#"<span style="display: inline-block; width: 2ch; text-align: center;">❌</span>"#));
        
        // Should NOT wrap 1-width symbols
        assert!(!result.contains(r#"<span style="display: inline-block; width: 2ch; text-align: center;">✓</span>"#));
        assert!(!result.contains(r#"<span style="display: inline-block; width: 2ch; text-align: center;">⚠</span>"#));
        
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
