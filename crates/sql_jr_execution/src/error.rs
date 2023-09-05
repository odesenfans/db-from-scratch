use miette::Diagnostic;
use thiserror::Error;

/// Errors during query execution
#[derive(Error, Debug, Diagnostic)]
#[error("Query Execution Error")]
pub enum QueryExecutionError {
    #[error("Table {0} was not found")]
    TableNotFound(String),

    #[error("Table {0} already exists")]
    TableAlreadyExists(String),

    #[error("Column {0} does not exist")]
    ColumnDoesNotExist(String),
}
