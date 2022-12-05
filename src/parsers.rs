use nom::{
    character::complete::{digit1, multispace0, one_of},
    combinator::map,
    error::ParseError,
    multi::many1,
    sequence::terminated,
    AsChar, IResult, InputTakeAtPosition,
};

use crate::utils::parse_int;

#[allow(dead_code)]
pub fn single_digit_line(input: &[u8]) -> IResult<&[u8], Vec<usize>> {
    terminated(
        many1(map(one_of("0123456789"), |s| (s as u8 - b'0') as usize)),
        multispace0,
    )(input)
}

pub fn number<S, E>(input: S) -> IResult<S, usize, E>
where
    S: AsRef<[u8]> + Clone + InputTakeAtPosition,
    E: ParseError<S>,
    <S as InputTakeAtPosition>::Item: AsChar,
{
    map(digit1, |d: S| parse_int(d.as_ref()))(input)
}
