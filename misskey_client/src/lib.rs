//! Misskey API へのアクセスを提供するクレート

use errors::ServerError;
use http::uri::{Authority, InvalidUri};

pub use traits::MisskeyClientRequest;
pub use traits::json::{FixedEndpointJsonRequest, JsonRequest};

use crate::errors::MisskeyConnectionResult;
use crate::miauth::MiAuthBuilder;

// TODO レスポンス型に Clone トレイトを実装するべきか否かの検討。

pub mod errors;
pub mod requests;
pub mod responses;
pub mod traits;
pub mod miauth;
#[cfg(not(feature = "async"))]
pub mod sync;
#[cfg(feature = "async")]
pub mod r#async;
pub mod common;

pub type UnknownValue = serde_json::Value;

pub struct MisskeyHttpClient<T> {
    access_token: Option<String>,
    authority: Authority,
    stream: T,
}

impl<T> MisskeyHttpClient<T> {
    #[inline]
    pub fn new(stream: T, authority: impl TryInto<Authority, Error = InvalidUri>) -> MisskeyConnectionResult<Self> {
        Ok(Self::internal_new(stream, authority.try_into()?, None))
    }
    
    #[inline]
    pub fn login(self, access_token: impl Into<String>) -> Self {
        Self::internal_new(self.stream, self.authority, Some(access_token.into()))
    }
    
    #[inline]
    pub fn logout(self) -> Self {
        Self::internal_new(self.stream, self.authority, None)
    }
    
    #[inline]
    fn internal_new(stream: T, authority: Authority, access_token: Option<String>) -> MisskeyHttpClient<T> {
        MisskeyHttpClient { access_token, authority, stream }
    }

    #[inline]
    pub fn miauth(self) -> MiAuthBuilder<T> {
        MiAuthBuilder::new(self)
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
