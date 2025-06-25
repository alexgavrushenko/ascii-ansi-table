use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub fn calculate_string_width(text: &str) -> usize {
    UnicodeWidthStr::width(text)
}

pub fn calculate_grapheme_width(text: &str) -> usize {
    text.graphemes(true).count()
}

pub fn truncate_string(text: &str, max_width: usize) -> String {
    let width = calculate_string_width(text);

    if width <= max_width {
        return text.to_string();
    }

    let mut result = String::new();
    let mut current_width = 0;

    for grapheme in text.graphemes(true) {
        let grapheme_width = UnicodeWidthStr::width(grapheme);
        if current_width + grapheme_width > max_width {
            break;
        }
        result.push_str(grapheme);
        current_width += grapheme_width;
    }

    result
}

pub fn pad_string(text: &str, width: usize, alignment: crate::types::Alignment) -> String {
    let current_width = calculate_string_width(text);

    if current_width >= width {
        return text.to_string();
    }

    let padding = width - current_width;

    match alignment {
        crate::types::Alignment::Left => format!("{}{}", text, " ".repeat(padding)),
        crate::types::Alignment::Right => format!("{}{}", " ".repeat(padding), text),
        crate::types::Alignment::Center => {
            let left_padding = padding / 2;
            let right_padding = padding - left_padding;
            format!(
                "{}{}{}",
                " ".repeat(left_padding),
                text,
                " ".repeat(right_padding)
            )
        }
        crate::types::Alignment::Justify => justify_string(text, width),
    }
}

pub fn justify_string(text: &str, width: usize) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();

    if words.len() <= 1 {
        return text.to_string();
    }

    let total_chars: usize = words.iter().map(|w| calculate_string_width(w)).sum();
    let total_spaces = width - total_chars;
    let gaps = words.len() - 1;

    if gaps == 0 {
        return text.to_string();
    }

    let spaces_per_gap = total_spaces / gaps;
    let extra_spaces = total_spaces % gaps;

    let mut result = String::new();

    for (i, word) in words.iter().enumerate() {
        result.push_str(word);

        if i < words.len() - 1 {
            let spaces = spaces_per_gap + if i < extra_spaces { 1 } else { 0 };
            result.push_str(&" ".repeat(spaces));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_string_width() {
        assert_eq!(calculate_string_width("hello"), 5);
        assert_eq!(calculate_string_width("测试"), 4);
        assert_eq!(calculate_string_width("🌟"), 2);
    }

    #[test]
    fn test_symbol_widths() {
        // Test various symbols used in our demos
        assert_eq!(calculate_string_width("✓"), 1); // Check mark
        assert_eq!(calculate_string_width("✗"), 1); // X mark
        assert_eq!(calculate_string_width("⚠"), 1); // Warning sign
        assert_eq!(calculate_string_width("🚀"), 2); // Rocket emoji
        assert_eq!(calculate_string_width("📦"), 2); // Package emoji
        assert_eq!(calculate_string_width("✅"), 2); // Check mark emoji
        assert_eq!(calculate_string_width("🎯"), 2); // Target emoji
        assert_eq!(calculate_string_width("🌊"), 2); // Wave emoji
        assert_eq!(calculate_string_width("❌"), 2); // Cross mark emoji
        println!("Symbol width analysis with Unicode points:");
        println!(
            "'✓' (U+{:04X}) -> {} chars",
            '✓' as u32,
            calculate_string_width("✓")
        );
        println!(
            "'✗' (U+{:04X}) -> {} chars",
            '✗' as u32,
            calculate_string_width("✗")
        );
        println!(
            "'⚠' (U+{:04X}) -> {} chars",
            '⚠' as u32,
            calculate_string_width("⚠")
        );
        println!(
            "'🚀' (U+{:04X}) -> {} chars",
            '🚀' as u32,
            calculate_string_width("🚀")
        );
        println!(
            "'📦' (U+{:04X}) -> {} chars",
            '📦' as u32,
            calculate_string_width("📦")
        );
        println!(
            "'✅' (U+{:04X}) -> {} chars",
            '✅' as u32,
            calculate_string_width("✅")
        );
        println!(
            "'❌' (U+{:04X}) -> {} chars",
            '❌' as u32,
            calculate_string_width("❌")
        );
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("hello world", 5), "hello");
        assert_eq!(truncate_string("测试文本", 6), "测试文");
        assert_eq!(truncate_string("short", 10), "short");
    }

    #[test]
    fn test_pad_string() {
        assert_eq!(
            pad_string("hello", 10, crate::types::Alignment::Left),
            "hello     "
        );
        assert_eq!(
            pad_string("hello", 10, crate::types::Alignment::Right),
            "     hello"
        );
        assert_eq!(
            pad_string("hello", 10, crate::types::Alignment::Center),
            "  hello   "
        );
    }

    #[test]
    fn test_justify_string() {
        assert_eq!(justify_string("hello world", 15), "hello     world");
        assert_eq!(justify_string("a b c", 7), "a  b  c");
        assert_eq!(justify_string("single", 10), "single");
    }
}
