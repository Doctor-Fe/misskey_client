use std::{io::{Read, Write}, net::TcpStream};

use errors::{MisskeyConnectionResult, ServerError};
use miauth::MiAuthBuilder;
use native_tls::TlsStream;
use serde::{Deserialize, Serialize};

// TODO レスポンス型に Clone トレイトを実装するべきか否かの検討。

pub mod errors;
pub mod requests;
pub mod responses;
mod http;
pub mod traits;
pub mod miauth;

pub type UnknownValue = serde_json::Value;

pub struct MisskeyHttpClient<T> where T: Read + Write {
    access_token: Option<String>,
    server_address: String,
    stream: T,
}

impl MisskeyHttpClient<TlsStream<TcpStream>> {
    #[inline]
    pub fn new_ssl(server_address: impl Into<String>) -> MisskeyConnectionResult<MisskeyHttpClient<TlsStream<TcpStream>>> {
        let server_address = server_address.into();
        let connector = native_tls::TlsConnector::new()?;
        let stream = std::net::TcpStream::connect(format!("{}:443", server_address))?;
        let stream = connector.connect(&server_address, stream)?;
        Ok(MisskeyHttpClient::internal_new(stream, server_address.into(), None))
    }
}

impl<T> MisskeyHttpClient<T> where T: Read + Write {
    #[inline]
    pub fn new(stream: T, server_address: impl Into<String>) -> MisskeyHttpClient<T> {
        MisskeyHttpClient::internal_new(stream, server_address.into(), None)
    }

    #[inline]
    pub fn login(self, access_token: impl Into<String>) -> MisskeyHttpClient<T> {
        MisskeyHttpClient::internal_new(self.stream, self.server_address, Some(access_token.into()))
    }

    #[inline]
    pub fn logout(self) -> MisskeyHttpClient<T> {
        MisskeyHttpClient::internal_new(self.stream, self.server_address, None)
    }

    #[inline]
    pub fn miauth(self) -> MiAuthBuilder<T> {
        MiAuthBuilder::new(self)
    }

    fn internal_new(stream: T, server_address: String, access_token: Option<String>) -> MisskeyHttpClient<T> {
        MisskeyHttpClient { stream, access_token, server_address }
    }

    pub fn request<R>(&mut self, request: &R) -> MisskeyConnectionResult<R::Response> where R: MisskeyClientRequest {
        let with_token: RequestWithToken<R> = RequestWithToken::new(&self.access_token, request);
        let data = serde_json::ser::to_string(&with_token)?;
        let length = data.as_bytes().len();
        let req = http::requests::HttpRequest::new(http::requests::Method::Post, format!("/api{}", R::ENDPOINT), "HTTP/1.1")
            .header("Accept-Chatset", "UTF-8")
            .header("Accept-Encoding", "identity")
            .header("Connection", "keep-alive")
            .header("Content-Length", length)
            .header("Content-Type", "application/json; Charset=UTF-8")
            .header("Host", &self.server_address)
            .body(data.bytes());

        let val: Vec<u8> = req.into_iter().collect();

        self.stream.write(&val)?;
        self.stream.flush()?;

        let response_str = http::read(&mut self.stream)?;

        if let Ok(result) = serde_json::from_str::<R::Response>(&response_str) {
            return Ok(result);
        }

        match serde_json::from_str::<ServerErrorResponse>(&response_str) {
            Ok(e) => Err(e.error.into()),
            Err(e) => Err(e.into()),
        }
    }
}

/// Misskey サーバーへ送信可能な構造体であることを示すトレイト
pub trait MisskeyClientRequest : Serialize where for<'de> Self::Response: Deserialize<'de> {
    /// リクエスト先のエンドポイントのアドレス。<br />
    /// 先頭にスラッシュが必要。`/api` は不要。
    const ENDPOINT: &'static str;
    /// レスポンスの型
    type Response;
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
