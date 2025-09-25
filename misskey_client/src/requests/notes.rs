use misskey_client_macroes::ConstParamJsonRequest;
use serde_derive::Serialize;

use crate::{common::{NoteVisibility, ReactionAcceptance}, responses::notes::{CreatedNoteInfo, NoteInfo}, traits::{ChannelId, NoteId}};

#[derive(Debug, Serialize, ConstParamJsonRequest)]
#[misskey_client(endpoint = "/notes/create", response = CreatedNoteInfo, can_be_empty = true)]
#[serde(rename_all = "camelCase")]
pub struct CreateNote {
    visibility: NoteVisibility,
    #[serde(skip_serializing_if = "Vec::is_empty")] visible_user_ids: Vec<String>,
    cw: Option<String>,
    local_only: bool,
    reaction_acceptance: Option<ReactionAcceptance>,
    no_extract_mentions: bool,
    no_extract_hashtags: bool,
    no_extract_emojis: bool,
    reply_id: Option<String>,
    renote_id: Option<String>,
    channel_id: Option<String>,
    text: String,
    // file_ids: Vec<String>,
    // media_ids: Vec<String>,
    // poll: Poll,
    // scheduled_at: usize,
    no_created_note: bool,
}

impl CreateNote {
    pub fn renote(renote_id: impl NoteId) -> Self {
        Self {
            visibility: NoteVisibility::Public,
            visible_user_ids: vec![],
            cw: None,
            local_only: false,
            reaction_acceptance: None,
            no_extract_hashtags: false,
            no_extract_mentions: false,
            no_extract_emojis: false,
            reply_id: None,
            renote_id: Some(renote_id.to_note_id()),
            channel_id: None,
            text: String::new(),
            no_created_note: false,
        }
    }

    pub fn note(text: impl ToString) -> Self {
        Self {
            visibility: NoteVisibility::Public,
            visible_user_ids: vec![],
            cw: None,
            local_only: false,
            reaction_acceptance: None,
            no_extract_hashtags: false,
            no_extract_mentions: false,
            no_extract_emojis: false,
            reply_id: None,
            renote_id: None,
            channel_id: None,
            text: text.to_string(),
            no_created_note: false,
        }
    }

    pub fn quote(text: impl ToString, renote_id: impl NoteId) -> Self {
        Self {
            visibility: NoteVisibility::Public,
            visible_user_ids: vec![],
            cw: None,
            local_only: false,
            reaction_acceptance: None,
            no_extract_hashtags: false,
            no_extract_mentions: false,
            no_extract_emojis: false,
            reply_id: None,
            renote_id: Some(renote_id.to_note_id()),
            channel_id: None,
            text: text.to_string(),
            no_created_note: false,
        }
    }

    pub fn visibility(self, visibility: NoteVisibility) -> Self {
        Self {
            visibility,
            .. self
        }
    }

    /// 閲覧可能なユーザー ID を指定する関数。
    /// 公開範囲は自動で `NoteVisibility::Specified` に変更される。
    pub fn visible_users(self, users: Vec<String>) -> Self {
        Self {
            visibility: NoteVisibility::Specified,
            visible_user_ids: users,
            .. self
        }
    }

    pub fn cw(mut self, cw: impl ToString) -> Self {
        self.cw = Some(cw.to_string());
        self
    }

    pub fn local_only(self, local_only: bool) -> Self {
        Self {
            local_only,
            .. self
        }
    }

    pub fn reaction_acceptance(self, acceptance: ReactionAcceptance) -> Self {
        Self {
            reaction_acceptance: Some(acceptance),
            .. self
        }
    }

    pub fn reply(self, reply_id: impl NoteId) -> Self {
        Self {
            reply_id: Some(reply_id.to_note_id()),
            .. self
        }
    }

    pub fn channel(self, channel_id: impl ChannelId) -> Self {
        Self {
             channel_id: Some(channel_id.to_channel_id()),
             .. self
        }
    }

    /// `true` をセットすると、作製に成功したときのレスポンスが `204` になります。
    pub fn no_created_note(self, no_created_note: bool) -> Self {
        Self {
            no_created_note,
            .. self
        }
    }
}

#[derive(Debug, Serialize, ConstParamJsonRequest)]
#[misskey_client(endpoint = "/notes/search", response = Vec<NoteInfo>)]
#[serde(rename_all = "camelCase")]
pub struct SearchNote<'a> {
    query: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")] since_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] until_id: Option<&'a str>,
    limit: usize,
    offset: usize,
    #[serde(skip_serializing_if = "Option::is_none")] host: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] channel_id: Option<String>,
}

impl<'a> SearchNote<'a> {
    pub fn new(query: &'a str) -> Self {
        Self {
            query,
            since_id: None,
            until_id: None,
            limit: 10,
            offset: 0,
            host: None,
            user_id: None,
            channel_id: None,
        }
    }

    pub fn since(mut self, since_id: &'a str) -> Self {
        self.since_id = Some(since_id);
        self
    }

    pub fn until(mut self, until_id: &'a str) -> Self {
        self.until_id = Some(until_id);
        self
    }

    pub fn host(mut self, host: &'a str) -> Self {
        self.host = Some(host);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn channel_id(mut self, channel_id: String) -> Self {
        self.channel_id = Some(channel_id);
        self
    }
}

#[derive(Clone, Debug, Serialize, ConstParamJsonRequest)]
#[misskey_client(endpoint = "/notes/delete", response = (), can_be_empty = true)]
#[serde(rename_all = "camelCase")]
pub struct DeleteNote {
    note_id: String,
}

impl DeleteNote {
    pub fn new(note_id: impl NoteId) -> Self {
        Self { note_id: note_id.to_note_id() }
    }
}
