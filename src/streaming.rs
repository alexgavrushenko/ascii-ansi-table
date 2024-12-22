use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct StreamingTableConfig {
    pub border: crate::BorderChars,
    pub options: crate::RenderOptions,
    pub column_configs: Vec<crate::alignment::ColumnConfig>,
    pub buffer_size: usize,
    pub flush_on_row: bool,
}

impl Default for StreamingTableConfig {
    fn default() -> Self {
        Self {
            border: crate::BorderChars::default(),
            options: crate::RenderOptions::default(),
            column_configs: Vec::new(),
            buffer_size: 8192, // 8KB buffer
            flush_on_row: false,
        }
    }
}

impl StreamingTableConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_border(mut self, border: crate::BorderChars) -> Self {
        self.border = border;
        self
    }
    
    pub fn with_options(mut self, options: crate::RenderOptions) -> Self {
        self.options = options;
        self
    }
    
    pub fn with_column_configs(mut self, configs: Vec<crate::alignment::ColumnConfig>) -> Self {
        self.column_configs = configs;
        self
    }
    
    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }
    
    pub fn with_flush_on_row(mut self) -> Self {
        self.flush_on_row = true;
        self
    }
}

/// Streaming table writer that can render tables incrementally
pub struct StreamingTableWriter<W: Write> {
    writer: W,
    config: StreamingTableConfig,
    column_widths: Vec<usize>,
    buffer: String,
    header_written: bool,
    row_count: usize,
    column_count: usize,
}

impl<W: Write> StreamingTableWriter<W> {
    pub fn new(writer: W, config: StreamingTableConfig) -> Self {
        let buffer_size = config.buffer_size;
        Self {
            writer,
            config,
            column_widths: Vec::new(),
            buffer: String::with_capacity(buffer_size),
            header_written: false,
            row_count: 0,
            column_count: 0,
        }
    }
    
    /// Initialize the table with column information
    pub fn initialize(&mut self, column_widths: Vec<usize>) -> io::Result<()> {
        self.column_widths = column_widths;
        self.column_count = self.column_widths.len();
        
        // Write top border if configured
        if self.config.options.show_top_border {
            self.write_top_border()?;
        }
        
        Ok(())
    }
    
