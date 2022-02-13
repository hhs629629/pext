mod request;
mod response;

mod basic_combinator;
mod error;
mod http_combinator;

use nom::error::Error;
use nom::Err;

pub trait FromUtf8 {
    fn from_utf8<'a>(buf: &'a [u8]) -> Result<Self, Err<Error<&[u8]>>>
    where
        Self: Sized;
}
