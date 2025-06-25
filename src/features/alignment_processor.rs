use crate::types::{Alignment, VerticalAlignment};
use crate::utils::ansi::pad_ansi_string;

pub struct AlignmentProcessor;

impl AlignmentProcessor {
    pub fn align_text_horizontal(text: &str, width: usize, alignment: Alignment) -> String {
        let padded = pad_ansi_string(text, width, alignment);
        padded.content
    }

    pub fn align_text_vertical(
        lines: &[String],
        height: usize,
        alignment: VerticalAlignment,
    ) -> Vec<String> {
        if lines.len() >= height {
            return lines.to_vec();
        }

        let empty_lines = height - lines.len();
        let mut result = Vec::new();

        match alignment {
            VerticalAlignment::Top => {
                result.extend_from_slice(lines);
                result.resize(height, String::new());
            }
            VerticalAlignment::Bottom => {
                result.resize(empty_lines, String::new());
                result.extend_from_slice(lines);
            }
            VerticalAlignment::Middle => {
                let top_padding = empty_lines / 2;
                result.resize(top_padding, String::new());
                result.extend_from_slice(lines);
                result.resize(height, String::new());
            }
        }

        result
    }

    pub fn distribute_content_evenly(content: &str, width: usize) -> String {
        let words: Vec<&str> = content.split_whitespace().collect();

        if words.len() <= 1 {
            return content.to_string();
        }

        let total_word_width: usize = words.iter().map(|w| w.len()).sum();
        let total_spaces = width.saturating_sub(total_word_width);
        let gaps = words.len() - 1;

        if gaps == 0 {
            return content.to_string();
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_text_horizontal() {
        let text = "hello";

        let left_aligned = AlignmentProcessor::align_text_horizontal(text, 10, Alignment::Left);
        assert_eq!(left_aligned, "hello     ");

        let right_aligned = AlignmentProcessor::align_text_horizontal(text, 10, Alignment::Right);
        assert_eq!(right_aligned, "     hello");

        let center_aligned = AlignmentProcessor::align_text_horizontal(text, 10, Alignment::Center);
        assert_eq!(center_aligned, "  hello   ");
    }

    #[test]
    fn test_align_text_vertical() {
        let lines = vec!["line1".to_string(), "line2".to_string()];

        let top_aligned =
            AlignmentProcessor::align_text_vertical(&lines, 5, VerticalAlignment::Top);
        assert_eq!(top_aligned, vec!["line1", "line2", "", "", ""]);

        let bottom_aligned =
            AlignmentProcessor::align_text_vertical(&lines, 5, VerticalAlignment::Bottom);
        assert_eq!(bottom_aligned, vec!["", "", "", "line1", "line2"]);

        let middle_aligned =
            AlignmentProcessor::align_text_vertical(&lines, 5, VerticalAlignment::Middle);
        assert_eq!(middle_aligned, vec!["", "line1", "line2", "", ""]);
    }

    #[test]
    fn test_distribute_content_evenly() {
        let content = "hello world test";
        let distributed = AlignmentProcessor::distribute_content_evenly(content, 20);
        assert_eq!(distributed, "hello   world   test");

        let single_word = "hello";
        let single_distributed = AlignmentProcessor::distribute_content_evenly(single_word, 10);
        assert_eq!(single_distributed, "hello");
    }
}
