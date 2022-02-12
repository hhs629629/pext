use http::{Request, Version};
use nom::{
    branch::*,
    bytes::complete::*,
    character::{complete::*, is_hex_digit},
    combinator::*,
    multi::*,
    sequence::*,
    IResult,
};

use nom::error::Error;
use nom::Err;

use crate::http_combinator::*;

trait FromUtf8 {
    fn from_utf8<'a>(buf: &'a [u8]) -> Result<Self, Err<Error<&[u8]>>>
    where
        Self: Sized;
}

impl FromUtf8 for Request<Vec<u8>> {
    fn from_utf8<'a>(buf: &'a [u8]) -> Result<Self, Err<Error<&[u8]>>> {
        let (rest, method) = terminated(method, tag(" "))(buf)?;
        let (rest, uri) = terminated(is_not(" "), tag(" "))(rest)?;
        let (rest, http_version) = terminated(http_version, tag("\r\n"))(rest)?;
        
        let (rest, headers) = terminated(
            separated_list0(
                tag("\r\n"),
                separated_pair(field_name, tag(":"), opt(field_value)),
            ),
            tag("\r\n"),
        )(rest)?;

        let mut builder = Request::builder()
            .method(method)
            .uri(uri)
            .version(guess_version(http_version));

        for (name, value) in headers {
            builder = builder.header(name, value.unwrap_or(b""));
        }

        Ok(builder.body(rest.to_vec()).unwrap())
    }
}

fn guess_version(version: &[u8]) -> Version {
    match version {
        b"HTTP/0.9" => Version::HTTP_09,
        b"HTTP/1.0" => Version::HTTP_10,
        b"HTTP/1.1" => Version::HTTP_11,
        b"HTTP/2" => Version::HTTP_2,
        b"HTTP/3" => Version::HTTP_3,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_utf8() {
        let something = b"GET /hello.htm HTTP/1.1\r\nUser-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)\r\nHost: www.tutorialspoint.com\r\nAccept-Language:\r\nAccept-Encoding: gzip, deflate\r\nConnection: Keep-Alive\r\n\r\nThisIsBody";

        let req = Request::from_utf8(something).unwrap();

        println!("{:?}", req);
    }
}
