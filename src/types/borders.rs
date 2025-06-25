use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BorderConfig {
    pub top_body: String,
    pub top_join: String,
    pub top_left: String,
    pub top_right: String,
    pub bottom_body: String,
    pub bottom_join: String,
    pub bottom_left: String,
    pub bottom_right: String,
    pub body_left: String,
    pub body_right: String,
    pub body_join: String,
    pub header_join: String,
    pub join_body: String,
    pub join_left: String,
    pub join_right: String,
    pub join_join: String,
}

impl Default for BorderConfig {
    fn default() -> Self {
        get_border_characters("honeywell").unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BorderUserConfig {
    pub top_body: Option<String>,
    pub top_join: Option<String>,
    pub top_left: Option<String>,
    pub top_right: Option<String>,
    pub bottom_body: Option<String>,
    pub bottom_join: Option<String>,
    pub bottom_left: Option<String>,
    pub bottom_right: Option<String>,
    pub body_left: Option<String>,
    pub body_right: Option<String>,
    pub body_join: Option<String>,
    pub header_join: Option<String>,
    pub join_body: Option<String>,
    pub join_left: Option<String>,
    pub join_right: Option<String>,
    pub join_join: Option<String>,
}

impl BorderUserConfig {
    pub fn merge_with_default(self, default: &BorderConfig) -> BorderConfig {
        BorderConfig {
            top_body: self.top_body.unwrap_or_else(|| default.top_body.clone()),
            top_join: self.top_join.unwrap_or_else(|| default.top_join.clone()),
            top_left: self.top_left.unwrap_or_else(|| default.top_left.clone()),
            top_right: self.top_right.unwrap_or_else(|| default.top_right.clone()),
            bottom_body: self
                .bottom_body
                .unwrap_or_else(|| default.bottom_body.clone()),
            bottom_join: self
                .bottom_join
                .unwrap_or_else(|| default.bottom_join.clone()),
            bottom_left: self
                .bottom_left
                .unwrap_or_else(|| default.bottom_left.clone()),
            bottom_right: self
                .bottom_right
                .unwrap_or_else(|| default.bottom_right.clone()),
            body_left: self.body_left.unwrap_or_else(|| default.body_left.clone()),
            body_right: self
                .body_right
                .unwrap_or_else(|| default.body_right.clone()),
            body_join: self.body_join.unwrap_or_else(|| default.body_join.clone()),
            header_join: self
                .header_join
                .unwrap_or_else(|| default.header_join.clone()),
            join_body: self.join_body.unwrap_or_else(|| default.join_body.clone()),
            join_left: self.join_left.unwrap_or_else(|| default.join_left.clone()),
            join_right: self
                .join_right
                .unwrap_or_else(|| default.join_right.clone()),
            join_join: self.join_join.unwrap_or_else(|| default.join_join.clone()),
        }
    }
}

pub fn get_border_characters(name: &str) -> Result<BorderConfig, crate::types::TableError> {
    match name {
        "honeywell" => Ok(BorderConfig {
            top_body: "─".to_string(),
            top_join: "┬".to_string(),
            top_left: "┌".to_string(),
            top_right: "┐".to_string(),
            bottom_body: "─".to_string(),
            bottom_join: "┴".to_string(),
            bottom_left: "└".to_string(),
            bottom_right: "┘".to_string(),
            body_left: "│".to_string(),
            body_right: "│".to_string(),
            body_join: "│".to_string(),
            header_join: "─".to_string(),
            join_body: "─".to_string(),
            join_left: "├".to_string(),
            join_right: "┤".to_string(),
            join_join: "┼".to_string(),
        }),
        "norc" => Ok(BorderConfig {
            top_body: "─".to_string(),
            top_join: "┬".to_string(),
            top_left: "┌".to_string(),
            top_right: "┐".to_string(),
            bottom_body: "─".to_string(),
            bottom_join: "┴".to_string(),
            bottom_left: "└".to_string(),
            bottom_right: "┘".to_string(),
            body_left: "│".to_string(),
            body_right: "│".to_string(),
            body_join: "│".to_string(),
            header_join: "─".to_string(),
            join_body: "─".to_string(),
            join_left: "├".to_string(),
            join_right: "┤".to_string(),
            join_join: "┼".to_string(),
        }),
        "ramac" => Ok(BorderConfig {
            top_body: "-".to_string(),
            top_join: "+".to_string(),
            top_left: "+".to_string(),
            top_right: "+".to_string(),
            bottom_body: "-".to_string(),
            bottom_join: "+".to_string(),
            bottom_left: "+".to_string(),
            bottom_right: "+".to_string(),
            body_left: "|".to_string(),
            body_right: "|".to_string(),
            body_join: "|".to_string(),
            header_join: "-".to_string(),
            join_body: "-".to_string(),
            join_left: "+".to_string(),
            join_right: "+".to_string(),
            join_join: "+".to_string(),
        }),
        "void" => Ok(BorderConfig {
            top_body: "".to_string(),
            top_join: "".to_string(),
            top_left: "".to_string(),
            top_right: "".to_string(),
            bottom_body: "".to_string(),
            bottom_join: "".to_string(),
            bottom_left: "".to_string(),
            bottom_right: "".to_string(),
            body_left: "".to_string(),
            body_right: "".to_string(),
            body_join: " ".to_string(),
            header_join: "".to_string(),
            join_body: "".to_string(),
            join_left: "".to_string(),
            join_right: "".to_string(),
            join_join: "".to_string(),
        }),
        _ => Err(crate::types::TableError::InvalidConfig(format!(
            "Unknown border style: {}",
            name
        ))),
    }
}
