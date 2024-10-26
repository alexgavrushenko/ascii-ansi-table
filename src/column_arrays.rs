#[derive(Debug, Clone)]
pub struct ColumnConfigArray {
    pub configs: Vec<crate::alignment::ColumnConfig>,
    pub repeat_pattern: bool,
    pub default_config: crate::alignment::ColumnConfig,
}

impl Default for ColumnConfigArray {
    fn default() -> Self {
        Self {
            configs: Vec::new(),
            repeat_pattern: false,
            default_config: crate::alignment::ColumnConfig::default(),
        }
    }
}

impl ColumnConfigArray {
    pub fn new(configs: Vec<crate::alignment::ColumnConfig>) -> Self {
        Self {
            configs,
            repeat_pattern: false,
            default_config: crate::alignment::ColumnConfig::default(),
        }
    }
    
    pub fn with_repeat_pattern(mut self) -> Self {
        self.repeat_pattern = true;
        self
    }
    
    pub fn with_default_config(mut self, config: crate::alignment::ColumnConfig) -> Self {
        self.default_config = config;
        self
    }
    
    /// Get configuration for a specific column index
    pub fn get_config(&self, column_index: usize) -> &crate::alignment::ColumnConfig {
        if self.configs.is_empty() {
            return &self.default_config;
        }
        
        if self.repeat_pattern {
            // Cycle through the pattern
            &self.configs[column_index % self.configs.len()]
        } else {
            // Use config if available, otherwise default
            self.configs.get(column_index).unwrap_or(&self.default_config)
        }
    }
    
    /// Generate column configs for a given number of columns
    pub fn generate_configs(&self, column_count: usize) -> Vec<crate::alignment::ColumnConfig> {
        let mut result = Vec::with_capacity(column_count);
        
        for i in 0..column_count {
            result.push(self.get_config(i).clone());
        }
        
        result
    }
    
    /// Add a new column configuration
    pub fn add_config(mut self, config: crate::alignment::ColumnConfig) -> Self {
        self.configs.push(config);
        self
    }
    
    /// Set configurations from a slice
    pub fn from_slice(configs: &[crate::alignment::ColumnConfig]) -> Self {
        Self::new(configs.to_vec())
    }
}

/// Builder for creating column configuration arrays with convenience methods
#[derive(Debug)]
pub struct ColumnArrayBuilder {
    configs: Vec<crate::alignment::ColumnConfig>,
    repeat_pattern: bool,
    default_config: crate::alignment::ColumnConfig,
}

impl Default for ColumnArrayBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ColumnArrayBuilder {
    pub fn new() -> Self {
        Self {
            configs: Vec::new(),
            repeat_pattern: false,
            default_config: crate::alignment::ColumnConfig::default(),
        }
    }
    
    pub fn add_column(mut self, config: crate::alignment::ColumnConfig) -> Self {
        self.configs.push(config);
        self
    }
    
    pub fn add_columns(mut self, configs: Vec<crate::alignment::ColumnConfig>) -> Self {
        self.configs.extend(configs);
        self
    }
    
    pub fn with_repeat_pattern(mut self) -> Self {
        self.repeat_pattern = true;
        self
    }
    
    pub fn with_default_config(mut self, config: crate::alignment::ColumnConfig) -> Self {
        self.default_config = config;
        self
    }
    
    /// Add a left-aligned column with specified width
    pub fn left_column(self, width: usize) -> Self {
        self.add_column(
            crate::alignment::ColumnConfig::new()
                .with_width(width)
                .with_alignment(crate::alignment::Alignment::Left)
        )
    }
    
    /// Add a center-aligned column with specified width
    pub fn center_column(self, width: usize) -> Self {
        self.add_column(
            crate::alignment::ColumnConfig::new()
                .with_width(width)
                .with_alignment(crate::alignment::Alignment::Center)
        )
    }
    
    /// Add a right-aligned column with specified width
    pub fn right_column(self, width: usize) -> Self {
        self.add_column(
            crate::alignment::ColumnConfig::new()
                .with_width(width)
                .with_alignment(crate::alignment::Alignment::Right)
        )
    }
    
    /// Create alternating left/right columns
    pub fn alternating_alignment(mut self, widths: &[usize]) -> Self {
        for (i, &width) in widths.iter().enumerate() {
            let alignment = if i % 2 == 0 {
                crate::alignment::Alignment::Left
            } else {
                crate::alignment::Alignment::Right
            };
            
            self = self.add_column(
                crate::alignment::ColumnConfig::new()
                    .with_width(width)
                    .with_alignment(alignment)
            );
        }
        self
    }
    
