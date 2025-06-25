use crate::core::calculator::{
    calculate_output_column_widths, calculate_row_heights, map_data_using_row_heights,
};
use crate::core::renderer::BorderType;
use crate::core::renderer::{draw_border_line, draw_row};
use crate::types::{ColumnConfig, StreamConfig, StreamUserConfig, TableError, TableResult};
use std::io::Write;

pub struct TableStream {
    config: StreamConfig,
    column_widths: Vec<usize>,
    first_row: bool,
    row_count: usize,
}

impl TableStream {
    pub fn new(user_config: Option<StreamUserConfig>) -> Self {
        let config = if let Some(user_config) = user_config {
            user_config.merge_with_default(&StreamConfig::default())
        } else {
            StreamConfig::default()
        };

        Self {
            config,
            column_widths: Vec::new(),
            first_row: true,
            row_count: 0,
        }
    }

    pub fn initialize_columns(&mut self, column_count: usize) {
        if self.config.columns.is_empty() {
            let default_column = ColumnConfig {
                width: 10,
                ..Default::default()
            };
            self.config.columns = vec![default_column; column_count];
        }

        let dummy_data = vec![vec![String::new(); column_count]];
        self.column_widths =
            calculate_output_column_widths(&dummy_data, &self.config.clone().into());
    }

    pub fn write_row(&mut self, row: &[String]) -> TableResult<String> {
        if self.column_widths.is_empty() {
            self.initialize_columns(row.len());
        }

        if row.len() != self.column_widths.len() {
            return Err(TableError::InconsistentRowLength);
        }

        let mut result = String::new();

        if self.first_row {
            if (self.config.draw_horizontal_line)(0, 1) {
                result.push_str(&draw_border_line(
                    &self.column_widths,
                    &self.config.border,
                    BorderType::Top,
                ));
                result.push('\n');
            }
            self.first_row = false;
        } else if (self.config.draw_horizontal_line)(self.row_count, self.row_count + 1) {
            result.push_str(&draw_border_line(
                &self.column_widths,
                &self.config.border,
                BorderType::Join,
            ));
            result.push('\n');
        }

        // Handle multiline cells by calculating row height and mapping data
        let rows_data = vec![row.to_vec()];
        let table_config = self.config.clone().into();
        let row_heights = calculate_row_heights(&rows_data, &table_config);
        let mapped_rows = map_data_using_row_heights(&rows_data, &row_heights, &table_config);

        // Render all sub-rows for this multiline row
        if let Some(sub_rows) = mapped_rows.first() {
            for sub_row in sub_rows {
                let processed_row = self.process_row_data(sub_row);
                result.push_str(&draw_row(
                    &processed_row,
                    &self.column_widths,
                    &self.config.border,
                ));
                result.push('\n');
            }
        }

        self.row_count += 1;

        Ok(result)
    }

    pub fn finalize(&mut self) -> String {
        let mut result = String::new();

        if !self.first_row && (self.config.draw_horizontal_line)(self.row_count, self.row_count) {
            result.push_str(&draw_border_line(
                &self.column_widths,
                &self.config.border,
                BorderType::Bottom,
            ));
        }

        result
    }

    fn process_row_data(&self, row: &[String]) -> Vec<String> {
        let mut processed = Vec::new();

        for (col_idx, cell) in row.iter().enumerate() {
            let column_config = self
                .config
                .columns
                .get(col_idx)
                .unwrap_or(&self.config.column_default);
            let target_width = self.column_widths.get(col_idx).unwrap_or(&0);

            let mut cell_content = cell.clone();

            if column_config.truncate > 0 {
                cell_content = self.truncate_cell_content(&cell_content, column_config.truncate);
            }

            let content_width = target_width
                .saturating_sub(column_config.padding_left + column_config.padding_right);
            let aligned =
                self.align_cell_content(&cell_content, content_width, column_config.alignment);
            let padded = self.pad_cell_content(
                &aligned,
                column_config.padding_left,
                column_config.padding_right,
            );

            processed.push(padded);
        }

        processed
    }

    fn align_cell_content(
        &self,
        content: &str,
        width: usize,
        alignment: crate::types::Alignment,
    ) -> String {
        use crate::utils::ansi::pad_ansi_string;
        let padded = pad_ansi_string(content, width, alignment);
        padded.content
    }

    fn pad_cell_content(&self, content: &str, left_padding: usize, right_padding: usize) -> String {
        let left_pad = " ".repeat(left_padding);
        let right_pad = " ".repeat(right_padding);
        format!("{}{}{}", left_pad, content, right_pad)
    }

    fn truncate_cell_content(&self, content: &str, max_width: usize) -> String {
        use crate::utils::ansi::truncate_ansi_string;
        let truncated = truncate_ansi_string(content, max_width);
        truncated.content
    }
}

