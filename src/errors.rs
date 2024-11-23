use std::{error::Error, fmt::Display, io, net::TcpStream};

use derive_getters::Getters;
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
    
    // クライアント側で発生したエラー。
    /// シリアル化または逆シリアル化に失敗したとき
    SerdeError(serde_json::Error),
    
    // Misskey サーバーからエラーの応答があったとき。
    ServerResponseError(ServerError),
}

impl Error for MisskeyConnectionError {}

impl Display for MisskeyConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
