use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, Utc};
use derive_getters::Getters;
use serde_derive::{Deserialize, Serialize};

use super::{channels::LiteChannelInfo, users::LiteUserInfo};

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct NoteInfo {
    id: String,
    created_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
    text: Option<String>,
    cw: Option<String>,
    user_id: String,
    user: LiteUserInfo,
    reply_id: Option<String>,
    renote_id: Option<String>,
    reply: Option<Box<NoteInfo>>,
    renote: Option<Box<NoteInfo>>,
    #[serde(default)]
    is_hidden: bool,
    visibility: NoteVisibility,
    #[serde(default)]
    mentions: Vec<String>,
    #[serde(default)]
    visible_user_ids: Vec<String>,
    #[serde(default)]
    file_ids: Vec<String>,
    // files
    #[serde(default)]
    tags: Vec<String>,
    poll: Option<PollInfo>,
    emojis: Option<serde_json::Value>, // 用途不明
    channel_id: Option<String>,
    channel: Option<LiteChannelInfo>,
    local_only: bool,
    reaction_acceptance: Option<String>,
    reaction_emojis: serde_json::Value, // 型不明
    reactions: serde_json::Value, // 同じく
    reaction_count: usize,
    renote_count: usize,
    replies_count: usize,
    uri: Option<String>,
    url: Option<String>,
    #[serde(default)]
    reaction_and_user_pair_cache: Vec<String>,
    clipped_count: usize,
    my_reaction: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NoteVisibility {
    Public,
    Home,
    Followers,
    Specified,
}

impl Display for NoteVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Public => "public",
            Self::Home => "home",
            Self::Followers => "followers",
            Self::Specified => "specified",
        })
    }
}

impl FromStr for NoteVisibility {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "public" => Self::Public,
            "home" => Self::Home,
            "followers" => Self::Followers,
            "specified" => Self::Specified,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct PollInfo {
    expires_at: Option<DateTime<Utc>>,
    can_choose_multiple: bool,
    choices: Vec<PollChoiceInfo>,
}

#[derive(Debug, Deserialize, Getters)]
pub struct PollChoiceInfo {
    #[serde(rename = "isVoted")]
    is_voted: bool,
    text: String,
    votes: usize,
}

#[derive(Debug, Deserialize)]
pub struct CreatedNoteInfo {
    #[serde(rename = "createdNote")]
    created_note: NoteInfo
}

impl CreatedNoteInfo {
    pub fn created_note(self) -> NoteInfo {
        self.created_note
    }
}
