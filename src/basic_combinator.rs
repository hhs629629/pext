use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, multi::*, sequence::*,
    IResult,
};

pub fn field_content(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, _) = tuple((
        field_vchar,
        opt(tuple((many1(alt((space1, htab))), field_vchar))),
    ))(input)?;
    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
}

pub fn field_vchar(input: &[u8]) -> IResult<&[u8], &[u8]> {
    alt((vchar, obs_text))(input)
}

pub fn token(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, _token) = many1(tchar)(input)?;
    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
}

pub fn tchar(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map_parser(take(1usize), alt((is_a("!#$%&'*+-.^_`|~"), digit1, alpha1)))(input)
}

pub fn htab(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag("\t")(input)
}

pub fn separator(input: &[u8]) -> IResult<&[u8], &[u8]> {
    map_parser(take(1usize), is_a("()<>@,;:\\\"/[]?={} \t"))(input)
}

pub fn vchar(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, _) = satisfy(|c| c >= 0x21 as char && c <= 0x7e as char)(input)?;
    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
}

pub fn obs_text(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let (rest, _) = satisfy(|c| c >= 0x80 as char && c <= 0xFF as char)(input)?;
    let len = input.len() - rest.len();

    Ok((rest, &input[0..len]))
}

#[cfg(test)]
mod test {
    use super::*;
}
