use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*,
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

pub fn http_version(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, _version) = tuple((tag("HTTP/"), digit1, tag("."), digit1))(input)?;
    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
}

pub fn status_code(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map_parser(take(3usize), digit1)(input)
}

pub fn reason_phrase(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, _reason_phrase) = many0(alt((htab, tag(" "), vchar, obs_text)))(input)?;
    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
}

pub fn field_name(input: &[u8]) -> IResult<&[u8], &[u8]> {
    token(input)
}

pub fn field_value(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, _) = many0(field_content)(input)?;
    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
}

pub fn ows(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, _) = many0(alt((space1, htab)))(input)?;
    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
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

    #[test]
    fn test_http_version() {
        let (rest, mthd) = http_version(b"HTTP/1.1Free").unwrap();
        assert_eq!(b"HTTP/1.1", mthd);
        assert_eq!(b"Free", rest);

        let (rest, mthd) = http_version(b"HTTP/233.12 Version").unwrap();
        assert_eq!(b"HTTP/233.12", mthd);
        assert_eq!(b" Version", rest);
    }

    #[test]
    fn test_field_name() {
        let (rest, name) =
            field_name(b"User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)\r\n").unwrap();
        assert_eq!(b"User-Agent", name);
        assert_eq!(
            b": Mozilla/4.0 (compatible; MSIE5.01; Windows NT)\r\n",
            rest
        );
    }

    #[test]
    fn test_field_value() {
        let (rest, val) =
            field_value(b"Mozilla/4.0 (compatible; MSIE5.01; Windows NT)\r\n").unwrap();
        assert_eq!(b"Mozilla/4.0 (compatible; MSIE5.01; Windows NT)", val);
        assert_eq!(b"\r\n", rest);
    }

    #[test]
    fn test_header_field() {
        let (rest, (name, value)) =
            terminated(
                separated_pair(field_name, tuple((tag(":"), ows)), field_value),
                ows,
            )(b"User-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)\r\n")
            .unwrap();

        assert_eq!(b"User-Agent", name);
        assert_eq!(b"Mozilla/4.0 (compatible; MSIE5.01; Windows NT)", value);
        assert_eq!(b"\r\n", rest);
    }
}
