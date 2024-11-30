use std::collections::HashMap;

/// Performance optimization utilities for table rendering
pub struct PerformanceConfig {
    pub enable_caching: bool,
    pub cache_size_limit: usize,
    pub use_string_pool: bool,
    pub optimize_memory: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_caching: true,
            cache_size_limit: 1000,
            use_string_pool: true,
            optimize_memory: true,
        }
    }
}

impl PerformanceConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_caching(mut self, enabled: bool) -> Self {
        self.enable_caching = enabled;
        self
    }
    
    pub fn with_cache_limit(mut self, limit: usize) -> Self {
        self.cache_size_limit = limit;
        self
    }
    
    pub fn with_string_pool(mut self, enabled: bool) -> Self {
        self.use_string_pool = enabled;
        self
    }
    
    pub fn with_memory_optimization(mut self, enabled: bool) -> Self {
        self.optimize_memory = enabled;
        self
    }
}

/// Cache for expensive operations like text alignment and padding
pub struct RenderCache {
    alignment_cache: HashMap<(String, usize, crate::alignment::Alignment), String>,
    padding_cache: HashMap<(String, crate::padding::Padding), String>,
    truncation_cache: HashMap<(String, usize, String), String>,
    size_limit: usize,
}

impl RenderCache {
    pub fn new(size_limit: usize) -> Self {
        Self {
            alignment_cache: HashMap::new(),
            padding_cache: HashMap::new(),
            truncation_cache: HashMap::new(),
            size_limit,
        }
    }
    
    pub fn get_aligned_text(
        &mut self, 
        text: &str, 
        width: usize, 
        alignment: crate::alignment::Alignment
    ) -> String {
        let key = (text.to_string(), width, alignment);
        
        if let Some(cached) = self.alignment_cache.get(&key) {
            return cached.clone();
        }
        
        let result = crate::alignment::align_text(text, width, alignment);
        
        if self.alignment_cache.len() < self.size_limit {
            self.alignment_cache.insert(key, result.clone());
        }
        
        result
    }
    
    pub fn get_padded_text(
        &mut self, 
        text: &str, 
        padding: crate::padding::Padding
    ) -> String {
        let key = (text.to_string(), padding);
        
        if let Some(cached) = self.padding_cache.get(&key) {
            return cached.clone();
        }
        
        let result = crate::padding::apply_padding(text, padding);
        
        if self.padding_cache.len() < self.size_limit {
            self.padding_cache.insert(key, result.clone());
        }
        
        result
    }
    
    pub fn get_truncated_text(
        &mut self, 
        text: &str, 
        max_width: usize, 
        ellipsis: &str
    ) -> String {
        let key = (text.to_string(), max_width, ellipsis.to_string());
        
        if let Some(cached) = self.truncation_cache.get(&key) {
            return cached.clone();
        }
        
        let config = crate::truncation::TruncationConfig::new()
            .with_max_width(max_width)
            .with_ellipsis(ellipsis.to_string());
            
        let result = crate::truncation::truncate_text(text, &config);
        
        if self.truncation_cache.len() < self.size_limit {
            self.truncation_cache.insert(key, result.clone());
        }
        
        result
    }
    
    pub fn clear(&mut self) {
        self.alignment_cache.clear();
        self.padding_cache.clear();
        self.truncation_cache.clear();
    }
    
    pub fn cache_size(&self) -> usize {
        self.alignment_cache.len() + self.padding_cache.len() + self.truncation_cache.len()
    }
}

/// String pool for reusing common strings to reduce allocations
pub struct StringPool {
    pool: HashMap<String, String>,
    max_size: usize,
}

impl StringPool {
    pub fn new(max_size: usize) -> Self {
        Self {
            pool: HashMap::new(),
            max_size,
        }
    }
    
