/// Emoji width detection and handling utilities
use std::collections::HashMap;

/// Configuration for emoji width calculation
#[derive(Debug, Clone)]
pub struct EmojiConfig {
    pub use_unicode_width: bool,
    pub emoji_width: usize,
    pub variation_selector_width: usize,
    pub zero_width_joiner_aware: bool,
    pub regional_indicator_width: usize,
    pub custom_widths: HashMap<String, usize>,
}

impl Default for EmojiConfig {
    fn default() -> Self {
        Self {
            use_unicode_width: true,
            emoji_width: 2,
            variation_selector_width: 0,
            zero_width_joiner_aware: true,
            regional_indicator_width: 2,
            custom_widths: HashMap::new(),
        }
    }
}

impl EmojiConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_emoji_width(mut self, width: usize) -> Self {
        self.emoji_width = width;
        self
    }

    pub fn with_regional_indicator_width(mut self, width: usize) -> Self {
        self.regional_indicator_width = width;
        self
    }

    pub fn with_zwj_awareness(mut self, aware: bool) -> Self {
        self.zero_width_joiner_aware = aware;
        self
    }

    pub fn with_custom_width(mut self, emoji: &str, width: usize) -> Self {
        self.custom_widths.insert(emoji.to_string(), width);
        self
    }

    pub fn without_unicode_width(mut self) -> Self {
        self.use_unicode_width = false;
        self
    }

    /// Terminal-optimized preset (most terminals display emojis as 2 chars wide)
    pub fn terminal() -> Self {
        Self::new()
            .with_emoji_width(2)
            .with_regional_indicator_width(2)
            .with_zwj_awareness(true)
    }

    /// Monospace font preset (emojis often take 2 character spaces)
    pub fn monospace() -> Self {
        Self::new()
            .with_emoji_width(2)
            .with_regional_indicator_width(2)
            .without_unicode_width()
    }

    /// Web display preset (emojis usually display as 1 character width in web)
    pub fn web() -> Self {
        Self::new()
            .with_emoji_width(1)
            .with_regional_indicator_width(1)
    }

    /// Conservative preset (assume all emojis take 2 characters for safety)
    pub fn conservative() -> Self {
        Self::new()
            .with_emoji_width(2)
            .with_regional_indicator_width(4) // Flag emojis might be extra wide
    }
}

/// Emoji width calculator with configurable behavior
pub struct EmojiWidthCalculator {
    config: EmojiConfig,
}

impl EmojiWidthCalculator {
    pub fn new(config: EmojiConfig) -> Self {
        Self { config }
    }

    /// Calculate the display width of text containing emojis
    pub fn text_width(&self, text: &str) -> usize {
        let mut total_width = 0;
        let mut chars = text.chars().peekable();

        while let Some(ch) = chars.next() {
            // Check for custom width first
            if let Some(&width) = self.config.custom_widths.get(&ch.to_string()) {
                total_width += width;
                continue;
            }

            // Handle different character types
            if self.is_emoji_character(ch) {
                total_width += self.calculate_emoji_sequence_width(&mut chars, ch);
            } else if self.is_regional_indicator(ch) {
                total_width += self.handle_regional_indicators(&mut chars, ch);
            } else if self.is_variation_selector(ch) {
                total_width += self.config.variation_selector_width;
            } else if self.is_zero_width_joiner(ch) {
                // ZWJ doesn't contribute to width
                continue;
            } else {
                // Regular character
                total_width += self.character_width(ch);
            }
        }

        total_width
    }

