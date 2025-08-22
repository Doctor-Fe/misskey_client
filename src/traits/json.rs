use serde::{Deserialize, Serialize};

use super::MisskeyClientRequest;

pub trait FixedEndpointJsonRequest : Serialize where for<'de> Self::Response: Deserialize<'de> {
    /// レスポンスの型
    type Response;
    /// リクエスト先のエンドポイントのアドレス。<br />
    /// 先頭にスラッシュが必要。`/api` は不要。
    const ENDPOINT: &'static str;
}

impl<T> JsonRequest for T where T: FixedEndpointJsonRequest {
    type Response = T::Response;
    
    fn endpoint(&self) -> String {
        Self::ENDPOINT.to_string()
    }
}

/// Misskey サーバーへ送信可能な構造体であることを示すトレイト
pub trait JsonRequest : Serialize where for<'de> Self::Response: Deserialize<'de> {
    /// レスポンスの型
    type Response;
    /// リクエスト先のエンドポイントのアドレス。<br />
    /// 先頭にスラッシュが必要。`/api` は不要。
    fn endpoint(&self) -> String;
}

impl<T> MisskeyClientRequest for T where T: JsonRequest {
    type Response = T::Response;

    fn endpoint(&self) -> String {
        JsonRequest::endpoint(self)
    }

    fn content_type(&self) -> Option<String> {
        Some("application/json".to_string())
    }

    fn body(&self, token: Option<&str>) -> String {
        serde_json::to_string(&RequestWithToken::new(token, self)).unwrap()
    }
}

#[derive(Debug, serde_derive::Serialize)]
struct RequestWithToken<'a, T> where T: MisskeyClientRequest + Serialize {
    #[serde(rename = "i", skip_serializing_if = "Option::is_none")]
    token: Option<&'a str>,
    #[serde(flatten)]
    request: &'a T,
}

impl<'a, T> RequestWithToken<'a, T> where T: MisskeyClientRequest + Serialize {
    fn new(token: Option<&'a str>, request: &'a T) -> Self {
        Self {
            token,
            request,
        }
    }
}
