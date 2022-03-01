use crate::FromUtf8Err;

pub trait FromUtf8 {
    fn from_utf8<'a>(buf: &'a [u8]) -> Result<Self, FromUtf8Err>
    where
        Self: Sized;
}

pub trait IntoUtf8 {
    fn into_utf8(&self) -> Result<Vec<u8>, ()>;
}
