/// Fluent builder patterns and high-level API for table construction and rendering
use std::collections::HashMap;

/// High-level table builder with fluent API
pub struct TableBuilder {
    data: crate::TableData,
    column_configs: Vec<crate::alignment::ColumnConfig>,
    border_style: String,
    render_options: crate::RenderOptions,
    validation_config: Option<crate::validation::ValidationConfig>,
    performance_config: Option<crate::performance::PerformanceConfig>,
}

impl TableBuilder {
    /// Create a new table builder
    pub fn new() -> Self {
        Self {
            data: crate::TableData::new(vec![]),
            column_configs: Vec::new(),
            border_style: "default".to_string(),
            render_options: crate::RenderOptions::default(),
            validation_config: None,
            performance_config: None,
        }
    }

    /// Create builder from existing data
    pub fn from_data(data: crate::TableData) -> Self {
        let column_count = data.column_count();
        Self {
            column_configs: vec![crate::alignment::ColumnConfig::default(); column_count],
            data,
            border_style: "default".to_string(),
            render_options: crate::RenderOptions::default(),
            validation_config: None,
            performance_config: None,
        }
    }

    /// Create builder from 2D vector
    pub fn from_rows(rows: Vec<Vec<String>>) -> Self {
        Self::from_data(crate::TableData::new(rows))
    }

    /// Create builder from CSV string
    pub fn from_csv(csv: &str, separator: Option<&str>) -> Result<Self, String> {
        let sep = separator.unwrap_or(",");
        let mut rows = Vec::new();

        for line in csv.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let row: Vec<String> = line
                .split(sep)
                .map(|field| field.trim_matches('"').trim().to_string())
                .collect();

            rows.push(row);
        }

        if rows.is_empty() {
            return Err("No data found in CSV".to_string());
        }