    pub fn build(self) -> ColumnConfigArray {
        ColumnConfigArray {
            configs: self.configs,
            repeat_pattern: self.repeat_pattern,
            default_config: self.default_config,
        }
    }
}

/// Render table using column configuration array
pub fn render_table_with_column_array(
    data: &crate::TableData,
    border: &crate::BorderChars,
    options: &crate::RenderOptions,
    column_array: &ColumnConfigArray,
) -> Result<String, String> {
    let column_configs = column_array.generate_configs(data.column_count());
    crate::render_table_with_column_config(data, border, options, &column_configs)
}

/// Create common column array patterns
pub mod patterns {
    use super::*;
    
    /// Create a pattern for financial/numeric data (right-aligned numbers)
    pub fn financial_columns(widths: &[usize]) -> ColumnConfigArray {
        let mut builder = ColumnArrayBuilder::new();
        
        // First column (usually description) is left-aligned
        if let Some(&first_width) = widths.first() {
            builder = builder.left_column(first_width);
        }
        
        // Remaining columns (numbers) are right-aligned
        for &width in widths.iter().skip(1) {
            builder = builder.right_column(width);
        }
        
        builder.build()
    }
    
    /// Create a pattern for mixed content (alternating alignments)
    pub fn mixed_content(widths: &[usize]) -> ColumnConfigArray {
        ColumnArrayBuilder::new()
            .alternating_alignment(widths)
            .build()
    }
    
    /// Create a centered header pattern
    pub fn centered_headers(width: usize, count: usize) -> ColumnConfigArray {
        let config = crate::alignment::ColumnConfig::new()
            .with_width(width)
            .with_alignment(crate::alignment::Alignment::Center);
            
        ColumnConfigArray::new(vec![config; count])
    }
    