    /// Add a single row to the streaming table
    pub fn add_row(&mut self, row: &[String]) -> io::Result<()> {
        if row.len() != self.column_count && self.column_count > 0 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Row has {} columns, expected {}", row.len(), self.column_count)
            ));
        }
        
        // If this is the first row and no column widths were set, calculate them
        if self.column_widths.is_empty() {
            self.column_widths = row.iter().map(|cell| cell.len()).collect();
            self.column_count = self.column_widths.len();
            
            if self.config.options.show_top_border {
                self.write_top_border()?;
            }
        }
        
        // Write row separator if needed (not before first row)
        if self.config.options.show_row_separators && self.row_count > 0 {
            self.write_row_separator()?;
        }
        
        // Render the row
        self.write_data_row(row)?;
        self.row_count += 1;
        
        // Flush if configured
        if self.config.flush_on_row {
            self.flush()?;
        }
        
        Ok(())
    }
    
    /// Add multiple rows in batch
    pub fn add_rows(&mut self, rows: &[Vec<String>]) -> io::Result<()> {
        for row in rows {
            self.add_row(row)?;
        }
        Ok(())
    }
    
    /// Finalize the table (write bottom border, flush remaining buffer)
    pub fn finalize(mut self) -> io::Result<W> {
        if self.config.options.show_bottom_border {
            self.write_bottom_border()?;
        }
        
        self.flush()?;
        Ok(self.writer)
    }
    
    fn write_top_border(&mut self) -> io::Result<()> {
        self.buffer.push(self.config.border.top_left);
        for (i, &width) in self.column_widths.iter().enumerate() {
            // Calculate total width including padding
            let total_width = if let Some(config) = self.config.column_configs.get(i) {
                width + config.padding.total()
            } else {
                width + 2 // Default padding
            };
            
            self.buffer.push_str(&self.config.border.horizontal.to_string().repeat(total_width));
            if i < self.column_widths.len() - 1 {
                self.buffer.push(self.config.border.top_junction);
            }
        }
        self.buffer.push(self.config.border.top_right);
        self.buffer.push('\n');
        
        self.flush_if_needed()
    }
    
    fn write_bottom_border(&mut self) -> io::Result<()> {
        self.buffer.push(self.config.border.bottom_left);
        for (i, &width) in self.column_widths.iter().enumerate() {
            // Calculate total width including padding
            let total_width = if let Some(config) = self.config.column_configs.get(i) {
                width + config.padding.total()
            } else {
                width + 2 // Default padding
            };
            
            self.buffer.push_str(&self.config.border.horizontal.to_string().repeat(total_width));
            if i < self.column_widths.len() - 1 {
                self.buffer.push(self.config.border.bottom_junction);
            }
        }
        self.buffer.push(self.config.border.bottom_right);
        self.buffer.push('\n');
        
        self.flush_if_needed()
    }
    
    fn write_row_separator(&mut self) -> io::Result<()> {
        self.buffer.push('├');
        for (i, &width) in self.column_widths.iter().enumerate() {
            // Calculate total width including padding
            let total_width = if let Some(config) = self.config.column_configs.get(i) {
                width + config.padding.total()
            } else {
                width + 2 // Default padding
            };
            
            self.buffer.push_str(&self.config.border.horizontal.to_string().repeat(total_width));
            if i < self.column_widths.len() - 1 {
                self.buffer.push('┼');
            }
        }
        self.buffer.push('┤');
        self.buffer.push('\n');
        
        self.flush_if_needed()
    }
    
    fn write_data_row(&mut self, row: &[String]) -> io::Result<()> {
        self.buffer.push(self.config.border.vertical);
        
        for (i, cell) in row.iter().enumerate() {
            let default_config = crate::alignment::ColumnConfig::default();
            let config = self.config.column_configs.get(i)
                .unwrap_or(&default_config);
            let content_width = self.column_widths[i];
            
            // Apply truncation, alignment, and padding
            let truncated_cell = crate::truncation::truncate_text(cell, &config.truncation);
            let aligned_cell = crate::alignment::align_text(&truncated_cell, content_width, config.alignment);
            let padded_cell = crate::padding::apply_padding(&aligned_cell, config.padding);
            
            self.buffer.push_str(&padded_cell);
            self.buffer.push(self.config.border.vertical);
        }
        self.buffer.push('\n');
        
        self.flush_if_needed()
    }
    
    fn flush_if_needed(&mut self) -> io::Result<()> {
        if self.buffer.len() >= self.config.buffer_size {
            self.flush()
        } else {
            Ok(())
        }
    }
    
    fn flush(&mut self) -> io::Result<()> {
        if !self.buffer.is_empty() {
            self.writer.write_all(self.buffer.as_bytes())?;
            self.writer.flush()?;
            self.buffer.clear();
        }
        Ok(())
    }
}

/// Convenient function to stream a table to a writer
pub fn stream_table_to_writer<W: Write>(
    writer: W,
    rows: impl Iterator<Item = Vec<String>>,
    config: StreamingTableConfig,
) -> io::Result<W> {
    let mut streaming_writer = StreamingTableWriter::new(writer, config);
    
    // Process rows one by one
    for row in rows {
        streaming_writer.add_row(&row)?;
    }
    
    streaming_writer.finalize()
}

/// Stream table to stdout with default configuration
pub fn stream_table_to_stdout(
    rows: impl Iterator<Item = Vec<String>>,
) -> io::Result<()> {
    let config = StreamingTableConfig::default();
    let stdout = io::stdout();
    stream_table_to_writer(stdout, rows, config)?;
    Ok(())
}

/// Create a streaming table builder
pub struct StreamingTableBuilder {
    config: StreamingTableConfig,
}

impl Default for StreamingTableBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StreamingTableBuilder {
    pub fn new() -> Self {
        Self {
            config: StreamingTableConfig::default(),
        }
    }
    
    pub fn with_border(mut self, border: crate::BorderChars) -> Self {
        self.config.border = border;
        self
    }
    
    pub fn with_options(mut self, options: crate::RenderOptions) -> Self {
        self.config.options = options;
        self
    }
    
