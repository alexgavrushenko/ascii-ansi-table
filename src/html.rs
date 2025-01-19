/// HTML table generation and conversion utilities
use std::fmt::Write;

/// Configuration for HTML table generation
#[derive(Debug, Clone)]
pub struct HtmlConfig {
    pub table_class: Option<String>,
    pub table_id: Option<String>,
    pub table_style: Option<String>,
    pub include_thead: bool,
    pub include_tbody: bool,
    pub cell_padding: Option<String>,
    pub border_collapse: bool,
    pub responsive: bool,
    pub striped_rows: bool,
    pub hover_effect: bool,
    pub custom_attributes: Vec<(String, String)>,
}

impl Default for HtmlConfig {
    fn default() -> Self {
        Self {
            table_class: None,
            table_id: None,
            table_style: None,
            include_thead: true,
            include_tbody: true,
            cell_padding: None,
            border_collapse: true,
            responsive: false,
            striped_rows: false,
            hover_effect: false,
            custom_attributes: Vec::new(),
        }
    }
}

impl HtmlConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_class(mut self, class: &str) -> Self {
        self.table_class = Some(class.to_string());
        self
    }

    pub fn with_id(mut self, id: &str) -> Self {
        self.table_id = Some(id.to_string());
        self
    }

    pub fn with_style(mut self, style: &str) -> Self {
        self.table_style = Some(style.to_string());
        self
    }

    pub fn with_cell_padding(mut self, padding: &str) -> Self {
        self.cell_padding = Some(padding.to_string());
        self
    }

    pub fn with_responsive(mut self) -> Self {
        self.responsive = true;
        self
    }

    pub fn with_striped_rows(mut self) -> Self {
        self.striped_rows = true;
        self
    }

    pub fn with_hover_effect(mut self) -> Self {
        self.hover_effect = true;
        self
    }

    pub fn without_thead(mut self) -> Self {
        self.include_thead = false;
        self
    }

    pub fn without_tbody(mut self) -> Self {
        self.include_tbody = false;
        self
    }

    pub fn without_border_collapse(mut self) -> Self {
        self.border_collapse = false;
        self
    }

    pub fn with_attribute(mut self, name: &str, value: &str) -> Self {
        self.custom_attributes.push((name.to_string(), value.to_string()));
        self
    }

    /// Bootstrap 5 preset
    pub fn bootstrap() -> Self {
        Self::new()
            .with_class("table table-striped table-hover")
            .with_responsive()
            .with_striped_rows()
            .with_hover_effect()
    }

    /// Material Design preset
    pub fn material() -> Self {
        Self::new()
            .with_class("mdc-data-table__table")
            .with_style("box-shadow: 0 2px 4px rgba(0,0,0,0.1); border-radius: 4px;")
            .with_cell_padding("12px")
    }

    /// Minimal clean preset
    pub fn minimal() -> Self {
        Self::new()
            .with_style("border: 1px solid #ddd; border-collapse: collapse;")
            .with_cell_padding("8px")
    }

    /// Dark theme preset
    pub fn dark() -> Self {
        Self::new()
            .with_class("table-dark")
            .with_style("background-color: #2d3748; color: #e2e8f0; border-color: #4a5568;")
            .with_striped_rows()
    }
}

/// HTML table renderer with advanced formatting options
pub struct HtmlTableRenderer {
    config: HtmlConfig,
}

impl HtmlTableRenderer {
    pub fn new(config: HtmlConfig) -> Self {
        Self { config }
    }

    /// Render table data to HTML
    pub fn render(&self, data: &crate::TableData) -> Result<String, String> {
        if data.is_empty() {
            return Ok(String::new());
        }

        let mut html = String::new();

        // Generate CSS if needed
        if self.config.responsive || self.config.striped_rows || self.config.hover_effect {
            html.push_str(&self.generate_css());
        }

        // Opening table tag
        self.write_table_opening(&mut html)?;

        // Table head
        if self.config.include_thead && !data.rows.is_empty() {
            self.write_thead(&mut html, &data.rows[0])?;
        }

        // Table body
        if self.config.include_tbody {
            let data_rows = if self.config.include_thead && !data.rows.is_empty() {
                &data.rows[1..]
            } else {
                &data.rows
            };
            self.write_tbody(&mut html, data_rows)?;
        } else {
            // Write all rows as regular rows
            let rows_to_write = if self.config.include_thead && !data.rows.is_empty() {
                &data.rows[1..]
            } else {
                &data.rows
            };
            for (idx, row) in rows_to_write.iter().enumerate() {
                self.write_row(&mut html, row, idx, false)?;
            }
        }

        // Closing table tag
        html.push_str("</table>\n");

        Ok(html)
    }