        Ok(Self::from_rows(rows))
    }

    /// Add a single row to the table
    pub fn add_row(mut self, row: Vec<String>) -> Self {
        self.data.rows.push(row);
        self
    }

    /// Add multiple rows to the table
    pub fn add_rows(mut self, rows: Vec<Vec<String>>) -> Self {
        self.data.rows.extend(rows);
        self
    }

    /// Add a header row
    pub fn with_header(self, header: Vec<String>) -> Self {
        let mut rows = vec![header];
        rows.extend(self.data.rows);
        Self::from_data(crate::TableData::new(rows))
            .with_all_configs_from(&self)
    }

    /// Set border style
    pub fn with_border_style(mut self, style: &str) -> Self {
        self.border_style = style.to_string();
        self
    }

    /// Set render options
    pub fn with_render_options(mut self, options: crate::RenderOptions) -> Self {
        self.render_options = options;
        self
    }

    /// Enable top border
    pub fn with_top_border(mut self) -> Self {
        self.render_options.show_top_border = true;
        self
    }

    /// Enable bottom border
    pub fn with_bottom_border(mut self) -> Self {
        self.render_options.show_bottom_border = true;
        self
    }

    /// Enable row separators
    pub fn with_row_separators(mut self) -> Self {
        self.render_options.show_row_separators = true;
        self
    }

    /// Set column configuration for a specific column
    pub fn with_column_config(mut self, column: usize, config: crate::alignment::ColumnConfig) -> Self {
        // Ensure we have enough column configs
        while self.column_configs.len() <= column {
            self.column_configs.push(crate::alignment::ColumnConfig::default());
        }
        self.column_configs[column] = config;
        self
    }

    /// Set column configurations for all columns
    pub fn with_column_configs(mut self, configs: Vec<crate::alignment::ColumnConfig>) -> Self {
        self.column_configs = configs;
        self
    }

    /// Set alignment for all columns
    pub fn with_alignment(mut self, alignment: crate::alignment::Alignment) -> Self {
        let column_count = self.data.column_count();
        self.column_configs = vec![
            crate::alignment::ColumnConfig::new().with_alignment(alignment);
            column_count
        ];
        self
    }

    /// Set width for all columns
    pub fn with_width(mut self, width: usize) -> Self {
        for config in &mut self.column_configs {
            *config = config.clone().with_width(width);
        }
        self
    }

    /// Set padding for all columns
    pub fn with_padding(mut self, padding: crate::padding::Padding) -> Self {
        for config in &mut self.column_configs {
            *config = config.clone().with_padding(padding);
        }
        self
    }

    /// Enable validation with default config
    pub fn with_validation(mut self) -> Self {
        self.validation_config = Some(crate::validation::ValidationConfig::default());
        self
    }

    /// Enable validation with custom config
    pub fn with_validation_config(mut self, config: crate::validation::ValidationConfig) -> Self {
        self.validation_config = Some(config);
        self
    }

    /// Enable performance optimizations
    pub fn with_performance_optimization(mut self) -> Self {
        self.performance_config = Some(crate::performance::PerformanceConfig::default());
        self
    }

    /// Enable performance optimizations with custom config
    pub fn with_performance_config(mut self, config: crate::performance::PerformanceConfig) -> Self {
        self.performance_config = Some(config);
        self
    }

    /// Build and render the table
    pub fn build(self) -> Result<String, String> {
        // Validate if validation is enabled
        if let Some(validation_config) = &self.validation_config {
            let validator = crate::validation::TableValidator::new(validation_config.clone());
            let result = validator.validate_with_config(&self.data, &self.column_configs);
            
            if !result.is_valid {
                return Err(format!("Table validation failed:\n{}", result.report()));
            }
        }

        // Get border chars
        let border = crate::get_border_style(&self.border_style)?;

        // Render with performance optimization if enabled
        if let Some(perf_config) = self.performance_config {
            let mut renderer = crate::performance::FastTableRenderer::new(perf_config);
            renderer.render_table(&self.data, &border, &self.render_options, &self.column_configs)
        } else {
            crate::render_table_with_column_config(&self.data, &border, &self.render_options, &self.column_configs)
        }
    }

    /// Build without validation (for cases where validation might fail but rendering should proceed)
    pub fn build_unchecked(self) -> Result<String, String> {
        let border = crate::get_border_style(&self.border_style)?;
        
        if let Some(perf_config) = self.performance_config {
            let mut renderer = crate::performance::FastTableRenderer::new(perf_config);
            renderer.render_table(&self.data, &border, &self.render_options, &self.column_configs)
        } else {
            crate::render_table_with_column_config(&self.data, &border, &self.render_options, &self.column_configs)
        }
    }

    /// Get the table data without rendering
    pub fn get_data(&self) -> &crate::TableData {
        &self.data
    }

    /// Preview the table configuration
    pub fn preview_config(&self) -> String {
        let mut preview = String::new();
        preview.push_str("=== Table Configuration Preview ===\n");
        preview.push_str(&format!("Rows: {}\n", self.data.row_count()));
        preview.push_str(&format!("Columns: {}\n", self.data.column_count()));
        preview.push_str(&format!("Border Style: {}\n", self.border_style));
        preview.push_str(&format!("Render Options: {:?}\n", self.render_options));
        preview.push_str(&format!("Column Configs: {} configured\n", self.column_configs.len()));
        preview.push_str(&format!("Validation: {}\n", if self.validation_config.is_some() { "Enabled" } else { "Disabled" }));
        preview.push_str(&format!("Performance: {}\n", if self.performance_config.is_some() { "Enabled" } else { "Disabled" }));
        preview
    }

    /// Copy all configurations from another builder
    fn with_all_configs_from(mut self, other: &Self) -> Self {
        self.column_configs = other.column_configs.clone();
        self.border_style = other.border_style.clone();
        self.render_options = other.render_options.clone();
        self.validation_config = other.validation_config.clone();
        self.performance_config = other.performance_config.clone();
        self
    }
}

impl Default for TableBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Specialized builders for common use cases

/// Builder for creating tables with specific themes
pub struct ThemedTableBuilder {
    builder: TableBuilder,
}

impl ThemedTableBuilder {
    /// Create a minimalist table
    pub fn minimal() -> TableBuilder {
        TableBuilder::new()
            .with_border_style("ascii")
            .with_alignment(crate::alignment::Alignment::Left)
            .with_padding(crate::padding::Padding::new(1, 1))
    }

