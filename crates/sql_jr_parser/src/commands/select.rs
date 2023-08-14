use crate::parse::{comma_sep, identifier, Parse, ParseResult, RawSpan};
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace1;
use nom::error::context;
use nom::sequence::tuple;
use nom_supreme::ParserExt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SelectStatement {
    pub table: String,
    pub fields: Vec<String>,
}

impl<'a> Parse<'a> for SelectStatement {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        let (remaining_input, (_, _, fields, _, _, _, table)) = context(
            "Select Statement",
            tuple((
                tag_no_case("select"),
                multispace1,
                comma_sep(identifier).context("Select Columns"),
                multispace1,
                tag_no_case("from"),
                multispace1,
                identifier.context("From Table"),
            )),
        )(input)?;

        Ok((remaining_input, SelectStatement { fields, table }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select() {
        let expected = SelectStatement {
            table: "t1".to_string(),
            fields: vec!["foo".to_string(), "bar".to_string()],
        };

        assert_eq!(
            SelectStatement::parse_from_raw("select foo, bar from t1;")
                .unwrap()
                .1,
            expected
        )
    }
}
