use std::io::{Read, Write};

use errors::{MisskeyConnectionResult, ServerError};
use http::{header, uri::{Authority, InvalidUri}, Request, Response, Version};
use itertools::Itertools;
use miauth::MiAuthBuilder;

pub use traits::MisskeyClientRequest;
pub use traits::json::{FixedEndpointJsonRequest, JsonRequest};

// TODO レスポンス型に Clone トレイトを実装するべきか否かの検討。

pub mod errors;
pub mod requests;
pub mod responses;
pub mod traits;
pub mod miauth;

pub type UnknownValue = serde_json::Value;

pub struct MisskeyHttpClient<T> where T: Read + Write {
    access_token: Option<String>,
    authority: Authority,
    stream: T,
}

impl<T> MisskeyHttpClient<T> where T: Read + Write {
    #[inline]
    pub fn new(stream: T, authority: impl TryInto<Authority, Error = InvalidUri>) -> MisskeyConnectionResult<MisskeyHttpClient<T>> {
        Ok(MisskeyHttpClient::internal_new(stream, authority.try_into()?, None))
    }

    #[inline]
    pub fn login(self, access_token: impl Into<String>) -> MisskeyHttpClient<T> {
        MisskeyHttpClient::internal_new(self.stream, self.authority, Some(access_token.into()))
    }

    #[inline]
    pub fn logout(self) -> MisskeyHttpClient<T> {
        MisskeyHttpClient::internal_new(self.stream, self.authority, None)
    }

    #[inline]
    pub fn miauth(self) -> MiAuthBuilder<T> {
        MiAuthBuilder::new(self)
    }

    #[inline]
    fn internal_new(stream: T, authority: Authority, access_token: Option<String>) -> MisskeyHttpClient<T> {
        MisskeyHttpClient { stream, access_token, authority }
    }

    pub fn request<R>(&mut self, request: &R) -> MisskeyConnectionResult<Response<R::Response>> where R: MisskeyClientRequest {
        let data = request.body(self.access_token.as_deref());
        let length = data.as_bytes().len();
        let req = Request::post(format!("/api{}", request.endpoint()))
            .version(Version::HTTP_11)
            .header(header::ACCEPT_CHARSET, "UTF-8")
            .header(header::ACCEPT_ENCODING, "identity")
            .header(header::CONNECTION, "keep-alive")
            .header(header::CONTENT_LENGTH, length)
            .header(header::CONTENT_TYPE, format!("{}; Charset=UTF-8", request.content_type()))
            .header(header::HOST, self.authority.host())
            .body(data.bytes())?;

        let (parts, body) = self.internal_request(req)?.into_parts();

        return if let Ok(result) = serde_json::from_str::<R::Response>(&body) {
            Ok(Response::from_parts(parts, result))
        } else {
            Err(serde_json::from_str::<ServerErrorResponse>(&body)?.error.into())
        };
    }

    fn internal_request<R>(&mut self, request: Request<R>) -> MisskeyConnectionResult<Response<String>> where R: IntoIterator<Item = u8> {
        let (parts, body) = request.into_parts();
        let request_bin: Vec<u8> = format!("{} {} {:?}\r\n", parts.method, parts.uri, parts.version).bytes()
            .chain(parts.headers.into_iter().filter_map(|a| a.0.map(|b| (b, a.1))).map(|a| a.0.as_str().bytes().chain(*b": ").chain(a.1.as_bytes().into_iter().map(|a| *a)).chain([b'\r', b'\n']).collect::<Vec<_>>()).flatten())
            .chain(*b"\r\n")
            .chain(body)
            .collect();

        self.stream.write(&request_bin)?;
        self.stream.flush()?;
        const BUFF_SIZE: usize = 1;
        let mut result = Vec::new();
        let mut buff: [u8; BUFF_SIZE] = [0; BUFF_SIZE];
        while !result.ends_with(b"\r\n\r\n") {
            let size = self.stream.read(&mut buff)?;
            for i in 0..size {
                result.push(buff[i]);
            }
        }
        let headers = String::from_utf8_lossy(&result);
        let mut headers = headers.split('\n').map(|a| a.trim());
        let mut first = headers.next().unwrap().split(' ').peekable();
        let version = first.next().unwrap();
        let code = first.next().unwrap();
        let mut response = Response::builder().version(match version {
            "HTTP/0.9" => Version::HTTP_09,
            "HTTP/1.0" => Version::HTTP_10,
            "HTTP/1.1" => Version::HTTP_11,
            "HTTP/2" => Version::HTTP_2,
            "HTTP/3" => Version::HTTP_3,
            _ => unimplemented!(),
        }).status(code);
        let mut message = String::new();
        while let Some(a) = first.next() {
            message.push_str(a);
            if first.peek().is_some() {
                message.push(' ');
            }
        }
        let mut length: Option<usize> = None;
        for i in headers {
            let splitted: Vec<&str> = i.split(':').collect();
            let key = splitted[0].trim().to_ascii_lowercase();
            let value = splitted.into_iter().skip(1).join(":").trim().to_string();
            if key.starts_with("content-length") {
                length = value.parse::<usize>().ok()
            }
            response = response.header(key, value);
        }
        let length = length.unwrap_or(0);
        let mut buff;
        if length > 0 {
            buff = vec![0; length];
            self.stream.read_exact(&mut buff)?;
        } else {
            buff = Vec::with_capacity(0);
        }
        return Ok(response.body(String::from_utf8(buff)?)?);
    }
}

#[derive(Debug, serde_derive::Deserialize)]
struct ServerErrorResponse {
    error: ServerError,
}

#[derive(Debug, serde_derive::Deserialize)]
pub enum MaybeMultiple<T> where T: std::fmt::Debug {
    Single(T),
    Multiple(Vec<T>),
}