    fn generate_css(&self) -> String {
        let mut css = String::from("<style>\n");

        if self.config.responsive {
            css.push_str(".responsive-table {\n");
            css.push_str("  overflow-x: auto;\n");
            css.push_str("}\n");
            css.push_str("@media (max-width: 768px) {\n");
            css.push_str("  .responsive-table table {\n");
            css.push_str("    font-size: 14px;\n");
            css.push_str("  }\n");
            css.push_str("}\n");
        }

        if self.config.striped_rows {
            css.push_str("table tr:nth-child(even) {\n");
            css.push_str("  background-color: #f8f9fa;\n");
            css.push_str("}\n");
        }

        if self.config.hover_effect {
            css.push_str("table tr:hover {\n");
            css.push_str("  background-color: #e9ecef;\n");
            css.push_str("  transition: background-color 0.15s ease;\n");
            css.push_str("}\n");
        }

        css.push_str("</style>\n");
        css
    }

    fn write_table_opening(&self, html: &mut String) -> Result<(), String> {
        if self.config.responsive {
            html.push_str("<div class=\"responsive-table\">\n");
        }

        html.push_str("<table");

        // Add ID
        if let Some(id) = &self.config.table_id {
            write!(html, " id=\"{}\"", html_escape(id)).map_err(|_| "Failed to write ID")?;
        }

        // Add class
        if let Some(class) = &self.config.table_class {
            write!(html, " class=\"{}\"", html_escape(class)).map_err(|_| "Failed to write class")?;
        }

        // Add style
        let mut style_parts = Vec::new();
        
        if self.config.border_collapse {
            style_parts.push("border-collapse: collapse".to_string());
        }

        if let Some(padding) = &self.config.cell_padding {
            style_parts.push(format!("td, th {{ padding: {}; }}", padding));
        }

        if let Some(custom_style) = &self.config.table_style {
            style_parts.push(custom_style.clone());
        }

        if !style_parts.is_empty() {
            write!(html, " style=\"{}\"", style_parts.join("; ")).map_err(|_| "Failed to write style")?;
        }

        // Add custom attributes
        for (name, value) in &self.config.custom_attributes {
            write!(html, " {}=\"{}\"", html_escape(name), html_escape(value))
                .map_err(|_| "Failed to write custom attribute")?;
        }

        html.push_str(">\n");
        Ok(())
    }

    fn write_thead(&self, html: &mut String, header_row: &[String]) -> Result<(), String> {
        html.push_str("  <thead>\n");
        html.push_str("    <tr>\n");

        for cell in header_row {
            write!(html, "      <th>{}</th>\n", html_escape(cell))
                .map_err(|_| "Failed to write header cell")?;
        }

        html.push_str("    </tr>\n");
        html.push_str("  </thead>\n");
        Ok(())
    }

    fn write_tbody(&self, html: &mut String, rows: &[Vec<String>]) -> Result<(), String> {
        html.push_str("  <tbody>\n");

        for (idx, row) in rows.iter().enumerate() {
            self.write_row(html, row, idx, true)?;
        }

        html.push_str("  </tbody>\n");
        Ok(())
    }

    fn write_row(&self, html: &mut String, row: &[String], index: usize, in_tbody: bool) -> Result<(), String> {
        let indent = if in_tbody { "    " } else { "  " };

        write!(html, "{}< tr", indent).map_err(|_| "Failed to write row opening")?;

        // Add row classes for styling
        let mut row_classes = Vec::new();
        if self.config.striped_rows && index % 2 == 0 {
            row_classes.push("even");
        }
        if self.config.striped_rows && index % 2 == 1 {
            row_classes.push("odd");
        }

        if !row_classes.is_empty() {
            write!(html, " class=\"{}\"", row_classes.join(" "))
                .map_err(|_| "Failed to write row class")?;
        }

        html.push_str(">\n");

        // Write cells
        for cell in row {
            write!(html, "{}  <td>{}</td>\n", indent, html_escape(cell))
                .map_err(|_| "Failed to write cell")?;
        }

        write!(html, "{}</tr>\n", indent).map_err(|_| "Failed to write row closing")?;
        Ok(())
    }
}

/// Convert table data to HTML with default configuration
pub fn table_to_html(data: &crate::TableData) -> Result<String, String> {
    let renderer = HtmlTableRenderer::new(HtmlConfig::default());
    renderer.render(data)
}

/// Convert table data to HTML with custom configuration
pub fn table_to_html_with_config(data: &crate::TableData, config: HtmlConfig) -> Result<String, String> {
    let renderer = HtmlTableRenderer::new(config);
    renderer.render(data)
}

