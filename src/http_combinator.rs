use nom::{
    branch::*,
    bytes::complete::*,
    character::{complete::*, is_hex_digit},
    combinator::*,
    multi::*,
    sequence::*,
    IResult,
};

use crate::basic_combinator::*;

pub fn method(input: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((
        tag("OPTIONS"),
        tag("GET"),
        tag("HEAD"),
        tag("POST"),
        tag("PUT"),
        tag("DELETE"),
        tag("TRACE"),
        tag("CONNECT"),
        tag("PATCH"),
    ))(input)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_method() {
        let (rest, mthd) =
            method(b"GET http://www.w3.org/pub/WWW/TheProject.html HTTP/1.1").unwrap();
        assert_eq!(b"GET", mthd);
        assert_eq!(b" http://www.w3.org/pub/WWW/TheProject.html HTTP/1.1", rest);

        let (rest, mthd) =
            method(b"POST http://www.w3.org/pub/WWW/TheProject.html HTTP/1.1").unwrap();
        assert_eq!(b"POST", mthd);
        assert_eq!(b" http://www.w3.org/pub/WWW/TheProject.html HTTP/1.1", rest);
    }
}
