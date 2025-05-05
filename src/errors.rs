use std::{error::Error, fmt::Display, io, net::TcpStream, string::FromUtf8Error};

use derive_getters::Getters;
use http::uri::InvalidUri;
use serde_derive::Deserialize;

pub type MisskeyConnectionResult<T> = Result<T, MisskeyConnectionError>;

#[derive(Debug)]
pub enum MisskeyConnectionError {
    // リクエスト送受信処理中に発生する可能性のあるエラーが発生したとき。

    /// TLS 接続時にエラーが発生したとき。
    TlsError(native_tls::Error),
    /// TLS ハンドシェイク時にエラーが発生したとき。
    TlsHandshakeError(native_tls::HandshakeError<TcpStream>),
    /// TCP 通信にエラーが発生したとき。
    IoError(io::Error),
    /// HTTP 通信でエラーが発生したとき。
    HttpError(http::Error),
    
    // クライアント側で発生したエラー。
    /// UTF-8以外の文字列
    NotUtf8Error(FromUtf8Error),
    /// 無効な URI
    InvalidUriError(http::uri::InvalidUri),
    /// 無効なアドレス
    InvalidAuthorityError,
    /// シリアル化または逆シリアル化に失敗したとき
    SerdeError(serde_json::Error),
    /// Misskey サーバーからエラーの応答があったとき。
    ServerResponseError(ServerError),
}

impl Error for MisskeyConnectionError {}

impl Display for MisskeyConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<io::Error> for MisskeyConnectionError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<http::Error> for MisskeyConnectionError {
    fn from(value: http::Error) -> Self {
        Self::HttpError(value)
    }
}

impl From<native_tls::Error> for MisskeyConnectionError {
    fn from(value: native_tls::Error) -> Self {
        Self::TlsError(value)
    }
}

impl From<native_tls::HandshakeError<TcpStream>> for MisskeyConnectionError {
    fn from(value: native_tls::HandshakeError<TcpStream>) -> Self {
        Self::TlsHandshakeError(value)
    }
}

impl From<serde_json::Error> for MisskeyConnectionError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value)
    }
}

impl From<FromUtf8Error> for MisskeyConnectionError {
    fn from(value: FromUtf8Error) -> Self {
        Self::NotUtf8Error(value)
    }
}

impl From<ServerError> for MisskeyConnectionError {
    fn from(value: ServerError) -> Self {
        Self::ServerResponseError(value)
    }
}

impl From<InvalidUri> for MisskeyConnectionError {
    fn from(value: InvalidUri) -> Self {
        Self::InvalidUriError(value)
    }
}

#[derive(Debug, Getters, Deserialize)]
pub struct ServerError {
    message: String,
    code: String,
    id: String,
    kind: String,
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum ServerErrorType {
//     /// サーバーから CREDENTIAL_REQUIRED の応答があったとき。
//     CredentialRequired,
//     /// サーバーから RATE_LIMIT_EXCEEDED の応答があったとき。
//     RateLimitExceeded,
//     /// サーバーから ACCESS_DENIED の応答があったとき。
//     AccessDenied,
//     /// サーバーから PERMISSION_DENIED の応答があったとき。
//     PermissionDenied,
//     /// サーバーから YOUR_ACCOUNT_SUSPENDED の応答があったとき。
//     YourAccountSuspended,
//     /// サーバーから INVALID_PARAM の応答があったとき。
//     InvalidParam,
//     /// サーバーから INTERNAL_ERROR の応答があったとき。
//     InternalError,
// }
