use nom::branch::alt;
use nom::character::complete::{char, multispace1};
use nom::combinator::map;
use nom::error::context;
use nom::sequence::{preceded, separated_pair, tuple};
// Using tag_no_case from nom_supreme since its error is nicer
// ParserExt is mostly for adding .context on calls to identifier to say what kind of identifier
// we want.
use crate::parse::{comma_sep, identifier, Parse, ParseResult, RawSpan};
use nom_supreme::{tag::complete::tag_no_case, ParserExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SqlTypeInfo {
    String,
    Int,
}

// parses "string | int"
impl<'a> Parse<'a> for SqlTypeInfo {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        // context will help give better error messages later on
        context(
            "Column Type",
            alt((
                map(tag_no_case("string"), |_| Self::String),
                map(tag_no_case("int"), |_| Self::Int),
            )),
        )(input)
    }
}

/// A column's name + type
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub type_info: SqlTypeInfo,
}

impl<'a> Parse<'a> for Column {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        context(
            "Create Column",
            map(
                separated_pair(
                    identifier.context("Column Name"),
                    multispace1,
                    SqlTypeInfo::parse,
                ),
                |(name, type_info)| Self { name, type_info },
            ),
        )(input)
    }
}

/// The table and its columns to create
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct CreateStatement {
    pub table: String,
    pub columns: Vec<Column>,
}

// parses a comma separated list of column definitions contained in parens
fn column_definitions(input: RawSpan<'_>) -> ParseResult<'_, Vec<Column>> {
    context(
        "Column Definitions",
        map(
            tuple((char('('), comma_sep(Column::parse), char(')'))),
            |(_, cols, _)| cols,
        ),
    )(input)
}

// parses "CREATE TABLE <table name> <column defs>
impl<'a> Parse<'a> for CreateStatement {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        map(
            separated_pair(
                // table name
                preceded(
                    tuple((
                        tag_no_case("create"),
                        multispace1,
                        tag_no_case("table"),
                        multispace1,
                    )),
                    identifier.context("Table Name"),
                ),
                multispace1,
                // column defs
                column_definitions,
            )
            .context("Create Table"),
            |(table, columns)| Self { table, columns },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let expected = CreateStatement {
            table: "foo".into(),
            columns: vec![
                Column {
                    name: "col1".into(),
                    type_info: SqlTypeInfo::Int,
                },
                Column {
                    name: "col2".into(),
                    type_info: SqlTypeInfo::String,
                },
                Column {
                    name: "col3".into(),
                    type_info: SqlTypeInfo::String,
                },
            ],
        };

        assert_eq!(
            CreateStatement::parse_from_raw(
                "CREATE TABLE foo (col1 int, col2 string, col3 string)"
            )
            .unwrap()
            .1,
            expected
        )
    }
}
