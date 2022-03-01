use http::Response;

use crate::IntoUtf8;

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