impl StreamUserConfig {
    pub fn merge_with_default(self, default: &StreamConfig) -> StreamConfig {
        let border = self
            .border
            .map(|b| b.merge_with_default(&default.border))
            .unwrap_or_else(|| default.border.clone());

        let column_default = self
            .column_default
            .map(|c| c.merge_with_default(&default.column_default))
            .unwrap_or_else(|| default.column_default.clone());

        let columns = self
            .columns
            .map(|cols| {
                cols.into_iter()
                    .map(|c| c.merge_with_default(&column_default))
                    .collect()
            })
            .unwrap_or_else(|| default.columns.clone());

        StreamConfig {
            border,
            columns,
            column_default,
            draw_vertical_line: default.draw_vertical_line,
            draw_horizontal_line: default.draw_horizontal_line,
            single_line: self.single_line.unwrap_or(default.single_line),
        }
    }
}

pub trait WritableStream {
    fn write(&mut self, chunk: &str) -> TableResult<()>;
}

pub struct StreamWriter<W: Write> {
    writer: W,
    stream: TableStream,
}

impl<W: Write> StreamWriter<W> {
    pub fn new(writer: W, config: Option<StreamUserConfig>) -> Self {
        Self {
            writer,
            stream: TableStream::new(config),
        }
    }

    pub fn write_row(&mut self, row: &[String]) -> TableResult<()> {
        let output = self.stream.write_row(row)?;
        self.writer
            .write_all(output.as_bytes())
            .map_err(|_| TableError::InvalidConfig("Failed to write to stream".to_string()))?;
        Ok(())
    }

    pub fn finalize(mut self) -> TableResult<()> {
        let output = self.stream.finalize();
        self.writer
            .write_all(output.as_bytes())
            .map_err(|_| TableError::InvalidConfig("Failed to write to stream".to_string()))?;
        self.writer
            .flush()
            .map_err(|_| TableError::InvalidConfig("Failed to flush stream".to_string()))?;
        Ok(())
    }
}

pub fn create_stream<W: Write>(writer: W, config: Option<StreamUserConfig>) -> StreamWriter<W> {
    StreamWriter::new(writer, config)
}

pub fn create_string_stream(config: Option<StreamUserConfig>) -> TableStream {
    TableStream::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_table_stream_creation() {
        let stream = TableStream::new(None);
        assert!(stream.first_row);
        assert_eq!(stream.row_count, 0);
    }

    #[test]
    fn test_stream_write_row() {
        let mut stream = TableStream::new(None);

        let row1 = vec!["Name".to_string(), "Age".to_string()];
        let result1 = stream.write_row(&row1).unwrap();
        assert!(result1.contains("Name"));
        assert!(result1.contains("Age"));
        assert!(result1.contains("┌"));

        let row2 = vec!["John".to_string(), "30".to_string()];
        let result2 = stream.write_row(&row2).unwrap();
        assert!(result2.contains("John"));
        assert!(result2.contains("30"));
        assert!(result2.contains("├"));
    }

    #[test]
    fn test_stream_finalize() {
        let mut stream = TableStream::new(None);

        let row = vec!["test".to_string(), "data".to_string()];
        let _ = stream.write_row(&row).unwrap();

        let finalized = stream.finalize();
        assert!(finalized.contains("└"));
    }

    #[test]
    fn test_stream_writer() {
        let mut cursor = Cursor::new(Vec::new());
        let mut writer = StreamWriter::new(&mut cursor, None);

        let row1 = vec!["Name".to_string(), "Age".to_string()];
        writer.write_row(&row1).unwrap();

        let row2 = vec!["John".to_string(), "30".to_string()];
        writer.write_row(&row2).unwrap();

        writer.finalize().unwrap();

        let output = String::from_utf8(cursor.into_inner()).unwrap();
        assert!(output.contains("Name"));
        assert!(output.contains("Age"));
        assert!(output.contains("John"));
        assert!(output.contains("30"));
    }

    #[test]
    fn test_stream_inconsistent_row_length() {
        let mut stream = TableStream::new(None);

        let row1 = vec!["Name".to_string(), "Age".to_string()];
        let _ = stream.write_row(&row1).unwrap();

        let row2 = vec!["John".to_string()];
        let result = stream.write_row(&row2);
        assert!(result.is_err());
    }

    #[test]
    fn test_create_string_stream() {
        let mut stream = create_string_stream(None);

        let row = vec!["test".to_string(), "data".to_string()];
        let result = stream.write_row(&row).unwrap();
        let finalized = stream.finalize();

        let complete_output = format!("{}{}", result, finalized);
        assert!(complete_output.contains("test"));
        assert!(complete_output.contains("data"));
        assert!(complete_output.contains("┌"));
        assert!(complete_output.contains("└"));
    }
}