    /// Create a fancy Unicode table
    pub fn fancy() -> TableBuilder {
        TableBuilder::new()
            .with_border_style("default")
            .with_top_border()
            .with_bottom_border()
            .with_alignment(crate::alignment::Alignment::Center)
            .with_padding(crate::padding::Padding::symmetric(2))
    }

    /// Create a compact table
    pub fn compact() -> TableBuilder {
        TableBuilder::new()
            .with_border_style("void")
            .with_alignment(crate::alignment::Alignment::Left)
            .with_padding(crate::padding::Padding::new(0, 1))
    }

    /// Create a data table optimized for numeric data
    pub fn data_table() -> TableBuilder {
        TableBuilder::new()
            .with_border_style("default")
            .with_top_border()
            .with_bottom_border()
            .with_alignment(crate::alignment::Alignment::Right)
            .with_padding(crate::padding::Padding::symmetric(1))
    }

    /// Create a report-style table
    pub fn report() -> TableBuilder {
        TableBuilder::new()
            .with_border_style("honeywell")
            .with_top_border()
            .with_bottom_border()
            .with_row_separators()
            .with_alignment(crate::alignment::Alignment::Left)
            .with_padding(crate::padding::Padding::new(1, 2))
    }
}

/// Column builder for fluent column configuration
pub struct ColumnBuilder {
    config: crate::alignment::ColumnConfig,
}

impl ColumnBuilder {
    pub fn new() -> Self {
        Self {
            config: crate::alignment::ColumnConfig::default(),
        }
    }

    pub fn width(mut self, width: usize) -> Self {
        self.config = self.config.with_width(width);
        self
    }

    pub fn alignment(mut self, alignment: crate::alignment::Alignment) -> Self {
        self.config = self.config.with_alignment(alignment);
        self
    }

    pub fn padding(mut self, padding: crate::padding::Padding) -> Self {
        self.config = self.config.with_padding(padding);
        self
    }

    pub fn left_aligned(self) -> Self {
        self.alignment(crate::alignment::Alignment::Left)
    }

    pub fn center_aligned(self) -> Self {
        self.alignment(crate::alignment::Alignment::Center)
    }

    pub fn right_aligned(self) -> Self {
        self.alignment(crate::alignment::Alignment::Right)
    }

    pub fn truncate(mut self, max_width: usize) -> Self {
        self.config = self.config.with_truncation(
            crate::truncation::TruncationConfig::new()
                .with_max_width(max_width)
        );
        self
    }

    pub fn wrap(mut self, width: usize) -> Self {
        self.config = self.config.with_wrapping(crate::wrapping::WrapConfig::new(width));
        self
    }

    pub fn build(self) -> crate::alignment::ColumnConfig {
        self.config
    }
}

impl Default for ColumnBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Quick builder functions for common scenarios
pub mod quick {
    use super::*;

    /// Create a simple table from data
    pub fn table(rows: Vec<Vec<String>>) -> Result<String, String> {
        TableBuilder::from_rows(rows).build()
    }

    /// Create a table with header
    pub fn table_with_header(header: Vec<String>, rows: Vec<Vec<String>>) -> Result<String, String> {
        TableBuilder::from_rows(rows)
            .with_header(header)
            .with_top_border()
            .with_bottom_border()
            .build()
    }

    /// Create a data table (right-aligned, borders)
    pub fn data_table(rows: Vec<Vec<String>>) -> Result<String, String> {
        ThemedTableBuilder::data_table()
            .add_rows(rows)
            .build()
    }

    /// Create a report table (with separators)
    pub fn report_table(header: Vec<String>, rows: Vec<Vec<String>>) -> Result<String, String> {
        ThemedTableBuilder::report()
            .with_header(header)
            .add_rows(rows)
            .build()
    }

    /// Create a minimal ASCII table
    pub fn ascii_table(rows: Vec<Vec<String>>) -> Result<String, String> {
        ThemedTableBuilder::minimal()
            .add_rows(rows)
            .build()
    }

    /// Create table from CSV with auto-detection
    pub fn csv_table(csv: &str) -> Result<String, String> {
        TableBuilder::from_csv(csv, None)?
            .with_top_border()
            .with_bottom_border()
            .build()
    }
}

