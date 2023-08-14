use nom::bytes::complete::take_while1;
use nom::combinator::map;
use nom::IResult;
use nom_locate::LocatedSpan;

// Use nom_locate's LocatedSpan as a wrapper around a string input
pub type RawSpan<'a> = LocatedSpan<&'a str>;

// the result for all of our parsers, they will have our span type as input and can have any output
// this will use a default error type but we will change that later
pub type ParseResult<'a, T> = IResult<RawSpan<'a>, T>;

pub(crate) fn identifier(i: RawSpan) -> ParseResult<String> {
    map(take_while1(|c: char| c.is_alphanumeric()), |s: RawSpan| {
        s.fragment().to_string()
    })(i)
}

/// Implement the parse function to more easily convert a span into a SQL command
pub trait Parse<'a>: Sized {
    /// Parse the given span into self
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self>;

    fn parse_from_raw(input: &'a str) -> ParseResult<'a, Self> {
        let i = LocatedSpan::new(input);
        Self::parse(i)
    }
}