/// Convert table data to Bootstrap-styled HTML
pub fn table_to_bootstrap_html(data: &crate::TableData) -> Result<String, String> {
    let renderer = HtmlTableRenderer::new(HtmlConfig::bootstrap());
    renderer.render(data)
}

/// Convert table data to Material Design HTML
pub fn table_to_material_html(data: &crate::TableData) -> Result<String, String> {
    let renderer = HtmlTableRenderer::new(HtmlConfig::material());
    renderer.render(data)
}

/// HTML-specific utilities
pub struct HtmlUtils;

impl HtmlUtils {
    /// Generate complete HTML document with table
    pub fn complete_document(table_html: &str, title: &str) -> String {
        format!(
            "<!DOCTYPE html>\n\
            <html lang=\"en\">\n\
            <head>\n\
            \x20\x20<meta charset=\"UTF-8\">\n\
            \x20\x20<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n\
            \x20\x20<title>{}</title>\n\
            \x20\x20<style>\n\
            \x20\x20\x20\x20body {{ font-family: Arial, sans-serif; margin: 20px; }}\n\
            \x20\x20\x20\x20table {{ width: 100%; border-collapse: collapse; }}\n\
            \x20\x20\x20\x20th, td {{ padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }}\n\
            \x20\x20\x20\x20th {{ background-color: #f2f2f2; font-weight: bold; }}\n\
            \x20\x20\x20\x20tr:hover {{ background-color: #f5f5f5; }}\n\
            \x20\x20</style>\n\
            </head>\n\
            <body>\n\
            \x20\x20<h1>{}</h1>\n\
            {}\n\
            </body>\n\
            </html>",
            html_escape(title),
            html_escape(title),
            table_html
        )
    }

    /// Add Bootstrap CDN links to HTML
    pub fn with_bootstrap_cdn(html: &str) -> String {
        html.replace(
            "</head>",
            "\x20\x20<link href=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css\" rel=\"stylesheet\">\n\
            \x20\x20<script src=\"https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/js/bootstrap.bundle.min.js\"></script>\n\
            </head>"
        )
    }

    /// Convert CSV to HTML table
    pub fn csv_to_html(csv: &str, separator: Option<&str>) -> Result<String, String> {
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

            if !row.is_empty() {
                rows.push(row);
            }
        }

        if rows.is_empty() {
            return Err("No valid data found in CSV".to_string());
        }

        let table_data = crate::TableData::new(rows);
        table_to_html(&table_data)
    }

    /// Extract table data from simple HTML table
    pub fn html_to_table_data(html: &str) -> Result<crate::TableData, String> {
        // Simple HTML parsing - in production would use a proper HTML parser
        let mut rows = Vec::new();
        let mut in_table = false;
        let mut in_row = false;
        let mut current_row = Vec::new();
        let mut current_cell = String::new();
        let mut in_cell = false;

        let lines = html.lines();
        for line in lines {
            let trimmed = line.trim();

            if trimmed.contains("<table") {
                in_table = true;
                continue;
            }

            if trimmed.contains("</table>") {
                if in_row && !current_row.is_empty() {
                    rows.push(current_row.clone());
                }
                break;
            }

            if !in_table {
                continue;
            }

            if trimmed.contains("<tr") {
                in_row = true;
                current_row.clear();
                continue;
            }

            if trimmed.contains("</tr>") {
                if !current_row.is_empty() {
                    rows.push(current_row.clone());
                }
                in_row = false;
                current_row.clear();
                continue;
            }

            if in_row {
                if trimmed.contains("<td") || trimmed.contains("<th") {
                    in_cell = true;
                    current_cell.clear();
                }

                if in_cell {
                    // Extract content between tags
                    let content = if let Some(start) = trimmed.find('>') {
                        if let Some(end) = trimmed.rfind('<') {
                            if start < end {
                                &trimmed[start + 1..end]
                            } else {
                                ""
                            }
                        } else {
                            &trimmed[start + 1..]
                        }
                    } else {
                        trimmed
                    };

                    if !content.is_empty() {
                        current_cell.push_str(content);
                    }
                }

                if trimmed.contains("</td>") || trimmed.contains("</th>") {
                    current_row.push(html_unescape(&current_cell));
                    current_cell.clear();
                    in_cell = false;
                }
            }
        }

        if rows.is_empty() {
            return Err("No table rows found in HTML".to_string());
        }

        Ok(crate::TableData::new(rows))
    }
}