    pub fn intern(&mut self, s: &str) -> &str {
        if self.pool.contains_key(s) {
            self.pool.get(s).unwrap()
        } else if self.pool.len() < self.max_size {
            let owned = s.to_string();
            self.pool.insert(owned.clone(), owned);
            self.pool.get(s).unwrap()
        } else {
            // Pool is full, return a new string (not optimal but prevents unbounded growth)
            s
        }
    }
    
    pub fn clear(&mut self) {
        self.pool.clear();
    }
    
    pub fn size(&self) -> usize {
        self.pool.len()
    }
}

/// Fast table renderer with performance optimizations
pub struct FastTableRenderer {
    cache: RenderCache,
    string_pool: StringPool,
    config: PerformanceConfig,
}

impl FastTableRenderer {
    pub fn new(config: PerformanceConfig) -> Self {
        Self {
            cache: RenderCache::new(config.cache_size_limit),
            string_pool: StringPool::new(config.cache_size_limit / 2),
            config,
        }
    }
    
    pub fn render_table(
        &mut self,
        data: &crate::TableData,
        border: &crate::BorderChars,
        options: &crate::RenderOptions,
        column_configs: &[crate::alignment::ColumnConfig],
    ) -> Result<String, String> {
        crate::validate_table_data(data)?;
        
        if data.is_empty() {
            return Ok(String::new());
        }

        // Pre-allocate result string with estimated size
        let estimated_size = self.estimate_output_size(data, column_configs);
        let mut result = if self.config.optimize_memory {
            String::with_capacity(estimated_size)
        } else {
            String::new()
        };
        
        // Calculate column widths
        let auto_widths = self.calculate_column_widths_fast(data);
        let mut column_widths = Vec::with_capacity(data.column_count());
        
        for i in 0..data.column_count() {
            let config = column_configs.get(i).unwrap_or(&crate::alignment::ColumnConfig::default());
            let content_width = config.width.unwrap_or(auto_widths[i]);
            let total_width = content_width + config.padding.total();
            column_widths.push(total_width);
        }
        
        // Top border
        if options.show_top_border {
            self.render_border_line(&mut result, &column_widths, border, true, false);
        }
        
        // Data rows
        for (row_idx, row) in data.rows.iter().enumerate() {
            self.render_data_row(&mut result, row, &column_widths, &auto_widths, column_configs, border)?;
            
            // Row separator (optional, not after last row)
            if options.show_row_separators && row_idx < data.rows.len() - 1 {
                self.render_border_line(&mut result, &column_widths, border, false, false);
            }
        }
        
        // Bottom border
        if options.show_bottom_border {
            self.render_border_line(&mut result, &column_widths, border, false, true);
        }
        
        Ok(result)
    }
    
    fn calculate_column_widths_fast(&self, data: &crate::TableData) -> Vec<usize> {
        let mut widths = vec![0; data.column_count()];
        
        // Single pass through data
        for row in &data.rows {
            for (i, cell) in row.iter().enumerate().take(data.column_count()) {
                let len = if self.config.optimize_memory {
                    // Use direct length calculation instead of display_width for better performance
                    cell.len()
                } else {
                    crate::unicode::display_width(cell)
                };
                widths[i] = widths[i].max(len);
            }
        }
        
        widths
    }
    
    fn render_data_row(
        &mut self,
        result: &mut String,
        row: &[String],
        column_widths: &[usize],
        auto_widths: &[usize],
        column_configs: &[crate::alignment::ColumnConfig],
        border: &crate::BorderChars,
    ) -> Result<(), String> {
        result.push(border.vertical);
        
        for (i, cell) in row.iter().enumerate() {
            let config = column_configs.get(i).unwrap_or(&crate::alignment::ColumnConfig::default());
            let content_width = config.width.unwrap_or(auto_widths[i]);
            
            // Process cell content with caching if enabled
            let processed_cell = if self.config.enable_caching {
                // Apply truncation
                let truncated = if let Some(max_width) = config.truncation.max_width {
                    self.cache.get_truncated_text(cell, max_width, &config.truncation.ellipsis)
                } else {
                    cell.clone()
                };
                
                // Apply alignment
                let aligned = self.cache.get_aligned_text(&truncated, content_width, config.alignment);
                
                // Apply padding
                self.cache.get_padded_text(&aligned, config.padding)
            } else {
                // Direct processing without caching
                let truncated_cell = crate::truncation::truncate_text(cell, &config.truncation);
                let aligned_cell = crate::alignment::align_text(&truncated_cell, content_width, config.alignment);
                crate::padding::apply_padding(&aligned_cell, config.padding)
            };
            
            result.push_str(&processed_cell);
            result.push(border.vertical);
        }
        result.push('\n');
        Ok(())
    }
    
