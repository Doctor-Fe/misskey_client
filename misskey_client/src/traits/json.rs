use serde::{Deserialize, Serialize};

use super::MisskeyClientRequest;

pub trait ConstParamJsonRequest : Serialize where for<'de> Self::Response: Deserialize<'de> {
    /// レスポンスの型
    type Response;
    /// リクエスト先のエンドポイントのアドレス。<br />
    /// 先頭にスラッシュが必要。`/api` は不要。
    const ENDPOINT: &'static str;
    const CAN_BE_EMPTY: bool = false;
}

impl<T> JsonRequest for T where T: ConstParamJsonRequest {
    type Response = T::Response;

    fn endpoint(&self) -> String {
        Self::ENDPOINT.to_string()
    }

    fn can_be_empty(&self) -> bool {
        Self::CAN_BE_EMPTY
    }
}

/// Misskey サーバーへ送信可能な構造体であることを示すトレイト
pub trait JsonRequest : Serialize where for<'de> Self::Response: Deserialize<'de> {
    /// レスポンスの型
    type Response;
    /// リクエスト先のエンドポイントのアドレス。<br />
    /// 先頭にスラッシュが必要。`/api` は不要。
    fn endpoint(&self) -> String;
    fn can_be_empty(&self) -> bool { false }
}

impl<T> MisskeyClientRequest for T where T: JsonRequest {
    type Response = T::Response;

    fn endpoint(&self) -> impl ToString {
        JsonRequest::endpoint(self)
    }

    fn content_type(&self) -> Option<impl ToString> {
        Some("application/json")
    }

    fn body(&self, token: Option<&str>) -> impl ToString {
        serde_json::to_string(&RequestWithToken::new(token, self)).unwrap()
    }

    fn can_be_empty(&self) -> bool {
        <Self as JsonRequest>::can_be_empty(self)
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
