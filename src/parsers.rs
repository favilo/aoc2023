use heapless::Vec;
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{char, digit1, multispace0, one_of},
    combinator::{map, opt},
    error::{ErrorKind, ParseError},
    multi::many1,
    sequence::{terminated, tuple},
    AsBytes, AsChar, Err, IResult, InputLength, InputTakeAtPosition, Parser,
};

use crate::utils::parse_int;

pub fn single_digit_line<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], std::vec::Vec<usize>, E>
where
    E: ParseError<&'a [u8]>,
{
    terminated(
        many1(map(one_of("0123456789"), |s| (s as u8 - b'0') as usize)),
        multispace0,
    )(input)
}

pub fn number<S, E>(input: S) -> IResult<S, usize, E>
where
    S: AsBytes + Clone + InputTakeAtPosition,
    E: ParseError<S>,
    <S as InputTakeAtPosition>::Item: AsChar,
{
    map(digit1, |d: S| parse_int(d.as_bytes()))(input)
}

pub fn signed_number<'input, E>(input: &'input [u8]) -> IResult<&'input [u8], isize, E>
where
    E: ParseError<&'input [u8]>,
{
    map(
        tuple((opt(tag(b"-")), digit1)),
        |(s, d): (Option<&[u8]>, &[u8])| {
            let parsed = parse_int(d.as_bytes()) as isize;
            if s.is_some() {
                -parsed as isize
            } else {
                parsed
            }
        },
    )(input)
}

#[allow(dead_code)]
pub fn many1_heapless<I, O, E, F, const N: usize>(
    mut f: F,
) -> impl FnMut(I) -> IResult<I, Vec<O, N>, E>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    E: ParseError<I>,
{
    move |mut i: I| match f.parse(i.clone()) {
        Err(Err::Error(err)) => Err(Err::Error(E::append(i, ErrorKind::Many1, err))),
        Err(e) => Err(e),
        Ok((i1, o)) => {
            let mut acc = heapless::Vec::<_, N>::new();
            acc.push(o)
                .map_err(|_| panic!("vector not big enough"))
                .unwrap();
            i = i1;

            loop {
                let len = i.input_len();
                match f.parse(i.clone()) {
                    Err(Err::Error(_)) => return Ok((i, acc)),
                    Err(e) => return Err(e),
                    Ok((i1, o)) => {
                        // infinite loop check: the parser must always consume
                        if i1.input_len() == len {
                            return Err(Err::Error(E::from_error_kind(i, ErrorKind::Many1)));
                        }

                        i = i1;
                        acc.push(o)
                            .map_err(|_| panic!("vector not big enough"))
                            .unwrap();
                    }
                }
            }
        }
    }
}
