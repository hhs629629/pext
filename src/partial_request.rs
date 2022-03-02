use http::header::HeaderName;
use http::{HeaderMap, HeaderValue, Method, Request, Uri, Version};

use nom::{bytes::complete::*, multi::*, sequence::*};

use std::marker::PhantomData;

use crate::error::*;
use crate::http_combinator::*;
use crate::FromUtf8;

pub struct NeedMethod;
pub struct NeedUri;
pub struct NeedVersion;
pub struct NeedHeader;
pub struct NeedBody;

pub struct PartialRequest {
    method: Option<Method>,
    uri: Option<Uri>,
    version: Option<Version>,
    headers: Option<HeaderMap>,
    rest: Vec<u8>,
}

impl PartialRequest {
    pub fn builder(input: &[u8]) -> Builder<NeedMethod> {
        let result = PartialRequest {
            method: None,
            uri: None,
            version: None,
            headers: None,
            rest: Vec::new(),
        };
        Builder::init(input, result)
    }

    pub fn parse_rest(mut self) -> Result<Request<Vec<u8>>, FromUtf8Err> {
        let mut buf = Vec::new();
        std::mem::swap(&mut buf, &mut self.rest);

        let result = if self.headers.is_some() {
            Builder::<NeedBody>::init(&buf, self)
        } else {
            if self.version.is_some() {
                Builder::<NeedHeader>::init(&buf, self)
            } else {
                if self.uri.is_some() {
                    Builder::<NeedVersion>::init(&buf, self)
                } else {
                    if self.method.is_some() {
                        Builder::<NeedUri>::init(&buf, self)
                    } else {
                        Builder::<NeedMethod>::init(&buf, self).method()?
                    }
                    .uri()?
                }
                .version()?
            }
            .headers()?
        }
        .body();

        Ok(result)
    }

    pub fn method(&self) -> &Option<Method> {
        &self.method
    }

    pub fn uri(&self) -> &Option<Uri> {
        &self.uri
    }

    pub fn version(&self) -> &Option<Version> {
        &self.version
    }

    pub fn headers(&self) -> &Option<HeaderMap> {
        &self.headers
    }
}

pub struct Builder<'a, T> {
    input: &'a [u8],
    result: PartialRequest,
    _phantom: PhantomData<T>,
}

impl<'a, T> Builder<'a, T> {
    fn init(input: &'a [u8], result: PartialRequest) -> Self {
        Self {
            input,
            result,
            _phantom: PhantomData,
        }
    }
    pub fn build(mut self) -> PartialRequest {
        self.result.rest = self.input.to_vec();
        self.result
    }
}

impl<'a> Builder<'a, NeedMethod> {
    pub fn method(mut self) -> Result<Builder<'a, NeedUri>, FromUtf8Err> {
        let (rest, method) = terminated(method, tag(" "))(self.input)
            .map_err(|e| e.into_parse_error(ErrorKind::Method))?;

        self.result.method = Method::from_bytes(method).ok();

        Ok(Builder {
            input: rest,
            result: self.result,
            _phantom: PhantomData,
        })
    }
}

impl<'a> Builder<'a, NeedUri> {
    pub fn uri(mut self) -> Result<Builder<'a, NeedVersion>, FromUtf8Err> {
        let (rest, uri) = terminated(is_not(" "), tag(" "))(self.input)
            .map_err(|e| e.into_parse_error(ErrorKind::Uri))?;

        let uri = String::from_utf8(uri.to_vec()).unwrap();
        let uri = uri
            .parse::<Uri>()
            .map_err(|_| FromUtf8Err::init(uri, ErrorKind::Uri))?;

        self.result.uri = Some(uri);

        Ok(Builder {
            input: rest,
            result: self.result,
            _phantom: PhantomData,
        })
    }
}

impl<'a> Builder<'a, NeedVersion> {
    pub fn version(mut self) -> Result<Builder<'a, NeedHeader>, FromUtf8Err> {
        let (rest, http_version) = terminated(http_version, tag("\r\n"))(self.input)
            .map_err(|e| e.into_parse_error(ErrorKind::Version))?;

        let version = Version::from_utf8(http_version)?;

        self.result.version = Some(version);

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
    pub fn body(self) -> Request<Vec<u8>> {
        unsafe {
            let mut builder = Request::builder()
                .method(self.result.method.unwrap_unchecked())
                .uri(self.result.uri.unwrap_unchecked())
                .version(self.result.version.unwrap_unchecked());

            for (name, value) in self.result.headers.unwrap_unchecked().iter() {
                builder = builder.header(name.clone(), value);
            }

            builder.body(self.input.to_vec()).unwrap()
        }
    }
}
