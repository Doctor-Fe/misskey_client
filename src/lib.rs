use std::{io::{self, Write}, net::TcpStream};

use errors::{MisskeyConnectionError, MisskeyConnectionResult};
use native_tls::{TlsConnector, TlsStream};
use serde::{Deserialize, Serialize};
#[cfg(feature = "log")]
use log::info;

pub mod errors;
pub mod requests;
pub mod responses;
mod http;

pub struct MisskeyHttpClient {
    access_token: Option<String>,
    server_address: String,
    stream: TlsStream<TcpStream>
}

impl MisskeyHttpClient {
    pub fn new(server_address: impl Into<String>) -> MisskeyConnectionResult<MisskeyHttpClient> {
        Self::internal_new(server_address, None)
    }

    pub fn new_with_access_token(server_address: impl Into<String>, access_token: impl Into<String>) -> MisskeyConnectionResult<MisskeyHttpClient> {
        Self::internal_new(server_address, Some(access_token.into()))
    }

    fn internal_new(server_address: impl Into<String>, access_token: Option<String>) -> MisskeyConnectionResult<MisskeyHttpClient> {
        let server_address: String = server_address.into();
        match TlsConnector::new() {
            Ok(connector) => match TcpStream::connect(format!("{}:443", server_address)) {
                Ok(stream) => connector.connect(&server_address, stream).map(|stream| Self {access_token, server_address, stream}).map_err(|e| MisskeyConnectionError::TlsHandshakeError(e)),
                Err(e) => Err(MisskeyConnectionError::IoError(e)),
            },
            Err(e) => Err(MisskeyConnectionError::TlsError(e)),
        }
    }

    pub fn request<T>(&mut self, request: &T) -> MisskeyConnectionResult<T::Response> where T: MisskeyClientRequest {
        let with_token: RequestWithToken<T> = RequestWithToken::new(self.access_token.as_ref().map(|a| a.as_str()), request);
        match serde_json::ser::to_string(&with_token) {
            Err(e) => Err(MisskeyConnectionError::SerdeError(e)),
            Ok(data) => {
                #[cfg(feature = "log")]
                info!("Sending request: {}", data);
                let length = data.as_bytes().len();
                let req = http::requests::HttpRequest::new(http::requests::Method::Post, format!("/api{}", T::ENDPOINT), "HTTP/1.1")
                    .header("Accept-Chatset", "UTF-8")
                    .header("Accept-Encoding", "identity")
                    .header("Connection", "keep-alive")
                    .header("Content-Length", length)
                    .header("Content-Type", "application/json; Charset=UTF-8")
                    .header("Host", &self.server_address)
                    .body(data.bytes().collect());

                _ = req.write_to(&mut self.stream).unwrap();
                _ = self.stream.flush().unwrap();

                match http::read(&mut self.stream) {
                    Err(e) => Err(MisskeyConnectionError::IoError(e)),
                    Ok(str) => {
                        #[cfg(feature = "log")]
                        log::info!("Received string: {}", str);
                        if let Ok(e) = serde_json::de::from_str(&str) {
                            return Err(MisskeyConnectionError::ServerResponseError(e))
                        }
        
                        match serde_json::de::from_str::<T::Response>(&str) {
                            Err(e) => Err(MisskeyConnectionError::SerdeError(e)),
                            Ok(response) => Ok(response),
                        }
                    }
                }
            }
        }
    }

    pub fn shutdown(&mut self) -> io::Result<()> {
        self.stream.shutdown()
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
    token: Option<&'a str>,
    #[serde(flatten)]
    request: &'a T,
}

impl<'a, T> RequestWithToken<'a, T> where T: MisskeyClientRequest + Serialize {
    pub fn new(token: Option<&'a str>, request: &'a T) -> Self {
        Self {
            token,
            request,
        }
    }
}
