

































pub mod types;
pub mod core;
pub mod features;
pub mod utils;
pub mod bindings;

#[cfg(feature = "cli")]
pub mod cli;


pub use types::{
    Row, TableConfig, TableUserConfig, TableResult, TableError,
    ColumnConfig, ColumnUserConfig, StreamConfig, StreamUserConfig,
    BorderConfig, BorderUserConfig, Alignment, VerticalAlignment,
    SpanningCellConfig, CellConfig, CellUserConfig, CellCoordinates,
    RangeCoordinate, RangeConfig
};


pub use core::renderer::draw_table;
pub use core::validator::{validate_table_data_with_config, validate_config};


pub use features::spanning::*;
pub use features::streaming::*;
pub use features::borders::*;
pub use features::alignment_processor::*;


pub use utils::formatting::{stringify_table_data, calculate_maximum_column_widths};
pub use utils::wrapping::{wrap_text, calculate_cell_height};
pub use utils::ansi::*;
pub use utils::unicode::*;

























pub fn table(data: &[Row], user_config: Option<&TableUserConfig>) -> TableResult<String> {
    if data.is_empty() {
        return Ok(String::new());
    }

    
    let string_data = stringify_table_data(data)?;
    
    
    let config = if let Some(user_config) = user_config {
        user_config.clone().merge_with_default(&TableConfig::default())
    } else {
        create_default_config(&string_data)
    };

    
    validate_config(&config)?;
    
    
    validate_table_data_with_config(&string_data, &config)?;
    
    
    let result = draw_table(&string_data, &config);
    
    Ok(result)
}


fn create_default_config(data: &[Row]) -> TableConfig {
    let mut config = TableConfig::default();
    
    if !data.is_empty() {
        let column_count = data[0].len();
        config.columns = vec![ColumnConfig::default(); column_count];
    }
    
    config
}








pub fn get_border_characters(name: &str) -> TableResult<BorderConfig> {
    types::borders::get_border_characters(name)
}