/// Macro for quick table creation
#[macro_export]
macro_rules! table {
    // Table with header
    (header: [$($header:expr),*], rows: [$([$($cell:expr),*]),*]) => {
        $crate::builder::quick::table_with_header(
            vec![$($header.to_string()),*],
            vec![$(vec![$($cell.to_string()),*]),*]
        )
    };
    
    // Table without header
    (rows: [$([$($cell:expr),*]),*]) => {
        $crate::builder::quick::table(
            vec![$(vec![$($cell.to_string()),*]),*]
        )
    };
    
    // Simple syntax
    ($([$($cell:expr),*]),*) => {
        $crate::builder::quick::table(
            vec![$(vec![$($cell.to_string()),*]),*]
        )
    };
}

/// Table template system for reusable configurations
pub struct TableTemplate {
    name: String,
    builder_fn: Box<dyn Fn() -> TableBuilder>,
}

impl TableTemplate {
    pub fn new<F>(name: &str, builder_fn: F) -> Self
    where
        F: Fn() -> TableBuilder + 'static,
    {
        Self {
            name: name.to_string(),
            builder_fn: Box::new(builder_fn),
        }
    }

    pub fn create(&self) -> TableBuilder {
        (self.builder_fn)()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Template registry for managing reusable table configurations
pub struct TemplateRegistry {
    templates: HashMap<String, TableTemplate>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            templates: HashMap::new(),
        };
        
        // Register built-in templates
        registry.register_builtin_templates();
        registry
    }

    pub fn register(&mut self, template: TableTemplate) {
        self.templates.insert(template.name().to_string(), template);
    }

    pub fn get(&self, name: &str) -> Option<TableBuilder> {
        self.templates.get(name).map(|t| t.create())
    }

    pub fn list_templates(&self) -> Vec<&str> {
        self.templates.keys().map(|s| s.as_str()).collect()
    }

    fn register_builtin_templates(&mut self) {
        self.register(TableTemplate::new("minimal", || ThemedTableBuilder::minimal()));
        self.register(TableTemplate::new("fancy", || ThemedTableBuilder::fancy()));
        self.register(TableTemplate::new("compact", || ThemedTableBuilder::compact()));
        self.register(TableTemplate::new("data", || ThemedTableBuilder::data_table()));
        self.register(TableTemplate::new("report", || ThemedTableBuilder::report()));
        
        // Web-optimized template
        self.register(TableTemplate::new("web", || {
            TableBuilder::new()
                .with_border_style("void")
                .with_validation_config(crate::validation::ValidationConfig::web_optimized())
                .with_alignment(crate::alignment::Alignment::Left)
        }));
    }
}

impl Default for TemplateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global template registry instance
static mut GLOBAL_REGISTRY: Option<TemplateRegistry> = None;
static mut REGISTRY_INIT: std::sync::Once = std::sync::Once::new();

/// Get the global template registry
pub fn global_templates() -> &'static TemplateRegistry {
    unsafe {
        REGISTRY_INIT.call_once(|| {
            GLOBAL_REGISTRY = Some(TemplateRegistry::new());
        });
        GLOBAL_REGISTRY.as_ref().unwrap()
    }
}

