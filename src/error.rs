pub struct ParseRequestError<'a> {
    inner: nom::Err<nom::error::Error<&'a [u8]>>,
}

impl<'a> ParseRequestError<'a> {
    pub fn new(inner: nom::Err<nom::error::Error<&'a [u8]>>) -> Self {
        Self { inner }
    }
}
