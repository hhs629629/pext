use nom::{
    branch::*,
    bytes::complete::*,
    character::{complete::*, is_digit, is_hex_digit},
    combinator::*,
    multi::*,
    sequence::*,
    IResult, Parser,
};

pub fn quoted_string(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, _) = tuple((tag("\""), many0(alt((qdtexts, quoted_pair))), tag("\"")))(input)?;
    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
}

pub fn texts(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, _) = many1(text)(input)?;
    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
}

pub fn text(input: &[u8]) -> IResult<&[u8], char> {
    satisfy(|c| c == 32 as char || c == 9 as char || (c > 32 as char && c != 127 as char))(input)
}

pub fn qdtexts(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, _) = many1(qdtext)(input)?;
    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
}

pub fn qdtext(input: &[u8]) -> IResult<&[u8], char> {
    satisfy(|c| {
        c == 13 as char
            || c == 10 as char
            || c == 32 as char
            || c == 9 as char
            || (c > 32 as char && c != 127 as char && c != '\"')
    })(input)
}

pub fn a_ascii_char(input: &[u8]) -> IResult<&[u8], char> {
    satisfy(|c| c < 128 as char)(input)
}

pub fn token(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, token) = many1(satisfy(|c| {
        let c = c as u8;
        32 < c
            && c < 127
            && !(58 <= c && c <= 64)
            && !(91 <= c && c <= 93)
            && c != 44
            && c != 34
            && c != 47
            && c != 123
            && c != 125
    }))(input)?;

    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
}

pub fn lws(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, _) = many1(alt((space1, ht)))(input)?;
    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
}

pub fn ht(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, _) = many1(satisfy(|c| c == 9 as char))(input)?;
    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
}

pub fn separators(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, _) = many1(separator)(input)?;
    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
}

pub fn separator(input: &[u8]) -> IResult<&[u8], char> {
    satisfy(|c| {
        c == '('
            || c == ')'
            || c == '<'
            || c == '>'
            || c == '@'
            || c == ','
            || c == ';'
            || c == ':'
            || c == '\\'
            || c == '\"'
            || c == '/'
            || c == '['
            || c == ']'
            || c == '?'
            || c == '='
            || c == '{'
            || c == '}'
            || c == ' '
            || c == 9 as char
    })(input)
}

pub fn quoted_pair(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, _) = tuple((char('\\'), a_ascii_char))(input)?;
    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_quoted_string() {
        let (rest, qtd) = quoted_string(b"\"inner_quote\"outer_quote").unwrap();
        assert_eq!(b"\"inner_quote\"", qtd);
        assert_eq!(b"outer_quote", rest);

        let (rest, qtd) = quoted_string(b"\"inner_qu\\\\ote\"outer_quote").unwrap();
        assert_eq!(b"\"inner_qu\\\\ote\"", qtd);
        assert_eq!(b"outer_quote", rest);
    }

    #[test]
    fn test_quoted_pair() {
        let (rest, qtd) = quoted_pair(b"\\\"not").unwrap();
        assert_eq!(b"\\\"", qtd);
        assert_eq!(b"not", rest);
    }
}
