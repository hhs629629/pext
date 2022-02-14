use http::Version;

use crate::*;

impl FromUtf8 for Version {
    fn from_utf8<'a>(buf: &'a [u8]) -> Result<Self, FromUtf8Err>
    where
        Self: Sized {
            Ok(match buf {
                b"HTTP/0.9" => Version::HTTP_09,
                b"HTTP/1.0" => Version::HTTP_10,
                b"HTTP/1.1" => Version::HTTP_11,
                b"HTTP/2.0" => Version::HTTP_2,
                b"HTTP/3.0" => Version::HTTP_3,
                _ => unreachable!(),
            })
    }
}