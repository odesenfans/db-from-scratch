use crate::commands::create::CreateStatement;
use crate::commands::insert::InsertStatement;
use crate::commands::select::SelectStatement;
use crate::parse::{peek_then_cut, Parse, ParseResult, RawSpan};
use nom::branch::alt;
use nom::character::complete::{char, multispace0};
use nom::combinator::map;
use nom::error::context;
use nom::sequence::{preceded, tuple};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SqlQuery {
    Select(SelectStatement),
    Insert(InsertStatement),
    Create(CreateStatement),
}

impl<'a> Parse<'a> for SqlQuery {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        let (remaining, (query, _, _, _)) = context(
            "Query",
            preceded(
                multispace0,
                tuple((
                    alt((
                        peek_then_cut("select", map(SelectStatement::parse, SqlQuery::Select)),
                        peek_then_cut("insert", map(InsertStatement::parse, SqlQuery::Insert)),
                        peek_then_cut("create", map(CreateStatement::parse, SqlQuery::Create)),
                    )),
                    multispace0,
                    char(';'),
                    multispace0,
                )),
            ),
        )(input)?;

        Ok((remaining, query))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::Value;

    #[test]
    fn test_error() {
        let query = SqlQuery::parse_from_raw("select fart;");
        assert!(query.is_err(), "expected_parse_to_fail, got {query:?}");
    }

    #[test]
    fn test_select() {
        let expected = SelectStatement {
            table: "t1".to_string(),
            fields: vec!["foo".to_string(), "bar".to_string()],
        };

        assert_eq!(
            SqlQuery::parse_from_raw("select foo, bar from t1;")
                .unwrap()
                .1,
            SqlQuery::Select(expected)
        );
    }

    #[test]
    fn test_insert() {
        let expected = InsertStatement {
            table: "t1".to_string(),
            values: vec![
                Value::String("foo".to_string()),
                Value::String("bar".to_string()),
            ],
        };
        assert_eq!(
            SqlQuery::parse_from_raw("insert into t1 values 'foo', 'bar';")
                .unwrap()
                .1,
            SqlQuery::Insert(expected)
        );
    }
}
