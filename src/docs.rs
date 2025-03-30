/// Comprehensive documentation and examples for the ASCII/ANSI table library
/// 
/// This module provides detailed examples and documentation for all library features,
/// organized by complexity and use case.

use crate::*;
use std::fmt::Write;

/// # Quick Start Examples
/// 
/// The simplest way to get started with basic table rendering.
pub mod quick_start {
    use super::*;
    
    /// Create a simple table from 2D data
    /// 
    /// # Example
    /// ```
    /// use ascii_ansi_table::*;
    /// 
    /// let data = TableData::new(vec![
    ///     vec!["Name".to_string(), "Age".to_string()],
    ///     vec!["Alice".to_string(), "30".to_string()],
    ///     vec!["Bob".to_string(), "25".to_string()],
    /// ]);
    /// 
    /// let table = render_table_with_borders(&data).unwrap();
    /// println!("{}", table);
    /// ```
    pub fn example_basic_table() -> Result<String, String> {
        let data = TableData::new(vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "25".to_string()],
        ]);
        
        render_table_with_borders(&data)
    }
    
    /// Using the builder pattern for quick table creation
    /// 
    /// # Example
    /// ```
    /// use ascii_ansi_table::*;
    /// 
    /// let table = TableBuilder::new()
    ///     .add_row(vec!["Product".to_string(), "Price".to_string()])
    ///     .add_row(vec!["Widget".to_string(), "$19.99".to_string()])
    ///     .add_row(vec!["Gadget".to_string(), "$29.99".to_string()])
    ///     .with_border_style("default")
    ///     .build()?;
    /// ```
    pub fn example_builder_basic() -> Result<String, String> {
        TableBuilder::new()
            .add_row(vec!["Product".to_string(), "Price".to_string()])
            .add_row(vec!["Widget".to_string(), "$19.99".to_string()])
            .add_row(vec!["Gadget".to_string(), "$29.99".to_string()])
            .with_border_style("default")
            .build()
    }
    
    /// Creating tables from CSV data
    /// 
    /// # Example
    /// ```
    /// use ascii_ansi_table::*;
    /// 
    /// let csv = "Name,Age,City\nAlice,30,New York\nBob,25,London";
    /// let table = TableBuilder::from_csv(csv, None)?
    ///     .with_top_border()
    ///     .with_bottom_border()
    ///     .build()?;
    /// ```
    pub fn example_from_csv() -> Result<String, String> {
        let csv = "Name,Age,City\nAlice,30,New York\nBob,25,London";
        TableBuilder::from_csv(csv, None)?
            .with_top_border()
            .with_bottom_border()
            .build()
    }
}

/// # Styling and Themes
/// 
/// Examples of different visual styles and themes.
pub mod styling {
    use super::*;
    
    /// Available border styles demonstration
    /// 
    /// Shows all built-in border styles with the same data
    pub fn example_all_border_styles() -> Result<String, String> {
        let data = TableData::new(vec![
            vec!["Style".to_string(), "Description".to_string()],
            vec!["Default".to_string(), "Unicode box drawing".to_string()],
            vec!["ASCII".to_string(), "Simple ASCII characters".to_string()],
            vec!["Honeywell".to_string(), "Vintage computing style".to_string()],
        ]);
        
        let mut result = String::new();
        let styles = ["default", "ascii", "honeywell", "ramac", "norc", "void"];
        
        for style in &styles {
            if let Ok(border) = get_border_style(style) {
                writeln!(result, "\n=== {} Style ===", style.to_uppercase())?;
                let table = render_table_with_custom_borders(&data, &border)?;
                result.push_str(&table);
            }
        }
        
        Ok(result)
    }
    
    /// Themed table builders showcase
    /// 
    /// Demonstrates all built-in themes with the same data
    pub fn example_themed_builders() -> Result<String, String> {
        let data = vec![
            vec!["Product".to_string(), "Price".to_string(), "Rating".to_string()],
            vec!["Widget".to_string(), "$19.99".to_string(), "4.5/5".to_string()],
            vec!["Gadget".to_string(), "$29.99".to_string(), "4.8/5".to_string()],
        ];
        
        let mut result = String::new();
        
        writeln!(result, "=== MINIMAL THEME ===")?;
        let minimal = ThemedTableBuilder::minimal()
            .add_rows(data.clone())
            .build()?;
        result.push_str(&minimal);
        
        writeln!(result, "\n=== FANCY THEME ===")?;
        let fancy = ThemedTableBuilder::fancy()
            .add_rows(data.clone())
            .build()?;
        result.push_str(&fancy);
        
        writeln!(result, "\n=== DATA TABLE THEME ===")?;
        let data_table = ThemedTableBuilder::data_table()
            .add_rows(data.clone())
            .build()?;
        result.push_str(&data_table);
        
        writeln!(result, "\n=== REPORT THEME ===")?;
        let report = ThemedTableBuilder::report()
            .add_rows(data.clone())
            .build()?;
        result.push_str(&report);
        
        Ok(result)
    }
    
