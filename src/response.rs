use http::Response;

use crate::{partial_response::PartialResponse, FromUtf8, IntoUtf8};

impl IntoUtf8 for Response<Vec<u8>> {
    fn into_utf8(&self) -> Result<Vec<u8>, ()> {
        let mut result = Vec::new();
        result.append(&mut self.version().into_utf8()?);
        result.push(b' ');

        result.append(&mut self.status().into_utf8()?);
        result.push(b'\r');
        result.push(b'\n');

        result.append(&mut self.headers().into_utf8()?);
        result.push(b'\r');
        result.push(b'\n');

        result.append(&mut self.body().clone());

        Ok(result)
    }
}

impl<T> FromUtf8<T> for Response<T> {
    fn from_utf8<'a>(buf: &'a [u8], body: T) -> Result<Self, crate::FromUtf8Err>
    where
        Self: Sized,
    {
        Ok(PartialResponse::builder(buf)
            .version()?
            .status()?
            .headers()?
            .body(body))
    }
}