    /// Create a repeating pattern (useful for large tables)
    pub fn repeating_pattern(pattern: Vec<crate::alignment::ColumnConfig>) -> ColumnConfigArray {
        ColumnConfigArray::new(pattern).with_repeat_pattern()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::padding::Padding;

    #[test]
    fn test_column_config_array_creation() {
        let configs = vec![
            crate::alignment::ColumnConfig::new().with_width(10),
            crate::alignment::ColumnConfig::new().with_width(15),
        ];
        
        let array = ColumnConfigArray::new(configs.clone());
        assert_eq!(array.configs.len(), 2);
        assert!(!array.repeat_pattern);
        
        // Test accessing configurations
        assert_eq!(array.get_config(0).width, Some(10));
        assert_eq!(array.get_config(1).width, Some(15));
        
        // Should return default for out-of-bounds
        let default_config = array.get_config(2);
        assert_eq!(default_config.width, None);
    }

    #[test]
    fn test_repeat_pattern() {
        let configs = vec![
            crate::alignment::ColumnConfig::new()
                .with_width(10)
                .with_alignment(crate::alignment::Alignment::Left),
            crate::alignment::ColumnConfig::new()
                .with_width(15)
                .with_alignment(crate::alignment::Alignment::Right),
        ];
        
        let array = ColumnConfigArray::new(configs).with_repeat_pattern();
        
        // Should cycle through pattern
        assert_eq!(array.get_config(0).alignment, crate::alignment::Alignment::Left);
        assert_eq!(array.get_config(1).alignment, crate::alignment::Alignment::Right);
        assert_eq!(array.get_config(2).alignment, crate::alignment::Alignment::Left);  // Cycles back
        assert_eq!(array.get_config(3).alignment, crate::alignment::Alignment::Right); // Cycles back
    }

    #[test]
    fn test_generate_configs() {
        let configs = vec![
            crate::alignment::ColumnConfig::new().with_width(10),
            crate::alignment::ColumnConfig::new().with_width(15),
        ];
        
        let array = ColumnConfigArray::new(configs).with_repeat_pattern();
        let generated = array.generate_configs(5);
        
        assert_eq!(generated.len(), 5);
        assert_eq!(generated[0].width, Some(10));
        assert_eq!(generated[1].width, Some(15));
        assert_eq!(generated[2].width, Some(10)); // Repeats
        assert_eq!(generated[3].width, Some(15)); // Repeats
        assert_eq!(generated[4].width, Some(10)); // Repeats
    }

    #[test]
    fn test_column_array_builder() {
        let array = ColumnArrayBuilder::new()
            .left_column(10)
            .center_column(15)
            .right_column(20)
            .build();
        
        assert_eq!(array.configs.len(), 3);
        assert_eq!(array.get_config(0).alignment, crate::alignment::Alignment::Left);
        assert_eq!(array.get_config(1).alignment, crate::alignment::Alignment::Center);
        assert_eq!(array.get_config(2).alignment, crate::alignment::Alignment::Right);
    }

    #[test]
    fn test_alternating_alignment() {
        let widths = [10, 15, 20, 25];
        let array = ColumnArrayBuilder::new()
            .alternating_alignment(&widths)
            .build();
        
        assert_eq!(array.configs.len(), 4);
        assert_eq!(array.get_config(0).alignment, crate::alignment::Alignment::Left);  // Even index
        assert_eq!(array.get_config(1).alignment, crate::alignment::Alignment::Right); // Odd index
        assert_eq!(array.get_config(2).alignment, crate::alignment::Alignment::Left);  // Even index
        assert_eq!(array.get_config(3).alignment, crate::alignment::Alignment::Right); // Odd index
    }

    #[test]
    fn test_render_with_column_array() {
        let data = crate::TableData::new(vec![
            vec!["Left".to_string(), "Center".to_string(), "Right".to_string()],
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
        ]);
        
        let array = ColumnArrayBuilder::new()
            .left_column(8)
            .center_column(8)
            .right_column(8)
            .build();
        
        let border = crate::BorderChars::default();
        let options = crate::RenderOptions::default();
        let result = render_table_with_column_array(&data, &border, &options, &array).unwrap();
        
        assert!(result.contains("Left"));
        assert!(result.contains("Center"));
        assert!(result.contains("Right"));
        assert!(result.contains("A"));
        assert!(result.contains("B"));
        assert!(result.contains("C"));
    }

    #[test]
    fn test_financial_pattern() {
        let widths = [15, 10, 10, 12];
        let array = patterns::financial_columns(&widths);
        
        assert_eq!(array.configs.len(), 4);
        assert_eq!(array.get_config(0).alignment, crate::alignment::Alignment::Left);  // Description
        assert_eq!(array.get_config(1).alignment, crate::alignment::Alignment::Right); // Number
        assert_eq!(array.get_config(2).alignment, crate::alignment::Alignment::Right); // Number
        assert_eq!(array.get_config(3).alignment, crate::alignment::Alignment::Right); // Number
    }

    #[test]
    fn test_mixed_content_pattern() {
        let widths = [10, 15, 12, 18];
        let array = patterns::mixed_content(&widths);
        
        assert_eq!(array.configs.len(), 4);
        assert_eq!(array.get_config(0).alignment, crate::alignment::Alignment::Left);
        assert_eq!(array.get_config(1).alignment, crate::alignment::Alignment::Right);
        assert_eq!(array.get_config(2).alignment, crate::alignment::Alignment::Left);
        assert_eq!(array.get_config(3).alignment, crate::alignment::Alignment::Right);
    }

    #[test]
    fn test_centered_headers_pattern() {
        let array = patterns::centered_headers(12, 3);
        
        assert_eq!(array.configs.len(), 3);
        for i in 0..3 {
            assert_eq!(array.get_config(i).width, Some(12));
            assert_eq!(array.get_config(i).alignment, crate::alignment::Alignment::Center);
        }
    }

    #[test]
    fn test_repeating_pattern() {
        let pattern = vec![
            crate::alignment::ColumnConfig::new().with_width(10),
            crate::alignment::ColumnConfig::new().with_width(20),
        ];
        
        let array = patterns::repeating_pattern(pattern);
        assert!(array.repeat_pattern);
        
        // Should repeat the pattern
        assert_eq!(array.get_config(0).width, Some(10));
        assert_eq!(array.get_config(1).width, Some(20));
        assert_eq!(array.get_config(2).width, Some(10)); // Repeats
        assert_eq!(array.get_config(3).width, Some(20)); // Repeats
    }

    #[test]
    fn test_empty_config_array() {
        let array = ColumnConfigArray::new(vec![]);
        
        // Should use default config for all columns
        let config = array.get_config(0);
        assert_eq!(config.width, None);
        assert_eq!(config.alignment, crate::alignment::Alignment::Left);
    }

    #[test]
    fn test_custom_default_config() {
        let custom_default = crate::alignment::ColumnConfig::new()
            .with_width(25)
            .with_alignment(crate::alignment::Alignment::Center)
            .with_padding(Padding::symmetric(2));
            
        let array = ColumnConfigArray::new(vec![])
            .with_default_config(custom_default);
        
        let config = array.get_config(0);
        assert_eq!(config.width, Some(25));
        assert_eq!(config.alignment, crate::alignment::Alignment::Center);
        assert_eq!(config.padding.left, 2);
        assert_eq!(config.padding.right, 2);
    }
}