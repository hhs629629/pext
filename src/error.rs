use std::fmt::Display;

use nom::error::Error;
use nom::Err;

#[derive(Debug)]
pub enum ErrorKind {
    Method,
    Uri,
    Version,
    Header,
    StatusCode,
}

#[derive(Debug)]
pub struct FromUtf8Err {
    input: String,
    kind: ErrorKind,
}

impl FromUtf8Err {
    pub fn init(input: String, kind: ErrorKind) -> Self {
        Self { input, kind }
    }
}

impl std::error::Error for FromUtf8Err {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl Display for FromUtf8Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error from input: [\"{}\"] ErrorKind: [{:?}]",
            self.input, self.kind
        )
    }
}

pub trait IntoFromUtf8Err {
    fn into_parse_error(self, kind: ErrorKind) -> FromUtf8Err;
}

impl IntoFromUtf8Err for Err<Error<&[u8]>> {
    fn into_parse_error(self, kind: ErrorKind) -> FromUtf8Err {
        let error = match self {
            Err::Incomplete(_) => unimplemented!(),
            Err::Error(e) => e,
            Err::Failure(e) => e,
        };

        FromUtf8Err {
            input: String::from_utf8(error.input.to_vec()).unwrap(),
            kind,
        }
    }
}
