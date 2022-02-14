mod http_elements;
mod request;
mod response;

mod basic_combinator;
mod error;

pub trait FromUtf8 {
    fn from_utf8<'a>(buf: &'a [u8]) -> Result<Self, FromUtf8Err>
    where
        Self: Sized;
}

pub trait IntoUtf8 {
    fn into_utf8(self) -> Result<Vec<u8>, ()>;
}

pub mod http_combinator;
pub use crate::error::FromUtf8Err;