    pub fn with_column_configs(mut self, configs: Vec<crate::alignment::ColumnConfig>) -> Self {
        self.config.column_configs = configs;
        self
    }
    
    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.config.buffer_size = size;
        self
    }
    
    pub fn with_auto_flush(mut self) -> Self {
        self.config.flush_on_row = true;
        self
    }
    
    pub fn build<W: Write>(self, writer: W) -> StreamingTableWriter<W> {
        StreamingTableWriter::new(writer, self.config)
    }
    
    pub fn stream_to_writer<W: Write>(
        self,
        writer: W,
        rows: impl Iterator<Item = Vec<String>>,
    ) -> io::Result<W> {
        stream_table_to_writer(writer, rows, self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_streaming_table_writer() {
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        
        let config = StreamingTableConfig::new()
            .with_column_configs(vec![
                crate::alignment::ColumnConfig::new().with_width(8),
                crate::alignment::ColumnConfig::new().with_width(10),
            ]);
        
        let mut writer = StreamingTableWriter::new(cursor, config);
        writer.initialize(vec![8, 10]).unwrap();
        
        writer.add_row(&["Name".to_string(), "Age".to_string()]).unwrap();
        writer.add_row(&["Alice".to_string(), "30".to_string()]).unwrap();
        writer.add_row(&["Bob".to_string(), "25".to_string()]).unwrap();
        
        let result_cursor = writer.finalize().unwrap();
        let output = String::from_utf8(result_cursor.into_inner()).unwrap();
        
        assert!(output.contains("Name"));
        assert!(output.contains("Alice"));
        assert!(output.contains("Bob"));
        assert!(output.contains("│"));
        assert!(output.contains("┌"));
        assert!(output.contains("└"));
    }

    #[test]
    fn test_streaming_batch_add() {
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        
        let config = StreamingTableConfig::default();
        let mut writer = StreamingTableWriter::new(cursor, config);
        
        let rows = vec![
            vec!["Header 1".to_string(), "Header 2".to_string()],
            vec!["Data 1".to_string(), "Data 2".to_string()],
            vec!["Data 3".to_string(), "Data 4".to_string()],
        ];
        
        writer.add_rows(&rows).unwrap();
        let result_cursor = writer.finalize().unwrap();
        let output = String::from_utf8(result_cursor.into_inner()).unwrap();
        
        assert!(output.contains("Header 1"));
        assert!(output.contains("Data 1"));
        assert!(output.contains("Data 3"));
    }

    #[test]
    fn test_stream_table_function() {
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        
        let rows = vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string(), "2".to_string()],
            vec!["3".to_string(), "4".to_string()],
        ];
        
        let config = StreamingTableConfig::default();
        let result_cursor = stream_table_to_writer(cursor, rows.into_iter(), config).unwrap();
        let output = String::from_utf8(result_cursor.into_inner()).unwrap();
        
        assert!(output.contains("A"));
        assert!(output.contains("B"));
        assert!(output.contains("1"));
        assert!(output.contains("2"));
        assert!(output.contains("3"));
        assert!(output.contains("4"));
    }

    #[test]
    fn test_streaming_builder() {
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        
        let rows = vec![
            vec!["Name".to_string(), "Score".to_string()],
            vec!["Alice".to_string(), "95".to_string()],
            vec!["Bob".to_string(), "87".to_string()],
        ];
        
        let result_cursor = StreamingTableBuilder::new()
            .with_border(crate::BorderChars::default())
            .with_auto_flush()
            .stream_to_writer(cursor, rows.into_iter())
            .unwrap();
            
        let output = String::from_utf8(result_cursor.into_inner()).unwrap();
        
        assert!(output.contains("Name"));
        assert!(output.contains("Alice"));
        assert!(output.contains("95"));
    }

    #[test]
    fn test_column_mismatch_error() {
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        
        let config = StreamingTableConfig::default();
        let mut writer = StreamingTableWriter::new(cursor, config);
        
        // Initialize with 2 columns
        writer.initialize(vec![10, 10]).unwrap();
        
        // Try to add row with 3 columns
        let result = writer.add_row(&["A".to_string(), "B".to_string(), "C".to_string()]);
        assert!(result.is_err());
    }

    #[test]
    fn test_auto_column_detection() {
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        
        let config = StreamingTableConfig::default();
        let mut writer = StreamingTableWriter::new(cursor, config);
        
        // Don't initialize - let first row determine column count
        writer.add_row(&["Header 1".to_string(), "Header 2".to_string()]).unwrap();
        writer.add_row(&["Data 1".to_string(), "Data 2".to_string()]).unwrap();
        
        let result_cursor = writer.finalize().unwrap();
        let output = String::from_utf8(result_cursor.into_inner()).unwrap();
        
        assert!(output.contains("Header 1"));
        assert!(output.contains("Data 1"));
    }

    #[test]
    fn test_buffering() {
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        
        let config = StreamingTableConfig::new()
            .with_buffer_size(50); // Small buffer to test flushing
        
        let mut writer = StreamingTableWriter::new(cursor, config);
        
        // Add enough rows to exceed buffer
        for i in 0..10 {
            writer.add_row(&[format!("Row {}", i), format!("Data {}", i)]).unwrap();
        }
        
        let result_cursor = writer.finalize().unwrap();
        let output = String::from_utf8(result_cursor.into_inner()).unwrap();
        
        assert!(output.contains("Row 0"));
        assert!(output.contains("Row 9"));
    }
}