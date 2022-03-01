use http::{HeaderMap, StatusCode, Version};

use crate::{FromUtf8, FromUtf8Err, IntoUtf8};

impl FromUtf8 for Version {
    fn from_utf8<'a>(buf: &'a [u8]) -> Result<Self, FromUtf8Err>
    where
        Self: Sized,
    {
        Ok(match buf {
            b"HTTP/0.9" => Version::HTTP_09,
            b"HTTP/1.0" => Version::HTTP_10,
            b"HTTP/1.1" => Version::HTTP_11,
            _ => unreachable!(),
        })
    }
}

impl IntoUtf8 for Version {
    fn into_utf8(&self) -> Result<Vec<u8>, ()> {
        Ok(match self {
            &Version::HTTP_09 => b"HTTP/0.9",
            &Version::HTTP_10 => b"HTTP/1.0",
            &Version::HTTP_11 => b"HTTP/1.1",
            _ => unreachable!(),
        }
        .to_vec())
    }
}

impl IntoUtf8 for StatusCode {
    fn into_utf8(&self) -> Result<Vec<u8>, ()> {
        Ok(self.to_string().as_bytes().to_vec())
    }
}

impl IntoUtf8 for HeaderMap {
    fn into_utf8(&self) -> Result<Vec<u8>, ()> {
        let mut result = Vec::new();

        for (key, value) in self.iter() {
            result.append(&mut key.as_str().as_bytes().to_vec());
            result.append(&mut b": ".to_vec());
            result.append(&mut value.as_bytes().to_vec());
            result.append(&mut b"\r\n".to_vec());
        }

        Ok(result)
    }
}
