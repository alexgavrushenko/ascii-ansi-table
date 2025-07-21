use regex::Regex;

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

    for (code, color) in fg_colors {
        let pattern = format!("\x1b[{}m", code);
        let replacement = format!(r#"<span style="color: {};">"#, color);
        result = result.replace(&pattern, &replacement);
    }

    for (code, color) in bg_colors {
        let pattern = format!("\x1b[{}m", code);
        let replacement = format!(r#"<span style="background-color: {};">"#, color);
        result = result.replace(&pattern, &replacement);
    }

    result = result.replace("\x1b[1m", r#"<span style="font-weight: bold;">"#);
    result = result.replace("\x1b[4m", r#"<span style="text-decoration: underline;">"#);
    result = result.replace("\x1b[3m", r#"<span style="font-style: italic;">"#);
    result = result.replace(
        "\x1b[9m",
        r#"<span style="text-decoration: line-through;">"#,
    );
    result = result.replace("\x1b[2m", r#"<span style="opacity: 0.6;">"#);
    result = result.replace("\x1b[7m", r#"<span style="filter: invert(1);">"#);

    result = result.replace("\x1b[0m", "</span>");
    result = result.replace("\x1b[39m", "</span>");
    result = result.replace("\x1b[49m", "</span>");

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

    let re_cleanup = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    result = re_cleanup.replace_all(&result, "").to_string();

    result = format!(
        "<pre style=\"margin: 0; font-family: monospace; white-space: pre;\">{}</pre>",
        result
    );

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
}
