use chrono::{DateTime, Utc};
use derive_getters::Getters;
use serde_derive::Deserialize;

use crate::responses::notes::NoteInfo;

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct ChannelInfo {
    id: String,
    created_at: DateTime<Utc>,
    last_noted_at: Option<DateTime<Utc>>,
    name: String,
    description: Option<String>,
    user_id: Option<String>,
    banner_url: Option<String>,
    #[serde(default)] pinned_note_ids: Vec<String>,
    color: String,
    is_archived: bool,
    users_count: usize,
    notes_count: usize,
    is_sensitive: bool,
    allow_renote_to_external: bool,
    is_following: bool,
    is_favorited: bool,
    #[serde(default)] pinned_notes: Vec<NoteInfo>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct LiteChannelInfo {
    id: String,
    name: String,
    user_id: Option<String>,
    color: String,
    is_sensitive: bool,
    allow_renote_to_external: bool,
}
