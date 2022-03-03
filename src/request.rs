use crate::error::*;
use crate::FromUtf8;
use crate::PartialRequest;
use http::Request;

impl<T> FromUtf8<T> for Request<T> {
    fn from_utf8<'a>(buf: &'a [u8], body: T) -> Result<Self, FromUtf8Err>
    where
        Self: Sized,
    {
        Ok(PartialRequest::builder(buf)
            .method()?
            .uri()?
            .version()?
            .headers()?
            .body(body))
    }
}

#[cfg(test)]
mod test {
    use http::{Method, Version};

    use super::*;

    #[test]
    fn test_from_utf8() {
        let input = b"GET /hello.htm HTTP/1.1\r\nUser-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)\r\nHost: www.tutorialspoint.com\r\nAccept-Language:\r\nAccept-Encoding: gzip, deflate\r\nConnection: Keep-Alive\r\n\r\n";

        let req = Request::from_utf8(input, b"ThisIsBody").unwrap();

        assert_eq!(req.method(), Method::GET);
        assert_eq!(req.uri(), "/hello.htm");
        assert_eq!(req.version(), Version::HTTP_11);
        assert_eq!(
            req.headers().get("user-agent").unwrap(),
            "Mozilla/4.0 (compatible; MSIE5.01; Windows NT)"
        );
        assert_eq!(req.headers().get("Host").unwrap(), "www.tutorialspoint.com");
        assert_eq!(req.headers().get("Accept-Language").unwrap(), "");
        assert_eq!(req.body(), &b"ThisIsBody");
    }
}
