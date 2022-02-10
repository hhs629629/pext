use http::Request;
use nom::bytes::complete::{is_not, tag};
use nom::sequence::delimited;
use nom::AsBytes;

use crate::error::ParseRequestError;
use crate::http_combinator::{self, method};

trait FromUtf8 {
    fn from_utf8<'a>(vec: &'a [u8]) -> Result<Self, ParseRequestError<'a>>
    where
        Self: Sized;
}

impl FromUtf8 for Request<Vec<u8>> {
    fn from_utf8<'a>(vec: &'a [u8]) -> Result<Self, ParseRequestError<'a>> {
        let bytes = vec.as_bytes();

        let (rest, method) = method(bytes).map_err(|e| ParseRequestError::new(e))?;

        let (rest, uri) = delimited(tag(" "), is_not(" "), tag(" "))(rest)
            .map_err(|e| ParseRequestError::new(e))?;

        todo!();
    }
}