/// Create a table from a template
pub fn from_template(template_name: &str) -> Option<TableBuilder> {
    global_templates().get(template_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_builder_basic() {
        let result = TableBuilder::new()
            .add_row(vec!["Name".to_string(), "Age".to_string()])
            .add_row(vec!["Alice".to_string(), "30".to_string()])
            .add_row(vec!["Bob".to_string(), "25".to_string()])
            .build();

        assert!(result.is_ok());
        let table = result.unwrap();
        assert!(table.contains("Name"));
        assert!(table.contains("Alice"));
    }

    #[test]
    fn test_table_builder_from_data() {
        let data = crate::TableData::new(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ]);

        let result = TableBuilder::from_data(data)
            .with_border_style("ascii")
            .build();

        assert!(result.is_ok());
        assert!(result.unwrap().contains("A"));
    }

    #[test]
    fn test_table_builder_from_csv() {
        let csv = "Name,Age\nAlice,30\nBob,25";
        let result = TableBuilder::from_csv(csv, None);

        assert!(result.is_ok());
        let table = result.unwrap().build();
        assert!(table.is_ok());
        assert!(table.unwrap().contains("Alice"));
    }

    #[test]
    fn test_themed_builders() {
        let minimal = ThemedTableBuilder::minimal()
            .add_row(vec!["Test".to_string()])
            .build();
        assert!(minimal.is_ok());

        let fancy = ThemedTableBuilder::fancy()
            .add_row(vec!["Test".to_string()])
            .build();
        assert!(fancy.is_ok());

        let data_table = ThemedTableBuilder::data_table()
            .add_row(vec!["123".to_string()])
            .build();
        assert!(data_table.is_ok());
    }

    #[test]
    fn test_column_builder() {
        let config = ColumnBuilder::new()
            .width(10)
            .left_aligned()
            .padding(crate::padding::Padding::symmetric(2))
            .build();

        assert_eq!(config.width, Some(10));
        assert_eq!(config.alignment, crate::alignment::Alignment::Left);
        assert_eq!(config.padding.left, 2);
    }

    #[test]
    fn test_quick_builders() {
        let result = quick::table(vec![
            vec!["A".to_string(), "B".to_string()],
            vec!["1".to_string(), "2".to_string()],
        ]);
        assert!(result.is_ok());

        let header_result = quick::table_with_header(
            vec!["Name".to_string(), "Age".to_string()],
            vec![vec!["Alice".to_string(), "30".to_string()]],
        );
        assert!(header_result.is_ok());

        let csv_result = quick::csv_table("A,B\n1,2");
        assert!(csv_result.is_ok());
    }

    #[test]
    fn test_table_macro() {
        let result = table!(
            header: ["Name", "Age"],
            rows: [["Alice", "30"], ["Bob", "25"]]
        );
        assert!(result.is_ok());

        let simple_result = table!([["A", "B"], ["1", "2"]]);
        assert!(simple_result.is_ok());
    }

    #[test]
    fn test_template_registry() {
        let registry = TemplateRegistry::new();
        let templates = registry.list_templates();
        
        assert!(templates.contains(&"minimal"));
        assert!(templates.contains(&"fancy"));
        assert!(templates.contains(&"data"));

        let minimal_builder = registry.get("minimal");
        assert!(minimal_builder.is_some());
    }

    #[test]
    fn test_global_templates() {
        let builder = from_template("minimal");
        assert!(builder.is_some());

        let table = builder.unwrap()
            .add_row(vec!["Test".to_string()])
            .build();
        assert!(table.is_ok());
    }

    #[test]
    fn test_builder_with_validation() {
        let result = TableBuilder::new()
            .add_row(vec!["A".to_string(), "B".to_string()])
            .add_row(vec!["1".to_string(), "2".to_string()])
            .with_validation()
            .build();

        assert!(result.is_ok());

        // Test validation failure
        let invalid_result = TableBuilder::new()
            .add_row(vec!["A".to_string(), "B".to_string()])
            .add_row(vec!["1".to_string()]) // Missing column
            .with_validation()
            .build();

        assert!(invalid_result.is_err());
    }

    #[test]
    fn test_builder_preview() {
        let builder = TableBuilder::new()
            .add_row(vec!["Test".to_string()])
            .with_border_style("ascii")
            .with_validation();

        let preview = builder.preview_config();
        assert!(preview.contains("Configuration Preview"));
        assert!(preview.contains("Border Style: ascii"));
        assert!(preview.contains("Validation: Enabled"));
    }

    #[test]
    fn test_builder_fluent_api() {
        let result = TableBuilder::new()
            .add_row(vec!["Name".to_string(), "Score".to_string()])
            .add_row(vec!["Alice".to_string(), "95".to_string()])
            .with_header(vec!["Player".to_string(), "Points".to_string()])
            .with_alignment(crate::alignment::Alignment::Center)
            .with_width(10)
            .with_top_border()
            .with_bottom_border()
            .with_performance_optimization()
            .build();

        assert!(result.is_ok());
        let table = result.unwrap();
        assert!(table.contains("Player"));
        assert!(table.contains("Alice"));
    }
}