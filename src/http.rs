pub(crate) mod requests;

use std::{collections::BTreeMap, error::Error, fmt::Display, io::{self, Read}};

pub(crate) struct HttpResponse {
    version: String,
    status_code: StatusCode,
    headers: BTreeMap<String, String>,
    body: Vec<u8>,
}

pub(crate) enum StatusCode {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    ImUsed = 226,
    MultipleChoices = 300,
    MoveParmently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    // Received code 306
    TemporaryRedirect = 307,
    ParmanentRedirect = 308,
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    InvalidCsrfTokens = 419,
    MisdirectedRequest = 421,
    UnprocessableContent = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,
    InternalServerError = 500,
    NotImplemente = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

pub(crate) fn read(stream: &mut dyn Read) -> io::Result<String> {
    const BUFF_SIZE: usize = 1;
    let mut result = Vec::new();
    let mut buff: [u8; BUFF_SIZE] = [0; BUFF_SIZE];
    while !result.ends_with(b"\r\n\r\n") {
        match stream.read(&mut buff) {
            Ok(size) => {
                for i in 0..size {
                    result.push(buff[i]);
                }
            }
            Err(e) => return Err(e),
        }
    }
    let headers = String::from_utf8_lossy(&result);
    let mut headers = headers.split('\n').map(|a| a.trim());
    let mut first = headers.next().unwrap().split(' ').peekable();
    let _version = first.next().unwrap();
    let _code = first.next().unwrap();
    let mut message = String::new();
    while let Some(a) = first.next() {
        message.push_str(a);
        if first.peek().is_some() {
            message.push(' ');
        }
    }
    let mut length: Option<usize> = None;
    for i in headers {
        if i.to_ascii_lowercase().starts_with("content-length") {
            length = i.split(':').skip(1).next().map(|a| a.trim().parse::<usize>().ok()).flatten();
            break;
        }
    }
    let length = length.unwrap_or(0);
    if length > 0 {
        let mut buff = vec![0; length];
        stream.read_exact(&mut buff)?;
        return Ok(String::from_utf8_lossy(&buff).to_string());
    } else {
        return Ok(String::new());
    }
}

#[derive(Debug)]
pub(crate) struct HttpError;

impl Error for HttpError {}

impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Response was not a http response.")
    }
}
