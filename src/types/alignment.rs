use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    Left,
    Right,
    Center,
    Justify,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Left
    }
}

impl std::fmt::Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Alignment::Left => write!(f, "left"),
            Alignment::Right => write!(f, "right"),
            Alignment::Center => write!(f, "center"),
            Alignment::Justify => write!(f, "justify"),
        }
    }
}

impl std::str::FromStr for Alignment {
    type Err = crate::types::TableError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "left" => Ok(Alignment::Left),
            "right" => Ok(Alignment::Right),
            "center" => Ok(Alignment::Center),
            "justify" => Ok(Alignment::Justify),
            _ => Err(crate::types::TableError::InvalidAlignment),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VerticalAlignment {
    Top,
    Middle,
    Bottom,
}

impl Default for VerticalAlignment {
    fn default() -> Self {
        Self::Top
    }
}

impl std::fmt::Display for VerticalAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerticalAlignment::Top => write!(f, "top"),
            VerticalAlignment::Middle => write!(f, "middle"),
            VerticalAlignment::Bottom => write!(f, "bottom"),
        }
    }
}

impl std::str::FromStr for VerticalAlignment {
    type Err = crate::types::TableError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "top" => Ok(VerticalAlignment::Top),
            "middle" => Ok(VerticalAlignment::Middle),
            "bottom" => Ok(VerticalAlignment::Bottom),
            _ => Err(crate::types::TableError::InvalidAlignment),
        }
    }
}