#[cfg(test)]
pub fn benchmark_wrap_text(text: &str, width: usize, iterations: usize) -> u128 {
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _result = wrap_text(text, width, false);
    }
    start.elapsed().as_millis()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_basic() {
        let data = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
        ];
        
        
        let column_widths = crate::utils::formatting::calculate_maximum_column_widths(&data);
        println!("Column widths: {:?}", column_widths);
        
        let result = table(&data, None).unwrap();
        println!("Table output: '{}'", result);
        println!("Length: {}", result.len());
        assert!(result.contains("a"));
        assert!(result.contains("b"));
        assert!(result.contains("c"));
        assert!(result.contains("d"));
        assert!(result.contains("┌"));
        assert!(result.contains("└"));
    }

    #[test]
    fn test_table_empty() {
        let data: Vec<Vec<String>> = vec![];
        let result = table(&data, None).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_table_with_config() {
        let data = vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["John".to_string(), "30".to_string()],
        ];
        
        let config = TableUserConfig {
            border: None,
            columns: None,
            column_default: None,
            single_line: Some(false),
            spanning_cells: None,
            header: None,
        };
        
        let result = table(&data, Some(&config)).unwrap();
        assert!(result.contains("Name"));
        assert!(result.contains("Age"));
        assert!(result.contains("John"));
        assert!(result.contains("30"));
    }

    #[test]
    fn test_get_border_characters() {
        let border = get_border_characters("honeywell").unwrap();
        assert_eq!(border.top_left, "┌");
        assert_eq!(border.top_right, "┐");
        
        let ramac_border = get_border_characters("ramac").unwrap();
        assert_eq!(ramac_border.top_left, "+");
        assert_eq!(ramac_border.top_right, "+");
        
        let void_border = get_border_characters("void").unwrap();
        assert_eq!(void_border.top_left, "");
        assert_eq!(void_border.body_join, " ");
        
        assert!(get_border_characters("invalid").is_err());
    }

    #[test]
    fn test_ansi_sequences_in_wrapped_cells() {
        
        let data = vec![
            vec!["Header".to_string()],
            vec!["\u{1b}[31mRed text\u{1b}[0m that should be wrapped".to_string()],
        ];
        
        let config = TableUserConfig {
            columns: Some(vec![
                ColumnUserConfig {
                    width: Some(10), 
                    wrap_word: Some(false), 
                    ..Default::default()
                }
            ]),
            ..Default::default()
        };
        
        let result = table(&data, Some(&config)).unwrap();
        println!("ANSI wrapped table result:");
        println!("{}", result);
        
        
        assert!(result.contains("\u{1b}[31m"), "Should contain red color start sequence");
        assert!(result.contains("\u{1b}[0m"), "Should contain reset sequence");
        
        
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() > 3, "Should have multiple lines due to wrapping");
        
        
        let content_lines: Vec<&str> = lines.iter()
            .filter(|line| line.contains("Red") || line.contains("text") || line.contains("that"))
            .cloned()
            .collect();
        
        println!("Content lines: {:?}", content_lines);
        assert!(!content_lines.is_empty(), "Should have content lines with text");
    }

    #[test]
    fn test_ansi_sequences_wrap_preservation() {
        
        let ansi_text = "\u{1b}[31mRed text\u{1b}[0m that should be wrapped";
        
        
        let wrapped = wrap_text(ansi_text, 10, false);
        println!("Wrapped ANSI text: {:?}", wrapped);
        
        
        assert!(wrapped.len() > 1, "Should wrap into multiple lines");
        
        
        for line in &wrapped {
            let display_width = crate::utils::ansi::calculate_display_width(line);
            assert!(display_width <= 10, "Line '{}' has display width {} > 10", line, display_width);
        }
        
        
        let full_content = wrapped.join("");
        assert!(full_content.contains("\u{1b}[31m"), "Should preserve red color start");
        assert!(full_content.contains("\u{1b}[0m"), "Should preserve reset sequence");
    }

    #[test]
    fn test_ansi_sequence_continuation() {
        
        let data = vec![
            vec!["\u{1b}[31mThis is a very long red text that should be wrapped\u{1b}[0m".to_string()],
        ];
        
        let config = TableUserConfig {
            columns: Some(vec![
                ColumnUserConfig {
                    width: Some(5), 
                    wrap_word: Some(false), 
                    ..Default::default()
                }
            ]),
            ..Default::default()
        };
        
        let result = table(&data, Some(&config)).unwrap();
        println!("ANSI continuation test result:");
        println!("{}", result);
        
        
        assert!(result.contains("\u{1b}[31m"), "Should contain red color start sequence");
        assert!(result.contains("\u{1b}[0m"), "Should contain reset sequence");
        
        
        let lines: Vec<&str> = result.lines().collect();
        let content_lines: Vec<&str> = lines.iter()
            .filter(|line| !line.starts_with("┌") && !line.starts_with("├") && !line.starts_with("└"))
            .cloned()
            .collect();
        
        println!("Content lines count: {}", content_lines.len());
        assert!(content_lines.len() > 3, "Should have multiple content lines due to wrapping");
    }

    #[test]
    fn test_complex_ansi_lorem_ipsum_table() {
        
        let data = vec![
            
            vec![
                "\u{1b}[1;4mName\u{1b}[0m".to_string(),                    
                "\u{1b}[32mDescription\u{1b}[0m".to_string(),              
                "\u{1b}[33mCategory\u{1b}[0m".to_string(),                 
                "\u{1b}[36mStatus\u{1b}[0m".to_string(),                   
            ],
            
            vec![
                "\u{1b}[31mAlice\u{1b}[0m".to_string(),                   
                "\u{1b}[34mLorem ipsum\u{1b}[0m".to_string(),             
                "\u{1b}[35mA\u{1b}[0m".to_string(),                       
                "\u{1b}[32m✓\u{1b}[0m".to_string(),                       
            ],
            
            vec![
                "\u{1b}[37;41mBob Smith\u{1b}[0m".to_string(),            
                "\u{1b}[30;47mLorem ipsum dolor sit amet\u{1b}[0m".to_string(), 
                "\u{1b}[33;44mCategory B\u{1b}[0m".to_string(),           
                "\u{1b}[31m✗\u{1b}[0m".to_string(),                       
            ],
            
            vec![
                "\u{1b}[1;32mCharlotte\u{1b}[0m".to_string(),             
                "\u{1b}[34mLorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua\u{1b}[0m".to_string(), 
                "\u{1b}[35mVery Long Category Name\u{1b}[0m".to_string(),  
                "\u{1b}[33m⚠\u{1b}[0m".to_string(),                       
            ],
            
            vec![
                "\u{1b}[31mDavid\u{1b}[0m \u{1b}[32mJones\u{1b}[0m".to_string(), 
                "\u{1b}[1mUt enim ad\u{1b}[0m \u{1b}[3mminim veniam\u{1b}[0m, \u{1b}[4mquis nostrud\u{1b}[0m exercitation".to_string(), 
                "\u{1b}[36mC\u{1b}[31m-\u{1b}[32mMix\u{1b}[0m".to_string(), 
                "\u{1b}[32m✓\u{1b}[0m\u{1b}[33m!\u{1b}[0m".to_string(),   
            ],
            
            vec![
                "".to_string(),                                             
                "\u{1b}[37mQt\u{1b}[0m".to_string(),                      
                "\u{1b}[90mNull\u{1b}[0m".to_string(),                    
                "\u{1b}[2m-\u{1b}[0m".to_string(),                        
            ],
        ];
        
        let config = TableUserConfig {
            columns: Some(vec![
                ColumnUserConfig {
                    width: Some(12),
                    wrap_word: Some(true), 
                    ..Default::default()
                },
                ColumnUserConfig {
                    width: Some(25),
                    wrap_word: Some(true), 
                    ..Default::default()
                },
                ColumnUserConfig {
                    width: Some(15),
                    wrap_word: Some(false), 
                    ..Default::default()
                },
                ColumnUserConfig {
                    width: Some(8),
                    wrap_word: Some(true), 
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };
        
        let result = table(&data, Some(&config)).unwrap();
        println!("Complex ANSI Lorem Ipsum table:");
        println!("{}", result);
        
        
        assert!(!result.is_empty(), "Table should not be empty");
        assert!(result.contains("┌"), "Should contain top-left border");
        assert!(result.contains("└"), "Should contain bottom-left border");
        assert!(result.contains("│"), "Should contain vertical borders");
        assert!(result.contains("├"), "Should contain row separators");
        
        
        assert!(result.contains("\u{1b}[1;4m"), "Should contain bold+underline sequence");
        assert!(result.contains("\u{1b}[32m"), "Should contain green color");
        assert!(result.contains("\u{1b}[33m"), "Should contain yellow color");
        assert!(result.contains("\u{1b}[36m"), "Should contain cyan color");
        assert!(result.contains("\u{1b}[31m"), "Should contain red color");
        assert!(result.contains("\u{1b}[34m"), "Should contain blue color");
        assert!(result.contains("\u{1b}[35m"), "Should contain magenta color");
        assert!(result.contains("\u{1b}[37;41m"), "Should contain white on red background");
        assert!(result.contains("\u{1b}[30;47m"), "Should contain black on white background");
        assert!(result.contains("\u{1b}[0m"), "Should contain reset sequences");
        
        
        assert!(result.contains("Name"), "Should contain header 'Name'");
        assert!(result.contains("Description"), "Should contain header 'Description'");
        assert!(result.contains("Alice"), "Should contain 'Alice'");
        assert!(result.contains("Bob Smith"), "Should contain 'Bob Smith'");
        assert!(result.contains("Charlotte"), "Should contain 'Charlotte'");
        assert!(result.contains("David"), "Should contain 'David'");
        assert!(result.contains("Jones"), "Should contain 'Jones'");
        assert!(result.contains("Lorem ipsum"), "Should contain 'Lorem ipsum'");
        assert!(result.contains("dolor sit") || result.contains("dolor"), "Should contain 'dolor' (may be wrapped)");
        assert!(result.contains("amet"), "Should contain 'amet' (may be wrapped)");
        assert!(result.contains("consectetur"), "Should contain 'consectetur'");
        assert!(result.contains("✓"), "Should contain checkmark symbol");
        assert!(result.contains("✗"), "Should contain X symbol");
        assert!(result.contains("⚠"), "Should contain warning symbol");
        
        
        let lines: Vec<&str> = result.lines().collect();
        println!("Total lines in table: {}", lines.len());
        assert!(lines.len() > 15, "Should have many lines due to text wrapping");
        
        
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("│") {
                
                let separator_count = line.matches("│").count();
                assert!(separator_count >= 4, "Line {} should have at least 4 separators, got {}: '{}'", i, separator_count, line);
            }
        }
        
        
        let content_lines: Vec<&str> = lines.iter()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .take(10) 
            .cloned()
            .collect();
        
        for (i, line) in content_lines.iter().enumerate() {
            
            let parts: Vec<&str> = line.split("│").collect();
            if parts.len() >= 5 { 
                for j in 1..=4 {
                    let col_content = parts[j].trim();
                    let display_width = crate::utils::ansi::calculate_display_width(col_content);
                    let expected_widths = [12, 25, 15, 8];
                    assert!(display_width <= expected_widths[j-1], 
                        "Line {} column {} has width {} > expected {}: '{}'", 
                        i, j, display_width, expected_widths[j-1], col_content);
                }
            }
        }
        
        println!("✅ Complex ANSI Lorem Ipsum table test passed!");
    }

    #[test]
    fn test_vertical_alignment_with_bottom_padding() {
        
        let data = vec![
            vec!["Short".to_string(), "Medium content here".to_string()],
            vec!["A".to_string(), "Very long content that will wrap to multiple lines in this cell".to_string()],
        ];
        
        let config = TableUserConfig {
            columns: Some(vec![
                ColumnUserConfig {
                    width: Some(10),
                    vertical_alignment: Some(VerticalAlignment::Bottom),
                    ..Default::default()
                },
                ColumnUserConfig {
                    width: Some(15),
                    vertical_alignment: Some(VerticalAlignment::Top),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };
        
        let result = table(&data, Some(&config)).unwrap();
        println!("Vertical alignment test result:");
        println!("{}", result);
        
        
        assert!(!result.is_empty(), "Table should not be empty");
        assert!(result.contains("Short"), "Should contain 'Short'");
        assert!(result.contains("Medium"), "Should contain 'Medium' (may be wrapped)");
        assert!(result.contains("content"), "Should contain 'content'");
        
        
        let lines: Vec<&str> = result.lines().collect();
        println!("Total lines: {}", lines.len());
        assert!(lines.len() > 5, "Should have multiple lines due to wrapping and alignment");
        
        
        
        let content_lines: Vec<&str> = lines.iter()
            .filter(|line| line.starts_with("│") && !line.contains("─"))
            .cloned()
            .collect();
            
        println!("Content lines:");
        for (i, line) in content_lines.iter().enumerate() {
            println!("  {}: {}", i, line);
        }
    }

    #[test]
    fn test_bottom_padding_adds_newlines() {
        
        use crate::core::processor::apply_vertical_alignment;
        use crate::types::{TableConfig, ColumnConfig, VerticalAlignment};
        
        
        let _row_data = vec![
            vec!["Line1".to_string()],  
        ];
        
        
        let wrapped_data = vec![
            vec![
                vec!["Line1".to_string()],  
            ]
        ];
        
        let heights = vec![4];  
        
        let mut config = TableConfig::default();
        config.columns = vec![ColumnConfig {
            vertical_alignment: VerticalAlignment::Bottom,
            ..Default::default()
        }];
        
        let result = apply_vertical_alignment(&wrapped_data, &heights, &config);
        
        println!("Bottom padding test result:");
        for (i, row) in result.iter().enumerate() {
            println!("  Row {}: {:?}", i, row);
        }
        
        
        assert_eq!(result.len(), 4, "Should have 4 rows total");
        
        
        assert_eq!(result[0], vec![""], "First row should be empty (newline)");
        assert_eq!(result[1], vec![""], "Second row should be empty (newline)");
        assert_eq!(result[2], vec![""], "Third row should be empty (newline)");
        
        
        assert_eq!(result[3], vec!["Line1"], "Last row should have content");
    }

    #[test]
    fn test_middle_padding_adds_newlines() {
        
        use crate::core::processor::apply_vertical_alignment;
        use crate::types::{TableConfig, ColumnConfig, VerticalAlignment};
        
        let wrapped_data = vec![
            vec![
                vec!["Line1".to_string()],  
            ]
        ];
        
        let heights = vec![5];  
        
        let mut config = TableConfig::default();
        config.columns = vec![ColumnConfig {
            vertical_alignment: VerticalAlignment::Middle,
            ..Default::default()
        }];
        
        let result = apply_vertical_alignment(&wrapped_data, &heights, &config);
        
        println!("Middle padding test result:");
        for (i, row) in result.iter().enumerate() {
            println!("  Row {}: {:?}", i, row);
        }
        
        
        assert_eq!(result.len(), 5, "Should have 5 rows total");
        
        
        
        assert_eq!(result[0], vec![""], "First row should be empty (top padding)");
        assert_eq!(result[1], vec![""], "Second row should be empty (top padding)");
        assert_eq!(result[2], vec!["Line1"], "Third row should have content");
        assert_eq!(result[3], vec![""], "Fourth row should be empty (bottom padding)");
        assert_eq!(result[4], vec![""], "Fifth row should be empty (bottom padding)");
    }

    #[test]
    fn test_textwrap_performance() {
        let long_text = "a".repeat(1000);
        let time_ms = benchmark_wrap_text(&long_text, 3, 100); 
        println!("100 wrap operations took {}ms", time_ms);
        
        
        assert!(time_ms < 1000, "Textwrap should be reasonably fast, took {}ms", time_ms);
    }

    #[test]
    fn test_table_with_header() {
        let data = vec![
            vec!["Name".to_string(), "Age".to_string(), "City".to_string()],
            vec!["Alice".to_string(), "30".to_string(), "New York".to_string()],
            vec!["Bob".to_string(), "25".to_string(), "London".to_string()],
        ];
        
        let config = TableUserConfig {
            header: Some(Box::new(TableUserConfig {
                border: Some(BorderUserConfig {
                    top_body: Some("═".to_string()),
                    top_join: Some("╦".to_string()),
                    top_left: Some("╔".to_string()),
                    top_right: Some("╗".to_string()),
                    bottom_body: Some("═".to_string()),
                    bottom_join: Some("╩".to_string()),
                    bottom_left: Some("╚".to_string()),
                    bottom_right: Some("╝".to_string()),
                    body_left: Some("║".to_string()),
                    body_right: Some("║".to_string()),
                    body_join: Some("║".to_string()),
                    header_join: Some("═".to_string()),
                    join_body: Some("─".to_string()),
                    join_left: Some("╠".to_string()),
                    join_right: Some("╣".to_string()),
                    join_join: Some("╬".to_string()),
                }),
                columns: None,
                column_default: None,
                single_line: None,
                spanning_cells: None,
                header: None,
            })),
            ..Default::default()
        };
        
        let result = table(&data, Some(&config)).unwrap();
        println!("Table with header result:");
        println!("{}", result);
        
        
        assert!(result.contains("Name"));
        assert!(result.contains("Age"));
        assert!(result.contains("City"));
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
        assert!(result.contains("New York"));
        assert!(result.contains("London"));
        
        
        println!("Looking for header separator character '═' in output...");
        let lines: Vec<&str> = result.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            println!("Line {}: {}", i, line);
            if line.contains("═") {
                println!("Found header separator at line {}", i);
            }
        }
        assert!(result.contains("═"), "Header separator '═' not found in output");
        
        
        assert!(result.contains("║"));
        assert!(result.contains("╔"));
        assert!(result.contains("╗"));
    }

    #[test]
    fn test_table_with_header_different_styles() {
        
        let data = vec![
            vec!["ID".to_string(), "Product".to_string(), "Price".to_string()],
            vec!["1".to_string(), "Laptop".to_string(), "$999".to_string()],
            vec!["2".to_string(), "Mouse".to_string(), "$25".to_string()],
            vec!["3".to_string(), "Keyboard".to_string(), "$75".to_string()],
        ];
        
        
        let config = TableUserConfig {
            header: Some(Box::new(TableUserConfig {
                border: Some(BorderUserConfig {
                    top_body: Some("=".to_string()),
                    top_join: Some("+".to_string()),
                    top_left: Some("+".to_string()),
                    top_right: Some("+".to_string()),
                    bottom_body: Some("=".to_string()),
                    bottom_join: Some("+".to_string()),
                    bottom_left: Some("+".to_string()),
                    bottom_right: Some("+".to_string()),
                    body_left: Some("|".to_string()),
                    body_right: Some("|".to_string()),
                    body_join: Some("|".to_string()),
                    header_join: Some("=".to_string()),
                    join_body: Some("-".to_string()),
                    join_left: Some("+".to_string()),
                    join_right: Some("+".to_string()),
                    join_join: Some("+".to_string()),
                }),
                columns: None,
                column_default: None,
                single_line: None,
                spanning_cells: None,
                header: None,
            })),
            ..Default::default()
        };
        
        let result = table(&data, Some(&config)).unwrap();
        println!("Table with ramac-style header result:");
        println!("{}", result);
        
        
        assert!(result.contains("ID"));
        assert!(result.contains("Product"));
        assert!(result.contains("Price"));
        assert!(result.contains("Laptop"));
        assert!(result.contains("Mouse"));
        assert!(result.contains("Keyboard"));
        
        
        assert!(result.contains("="), "Header separator '=' not found");
        assert!(result.contains("+"), "Header corners '+' not found");
        
        
        let lines: Vec<&str> = result.lines().collect();
        let header_sep_line = lines.iter().find(|line| line.contains("=") && line.contains("+"));
        assert!(header_sep_line.is_some(), "Header separator line not found");
        
        
        let body_lines: Vec<&str> = lines.iter()
            .filter(|line| line.contains("Laptop") || line.contains("Mouse") || line.contains("Keyboard"))
            .cloned()
            .collect();
        
        assert!(!body_lines.is_empty(), "Body content lines not found");
        for line in body_lines {
            assert!(line.contains("│"), "Body lines should use regular vertical separators");
        }
    }

    #[test]
    fn test_table_with_header_column_configuration() {
        
        let data = vec![
            vec!["Short".to_string(), "Very Long Header Name".to_string()],
            vec!["A".to_string(), "Some content here".to_string()],
            vec!["B".to_string(), "More content".to_string()],
        ];
        
        let config = TableUserConfig {
            header: Some(Box::new(TableUserConfig {
                border: Some(BorderUserConfig {
                    top_body: Some("═".to_string()),
                    top_join: Some("╤".to_string()),
                    top_left: Some("╒".to_string()),
                    top_right: Some("╕".to_string()),
                    bottom_body: Some("═".to_string()),
                    bottom_join: Some("╧".to_string()),
                    bottom_left: Some("╘".to_string()),
                    bottom_right: Some("╛".to_string()),
                    body_left: Some("│".to_string()),
                    body_right: Some("│".to_string()),
                    body_join: Some("│".to_string()),
                    header_join: Some("═".to_string()),
                    join_body: Some("─".to_string()),
                    join_left: Some("╞".to_string()),
                    join_right: Some("╡".to_string()),
                    join_join: Some("╪".to_string()),
                }),
                columns: Some(vec![
                    ColumnUserConfig {
                        width: Some(8),
                        alignment: Some(crate::types::Alignment::Center),
                        ..Default::default()
                    },
                    ColumnUserConfig {
                        width: Some(20),
                        alignment: Some(crate::types::Alignment::Left),
                        ..Default::default()
                    },
                ]),
                column_default: None,
                single_line: None,
                spanning_cells: None,
                header: None,
            })),
            ..Default::default()
        };
        
        let result = table(&data, Some(&config)).unwrap();
        println!("Table with header column configuration result:");
        println!("{}", result);
        
        
        assert!(result.contains("Short"));
        assert!(result.contains("Very Long Header Name"));
        assert!(result.contains("Some content here"));
        assert!(result.contains("More content"));
        
        
        assert!(result.contains("═"), "Header separator '═' not found");
        assert!(result.contains("╒"), "Header top-left corner '╒' not found");
        assert!(result.contains("╕"), "Header top-right corner '╕' not found");
        assert!(result.contains("╞"), "Header separator left '╞' not found");
        assert!(result.contains("╡"), "Header separator right '╡' not found");
        assert!(result.contains("╪"), "Header separator join '╪' not found");
        
        
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() >= 6, "Table should have at least 6 lines");
        
        
        let header_row = lines.iter().find(|line| line.contains("Short") && line.contains("Very Long Header Name"));
        assert!(header_row.is_some(), "Header row not found");
        
        
        let body_rows: Vec<&str> = lines.iter()
            .filter(|line| line.contains("Some content") || line.contains("More content"))
            .cloned()
            .collect();
        assert_eq!(body_rows.len(), 2, "Should have 2 body rows");
    }

    #[test]
    fn test_table_with_header_no_body_rows() {
        
        let data = vec![
            vec!["Column 1".to_string(), "Column 2".to_string(), "Column 3".to_string()],
        ];
        
        let config = TableUserConfig {
            header: Some(Box::new(TableUserConfig {
                border: Some(BorderUserConfig {
                    top_body: Some("═".to_string()),
                    top_join: Some("╦".to_string()),
                    top_left: Some("╔".to_string()),
                    top_right: Some("╗".to_string()),
                    bottom_body: Some("═".to_string()),
                    bottom_join: Some("╩".to_string()),
                    bottom_left: Some("╚".to_string()),
                    bottom_right: Some("╝".to_string()),
                    body_left: Some("║".to_string()),
                    body_right: Some("║".to_string()),
                    body_join: Some("║".to_string()),
                    header_join: Some("═".to_string()),
                    join_body: Some("─".to_string()),
                    join_left: Some("╠".to_string()),
                    join_right: Some("╣".to_string()),
                    join_join: Some("╬".to_string()),
                }),
                columns: None,
                column_default: None,
                single_line: None,
                spanning_cells: None,
                header: None,
            })),
            ..Default::default()
        };
        
        let result = table(&data, Some(&config)).unwrap();
        println!("Table with header only (no body rows) result:");
        println!("{}", result);
        
        
        assert!(result.contains("Column 1"));
        assert!(result.contains("Column 2"));
        assert!(result.contains("Column 3"));
        
        
        assert!(result.contains("╔"), "Header top-left corner not found");
        assert!(result.contains("╗"), "Header top-right corner not found");
        assert!(result.contains("║"), "Header vertical separators not found");
        
        
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() >= 3, "Should have at least 3 lines (top border, header row, bottom border)");
        
        
        let header_sep_lines: Vec<&str> = lines.iter()
            .filter(|line| line.contains("╠") && line.contains("╣") && line.contains("═"))
            .cloned()
            .collect();
        assert!(header_sep_lines.is_empty(), "Should not have header separator when there are no body rows");
    }

    #[test]
    fn test_table_without_header_configuration() {
        
        let data = vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "25".to_string()],
        ];
        
        let config = TableUserConfig {
            header: None, 
            ..Default::default()
        };
        
        let result = table(&data, Some(&config)).unwrap();
        println!("Table without header configuration result:");
        println!("{}", result);
        
        
        assert!(result.contains("Name"));
        assert!(result.contains("Age"));
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
        
        
        assert!(result.contains("┌"), "Regular top-left corner not found");
        assert!(result.contains("┐"), "Regular top-right corner not found");
        assert!(result.contains("│"), "Regular vertical separators not found");
        assert!(result.contains("├"), "Regular row separators not found");
        assert!(result.contains("└"), "Regular bottom-left corner not found");
        assert!(result.contains("┘"), "Regular bottom-right corner not found");
        
        
        assert!(!result.contains("╔"), "Should not contain header top-left corner");
        assert!(!result.contains("╗"), "Should not contain header top-right corner");
        assert!(!result.contains("║"), "Should not contain header vertical separators");
        assert!(!result.contains("╠"), "Should not contain header row separator left");
        assert!(!result.contains("╣"), "Should not contain header row separator right");
        assert!(!result.contains("═"), "Should not contain header horizontal lines");
        
        
        let lines: Vec<&str> = result.lines().collect();
        let content_lines: Vec<&str> = lines.iter()
            .filter(|line| line.contains("Name") || line.contains("Alice") || line.contains("Bob"))
            .cloned()
            .collect();
        
        assert_eq!(content_lines.len(), 3, "Should have 3 content lines");
        
        
        for line in content_lines {
            assert!(line.contains("│"), "All content lines should use regular vertical separators");
        }
    }

    #[test]
    fn test_simple_processor_debug() {
        
        let data = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["C".to_string(), "D".to_string()],
        ];
        
        let config = TableUserConfig {
            columns: Some(vec![
                ColumnUserConfig {
                    width: Some(5),
                    alignment: Some(crate::types::Alignment::Left),
                    padding_left: Some(1),
                    padding_right: Some(1),
                    ..Default::default()
                },
                ColumnUserConfig {
                    width: Some(5),
                    alignment: Some(crate::types::Alignment::Right),
                    padding_left: Some(1),
                    padding_right: Some(1),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };
        
        let result = table(&data, Some(&config)).unwrap();
        println!("Simple processor debug result:");
        println!("{}", result);
        
        
        assert!(result.contains("A"));
        assert!(result.contains("B"));
        assert!(result.contains("C"));
        assert!(result.contains("D"));
        assert!(result.contains("┌"));
        assert!(result.contains("┐"));
        assert!(result.contains("└"));
        assert!(result.contains("┘"));
    }

    #[test]
    fn test_inconsistent_column_widths_bug() {
        
        
        let data = vec![
            vec!["Header 1".to_string(), "Header 2".to_string(), "Header 3".to_string()],
            vec!["Row 1 Col 1".to_string(), "Row 1 Col 2 -".to_string(), "Row 1 Col 3 --".to_string()],
            vec!["Row 2 Col 1 -".to_string(), "Row 2 Col 2 --".to_string(), "Row 2 Col 3 ---".to_string()],
        ];
        
        let config = TableUserConfig {
            header: Some(Box::new(TableUserConfig {
                border: Some(BorderUserConfig {
                    top_body: Some("═".to_string()),
                    top_join: Some("╦".to_string()),
                    top_left: Some("╔".to_string()),
                    top_right: Some("╗".to_string()),
                    bottom_body: Some("═".to_string()),
                    bottom_join: Some("╩".to_string()),
                    bottom_left: Some("╚".to_string()),
                    bottom_right: Some("╝".to_string()),
                    body_left: Some("║".to_string()),
                    body_right: Some("║".to_string()),
                    body_join: Some("║".to_string()),
                    header_join: Some("═".to_string()),
                    join_body: Some("─".to_string()),
                    join_left: Some("╠".to_string()),
                    join_right: Some("╣".to_string()),
                    join_join: Some("╬".to_string()),
                }),
                column_default: Some(ColumnUserConfig {
                    padding_left: Some(1),
                    padding_right: Some(1),
                    ..Default::default()
                }),
                ..Default::default()
            })),
            ..Default::default()
        };
        
        let result = table(&data, Some(&config)).unwrap();
        println!("Column width consistency test result:");
        println!("{}", result);
        
        
        let lines: Vec<&str> = result.lines().collect();
        
        
        let border_lines: Vec<&str> = lines.iter()
            .filter(|line| line.contains("═") || line.contains("─"))
            .cloned()
            .collect();
        
        println!("Border lines:");
        for (i, line) in border_lines.iter().enumerate() {
            println!("  {}: {}", i, line);
        }
        
        
        if border_lines.len() >= 2 {
            let first_line_len = border_lines[0].len();
            for (i, line) in border_lines.iter().enumerate() {
                println!("Border line {} length: {}", i, line.len());
                assert_eq!(line.len(), first_line_len, 
                    "Border line {} has different length {} vs expected {}", 
                    i, line.len(), first_line_len);
            }
        }
        
        
        let content_lines: Vec<&str> = lines.iter()
            .filter(|line| line.contains("║") || line.contains("│"))
            .filter(|line| !line.contains("═") && !line.contains("─"))
            .cloned()
            .collect();
        
        println!("Content lines:");
        for (i, line) in content_lines.iter().enumerate() {
            println!("  {}: {}", i, line);
        }
        
        
        if content_lines.len() >= 2 {
            let first_content_len = content_lines[0].len();
            for (i, line) in content_lines.iter().enumerate() {
                println!("Content line {} length: {}", i, line.len());
                assert_eq!(line.len(), first_content_len,
                    "Content line {} has different length {} vs expected {}",
                    i, line.len(), first_content_len);
            }
        }
        
        
        assert!(result.contains("Header 1"));
        assert!(result.contains("Row 1 Col 1"));
        assert!(result.contains("Row 2 Col 1"));
        assert!(result.contains("═"), "Should contain header borders");
        assert!(result.contains("║"), "Should contain header vertical borders");
    }

    #[test]
    fn test_processor_functions_integration() {
        
        let data = vec![
            vec!["Left".to_string(), "Center".to_string(), "Right".to_string(), "VeryLongTextThatNeedsTruncation".to_string()],
            vec!["A".to_string(), "B".to_string(), "C".to_string(), "AnotherLongTextExample".to_string()],
        ];
        
        let config = TableUserConfig {
            columns: Some(vec![
                ColumnUserConfig {
                    width: Some(8),
                    alignment: Some(crate::types::Alignment::Left),
                    padding_left: Some(2),
                    padding_right: Some(1),
                    truncate: Some(0), 
                    ..Default::default()
                },
                ColumnUserConfig {
                    width: Some(10),
                    alignment: Some(crate::types::Alignment::Center),
                    padding_left: Some(1),
                    padding_right: Some(1),
                    truncate: Some(0), 
                    ..Default::default()
                },
                ColumnUserConfig {
                    width: Some(8),
                    alignment: Some(crate::types::Alignment::Right),
                    padding_left: Some(1),
                    padding_right: Some(2),
                    truncate: Some(0), 
                    ..Default::default()
                },
                ColumnUserConfig {
                    width: Some(12),
                    alignment: Some(crate::types::Alignment::Left),
                    padding_left: Some(1),
                    padding_right: Some(1),
                    truncate: Some(8), 
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };
        
        let result = table(&data, Some(&config)).unwrap();
        println!("Processor functions integration test result:");
        println!("{}", result);
        
        
        assert!(result.contains("Left"));
        assert!(result.contains("Center"));
        assert!(result.contains("Right"));
        assert!(result.contains("VeryLong") || result.contains("VeryL...")); 
        assert!(result.contains("A"));
        assert!(result.contains("B"));
        assert!(result.contains("C"));
        assert!(result.contains("AnotherL") || result.contains("Anoth...")); 
        
        
        assert!(result.contains("┌"), "Should have top-left border");
        assert!(result.contains("┐"), "Should have top-right border");
        assert!(result.contains("└"), "Should have bottom-left border");
        assert!(result.contains("┘"), "Should have bottom-right border");
        assert!(result.contains("│"), "Should have vertical separators");
        assert!(result.contains("├"), "Should have row separators");
        
        
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() >= 5, "Should have at least 5 lines (borders + content)");
        
        
        let header_line = lines.iter().find(|line| line.contains("Left"));
        assert!(header_line.is_some(), "Should find header line with 'Left'");
        let header_line = header_line.unwrap();
        let column_count = header_line.matches("│").count();
        assert_eq!(column_count, 5, "Should have 5 vertical separators (4 columns + borders)");
        
        
        for line in lines.iter().filter(|line| line.contains("│") && !line.contains("─")) {
            
            let segments: Vec<&str> = line.split("│").collect();
            if segments.len() >= 5 {
                
                assert_eq!(segments[1].len(), 8, "First column should be 8 characters wide");
                assert_eq!(segments[2].len(), 10, "Second column should be 10 characters wide");
                assert_eq!(segments[3].len(), 8, "Third column should be 8 characters wide");
                assert_eq!(segments[4].len(), 12, "Fourth column should be 12 characters wide");
            }
        }
        
        println!("✅ Processor functions integration test passed!");
    }

    #[test]
    fn test_streaming_example() {
        use crate::features::streaming::create_string_stream;
        
        println!("=== Streaming Table Example ===");
        
        let mut stream = create_string_stream(None);
        
        
        let header = vec!["ID".to_string(), "Name".to_string(), "Status".to_string()];
        let header_output = stream.write_row(&header).unwrap();
        println!("Header row:");
        println!("{}", header_output);
        
        
        let data_rows = vec![
            vec!["1".to_string(), "Alice".to_string(), "Active".to_string()],
            vec!["2".to_string(), "Bob".to_string(), "Inactive".to_string()],
            vec!["3".to_string(), "Charlie".to_string(), "Pending".to_string()],
        ];
        
        for (i, row) in data_rows.iter().enumerate() {
            let row_output = stream.write_row(row).unwrap();
            println!("Row {}:", i + 1);
            println!("{}", row_output);
        }
        
        
        let footer = stream.finalize();
        println!("Footer:");
        println!("{}", footer);
        
        println!("=== Streaming Complete ===");
    }

    #[test]
    fn test_streaming_with_config() {
        use crate::features::streaming::create_string_stream;
        
        println!("=== Streaming with Configuration Example ===");
        
        let config = crate::types::StreamUserConfig {
            columns: Some(vec![
                ColumnUserConfig {
                    width: Some(4),
                    alignment: Some(crate::types::Alignment::Center),
                    ..Default::default()
                },
                ColumnUserConfig {
                    width: Some(12),
                    alignment: Some(crate::types::Alignment::Left),
                    ..Default::default()
                },
                ColumnUserConfig {
                    width: Some(8),
                    alignment: Some(crate::types::Alignment::Right),
                    ..Default::default()
                },
            ]),
            border: None,
            column_default: None,
            single_line: None,
        };
        
        let mut stream = create_string_stream(Some(config));
        
        
        let streaming_data = vec![
            vec!["ID".to_string(), "Product".to_string(), "Price".to_string()],
            vec!["1".to_string(), "Laptop".to_string(), "$999".to_string()],
            vec!["2".to_string(), "Mouse".to_string(), "$25".to_string()],
            vec!["3".to_string(), "Monitor".to_string(), "$299".to_string()],
        ];
        
        for (i, row) in streaming_data.iter().enumerate() {
            let output = stream.write_row(row).unwrap();
            if i == 0 {
                println!("🎯 Header (centered ID, left-aligned Product, right-aligned Price):");
            } else {
                println!("📦 Product {}:", i);
            }
            println!("{}", output);
        }
        
        let footer = stream.finalize();
        println!("🏁 Complete table:");
        println!("{}", footer);
        
        println!("✅ Streaming with configuration complete!");
    }
}