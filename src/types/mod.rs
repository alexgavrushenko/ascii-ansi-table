pub mod alignment;
pub mod borders;
pub mod config;

pub use alignment::*;
pub use borders::*;
pub use config::*;

use thiserror::Error;

pub type Cell = String;
pub type Row = Vec<Cell>;

#[derive(Error, Debug)]
pub enum TableError {
    #[error("Table data must be an array")]
    InvalidData,
    #[error("Table row data must be an array")]
    InvalidRowData,
    #[error("Table must have consistent number of cells")]
    InconsistentRowLength,
    #[error("Control characters are not allowed")]
    ControlCharacters,
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("Width must be positive")]
    InvalidWidth,
    #[error("Invalid alignment value")]
    InvalidAlignment,
    #[error("Border characters cannot be empty")]
    EmptyBorderCharacters,
}

pub type TableResult<T> = Result<T, TableError>;