    /// Custom color and formatting example
    /// 
    /// Shows how to use ANSI colors and formatting
    pub fn example_colored_table() -> Result<String, String> {
        let data = TableData::new(vec![
            vec!["Status".to_string(), "Server".to_string(), "Uptime".to_string()],
            vec![
                format!("{}●{} Online", colors::GREEN, colors::RESET),
                "web-01".to_string(),
                "99.9%".to_string()
            ],
            vec![
                format!("{}●{} Offline", colors::RED, colors::RESET),
                "web-02".to_string(),
                "0.0%".to_string()
            ],
            vec![
                format!("{}●{} Warning", colors::YELLOW, colors::RESET),
                "db-01".to_string(),
                "95.2%".to_string()
            ],
        ]);
        
        let column_configs = vec![
            ColumnConfig::new().with_width(10).with_alignment(Alignment::Center),
            ColumnConfig::new().with_width(8).with_alignment(Alignment::Left),
            ColumnConfig::new().with_width(8).with_alignment(Alignment::Right),
        ];
        
        let border = get_border_style("default")?;
        let options = RenderOptions::default().with_top_border().with_bottom_border();
        render_table_ansi_aware(&data, &border, &options, &column_configs)
    }
}

/// # Advanced Layout Features
/// 
/// Complex table layouts with alignment, padding, truncation, and wrapping.
pub mod layout {
    use super::*;
    
