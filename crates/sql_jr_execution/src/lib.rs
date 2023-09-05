mod error;
mod row;
mod table;

use crate::error::QueryExecutionError;
use crate::row::Row;
use crate::table::Table;
use derive_more::Display;
use sql_jr_parser::commands::ast::SqlQuery;
use std::collections::HashMap;

#[derive(Debug, Display)]
pub enum ExecResponse<'a> {
    #[display(fmt = "{_0:?}")] // only show the values, not "Select(...)"
    Select(Vec<Row<'a>>),
    Insert,
    Create,
}

#[derive(Debug, Default)]
pub struct Execution {
    tables: HashMap<String, Table>,
}

impl Execution {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn run(&mut self, query: SqlQuery) -> Result<ExecResponse, QueryExecutionError> {
        match query {
            SqlQuery::Select(mut select) => {
                let table = select.table;
                let table = self
                    .tables
                    .get(&table)
                    .ok_or(QueryExecutionError::TableNotFound(table))?;
                let rows = table.iter().collect();
                Ok(ExecResponse::Select(rows))
            }
            SqlQuery::Insert(insert) => {
                let Some(table) = self.tables.get_mut(&insert.table) else {
                    return Err(QueryExecutionError::TableNotFound(insert.table))
                };
                table.insert(insert.values);
                Ok(ExecResponse::Insert)
            }
        }
    }
}