    /// Calculate width of an emoji sequence (considering ZWJ, modifiers, etc.)
    fn calculate_emoji_sequence_width(&self, chars: &mut std::iter::Peekable<std::str::Chars>, first_emoji: char) -> usize {
        let mut width = self.config.emoji_width;
        let mut sequence_chars = vec![first_emoji];

        while let Some(&next_ch) = chars.peek() {
            if self.is_zero_width_joiner(next_ch) && self.config.zero_width_joiner_aware {
                chars.next(); // consume ZWJ
                sequence_chars.push(next_ch);
                
                // Check if there's another emoji after ZWJ
                if let Some(&emoji_after_zwj) = chars.peek() {
                    if self.is_emoji_character(emoji_after_zwj) {
                        chars.next(); // consume the emoji
                        sequence_chars.push(emoji_after_zwj);
                        // ZWJ sequences typically display as one emoji
                        // Don't add extra width
                    }
                }
            } else if self.is_modifier(next_ch) {
                chars.next(); // consume modifier
                sequence_chars.push(next_ch);
                // Modifiers don't usually add width
            } else if self.is_variation_selector(next_ch) {
                chars.next(); // consume variation selector
                sequence_chars.push(next_ch);
                width += self.config.variation_selector_width;
            } else {
                break;
            }
        }

        // Check if we have a custom width for the full sequence
        let sequence_string: String = sequence_chars.iter().collect();
        if let Some(&custom_width) = self.config.custom_widths.get(&sequence_string) {
            return custom_width;
        }

        width
    }

    /// Handle regional indicator sequences (flag emojis)
    fn handle_regional_indicators(&self, chars: &mut std::iter::Peekable<std::str::Chars>, first_ri: char) -> usize {
        // Check if next character is also a regional indicator
        if let Some(&next_ch) = chars.peek() {
            if self.is_regional_indicator(next_ch) {
                chars.next(); // consume second regional indicator
                return self.config.regional_indicator_width;
            }
        }
        
        // Single regional indicator
        self.config.regional_indicator_width / 2
    }

    /// Get the display width of a regular character
    fn character_width(&self, ch: char) -> usize {
        if self.config.use_unicode_width {
            crate::unicode::char_display_width(ch)
        } else {
            if ch.is_ascii() { 1 } else { 2 }
        }
    }

    /// Check if character is an emoji
    fn is_emoji_character(&self, ch: char) -> bool {
        match ch as u32 {
            // Basic emoticons
            0x1F600..=0x1F64F => true, // Emoticons
            0x1F300..=0x1F5FF => true, // Miscellaneous Symbols and Pictographs
            0x1F680..=0x1F6FF => true, // Transport and Map Symbols
            0x1F1E6..=0x1F1FF => true, // Regional Indicator Symbols
            0x2600..=0x26FF => true,   // Miscellaneous Symbols
            0x2700..=0x27BF => true,   // Dingbats
            0x1F900..=0x1F9FF => true, // Supplemental Symbols and Pictographs
            0x1F018..=0x1F270 => true, // Various emoji blocks
            
            // Specific common emojis
            0x2764 => true, // ❤️
            0x2B50 => true, // ⭐
            0x2705 => true, // ✅
            0x274C => true, // ❌
            0x2728 => true, // ✨
            0x1F389 => true, // 🎉
            
            _ => false,
        }
    }

    /// Check if character is a regional indicator (used in flag emojis)
    fn is_regional_indicator(&self, ch: char) -> bool {
        matches!(ch as u32, 0x1F1E6..=0x1F1FF)
    }

    /// Check if character is a variation selector
    fn is_variation_selector(&self, ch: char) -> bool {
        matches!(ch as u32, 0xFE00..=0xFE0F | 0xE0100..=0xE01EF)
    }

    /// Check if character is Zero Width Joiner
    fn is_zero_width_joiner(&self, ch: char) -> bool {
        ch as u32 == 0x200D
    }

    /// Check if character is a modifier (skin tone, etc.)
    fn is_modifier(&self, ch: char) -> bool {
        matches!(ch as u32, 0x1F3FB..=0x1F3FF)
    }
}