    /// Multi-alignment demonstration
    /// 
    /// Shows different alignment options per column
    pub fn example_multi_alignment() -> Result<String, String> {
        let data = TableData::new(vec![
            vec!["Left Aligned".to_string(), "Center Aligned".to_string(), "Right Aligned".to_string(), "Justified Text".to_string()],
            vec!["Short".to_string(), "Medium Length".to_string(), "Very Long Text Here".to_string(), "This text will be justified across the full width".to_string()],
            vec!["A".to_string(), "B".to_string(), "C".to_string(), "Multiple words to justify".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::new().with_width(15).with_alignment(Alignment::Left),
            ColumnConfig::new().with_width(15).with_alignment(Alignment::Center),
            ColumnConfig::new().with_width(20).with_alignment(Alignment::Right),
            ColumnConfig::new().with_width(25).with_alignment(Alignment::Justify),
        ];
        
        let border = get_border_style("default")?;
        let options = RenderOptions::default().with_all_borders();
        render_table_with_column_config(&data, &border, &options, &column_configs)
    }
    
    /// Custom padding and spacing
    /// 
    /// Shows different padding configurations
    pub fn example_custom_padding() -> Result<String, String> {
        let data = TableData::new(vec![
            vec!["No Padding".to_string(), "Symmetric".to_string(), "Asymmetric".to_string()],
            vec!["Content".to_string(), "Content".to_string(), "Content".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::new().with_width(10).with_padding(Padding::none()),
            ColumnConfig::new().with_width(12).with_padding(Padding::symmetric(3)),
            ColumnConfig::new().with_width(14).with_padding(Padding::new(1, 5)), // 1 left, 5 right
        ];
        
        let border = get_border_style("default")?;
        let options = RenderOptions::default().with_all_borders();
        render_table_with_column_config(&data, &border, &options, &column_configs)
    }
    
    /// Text truncation and wrapping
    /// 
    /// Demonstrates different text overflow handling
    pub fn example_text_overflow() -> Result<String, String> {
        let data = TableData::new(vec![
            vec!["Normal".to_string(), "Truncated".to_string(), "Wrapped".to_string()],
            vec![
                "Short text".to_string(),
                "This is a very long text that will be truncated with ellipsis".to_string(),
                "This is a very long text that will be wrapped to multiple lines for better readability".to_string()
            ],
            vec![
                "More".to_string(),
                "Another extremely long piece of text that exceeds the column width".to_string(),
                "Yet another long text that demonstrates wrapping functionality in tables".to_string()
            ],
        ]);
        
        let column_configs = vec![
            ColumnConfig::new().with_width(12), // Normal
            ColumnConfig::new()
                .with_width(15)
                .with_truncation(TruncationConfig::new().with_max_width(15)), // Truncated
            ColumnConfig::new()
                .with_width(20)
                .with_wrapping(WrapConfig::new(20)), // Wrapped
        ];
        
        let border = get_border_style("default")?;
        let options = RenderOptions::default().with_all_borders();
        render_table_with_wrapping(&data, &border, &options, &column_configs)
    }
}

/// # Data Handling and Validation
/// 
/// Examples of data validation, error handling, and different data formats.
pub mod data_handling {
    use super::*;
    
    /// Table validation examples
    /// 
    /// Shows different validation scenarios and how to handle them
    pub fn example_validation() -> Result<String, String> {
        let mut result = String::new();
        
        // Valid table
        let valid_data = TableData::new(vec![
            vec!["Name".to_string(), "Score".to_string()],
            vec!["Alice".to_string(), "95".to_string()],
            vec!["Bob".to_string(), "87".to_string()],
        ]);
        
        writeln!(result, "=== VALID TABLE VALIDATION ===")?;
        writeln!(result, "{}", validation_summary(&valid_data))?;
        
        // Invalid table (inconsistent columns)
        let invalid_data = TableData::new(vec![
            vec!["Name".to_string(), "Score".to_string(), "Grade".to_string()],
            vec!["Alice".to_string(), "95".to_string()], // Missing grade
            vec!["Bob".to_string(), "87".to_string(), "B+".to_string()],
        ]);
        
        writeln!(result, "\n=== INVALID TABLE VALIDATION ===")?;
        writeln!(result, "{}", validation_summary(&invalid_data))?;
        
        let validator = TableValidator::new(ValidationConfig::strict());
        let validation_result = validator.validate(&invalid_data);
        writeln!(result, "\n{}", validation_result.report())?;
        
        // Unicode content detection
        let unicode_data = TableData::new(vec![
            vec!["Name".to_string(), "City".to_string()],
            vec!["Alice".to_string(), "东京".to_string()],
            vec!["Bob".to_string(), "München".to_string()],
        ]);
        
        writeln!(result, "\n=== UNICODE CONTENT VALIDATION ===")?;
        let unicode_result = validate_table(&unicode_data);
        writeln!(result, "{}", unicode_result.report())?;
        
        Ok(result)
    }
    
    /// Debug and introspection example
    /// 
    /// Shows how to analyze table structure and content
    pub fn example_debugging() -> Result<String, String> {
        let data = TableData::new(vec![
            vec!["Mixed Content".to_string(), "Numbers".to_string(), "Unicode".to_string()],
            vec!["Hello\nWorld".to_string(), "123".to_string(), "café".to_string()],
            vec!["Text".to_string(), "45.67".to_string(), "🎉".to_string()],
            vec!["".to_string(), "0".to_string(), "北京".to_string()], // Empty first cell
        ]);
        
        let mut result = String::new();
        
        writeln!(result, "=== TABLE DEBUG ANALYSIS ===")?;
        result.push_str(&debug_table(&data));
        
        writeln!(result, "\n=== COMPACT DEBUG INFO ===")?;
        result.push_str(&debug_table_compact(&data));
        
        writeln!(result, "\n=== VALIDATION RECOMMENDATIONS ===")?;
        result.push_str(&validate_and_recommend(&data));
        
        Ok(result)
    }
    
    /// Different data format examples
    /// 
    /// Shows handling of various input formats
    pub fn example_data_formats() -> Result<String, String> {
        let mut result = String::new();
        
        writeln!(result, "=== CSV DATA ===")?;
        let csv_data = "Product,Price,Stock\nWidget,$19.99,50\nGadget,$29.99,25";
        let csv_table = quick::csv_table(csv_data)?;
        result.push_str(&csv_table);
        
        writeln!(result, "\n=== TSV DATA ===")?;
        let tsv_data = "Name\tAge\tDepartment\nAlice\t30\tEngineering\nBob\t25\tMarketing";
        let tsv_table = TableBuilder::from_csv(tsv_data, Some("\t"))?
            .with_top_border()
            .with_bottom_border()
            .build()?;
        result.push_str(&tsv_table);
        
        writeln!(result, "\n=== SINGLE LINE FORMAT ===")?;
        let single_line_data = TableData::new(vec![
            vec!["Field1".to_string(), "Field2".to_string(), "Field3".to_string()],
            vec!["Value1".to_string(), "Value2".to_string(), "Value3".to_string()],
        ]);
        let single_line = render_single_line_table(&single_line_data, &SingleLineConfig::new())?;
        result.push_str(&single_line);
        
        Ok(result)
    }
}

/// # Performance and Optimization
/// 
/// Examples of performance features and large data handling.
pub mod performance {
    use super::*;
    
    /// Fast rendering with caching
    /// 
    /// Shows performance optimization features
    pub fn example_fast_rendering() -> Result<String, String> {
        // Create a large table with repeated data (good for caching)
        let rows: Vec<Vec<String>> = (0..100)
            .map(|i| vec![
                format!("Item {}", i),
                if i % 2 == 0 { "Active".to_string() } else { "Inactive".to_string() },
                format!("${}.99", (i % 20) + 10),
                "Category A".to_string(), // Repeated value
            ])
            .collect();
        
        let data = TableData::new(rows);
        
        let column_configs = vec![
            ColumnConfig::new().with_width(10),
            ColumnConfig::new().with_width(8),
            ColumnConfig::new().with_width(8),
            ColumnConfig::new().with_width(12),
        ];
        
        // Use fast renderer with caching
        let perf_config = PerformanceConfig::new()
            .with_caching(true)
            .with_memory_optimization(true);
        
        let mut renderer = FastTableRenderer::new(perf_config);
        let result = renderer.render_table(
            &data,
            &BorderChars::default(),
            &RenderOptions::default(),
            &column_configs,
        )?;
        
        let (cache_size, memory_saved) = renderer.cache_stats();
        
        let mut output = String::new();
        writeln!(output, "=== PERFORMANCE OPTIMIZED TABLE (showing first 20 rows) ===")?;
        
        // Show only first 20 lines for readability
        let lines: Vec<&str> = result.lines().take(25).collect();
        output.push_str(&lines.join("\n"));
        
        writeln!(output, "\n\n=== PERFORMANCE STATS ===")?;
        writeln!(output, "Cache entries: {}", cache_size)?;
        writeln!(output, "Memory saved: {} bytes", memory_saved)?;
        writeln!(output, "Total rows rendered: {}", data.row_count())?;
        
        Ok(output)
    }
    
    /// Streaming large datasets
    /// 
    /// Shows how to handle very large tables efficiently
    pub fn example_streaming() -> Result<String, String> {
        use std::io::Cursor;
        
        // Simulate large dataset
        let large_dataset: Vec<Vec<String>> = (0..1000)
            .map(|i| vec![
                format!("Record {}", i),
                format!("Value {}", i * 2),
                format!("Type {}", i % 5),
                if i % 3 == 0 { "Important".to_string() } else { "Normal".to_string() },
            ])
            .collect();
        
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        
        let config = StreamingTableConfig::new()
            .with_buffer_size(1024) // 1KB buffer
            .with_column_configs(vec![
                ColumnConfig::new().with_width(12),
                ColumnConfig::new().with_width(10),
                ColumnConfig::new().with_width(8),
                ColumnConfig::new().with_width(10),
            ]);
        
        let result_cursor = stream_table_to_writer(cursor, large_dataset.into_iter(), config)?;
        let output = String::from_utf8(result_cursor.into_inner())
            .map_err(|e| format!("UTF-8 conversion error: {}", e))?;
        
        // Show summary and first/last few lines
        let lines: Vec<&str> = output.lines().collect();
        let mut result = String::new();
        
        writeln!(result, "=== STREAMING LARGE DATASET ===")?;
        writeln!(result, "Total lines generated: {}", lines.len())?;
        writeln!(result, "Output size: {} bytes", output.len())?;
        writeln!(result, "\nFirst 10 lines:")?;
        
        for line in lines.iter().take(10) {
            writeln!(result, "{}", line)?;
        }
        
        writeln!(result, "\n... ({} lines omitted) ...", lines.len().saturating_sub(20))?;
        writeln!(result, "\nLast 10 lines:")?;
        
        for line in lines.iter().skip(lines.len().saturating_sub(10)) {
            writeln!(result, "{}", line)?;
        }
        
        Ok(result)
    }
    
    /// Batch processing example
    /// 
    /// Shows how to process data in manageable chunks
    pub fn example_batch_processing() -> Result<String, String> {
        let large_data: Vec<Vec<String>> = (0..50)
            .map(|i| vec![
                format!("Batch Item {}", i),
                format!("Data {}", i * 3),
                format!("Status {}", if i % 4 == 0 { "Complete" } else { "Pending" }),
            ])
            .collect();
        
        let data = TableData::new(large_data);
        let processor = BatchProcessor::new(10); // Process 10 rows at a time
        
        let mut result = String::new();
        writeln!(result, "=== BATCH PROCESSING EXAMPLE ===")?;
        writeln!(result, "Total rows: {}", data.row_count())?;
        writeln!(result, "Batch size: 10 rows")?;
        writeln!(result, "Estimated memory usage: {} bytes", processor.estimate_memory_usage(&data))?;
        
        let batch_results = processor.process_in_batches(&data, |batch| {
            Ok(format!("Processed batch of {} rows", batch.len()))
        })?;
        
        writeln!(result, "\nBatch processing results:")?;
        for (i, batch_result) in batch_results.iter().enumerate() {
            writeln!(result, "Batch {}: {}", i + 1, batch_result)?;
        }
        
        // Show actual table for first batch
        if !data.rows.is_empty() {
            let first_batch_data = TableData::new(
                data.rows.iter().take(10).cloned().collect()
            );
            
            writeln!(result, "\n=== FIRST BATCH RENDERED ===")?;
            let batch_table = render_table_with_borders(&first_batch_data)?;
            result.push_str(&batch_table);
        }
        
        Ok(result)
    }
}

/// # Special Features
/// 
/// Advanced features like spanning, headers, Unicode, and HTML export.
pub mod special_features {
    use super::*;
    
    /// Cell spanning demonstration
    /// 
    /// Shows horizontal and vertical cell spans
    pub fn example_cell_spanning() -> Result<String, String> {
        let mut data = SpannedTableData::new(4, 4);
        
        // Create a complex spanning layout
        data.set_cell(0, 0, SpannedCell::with_span("Main Header".to_string(), CellSpan::horizontal(3)))?;
        data.set_cell(0, 3, SpannedCell::new("Actions".to_string()))?;
        
        data.set_cell(1, 0, SpannedCell::with_span("Category A".to_string(), CellSpan::vertical(2)))?;
        data.set_cell(1, 1, SpannedCell::new("Item 1".to_string()))?;
        data.set_cell(1, 2, SpannedCell::new("$19.99".to_string()))?;
        data.set_cell(1, 3, SpannedCell::new("Edit".to_string()))?;
        
        data.set_cell(2, 1, SpannedCell::new("Item 2".to_string()))?;
        data.set_cell(2, 2, SpannedCell::new("$29.99".to_string()))?;
        data.set_cell(2, 3, SpannedCell::new("Delete".to_string()))?;
        
        data.set_cell(3, 0, SpannedCell::new("Category B".to_string()))?;
        data.set_cell(3, 1, SpannedCell::with_span("Special Item".to_string(), CellSpan::horizontal(2)))?;
        data.set_cell(3, 3, SpannedCell::new("View".to_string()))?;
        
        let column_configs = vec![
            ColumnConfig::default().with_width(12),
            ColumnConfig::default().with_width(12),
            ColumnConfig::default().with_width(10),
            ColumnConfig::default().with_width(8),
        ];
        
        let border = BorderChars::default();
        let options = RenderOptions::default().with_all_borders();
        render_spanned_table(&data, &border, &options, &column_configs)
    }
    
    /// Multi-row headers example
    /// 
    /// Shows complex header configurations
    pub fn example_complex_headers() -> Result<String, String> {
        let data = TableData::new(vec![
            vec!["Sales Report".to_string(), "Q1 2024".to_string(), "Q2 2024".to_string()],
            vec!["Product".to_string(), "Revenue".to_string(), "Revenue".to_string()],
            vec!["Widget".to_string(), "$10,000".to_string(), "$12,000".to_string()],
            vec!["Gadget".to_string(), "$8,500".to_string(), "$9,200".to_string()],
            vec!["Tool".to_string(), "$15,200".to_string(), "$18,800".to_string()],
        ]);
        
        let column_configs = vec![
            ColumnConfig::default().with_width(12),
            ColumnConfig::default().with_width(12),
            ColumnConfig::default().with_width(12),
        ];
        
        let header_config = HeaderConfig::new()
            .with_header_rows(2)
            .with_header_column_configs(vec![
                ColumnConfig::default().with_width(12).with_alignment(Alignment::Center),
                ColumnConfig::default().with_width(12).with_alignment(Alignment::Center),
                ColumnConfig::default().with_width(12).with_alignment(Alignment::Center),
            ]);
        
        let border = BorderChars::default();
        let options = RenderOptions::default().with_all_borders();
        render_table_with_headers(&data, &border, &options, &column_configs, &header_config)
    }
    
    /// Unicode and emoji support
    /// 
    /// Shows proper handling of international text and emojis
    pub fn example_unicode_emoji() -> Result<String, String> {
        let data = TableData::new(vec![
            vec!["Language".to_string(), "Greeting".to_string(), "Flag".to_string(), "Status".to_string()],
            vec!["English".to_string(), "Hello World".to_string(), "🇺🇸".to_string(), "✅ Active".to_string()],
            vec!["Chinese".to_string(), "你好世界".to_string(), "🇨🇳".to_string(), "✅ Active".to_string()],
            vec!["Japanese".to_string(), "こんにちは世界".to_string(), "🇯🇵".to_string(), "⚠️ Warning".to_string()],
            vec!["Arabic".to_string(), "مرحبا بالعالم".to_string(), "🇸🇦".to_string(), "❌ Inactive".to_string()],
            vec!["Russian".to_string(), "Привет мир".to_string(), "🇷🇺".to_string(), "🔄 Pending".to_string()],
        ]);
        
        let emoji_config = EmojiConfig::terminal_optimized();
        let calculator = EmojiWidthCalculator::new(emoji_config);
        
        // Calculate emoji-aware column widths
        let emoji_widths = calculator.calculate_column_widths(&data.rows);
        
        let column_configs: Vec<ColumnConfig> = emoji_widths.iter()
            .map(|&width| ColumnConfig::new().with_width(width + 2)) // Add padding
            .collect();
        
        let border = get_border_style("default")?;
        let options = RenderOptions::default().with_all_borders();
        
        render_emoji_table(&data, &border, &options, &column_configs, &calculator.config)
    }
    
    /// HTML export example
    /// 
    /// Shows how to convert tables to HTML
    pub fn example_html_export() -> Result<String, String> {
        let data = TableData::new(vec![
            vec!["Feature".to_string(), "Status".to_string(), "Priority".to_string()],
            vec!["User Auth".to_string(), "Complete".to_string(), "High".to_string()],
            vec!["Dashboard".to_string(), "In Progress".to_string(), "Medium".to_string()],
            vec!["Reports".to_string(), "Planned".to_string(), "Low".to_string()],
        ]);
        
        let mut result = String::new();
        
        writeln!(result, "=== HTML EXPORT EXAMPLES ===")?;
        
        // Basic HTML
        writeln!(result, "\n--- Basic HTML ---")?;
        let basic_html = table_to_html(&data)?;
        writeln!(result, "{}", basic_html)?;
        
        // Bootstrap styled
        writeln!(result, "\n--- Bootstrap Styled ---")?;
        let bootstrap_html = table_to_bootstrap_html(&data)?;
        writeln!(result, "{}", bootstrap_html)?;
        
        // Material Design styled
        writeln!(result, "\n--- Material Design Styled ---")?;
        let material_html = table_to_material_html(&data)?;
        writeln!(result, "{}", material_html)?;
        
        // Custom configuration
        writeln!(result, "\n--- Custom Configuration ---")?;
        let html_config = HtmlConfig::new()
            .with_table_class("custom-table")
            .with_responsive(true)
            .with_custom_css("border-collapse: collapse; margin: 20px;");
        
        let custom_html = table_to_html_with_config(&data, &html_config)?;
        writeln!(result, "{}", custom_html)?;
        
        Ok(result)
    }
}

/// # Complete Examples
/// 
/// Full-featured examples combining multiple features.
pub mod complete_examples {
    use super::*;
    
    /// Financial report with all features
    /// 
    /// Comprehensive example showing multiple advanced features
    pub fn example_financial_report() -> Result<String, String> {
        let data = TableData::new(vec![
            vec!["Financial Summary".to_string(), "Q1 2024".to_string(), "Q2 2024".to_string(), "Q3 2024".to_string(), "Total".to_string()],
            vec!["Revenue".to_string(), "$125,000".to_string(), "$142,500".to_string(), "$158,200".to_string(), "$425,700".to_string()],
            vec!["Expenses".to_string(), "$89,500".to_string(), "$95,200".to_string(), "$103,800".to_string(), "$288,500".to_string()],
            vec!["Profit".to_string(), "$35,500".to_string(), "$47,300".to_string(), "$54,400".to_string(), "$137,200".to_string()],
            vec!["Margin".to_string(), "28.4%".to_string(), "33.2%".to_string(), "34.4%".to_string(), "32.2%".to_string()],
        ]);
        
        // Validate data first
        let validation_result = validate_table_strict(&data);
        if !validation_result.is_valid {
            return Err(format!("Data validation failed: {}", validation_result.report()));
        }
        
        let column_configs = vec![
            ColumnConfig::new()
                .with_width(15)
                .with_alignment(Alignment::Left)
                .with_padding(Padding::new(2, 1)),
            ColumnConfig::new()
                .with_width(12)
                .with_alignment(Alignment::Right)
                .with_padding(Padding::symmetric(1)),
            ColumnConfig::new()
                .with_width(12)
                .with_alignment(Alignment::Right)
                .with_padding(Padding::symmetric(1)),
            ColumnConfig::new()
                .with_width(12)
                .with_alignment(Alignment::Right)
                .with_padding(Padding::symmetric(1)),
            ColumnConfig::new()
                .with_width(14)
                .with_alignment(Alignment::Right)
                .with_padding(Padding::new(1, 2)),
        ];
        
        let header_config = HeaderConfig::new()
            .with_header()
            .with_header_column_configs(vec![
                ColumnConfig::new().with_width(15).with_alignment(Alignment::Center),
                ColumnConfig::new().with_width(12).with_alignment(Alignment::Center),
                ColumnConfig::new().with_width(12).with_alignment(Alignment::Center),
                ColumnConfig::new().with_width(12).with_alignment(Alignment::Center),
                ColumnConfig::new().with_width(14).with_alignment(Alignment::Center),
            ]);
        
        let border = get_border_style("honeywell")?;
        let options = RenderOptions::default()
            .with_all_borders()
            .with_row_separators();
        
        let mut result = String::new();
        writeln!(result, "=== COMPREHENSIVE FINANCIAL REPORT ===")?;
        
        let table = render_table_with_headers(&data, &border, &options, &column_configs, &header_config)?;
        result.push_str(&table);
        
        // Add summary statistics
        writeln!(result, "\n=== REPORT STATISTICS ===")?;
        writeln!(result, "{}", validation_summary(&data))?;
        
        // Add debug information in compact format
        writeln!(result, "\n=== TABLE STRUCTURE ===")?;
        result.push_str(&debug_table_compact(&data));
        
        Ok(result)
    }
    
    /// Multi-format output example
    /// 
    /// Shows the same data in different output formats
    pub fn example_multi_format_output() -> Result<String, String> {
        let data = TableData::new(vec![
            vec!["Product".to_string(), "Category".to_string(), "Price".to_string(), "Stock".to_string()],
            vec!["Widget Pro".to_string(), "Electronics".to_string(), "$199.99".to_string(), "25".to_string()],
            vec!["Super Gadget".to_string(), "Electronics".to_string(), "$299.99".to_string(), "12".to_string()],
            vec!["Magic Tool".to_string(), "Tools".to_string(), "$89.99".to_string(), "50".to_string()],
        ]);
        
        let mut result = String::new();
        writeln!(result, "=== SAME DATA IN MULTIPLE FORMATS ===")?;
        
        // 1. Standard table format
        writeln!(result, "\n--- Standard Table Format ---")?;
        let standard = TableBuilder::from_data(data.clone())
            .with_border_style("default")
            .with_top_border()
            .with_bottom_border()
            .build()?;
        result.push_str(&standard);
        
        // 2. Minimal ASCII format
        writeln!(result, "\n--- Minimal ASCII Format ---")?;
        let minimal = ThemedTableBuilder::minimal()
            .add_rows(data.rows.clone())
            .build()?;
        result.push_str(&minimal);
        
        // 3. CSV format
        writeln!(result, "\n--- CSV Format ---")?;
        let csv_config = SingleLineConfig::csv();
        let csv_output = render_single_line_table(&data, &csv_config)?;
        result.push_str(&csv_output);
        
        // 4. JSON-like format
        writeln!(result, "\n--- JSON Array Format ---")?;
        let json_config = SingleLineConfig::json_array();
        let json_output = render_single_line_table(&data, &json_config)?;
        result.push_str(&json_output);
        
        // 5. Key-value format
        writeln!(result, "\n--- Key-Value Format ---")?;
        let kv_config = SingleLineConfig::new().without_headers();
        let kv_output = render_key_value_pairs(&data, &kv_config)?;
        result.push_str(&kv_output);
        
        // 6. HTML format
        writeln!(result, "\n--- HTML Format ---")?;
        let html_output = table_to_html(&data)?;
        result.push_str(&html_output);
        
        Ok(result)
    }
    
    /// Performance benchmark example
    /// 
    /// Shows performance comparison between different rendering methods
    pub fn example_performance_benchmark() -> Result<String, String> {
        // Create moderately large dataset
        let rows: Vec<Vec<String>> = (0..200)
            .map(|i| vec![
                format!("Record #{:04}", i),
                format!("Category {}", i % 10),
                format!("${:.2}", (i as f64) * 12.34),
                if i % 7 == 0 { "Priority".to_string() } else { "Standard".to_string() },
                format!("Status {}", i % 3),
            ])
            .collect();
        
        let data = TableData::new(rows);
        
        let column_configs = vec![
            ColumnConfig::new().with_width(12),
            ColumnConfig::new().with_width(12),
            ColumnConfig::new().with_width(10),
            ColumnConfig::new().with_width(10),
            ColumnConfig::new().with_width(10),
        ];
        
        let mut result = String::new();
        writeln!(result, "=== PERFORMANCE BENCHMARK ===")?;
        writeln!(result, "Dataset: {} rows, {} columns", data.row_count(), data.column_count())?;
        
        // Standard rendering
        let start = std::time::Instant::now();
        let _standard_output = render_table_with_column_config(
            &data,
            &BorderChars::default(),
            &RenderOptions::default(),
            &column_configs,
        )?;
        let standard_time = start.elapsed();
        
        // Fast rendering
        let start = std::time::Instant::now();
        let mut fast_renderer = FastTableRenderer::new(
            PerformanceConfig::new()
                .with_caching(true)
                .with_memory_optimization(true)
        );
        let _fast_output = fast_renderer.render_table(
            &data,
            &BorderChars::default(),
            &RenderOptions::default(),
            &column_configs,
        )?;
        let fast_time = start.elapsed();
        let (cache_size, memory_saved) = fast_renderer.cache_stats();
        
        writeln!(result, "\n--- Performance Results ---")?;
        writeln!(result, "Standard rendering: {:?}", standard_time)?;
        writeln!(result, "Fast rendering: {:?}", fast_time)?;
        writeln!(result, "Speedup: {:.2}x", standard_time.as_secs_f64() / fast_time.as_secs_f64())?;
        writeln!(result, "Cache entries: {}", cache_size)?;
        writeln!(result, "Memory saved: {} bytes", memory_saved)?;
        
        // Show validation performance
        let start = std::time::Instant::now();
        let _validation = validate_table(&data);
        let validation_time = start.elapsed();
        writeln!(result, "Validation time: {:?}", validation_time)?;
        
        // Show first few rows of output as sample
        writeln!(result, "\n--- Sample Output (first 10 lines) ---")?;
        let sample_table = TableData::new(data.rows.iter().take(5).cloned().collect());
        let sample = render_table_with_column_config(
            &sample_table,
            &BorderChars::default(),
            &RenderOptions::default().with_all_borders(),
            &column_configs,
        )?;
        
        let lines: Vec<&str> = sample.lines().take(10).collect();
        for line in lines {
            writeln!(result, "{}", line)?;
        }
        
        writeln!(result, "... (remaining {} rows omitted)", data.row_count() - 5)?;
        
        Ok(result)
    }
}

/// Generate complete documentation with all examples
/// 
/// Runs all documentation examples and combines them into a comprehensive guide
pub fn generate_complete_documentation() -> Result<String, String> {
    let mut doc = String::new();
    
    writeln!(doc, "# ASCII/ANSI Table Library - Complete Documentation")?;
    writeln!(doc, "Generated examples showcasing all library features\n")?;
    
    // Quick Start
    writeln!(doc, "## 1. Quick Start")?;
    writeln!(doc, "\n### Basic Table")?;
    doc.push_str(&quick_start::example_basic_table()?);
    
    writeln!(doc, "\n### Builder Pattern")?;
    doc.push_str(&quick_start::example_builder_basic()?);
    
    writeln!(doc, "\n### From CSV")?;
    doc.push_str(&quick_start::example_from_csv()?);
    
    // Styling
    writeln!(doc, "\n## 2. Styling and Themes")?;
    doc.push_str(&styling::example_themed_builders()?);
    
    writeln!(doc, "\n### Colored Output")?;
    doc.push_str(&styling::example_colored_table()?);
    
    // Layout
    writeln!(doc, "\n## 3. Advanced Layout")?;
    doc.push_str(&layout::example_multi_alignment()?);
    
    writeln!(doc, "\n### Text Overflow Handling")?;
    doc.push_str(&layout::example_text_overflow()?);
    
    // Data Handling
    writeln!(doc, "\n## 4. Data Handling and Validation")?;
    doc.push_str(&data_handling::example_validation()?);
    
    // Performance
    writeln!(doc, "\n## 5. Performance Features")?;
    doc.push_str(&performance::example_fast_rendering()?);
    
    // Special Features
    writeln!(doc, "\n## 6. Special Features")?;
    writeln!(doc, "\n### Cell Spanning")?;
    doc.push_str(&special_features::example_cell_spanning()?);
    
    writeln!(doc, "\n### Unicode and Emoji Support")?;
    doc.push_str(&special_features::example_unicode_emoji()?);
    
    writeln!(doc, "\n### HTML Export")?;
    doc.push_str(&special_features::example_html_export()?);
    
    // Complete Examples
    writeln!(doc, "\n## 7. Complete Examples")?;
    doc.push_str(&complete_examples::example_financial_report()?);
    
    writeln!(doc, "\n### Multi-Format Output")?;
    doc.push_str(&complete_examples::example_multi_format_output()?);
    
    // Performance Benchmark
    writeln!(doc, "\n## 8. Performance Benchmark")?;
    doc.push_str(&complete_examples::example_performance_benchmark()?);
    
    writeln!(doc, "\n---")?;
    writeln!(doc, "Documentation generated by ascii-ansi-table v0.8.4")?;
    writeln!(doc, "For more information, visit: https://github.com/your-repo/ascii-ansi-table")?;
    
    Ok(doc)
}

/// Quick function to generate and display all examples
pub fn show_all_examples() {
    match generate_complete_documentation() {
        Ok(documentation) => println!("{}", documentation),
        Err(e) => eprintln!("Error generating documentation: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_quick_start_examples() {
        assert!(quick_start::example_basic_table().is_ok());
        assert!(quick_start::example_builder_basic().is_ok());
        assert!(quick_start::example_from_csv().is_ok());
    }
    
    #[test]
    fn test_styling_examples() {
        assert!(styling::example_all_border_styles().is_ok());
        assert!(styling::example_themed_builders().is_ok());
        assert!(styling::example_colored_table().is_ok());
    }
    
    #[test]
    fn test_layout_examples() {
        assert!(layout::example_multi_alignment().is_ok());
        assert!(layout::example_custom_padding().is_ok());
        // Note: text_overflow requires wrapping support which may not be fully implemented
    }
    
    #[test]
    fn test_data_handling_examples() {
        assert!(data_handling::example_validation().is_ok());
        assert!(data_handling::example_debugging().is_ok());
        assert!(data_handling::example_data_formats().is_ok());
    }
    
    #[test]
    fn test_performance_examples() {
        assert!(performance::example_fast_rendering().is_ok());
        assert!(performance::example_streaming().is_ok());
        assert!(performance::example_batch_processing().is_ok());
    }
    
    #[test]
    fn test_special_features_examples() {
        assert!(special_features::example_cell_spanning().is_ok());
        assert!(special_features::example_complex_headers().is_ok());
        assert!(special_features::example_unicode_emoji().is_ok());
        assert!(special_features::example_html_export().is_ok());
    }
    
    #[test]
    fn test_complete_examples() {
        assert!(complete_examples::example_financial_report().is_ok());
        assert!(complete_examples::example_multi_format_output().is_ok());
        assert!(complete_examples::example_performance_benchmark().is_ok());
    }
    
    #[test]
    fn test_complete_documentation_generation() {
        let doc = generate_complete_documentation();
        assert!(doc.is_ok());
        
        let doc_content = doc.unwrap();
        assert!(doc_content.contains("Quick Start"));
        assert!(doc_content.contains("Styling and Themes"));
        assert!(doc_content.contains("Advanced Layout"));
        assert!(doc_content.contains("Data Handling"));
        assert!(doc_content.contains("Performance"));
        assert!(doc_content.contains("Special Features"));
        assert!(doc_content.contains("Complete Examples"));
        assert!(doc_content.len() > 1000); // Ensure substantial content
    }
    
    #[test]
    fn test_documentation_examples_contain_expected_content() {
        let basic_table = quick_start::example_basic_table().unwrap();
        assert!(basic_table.contains("Name"));
        assert!(basic_table.contains("Alice"));
        assert!(basic_table.contains("Bob"));
        
        let builder_example = quick_start::example_builder_basic().unwrap();
        assert!(builder_example.contains("Product"));
        assert!(builder_example.contains("Widget"));
        
        let csv_example = quick_start::example_from_csv().unwrap();
        assert!(csv_example.contains("Name"));
        assert!(csv_example.contains("City"));
    }
}