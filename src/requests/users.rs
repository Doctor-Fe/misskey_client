use chrono::{DateTime, Utc};
use serde_derive::Serialize;

use crate::{
    responses::{notes::NoteInfo, users::LiteUserInfo},
    MisskeyClientRequest,
};

/// ユーザー名をもとに、簡略化されたユーザー情報を取得する
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLiteUserInfo<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<&'a str>,
    detailed: bool,
}

impl<'a> GetLiteUserInfo<'a> {
    pub fn from_name(user_name: &'a str) -> Self {
        Self {
            username: Some(user_name), user_id: None, detailed: false,
        }
    }

    pub fn from_id(user_id: &'a str) -> Self {
        Self {
            username: None, user_id: Some(user_id), detailed: false,
        }
    }
}

impl MisskeyClientRequest for GetLiteUserInfo<'_> {
    const ENDPOINT: &'static str = "/api/users/show";

    type Response = LiteUserInfo;
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNotes<'a> {
    user_id: &'a str,
    with_replies: bool,
    with_renotes: bool,
    with_channel_notes: bool,
    /// 1以上100以下. 省略時は10.
    limit: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    since_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    since_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_date: Option<i64>,
    allow_partial: bool,
    with_files: bool,
}

impl<'a> GetNotes<'a> {
    pub fn new(user_id: &'a str) -> Self {
        Self {
            user_id,
            with_replies: false,
            with_renotes: true,
            with_channel_notes: false,
            limit: 10,
            since_id: None,
            until_id: None,
            since_date: None,
            until_date: None,
            allow_partial: false,
            with_files: false,
        }
    }

    pub fn with_replies(self, with_replies: bool) -> Self {
        Self {
            with_replies,
            .. self
        }
    }

    pub fn with_renotes(self, with_renotes: bool) -> Self {
        Self {
            with_renotes,
            .. self
        }
    }

    pub fn with_channel_notes(self, with_channel_notes: bool) -> Self {
        Self {
            with_channel_notes,
            .. self
        }
    }

    pub fn limit(self, limit: usize) -> Self {
        Self {
            limit,
            .. self
        }
    }

    pub fn since_id(self, since_id: String) -> Self {
        Self {
            since_id: Some(since_id),
            .. self
        }
    }

    pub fn until_id(self, until_id: String) -> Self {
        Self {
            until_id: Some(until_id),
            .. self
        }
    }

    pub fn since_date(self, since_date: DateTime<Utc>) -> Self {
        Self {
            since_date: Some(since_date.timestamp()),
            .. self
        }
    }

    pub fn until_date(self, until_date: DateTime<Utc>) -> Self {
        Self {
            until_date: Some(until_date.timestamp()),
            .. self
        }
    }

    pub fn allow_partial(self, allow_partial: bool) -> Self {
        Self {
            allow_partial,
            .. self
        }
    }

    pub fn with_files(self, with_files: bool) -> Self {
        Self {
            with_files,
            .. self
        }
    }
}

impl MisskeyClientRequest for GetNotes<'_> {
    const ENDPOINT: &'static str = "/api/users/notes";

    type Response = Vec<NoteInfo>;
}