/// Emoji-aware text alignment functions
pub fn emoji_align_text(text: &str, width: usize, alignment: crate::alignment::Alignment, config: &EmojiConfig) -> String {
    let calculator = EmojiWidthCalculator::new(config.clone());
    let text_width = calculator.text_width(text);
    
    if text_width >= width {
        return text.to_string();
    }
    
    let padding_needed = width - text_width;
    
    match alignment {
        crate::alignment::Alignment::Left => {
            format!("{}{}", text, " ".repeat(padding_needed))
        }
        crate::alignment::Alignment::Right => {
            format!("{}{}", " ".repeat(padding_needed), text)
        }
        crate::alignment::Alignment::Center => {
            let left_padding = padding_needed / 2;
            let right_padding = padding_needed - left_padding;
            format!("{}{}{}", " ".repeat(left_padding), text, " ".repeat(right_padding))
        }
        crate::alignment::Alignment::Justify => {
            // For emoji text, justify is complex, just center instead
            let left_padding = padding_needed / 2;
            let right_padding = padding_needed - left_padding;
            format!("{}{}{}", " ".repeat(left_padding), text, " ".repeat(right_padding))
        }
    }
}

/// Emoji-aware text truncation
pub fn emoji_truncate_text(text: &str, max_width: usize, config: &EmojiConfig, ellipsis: &str) -> String {
    let calculator = EmojiWidthCalculator::new(config.clone());
    let text_width = calculator.text_width(text);
    
    if text_width <= max_width {
        return text.to_string();
    }
    
    let ellipsis_width = calculator.text_width(ellipsis);
    if ellipsis_width >= max_width {
        return ellipsis[..max_width.min(ellipsis.len())].to_string();
    }
    
    let target_width = max_width - ellipsis_width;
    let mut result = String::new();
    let mut current_width = 0;
    
    let mut chars = text.chars().peekable();
    while let Some(ch) = chars.next() {
        let ch_width = if calculator.is_emoji_character(ch) {
            calculator.calculate_emoji_sequence_width(&mut chars, ch)
        } else {
            calculator.character_width(ch)
        };
        
        if current_width + ch_width > target_width {
            break;
        }
        
        result.push(ch);
        current_width += ch_width;
    }
    
    format!("{}{}", result, ellipsis)
}

/// Emoji-aware table column width calculation
pub fn calculate_emoji_column_widths(rows: &[Vec<String>], config: &EmojiConfig) -> Vec<usize> {
    if rows.is_empty() {
        return vec![];
    }
    
    let calculator = EmojiWidthCalculator::new(config.clone());
    let mut widths = vec![0; rows[0].len()];
    
    for row in rows {
        for (i, cell) in row.iter().enumerate().take(widths.len()) {
            let width = calculator.text_width(cell);
            widths[i] = widths[i].max(width);
        }
    }
    
    widths
}

/// Render table with emoji-aware width calculations
pub fn render_emoji_table(
    data: &crate::TableData,
    border: &crate::BorderChars,
    options: &crate::RenderOptions,
    config: &EmojiConfig,
) -> Result<String, String> {
    crate::validate_table_data(data)?;
    
    if data.is_empty() {
        return Ok(String::new());
    }
    
    let auto_widths = calculate_emoji_column_widths(&data.rows, config);
    let calculator = EmojiWidthCalculator::new(config.clone());
    let mut result = String::new();
    
    // Top border
    if options.show_top_border {
        result.push(border.top_left);
        for (i, width) in auto_widths.iter().enumerate() {
            result.push_str(&border.horizontal.to_string().repeat(*width + 2)); // +2 for padding
            if i < auto_widths.len() - 1 {
                result.push(border.top_junction);
            }
        }
        result.push(border.top_right);
        result.push('\n');
    }
    
    // Data rows
    for (row_idx, row) in data.rows.iter().enumerate() {
        result.push(border.vertical);
        
        for (i, cell) in row.iter().enumerate() {
            let width = auto_widths.get(i).copied().unwrap_or(0);
            let aligned_cell = emoji_align_text(cell, width, crate::alignment::Alignment::Left, config);
            result.push(' ');
            result.push_str(&aligned_cell);
            result.push(' ');
            result.push(border.vertical);
        }
        result.push('\n');
        
        // Row separators
        if options.show_row_separators && row_idx < data.rows.len() - 1 {
            result.push('├');
            for (i, width) in auto_widths.iter().enumerate() {
                result.push_str(&border.horizontal.to_string().repeat(*width + 2));
                if i < auto_widths.len() - 1 {
                    result.push('┼');
                }
            }
            result.push('┤');
            result.push('\n');
        }
    }
    
    // Bottom border
    if options.show_bottom_border {
        result.push(border.bottom_left);
        for (i, width) in auto_widths.iter().enumerate() {
            result.push_str(&border.horizontal.to_string().repeat(*width + 2));
            if i < auto_widths.len() - 1 {
                result.push(border.bottom_junction);
            }
        }
        result.push(border.bottom_right);
        result.push('\n');
    }
    
    Ok(result)
}

