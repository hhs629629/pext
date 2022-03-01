# pext
Helper for [u8 arr] < - > [Http request or response]
```rust
use http::Reqeust;
use pext::FromUtf8;

let input = b"GET /hello.htm HTTP/1.1\r\nUser-Agent: Mozilla/4.0 (compatible; MSIE5.01; Windows NT)\r\nHost: www.tutorialspoint.com\r\nAccept-Language:\r\nAccept-Encoding: gzip, deflate\r\nConnection: Keep-Alive\r\n\r\nThisIsBody";
let req = Request::from_utf8(input).unwrap();

assert_eq!(req.method(), Method::GET);
assert_eq!(req.uri(), "/hello.htm");
assert_eq!(req.version(), Version::HTTP_11);
assert_eq!(
    req.headers().get("user-agent").unwrap(),
    "Mozilla/4.0 (compatible; MSIE5.01; Windows NT)"
);
assert_eq!(req.headers().get("Host").unwrap(), "www.tutorialspoint.com");
assert_eq!(req.headers().get("Accept-Language").unwrap(), "");
assert_eq!(req.body(), b"ThisIsBody");

```