    fn render_border_line(
        &mut self,
        result: &mut String,
        column_widths: &[usize],
        border: &crate::BorderChars,
        is_top: bool,
        is_bottom: bool,
    ) {
        if is_top {
            result.push(border.top_left);
            for (i, &width) in column_widths.iter().enumerate() {
                self.push_repeated_char(result, border.horizontal, width);
                if i < column_widths.len() - 1 {
                    result.push(border.top_junction);
                }
            }
            result.push(border.top_right);
        } else if is_bottom {
            result.push(border.bottom_left);
            for (i, &width) in column_widths.iter().enumerate() {
                self.push_repeated_char(result, border.horizontal, width);
                if i < column_widths.len() - 1 {
                    result.push(border.bottom_junction);
                }
            }
            result.push(border.bottom_right);
        } else {
            result.push('├');
            for (i, &width) in column_widths.iter().enumerate() {
                self.push_repeated_char(result, border.horizontal, width);
                if i < column_widths.len() - 1 {
                    result.push('┼');
                }
            }
            result.push('┤');
        }
        result.push('\n');
    }
    
    fn push_repeated_char(&self, result: &mut String, ch: char, count: usize) {
        // Optimized character repetition
        if count > 0 {
            if count == 1 {
                result.push(ch);
            } else {
                result.push_str(&ch.to_string().repeat(count));
            }
        }
    }
    
    fn estimate_output_size(
        &self,
        data: &crate::TableData,
        column_configs: &[crate::alignment::ColumnConfig],
    ) -> usize {
        if data.is_empty() {
            return 0;
        }
        
        // Rough estimation based on content size and formatting overhead
        let content_size: usize = data.rows.iter()
            .map(|row| row.iter().map(|cell| cell.len()).sum::<usize>())
            .sum();
        
        let padding_overhead = column_configs.len() * 4; // Average padding per column
        let border_overhead = data.rows.len() * (column_configs.len() * 3); // Borders and separators
        
        content_size + padding_overhead + border_overhead
    }
    
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        self.string_pool.clear();
    }
    
    pub fn cache_stats(&self) -> (usize, usize) {
        (self.cache.cache_size(), self.string_pool.size())
    }
}

/// Batch processing utilities for large datasets
pub struct BatchProcessor {
    batch_size: usize,
}

impl BatchProcessor {
    pub fn new(batch_size: usize) -> Self {
        Self { batch_size }
    }
    
    /// Process table data in batches to reduce memory usage
    pub fn process_in_batches<F>(
        &self,
        data: &crate::TableData,
        mut processor: F,
    ) -> Result<Vec<String>, String>
    where
        F: FnMut(&[crate::Row]) -> Result<String, String>,
    {
        let mut results = Vec::new();
        
        for chunk in data.rows.chunks(self.batch_size) {
            let batch_result = processor(chunk)?;
            results.push(batch_result);
        }
        
        Ok(results)
    }
    
