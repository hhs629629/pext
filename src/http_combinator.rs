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

pub fn request_uri(input: &[u8]) -> IResult<&[u8], &[u8]> {
    todo!();
}

fn scheme(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, (_head, _tails)) = tuple((
        alpha1,
        many0(alt((alpha1, digit1, tag("+"), tag("-"), tag(".")))),
    ))(input)?;
    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
}

fn authority(input: &[u8]) -> IResult<&[u8], &[u8]> {
    todo!();
}

fn userinfo(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, _) = many0(alt((unreserved, pct_encoded, sub_delims, tag(":"))))(input)?;
    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
}

fn host(input: &[u8]) -> IResult<&[u8], &[u8]> {
    todo!();
}



fn unreserved(input: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((alpha1, digit1, tag("-"), tag("."), tag("_"), tag("~")))(input)
}

fn pct_encoded(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, _) = tuple((tag("%"), a_hex_digit, a_hex_digit))(input)?;
    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
}

fn sub_delims(input: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((
        tag("!"),
        tag("$"),
        tag("&"),
        tag("\'"),
        tag("("),
        tag(")"),
        tag("*"),
        tag("+"),
        tag(","),
        tag(";"),
        tag("="),
    ))(input)
}

fn h16(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, _) = many_m_n(1, 4, a_hex_digit)(input)?;

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
    fn test_scheme() {
        let (rest, sch) = scheme(b"https://www.github.com").unwrap();
        assert_eq!(b"https", sch);
        assert_eq!(b"://www.github.com", rest);

        let (rest, sch) = scheme(b"ftp://some.file.storage/").unwrap();
        assert_eq!(b"ftp", sch);
        assert_eq!(b"://some.file.storage/", rest);

        let (rest, sch) = scheme(b"sp-e+ci.41://special.page/").unwrap();
        assert_eq!(b"sp-e+ci.41", sch);
        assert_eq!(b"://special.page/", rest);
    }

    #[test]
    fn test_pct_encoded() {
        let (rest, pct) = pct_encoded(b"%2AFF").unwrap();
        assert_eq!(b"%2A", pct);
        assert_eq!(b"FF", rest);

        match pct_encoded(b"%2SFF") {
            Ok(_) => unreachable!(),
            Err(_) => assert!(true),
        }
    }
}
