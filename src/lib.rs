use std::io::{Read, Write};

use errors::{MisskeyConnectionResult, ServerError};
use http::{uri::Authority, Request, Version};
use miauth::MiAuthBuilder;
use serde::{Deserialize, Serialize};

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
    pub fn new(stream: T, authority: impl Into<String>) -> MisskeyConnectionResult<MisskeyHttpClient<T>> {
        Ok(MisskeyHttpClient::internal_new(stream, authority.into().parse()?, None))
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

    pub fn request<R>(&mut self, request: &R) -> MisskeyConnectionResult<R::Response> where R: MisskeyClientRequest {
        let with_token: RequestWithToken<R> = RequestWithToken::new(&self.access_token, request);
        let data = serde_json::ser::to_string(&with_token)?;
        let length = data.as_bytes().len();
        let req = http::Request::post(format!("/api{}", request.endpoint()))
            .version(Version::HTTP_11)
            .header(http::header::ACCEPT_CHARSET, "UTF-8")
            .header(http::header::ACCEPT_ENCODING, "identity")
            .header(http::header::CONNECTION, "keep-alive")
            .header(http::header::CONTENT_LENGTH, length)
            .header(http::header::CONTENT_TYPE, "application/json; Charset=UTF-8")
            .header(http::header::HOST, self.authority.host())
            .body(data.bytes())?;

        let response = self.internal_request(req)?;

        if let Ok(result) = serde_json::from_str::<R::Response>(&response) {
            return Ok(result);
        }

        match serde_json::from_str::<ServerErrorResponse>(&response) {
            Ok(e) => Err(e.error.into()),
            Err(e) => Err(e.into()),
        }
    }

    fn internal_request<R>(&mut self, request: Request<R>) -> MisskeyConnectionResult<String> where R: IntoIterator<Item = u8> {
    // fn internal_request<R>(&mut self, request: ::http::Request<R>) -> MisskeyConnectionResult<http::Response<Vec<u8>>> where R: Iterator<Item = u8> {
        let (parts, body) = request.into_parts();
        let request_bin: Vec<u8> = format!("{} {} {:?}\r\n", parts.method, parts.uri, parts.version).bytes()
            .chain(parts.headers.into_iter().filter_map(|a| a.0.map(|b| (b, a.1))).map(|a| a.0.as_str().bytes().chain(b": ".repeat(1)).chain(a.1.as_bytes().into_iter().map(|a| *a)).chain([b'\r', b'\n']).collect::<Vec<_>>()).flatten())
            .chain(b"\r\n".repeat(1))
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
            self.stream.read_exact(&mut buff)?;
            return Ok(String::from_utf8_lossy(&buff).to_string());
        } else {
            return Ok(String::new());
        }
        // return response.body(buff).map_err(|a| MisskeyConnectionError::HttpError(a));
    }
}

pub trait FixedEndpointMisskeyClientRequest : Serialize where for<'de> Self::Response: Deserialize<'de> {
    /// レスポンスの型
    type Response;
    /// リクエスト先のエンドポイントのアドレス。<br />
    /// 先頭にスラッシュが必要。`/api` は不要。
    const ENDPOINT: &'static str;
}

impl<T> MisskeyClientRequest for T where T: FixedEndpointMisskeyClientRequest {
    type Response = T::Response;
    
    fn endpoint(&self) -> &str {
        Self::ENDPOINT
    }
}

/// Misskey サーバーへ送信可能な構造体であることを示すトレイト
pub trait MisskeyClientRequest : Serialize where for<'de> Self::Response: Deserialize<'de> {
    /// レスポンスの型
    type Response;
    /// リクエスト先のエンドポイントのアドレス。<br />
    /// 先頭にスラッシュが必要。`/api` は不要。
    fn endpoint(&self) -> &str;
}

#[derive(Debug, serde_derive::Serialize)]
struct RequestWithToken<'a, T> where T: MisskeyClientRequest + Serialize {
    #[serde(rename = "i", skip_serializing_if = "Option::is_none")]
    token: &'a Option<String>,
    #[serde(flatten)]
    request: &'a T,
}

impl<'a, T> RequestWithToken<'a, T> where T: MisskeyClientRequest + Serialize {
    fn new(token: &'a Option<String>, request: &'a T) -> Self {
        Self {
            token,
            request,
        }
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
