use nom::{
    branch::*,
    bytes::complete::*,
    character::{complete::*, is_digit, is_hex_digit},
    combinator::*,
    multi::*,
    sequence::*,
    IResult,
};

pub fn a_digit(input: &[u8]) -> IResult<&[u8], &[u8]> {
    if is_digit(input[0]) {
        Ok((&input[1..], &input[0..1]))
    } else {
        Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Digit,
        }))
    }
}

pub fn a_hex_digit(input: &[u8]) -> IResult<&[u8], &[u8]> {
    if is_hex_digit(input[0]) {
        Ok((&input[1..], &input[0..1]))
    } else {
        Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::HexDigit,
        }))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_digit() {
        let (rest, d) = a_digit(b"12345").unwrap();

        assert_eq!(b"1", d);
        assert_eq!(b"2345", rest);
    }
}