/// Common emoji examples for testing
pub mod examples {
    pub const BASIC_EMOJIS: &[&str] = &[
        "😀", "😃", "😄", "😁", "😆", "😅", "🤣", "😂",
        "🙂", "🙃", "😉", "😊", "😇", "🥰", "😍", "🤩",
    ];
    
    pub const FLAG_EMOJIS: &[&str] = &[
        "🇺🇸", "🇬🇧", "🇫🇷", "🇩🇪", "🇯🇵", "🇨🇳", "🇰🇷", "🇷🇺",
    ];
    
    pub const ZWJ_SEQUENCES: &[&str] = &[
        "👨‍💻", "👩‍💻", "👨‍🔬", "👩‍🔬", "👨‍🎨", "👩‍🎨",
        "👨‍🚀", "👩‍🚀", "👨‍⚕️", "👩‍⚕️", "👨‍🏫", "👩‍🏫",
    ];
    
    pub const SKIN_TONE_EMOJIS: &[&str] = &[
        "👋🏻", "👋🏼", "👋🏽", "👋🏾", "👋🏿",
        "👍🏻", "👍🏼", "👍🏽", "👍🏾", "👍🏿",
    ];
    
    pub fn create_test_table() -> crate::TableData {
        crate::TableData::new(vec![
            vec!["Emoji".to_string(), "Name".to_string(), "Category".to_string()],
            vec!["😀".to_string(), "Grinning Face".to_string(), "Smileys".to_string()],
            vec!["🇺🇸".to_string(), "United States Flag".to_string(), "Flags".to_string()],
            vec!["👨‍💻".to_string(), "Man Technologist".to_string(), "People".to_string()],
            vec!["👍🏽".to_string(), "Thumbs Up (Medium)".to_string(), "Gestures".to_string()],
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emoji_width_calculator() {
        let config = EmojiConfig::terminal();
        let calculator = EmojiWidthCalculator::new(config);
        
        // Basic emoji should be 2 characters wide
        assert_eq!(calculator.text_width("😀"), 2);
        
        // Regular ASCII should be 1 character wide
        assert_eq!(calculator.text_width("A"), 1);
        
        // Mixed content
        assert_eq!(calculator.text_width("Hello 😀 World"), 13); // 6 + 2 + 6 - 1 = 13
    }

    #[test]
    fn test_flag_emoji_width() {
        let config = EmojiConfig::terminal();
        let calculator = EmojiWidthCalculator::new(config);
        
        // Flag emojis (regional indicators) should be handled specially
        assert_eq!(calculator.text_width("🇺🇸"), 2);
        assert_eq!(calculator.text_width("🇬🇧"), 2);
    }

    #[test]
    fn test_emoji_config_presets() {
        let terminal_config = EmojiConfig::terminal();
        assert_eq!(terminal_config.emoji_width, 2);
        assert!(terminal_config.zero_width_joiner_aware);
        
        let web_config = EmojiConfig::web();
        assert_eq!(web_config.emoji_width, 1);
        
        let conservative_config = EmojiConfig::conservative();
        assert_eq!(conservative_config.emoji_width, 2);
        assert_eq!(conservative_config.regional_indicator_width, 4);
    }

    #[test]
    fn test_custom_emoji_widths() {
        let config = EmojiConfig::new()
            .with_custom_width("😀", 3)
            .with_custom_width("🎉", 1);
        
        let calculator = EmojiWidthCalculator::new(config);
        
        assert_eq!(calculator.text_width("😀"), 3);
        assert_eq!(calculator.text_width("🎉"), 1);
    }

    #[test]
    fn test_emoji_alignment() {
        let config = EmojiConfig::terminal();
        
        let left = emoji_align_text("😀", 5, crate::alignment::Alignment::Left, &config);
        assert_eq!(left, "😀   ");
        
        let right = emoji_align_text("😀", 5, crate::alignment::Alignment::Right, &config);
        assert_eq!(right, "   😀");
        
        let center = emoji_align_text("😀", 5, crate::alignment::Alignment::Center, &config);
        assert_eq!(center, " 😀  ");
    }

    #[test]
    fn test_emoji_truncation() {
        let config = EmojiConfig::terminal();
        
        let result = emoji_truncate_text("Hello 😀 World", 8, &config, "...");
        // "Hello " = 6, "😀" = 2, would be 8 total, but ellipsis needs space
        assert!(result.len() <= 8);
        assert!(result.ends_with("..."));
    }

    #[test]
    fn test_emoji_column_width_calculation() {
        let config = EmojiConfig::terminal();
        
        let rows = vec![
            vec!["😀".to_string(), "Short".to_string()],
            vec!["A".to_string(), "Very Long Text".to_string()],
        ];
        
        let widths = calculate_emoji_column_widths(&rows, &config);
        assert_eq!(widths[0], 2); // "😀" is wider than "A"
        assert_eq!(widths[1], 14); // "Very Long Text"
    }

    #[test]
    fn test_zwj_sequences() {
        let config = EmojiConfig::terminal();
        let calculator = EmojiWidthCalculator::new(config);
        
        // ZWJ sequences like "👨‍💻" should typically display as one emoji
        let width = calculator.text_width("👨‍💻");
        assert_eq!(width, 2); // Should be treated as single emoji width
    }

    #[test]
    fn test_skin_tone_modifiers() {
        let config = EmojiConfig::terminal();
        let calculator = EmojiWidthCalculator::new(config);
        
        // Skin tone modifiers shouldn't add extra width
        assert_eq!(calculator.text_width("👍"), 2);
        assert_eq!(calculator.text_width("👍🏽"), 2);
    }

    #[test]
    fn test_variation_selectors() {
        let mut config = EmojiConfig::terminal();
        config.variation_selector_width = 0; // Usually variation selectors are zero-width
        
        let calculator = EmojiWidthCalculator::new(config);
        
        // Variation selectors shouldn't add significant width
        let base_width = calculator.text_width("❤");
        let with_vs_width = calculator.text_width("❤️"); // ❤ + variation selector
        assert_eq!(base_width, with_vs_width);
    }

    #[test]
    fn test_emoji_examples() {
        let table = examples::create_test_table();
        assert_eq!(table.row_count(), 5);
        assert_eq!(table.column_count(), 3);
        
        let config = EmojiConfig::terminal();
        let result = render_emoji_table(&table, &crate::BorderChars::default(), &crate::RenderOptions::default(), &config);
        assert!(result.is_ok());
        
        let output = result.unwrap();
        assert!(output.contains("😀"));
        assert!(output.contains("🇺🇸"));
        assert!(output.contains("👨‍💻"));
    }

    #[test]
    fn test_empty_table_handling() {
        let empty_table = crate::TableData::new(vec![]);
        let config = EmojiConfig::default();
        
        let result = render_emoji_table(&empty_table, &crate::BorderChars::default(), &crate::RenderOptions::default(), &config);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }
}