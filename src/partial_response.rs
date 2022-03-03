use http::header::HeaderName;
use http::{HeaderMap, HeaderValue, Response, StatusCode, Version};

use nom::{bytes::complete::*, multi::*, sequence::*};

use std::marker::PhantomData;

use crate::error::*;
use crate::http_combinator::*;
use crate::FromUtf8;

pub struct NeedVersion;
pub struct NeedStatus;
pub struct NeedHeader;
pub struct NeedBody;

pub struct PartialResponse {
    version: Option<Version>,
    status: Option<StatusCode>,
    headers: Option<HeaderMap>,
    rest: Vec<u8>,
}

impl PartialResponse {
    pub fn builder(input: &[u8]) -> Builder<NeedVersion> {
        let result = PartialResponse {
            version: None,
            status: None,
            headers: None,
            rest: Vec::new(),
        };
        Builder::init(input, result)
    }

    pub fn parse_rest<T>(mut self, body: T) -> Result<Response<T>, FromUtf8Err> {
        let mut buf = Vec::new();
        std::mem::swap(&mut buf, &mut self.rest);

        let result = if self.headers.is_some() {
            Builder::<NeedBody>::init(&buf, self)
        } else {
            if self.status.is_some() {
                Builder::<NeedHeader>::init(&buf, self)
            } else {
                if self.version.is_some() {
                    Builder::<NeedStatus>::init(&buf, self)
                } else {
                    Builder::<NeedVersion>::init(&buf, self).version()?
                }
                .status()?
            }
            .headers()?
        }
        .body(body);

        Ok(result)
    }
}

pub struct Builder<'a, T> {
    input: &'a [u8],
    result: PartialResponse,
    _phantom: PhantomData<T>,
}

impl<'a, T> Builder<'a, T> {
    fn init(input: &'a [u8], result: PartialResponse) -> Self {
        Self {
            input,
            result,
            _phantom: PhantomData,
        }
    }
    pub fn build(mut self) -> PartialResponse {
        self.result.rest = self.input.to_vec();
        self.result
    }
}

impl<'a> Builder<'a, NeedVersion> {
    pub fn version(mut self) -> Result<Builder<'a, NeedStatus>, FromUtf8Err> {
        let (rest, http_version) = terminated(http_version, tag(" "))(self.input)
            .map_err(|e| e.into_parse_error(ErrorKind::Version))?;

        let version = match http_version {
            b"HTTP/0.9" => Version::HTTP_09,
            b"HTTP/1.0" => Version::HTTP_10,
            b"HTTP/1.1" => Version::HTTP_11,
            _ => unreachable!(),
        };

        self.result.version = Some(version);

        Ok(Builder {
            input: rest,
            result: self.result,
            _phantom: PhantomData,
        })
    }
}

impl<'a> Builder<'a, NeedStatus> {
    pub fn status(mut self) -> Result<Builder<'a, NeedHeader>, FromUtf8Err> {
        let (rest, (status_code, _sp, _reason_phrase)) =
            terminated(tuple((status_code, tag(" "), reason_phrase)), tag("\r\n"))(self.input)
                .map_err(|e| e.into_parse_error(ErrorKind::Version))?;

        let status = StatusCode::from_bytes(status_code).map_err(|_| {
            FromUtf8Err::init(
                String::from_utf8(status_code.to_vec()).unwrap(),
                ErrorKind::StatusCode,
            )
        })?;

        self.result.status = Some(status);

        Ok(Builder {
            input: rest,
            result: self.result,
            _phantom: PhantomData,
        })
    }
}

impl<'a> Builder<'a, NeedHeader> {
    pub fn headers(mut self) -> Result<Builder<'a, NeedBody>, FromUtf8Err> {
        let (rest, headers) = terminated(
            separated_list0(
                tag("\r\n"),
                terminated(
                    separated_pair(field_name, tuple((tag(":"), ows)), field_value),
                    ows,
                ),
            ),
            tag("\r\n\r\n"),
        )(self.input)
        .map_err(|e| e.into_parse_error(ErrorKind::Header))?;

        let mut header_map = HeaderMap::new();

        for (key, value) in headers {
            let name = HeaderName::from_bytes(key).map_err(|_| {
                FromUtf8Err::init(String::from_utf8(key.to_vec()).unwrap(), ErrorKind::Header)
            })?;
            let val = HeaderValue::from_bytes(value).map_err(|_| {
                FromUtf8Err::init(
                    String::from_utf8(value.to_vec()).unwrap(),
                    ErrorKind::Header,
                )
            })?;

            header_map.insert(name, val);
        }

        self.result.headers = Some(header_map);

        Ok(Builder {
            input: rest,
            result: self.result,
            _phantom: PhantomData,
        })
    }
}

impl<'a> Builder<'a, NeedBody> {
    pub fn body<T>(self, body: T) -> Response<T> {
        unsafe {
            let mut builder = Response::builder()
                .version(self.result.version.unwrap_unchecked())
                .status(self.result.status.unwrap_unchecked());

            for (name, value) in self.result.headers.unwrap_unchecked().iter() {
                builder = builder.header(name.clone(), value);
            }

            builder.body(body).unwrap()
        }
    }
}