/// Escape HTML special characters
pub fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Unescape basic HTML entities
pub fn html_unescape(text: &str) -> String {
    text.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#39;", "'")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_config_builder() {
        let config = HtmlConfig::new()
            .with_class("test-table")
            .with_id("my-table")
            .with_responsive()
            .with_striped_rows();

        assert_eq!(config.table_class, Some("test-table".to_string()));
        assert_eq!(config.table_id, Some("my-table".to_string()));
        assert!(config.responsive);
        assert!(config.striped_rows);
    }

    #[test]
    fn test_basic_html_generation() {
        let data = crate::TableData::new(vec![
            vec!["Name".to_string(), "Age".to_string()],
            vec!["Alice".to_string(), "30".to_string()],
            vec!["Bob".to_string(), "25".to_string()],
        ]);

        let result = table_to_html(&data).unwrap();

        assert!(result.contains("<table"));
        assert!(result.contains("<thead>"));
        assert!(result.contains("<tbody>"));
        assert!(result.contains("<th>Name</th>"));
        assert!(result.contains("<td>Alice</td>"));
        assert!(result.contains("</table>"));
    }

    #[test]
    fn test_bootstrap_preset() {
        let config = HtmlConfig::bootstrap();
        
        assert!(config.table_class.is_some());
        assert!(config.responsive);
        assert!(config.striped_rows);
        assert!(config.hover_effect);
    }

    #[test]
    fn test_html_escaping() {
        assert_eq!(html_escape("<script>"), "&lt;script&gt;");
        assert_eq!(html_escape("A & B"), "A &amp; B");
        assert_eq!(html_escape("\"quoted\""), "&quot;quoted&quot;");
    }

    #[test]
    fn test_html_unescaping() {
        assert_eq!(html_unescape("&lt;script&gt;"), "<script>");
        assert_eq!(html_unescape("A &amp; B"), "A & B");
        assert_eq!(html_unescape("&quot;quoted&quot;"), "\"quoted\"");
    }

    #[test]
    fn test_csv_to_html_conversion() {
        let csv = "Name,Age,City\nAlice,30,New York\nBob,25,London";
        let result = HtmlUtils::csv_to_html(csv, None).unwrap();

        assert!(result.contains("<th>Name</th>"));
        assert!(result.contains("<td>Alice</td>"));
        assert!(result.contains("<td>New York</td>"));
    }

    #[test]
    fn test_complete_document_generation() {
        let table = "<table><tr><td>Test</td></tr></table>";
        let doc = HtmlUtils::complete_document(table, "Test Table");

        assert!(doc.contains("<!DOCTYPE html>"));
        assert!(doc.contains("<title>Test Table</title>"));
        assert!(doc.contains("<h1>Test Table</h1>"));
        assert!(doc.contains(table));
    }

    #[test]
    fn test_material_preset() {
        let config = HtmlConfig::material();
        
        assert!(config.table_class.is_some());
        assert!(config.table_style.is_some());
        assert!(config.cell_padding.is_some());
    }

    #[test]
    fn test_dark_theme_preset() {
        let config = HtmlConfig::dark();
        
        assert!(config.table_class.is_some());
        assert!(config.table_style.is_some());
        assert!(config.striped_rows);
    }

    #[test]
    fn test_custom_attributes() {
        let config = HtmlConfig::new()
            .with_attribute("data-sortable", "true")
            .with_attribute("role", "table");

        assert_eq!(config.custom_attributes.len(), 2);
        assert_eq!(config.custom_attributes[0], ("data-sortable".to_string(), "true".to_string()));
        assert_eq!(config.custom_attributes[1], ("role".to_string(), "table".to_string()));
    }

    #[test]
    fn test_no_thead_configuration() {
        let data = crate::TableData::new(vec![
            vec!["Row1".to_string(), "Data1".to_string()],
            vec!["Row2".to_string(), "Data2".to_string()],
        ]);

        let config = HtmlConfig::new().without_thead();
        let result = table_to_html_with_config(&data, config).unwrap();

        assert!(!result.contains("<thead>"));
        assert!(result.contains("<tbody>"));
        assert!(result.contains("<td>Row1</td>"));
    }

    #[test]
    fn test_simple_html_parsing() {
        let html = r#"
            <table>
                <tr>
                    <th>Name</th>
                    <th>Age</th>
                </tr>
                <tr>
                    <td>Alice</td>
                    <td>30</td>
                </tr>
            </table>
        "#;

        let result = HtmlUtils::html_to_table_data(html).unwrap();
        
        assert_eq!(result.row_count(), 2);
        assert_eq!(result.column_count(), 2);
        assert_eq!(result.rows[0][0], "Name");
        assert_eq!(result.rows[1][0], "Alice");
    }
}