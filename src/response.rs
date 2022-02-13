use http::Response;

use crate::FromUtf8;

impl FromUtf8 for Response<Vec<u8>> {
    fn from_utf8<'a>(buf: &'a [u8]) -> Result<Self, nom::Err<nom::error::Error<&[u8]>>>
    where
        Self: Sized,
    {
        todo!()
    }
}
