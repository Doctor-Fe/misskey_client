use std::{error::Error, fmt::{Debug, Display}, io, string::FromUtf8Error};

use derive_getters::Getters;
use http::uri::InvalidUri;
use serde_derive::Deserialize;

use crate::ServerErrorResponse;

pub type MisskeyConnectionResult<T> = Result<T, MisskeyConnectionError>;

#[derive(Debug)]
pub enum MisskeyConnectionError {
    /// TCP 通信にエラーが発生したとき。
    IoError(io::Error),

    /// HTTP 通信でエラーが発生したとき。
    HttpError(http::Error),
    /// 無効な URI
    InvalidUriError(http::uri::InvalidUri),
    InvalidUriPartsError(http::uri::InvalidUriParts),

    /// UTF-8以外の文字列
    NotUtf8Error(FromUtf8Error),

    /// シリアル化または逆シリアル化に失敗したとき
    SerdeError {
        /// レスポンスを正常な応答として解釈しようとした時のエラー
        parent_error: serde_json::Error,
        /// レスポンスをエラーの応答として解釈しようとした時のエラー
        error: serde_json::Error,
        /// 受け取った文字列
        raw_string: String
    },
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

impl From<http::uri::InvalidUriParts> for MisskeyConnectionError {
    fn from(value: http::uri::InvalidUriParts) -> Self {
        Self::InvalidUriPartsError(value)
    }
}

impl From<FromUtf8Error> for MisskeyConnectionError {
    fn from(value: FromUtf8Error) -> Self {
        Self::NotUtf8Error(value)
    }
}

impl From<ServerErrorResponse> for MisskeyConnectionError {
    fn from(value: ServerErrorResponse) -> Self {
        From::<ServerError>::from(value.error)
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

pub struct InvalidEnumString;

impl Debug for InvalidEnumString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("invalid enum string")
    }
}

impl Display for InvalidEnumString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for InvalidEnumString {}

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
