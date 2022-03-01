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
    pub fn builder(input: &[u8]) -> PartialRequestBuilder<NeedMethod> {
        let result = PartialRequest {
            method: None,
            uri: None,
            version: None,
            headers: None,
            rest: Vec::new(),
        };
        PartialRequestBuilder {
            input,
            result,
            _phantom: PhantomData,
        }
    }
}

pub struct PartialRequestBuilder<'a, T> {
    input: &'a [u8],
    result: PartialRequest,
    _phantom: PhantomData<T>,
}

impl<'a, T> PartialRequestBuilder<'a, T> {
    pub fn build(mut self) -> PartialRequest {
        self.result.rest = self.input.to_vec();
        self.result
    }
}

impl<'a> PartialRequestBuilder<'a, NeedMethod> {
    pub fn method(mut self) -> Result<PartialRequestBuilder<'a, NeedUri>, FromUtf8Err> {
        let (rest, method) = terminated(method, tag(" "))(self.input)
            .map_err(|e| e.into_parse_error(ErrorKind::Method))?;

        self.result.method = Method::from_bytes(method).ok();

        Ok(PartialRequestBuilder {
            input: rest,
            result: self.result,
            _phantom: PhantomData,
        })
    }
}

impl<'a> PartialRequestBuilder<'a, NeedUri> {
    pub fn uri(mut self) -> Result<PartialRequestBuilder<'a, NeedVersion>, FromUtf8Err> {
        let (rest, uri) = terminated(is_not(" "), tag(" "))(self.input)
            .map_err(|e| e.into_parse_error(ErrorKind::Uri))?;

        let uri = String::from_utf8(uri.to_vec()).unwrap();
        let uri = uri
            .parse::<Uri>()
            .map_err(|_| FromUtf8Err::init(uri, ErrorKind::Uri))?;

        self.result.uri = Some(uri);

        Ok(PartialRequestBuilder {
            input: rest,
            result: self.result,
            _phantom: PhantomData,
        })
    }
}

impl<'a> PartialRequestBuilder<'a, NeedVersion> {
    pub fn version(mut self) -> Result<PartialRequestBuilder<'a, NeedHeader>, FromUtf8Err> {
        let (rest, http_version) = terminated(http_version, tag("\r\n"))(self.input)
            .map_err(|e| e.into_parse_error(ErrorKind::Version))?;

        let version = Version::from_utf8(http_version)?;

        self.result.version = Some(version);

        Ok(PartialRequestBuilder {
            input: rest,
            result: self.result,
            _phantom: PhantomData,
        })
    }
}

impl<'a> PartialRequestBuilder<'a, NeedHeader> {
    pub fn headers(mut self) -> Result<PartialRequestBuilder<'a, NeedBody>, FromUtf8Err> {
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

        Ok(PartialRequestBuilder {
            input: rest,
            result: self.result,
            _phantom: PhantomData,
        })
    }
}

impl<'a> PartialRequestBuilder<'a, NeedBody> {
    pub fn body(self) -> Request<Vec<u8>> {
        let mut builder = Request::builder()
            .method(self.result.method.unwrap())
            .uri(self.result.uri.unwrap())
            .version(self.result.version.unwrap());

        for (name, value) in self.result.headers.unwrap() {
            builder = builder.header(name.unwrap(), value);
        }
        builder.body(self.input.to_vec()).unwrap()
    }
}
