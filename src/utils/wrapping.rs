use textwrap::{wrap, Options, WrapAlgorithm};

pub fn wrap_text(text: &str, width: usize, word_wrap: bool) -> Vec<String> {
    if width == 0 {
        return vec![];
    }

    let options = if word_wrap {
        Options::new(width)
            .break_words(false)
            .wrap_algorithm(WrapAlgorithm::FirstFit)
    } else {
        Options::new(width)
            .break_words(true)
            .wrap_algorithm(WrapAlgorithm::FirstFit)
    };

    let wrapped_lines = wrap(text, options);
    let mut result: Vec<String> = wrapped_lines
        .into_iter()
        .map(|line| line.to_string())
        .collect();

    result = fix_ansi_wrapping(result);

    result
}

fn fix_ansi_wrapping(lines: Vec<String>) -> Vec<String> {
    if lines.len() <= 1 {
        return lines;
    }

    let mut result = Vec::new();
    let mut active_sequences = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let mut processed_line = line.clone();

        if !active_sequences.is_empty() {
            processed_line = format!("{}{}", active_sequences.join(""), processed_line);
        }

        let sequences = extract_ansi_sequences(&processed_line);
        update_active_sequences(&mut active_sequences, &sequences);

        if i < lines.len() - 1 && !active_sequences.is_empty() {
            processed_line = format!("{}\u{1b}[0m", processed_line);
        }

        result.push(processed_line);
    }

    result
}

fn extract_ansi_sequences(text: &str) -> Vec<String> {
    let mut sequences = Vec::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' {
            if let Some(&'[') = chars.peek() {
                let mut sequence = String::from("\u{1b}[");
                chars.next();

                for ch in chars.by_ref() {
                    sequence.push(ch);
                    if ch.is_ascii_alphabetic() {
                        break;
                    }
                }

                sequences.push(sequence);
            }
        }
    }

    sequences
}

fn update_active_sequences(active: &mut Vec<String>, new_sequences: &[String]) {
    for seq in new_sequences {
        if seq.ends_with('m') {
            if seq == "\u{1b}[0m" {
                active.clear();
            } else if !active.contains(seq) {
                active.push(seq.clone());
            }
        }
    }
}

pub fn wrap_cell(text: &str, width: usize, word_wrap: bool) -> Vec<String> {
    wrap_text(text, width, word_wrap)
}

pub fn calculate_cell_height(text: &str, width: usize, word_wrap: bool) -> usize {
    let wrapped = wrap_text(text, width, word_wrap);
    wrapped.len().max(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_text_by_words() {
        let text = "This is a long line that should be wrapped";
        let result = wrap_text(text, 10, true);
        assert_eq!(
            result,
            vec!["This is a", "long line", "that", "should be", "wrapped"]
        );
    }

    #[test]
    fn test_wrap_text_by_characters() {
        let text = "verylongwordthatshouldbewrapped";
        let result = wrap_text(text, 10, false);
        assert_eq!(result, vec!["verylongwo", "rdthatshou", "ldbewrappe", "d"]);
    }

    #[test]
    fn test_wrap_cell() {
        let text = "hello world";
        let result = wrap_cell(text, 5, true);
        assert_eq!(result, vec!["hello", "world"]);
    }

    #[test]
    fn test_wrap_cell_empty() {
        let text = "";
        let result = wrap_cell(text, 5, true);
        assert_eq!(result, vec![""]);
    }

    #[test]
    fn test_calculate_cell_height() {
        let text = "This is a long line that should be wrapped";
        let height = calculate_cell_height(text, 10, true);
        assert_eq!(height, 5);
    }

    #[test]
    fn test_wrap_text_with_ansi() {
        let text = "\u{1b}[31mRed text\u{1b}[0m that should be wrapped";
        let result = wrap_text(text, 10, true);

        assert!(!result.is_empty());
    }

    #[test]
    fn test_ansi_wrapping_fix() {
        let text = "\u{1b}[31mThis is a long red text that should be wrapped\u{1b}[0m";
        let result = wrap_text(text, 10, true);

        println!("ANSI wrapping test result: {:?}", result);

        assert!(result.len() > 1, "Should wrap into multiple lines");

        for (i, line) in result.iter().enumerate() {
            if i < result.len() - 1 {
                assert!(
                    line.ends_with("\u{1b}[0m"),
                    "Line {} should end with reset: '{}'",
                    i,
                    line
                );
            }
        }

        for (i, line) in result.iter().enumerate() {
            if i > 0 {
                assert!(
                    line.starts_with("\u{1b}[31m"),
                    "Line {} should start with red color: '{}'",
                    i,
                    line
                );
            }
        }
    }

    #[test]
    fn test_multiple_ansi_sequences() {
        let text = "\u{1b}[31m\u{1b}[1mBold red text that should be wrapped\u{1b}[0m";
        let result = wrap_text(text, 10, true);

        println!("Multiple ANSI sequences test result: {:?}", result);

        assert!(result.len() > 1, "Should wrap into multiple lines");

        for (i, line) in result.iter().enumerate() {
            if i < result.len() - 1 {
                assert!(
                    line.ends_with("\u{1b}[0m"),
                    "Line {} should end with reset: '{}'",
                    i,
                    line
                );
            }
        }

        for (i, line) in result.iter().enumerate() {
            if i > 0 {
                assert!(
                    line.starts_with("\u{1b}[31m\u{1b}[1m"),
                    "Line {} should start with red+bold: '{}'",
                    i,
                    line
                );
            }
        }
    }

    #[test]
    fn test_wrap_text_with_repeated_newlines() {
        let text = "\n".repeat(10);
        let result = wrap_text(&text, 20, true);

        println!("Repeated newlines test result: {:?}", result);
        println!("Number of lines: {}", result.len());

        assert_eq!(result.len(), 11, "Should have 11 lines for 10 newlines");

        for (i, line) in result.iter().enumerate() {
            println!("Line {}: '{}'", i, line);
            assert_eq!(line, "", "Line {} should be empty", i);
        }
    }
}