    /// Estimate memory usage for a table
    pub fn estimate_memory_usage(&self, data: &crate::TableData) -> usize {
        data.rows.iter()
            .map(|row| row.iter().map(|cell| cell.len()).sum::<usize>())
            .sum::<usize>() * 2 // Rough estimate with formatting overhead
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_cache() {
        let mut cache = RenderCache::new(10);
        
        // Test alignment caching
        let result1 = cache.get_aligned_text("test", 10, crate::alignment::Alignment::Center);
        let result2 = cache.get_aligned_text("test", 10, crate::alignment::Alignment::Center);
        
        // Second call should use cached result
        assert_eq!(result1, result2);
        assert_eq!(cache.cache_size(), 1);
    }

    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new(5);
        
        let s1 = pool.intern("hello");
        let s2 = pool.intern("hello");
        
        // Should return same reference
        assert_eq!(s1, s2);
        assert_eq!(pool.size(), 1);
    }

    #[test]
    fn test_fast_table_renderer() {
        let config = PerformanceConfig::new()
            .with_caching(true)
            .with_memory_optimization(true);
        
        let mut renderer = FastTableRenderer::new(config);
        
        let data = crate::TableData::new(vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "25".to_string()],
        ]);
        
        let column_configs = vec![
            crate::alignment::ColumnConfig::new().with_width(8),
            crate::alignment::ColumnConfig::new().with_width(5),
        ];
        
        let result = renderer.render_table(
            &data,
            &crate::BorderChars::default(),
            &crate::RenderOptions::default(),
            &column_configs,
        ).unwrap();
        
        assert!(result.contains("Name"));
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
        
        // Check that cache was used
        let (cache_size, _) = renderer.cache_stats();
        assert!(cache_size > 0);
    }

    #[test]
    fn test_performance_config() {
        let config = PerformanceConfig::new()
            .with_caching(false)
            .with_cache_limit(500)
            .with_string_pool(false)
            .with_memory_optimization(false);
        
        assert!(!config.enable_caching);
        assert_eq!(config.cache_size_limit, 500);
        assert!(!config.use_string_pool);
        assert!(!config.optimize_memory);
    }

    #[test]
    fn test_batch_processor() {
        let processor = BatchProcessor::new(2);
        
        let data = crate::TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string(), "2".to_string()],
            vec!["3".to_string(), "4".to_string()],
            vec!["5".to_string(), "6".to_string()],
        ]);
        
        let results = processor.process_in_batches(&data, |batch| {
            Ok(format!("Batch size: {}", batch.len()))
        }).unwrap();
        
        assert_eq!(results.len(), 2); // 4 rows / 2 batch size = 2 batches
        assert_eq!(results[0], "Batch size: 2");
        assert_eq!(results[1], "Batch size: 2");
    }

    #[test]
    fn test_memory_estimation() {
        let processor = BatchProcessor::new(10);
        
        let data = crate::TableData::new(vec![
            vec!["Hello".to_string(), "World".to_string()],
            vec!["Test".to_string(), "Data".to_string()],
        ]);
        
        let estimated = processor.estimate_memory_usage(&data);
        assert!(estimated > 0);
        
        // Should be roughly 2x the content size
        let content_size = "HelloWorldTestData".len();
        assert!(estimated >= content_size);
    }

    #[test]
    fn test_cache_size_limit() {
        let mut cache = RenderCache::new(2);
        
        // Fill cache to limit
        cache.get_aligned_text("test1", 10, crate::alignment::Alignment::Left);
        cache.get_aligned_text("test2", 10, crate::alignment::Alignment::Left);
        
        assert_eq!(cache.cache_size(), 2);
        
        // Adding more should not exceed limit
        cache.get_aligned_text("test3", 10, crate::alignment::Alignment::Left);
        assert_eq!(cache.cache_size(), 2); // Should not exceed limit
    }

    #[test]
    fn test_fast_column_width_calculation() {
        let config = PerformanceConfig::new().with_memory_optimization(true);
        let renderer = FastTableRenderer::new(config);
        
        let data = crate::TableData::new(vec![
            vec!["Short".to_string(), "Very Long Text".to_string()],
            vec!["A".to_string(), "B".to_string()],
        ]);
        
        let widths = renderer.calculate_column_widths_fast(&data);
        assert_eq!(widths[0], 5); // "Short"
        assert_eq!(widths[1], 14); // "Very Long Text"
    }
}