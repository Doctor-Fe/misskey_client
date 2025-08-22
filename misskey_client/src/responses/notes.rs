use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use derive_getters::Getters;
use serde_derive::Deserialize;

use crate::{common::NoteVisibility, traits::NoteId, UnknownValue};

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
    #[serde(default)] is_hidden: bool,
    visibility: NoteVisibility,
    #[serde(default)] mentions: Vec<String>,
    #[serde(default)] visible_user_ids: Vec<String>,
    #[serde(default)] file_ids: Vec<String>,
    #[serde(default)] files: Vec<FileInfo>,
    #[serde(default)] tags: Vec<String>,
    poll: Option<PollInfo>,
    emojis: Option<UnknownValue>, // 用途不明
    channel_id: Option<String>,
    channel: Option<LiteChannelInfo>,
    local_only: bool,
    reaction_acceptance: Option<String>,
    reaction_emojis: BTreeMap<String, usize>,
    reactions: BTreeMap<String, usize>,
    reaction_count: usize,
    renote_count: usize,
    replies_count: usize,
    uri: Option<String>,
    url: Option<String>,
    #[serde(default)] reaction_and_user_pair_cache: Vec<String>,
    clipped_count: usize,
    my_reaction: Option<String>,
}

impl NoteId for NoteInfo {
    fn to_note_id(self) -> String {
        self.id
    }
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    blurhash: Option<String>,
    comment: Option<String>,
    created_at: DateTime<Utc>,
    folder_id: Option<String>,
    id: String,
    is_sensitive: bool,
    md5: String,
    name: String,
    properties: FileProperties,
    size: usize,
    thumbnail_url: Option<String>,
    #[serde(rename = "type")] file_type: String,
    url: String,
    user_id: Option<String>,
    folder: Option<FolderInfo>,
    user: Option<LiteUserInfo>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct FolderInfo {
    created_at: DateTime<Utc>,
    id: String,
    name: String,
    parent_id: Option<String>,
    filed_count: usize,
    folders_count: usize,
    parent: Option<Box<FolderInfo>>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct FileProperties {
    avg_color: Option<String>,
    height: Option<usize>,
    orientation: Option<usize>,
    width: Option<usize>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct PollInfo {
    expires_at: Option<DateTime<Utc>>,
    can_choose_multiple: bool,
    choices: Vec<PollChoiceInfo>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct PollChoiceInfo {
    is_voted: bool,
    text: String,
    votes: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedNoteInfo {
    created_note: NoteInfo,
}

impl CreatedNoteInfo {
    pub fn created_note(self) -> NoteInfo {
        self.created_note
    }
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AchievementInfo {
    name: String,
    unlocked_at: usize,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct FieldInfo {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct UserPolicies {
    always_mark_nsfw: bool,
    antenna_limit: usize,
    antenna_notes_limit: usize,
    avatar_decoration_limit: usize,
    can_create_content: bool,
    can_delete_content: bool,
    can_hide_ads: bool,
    can_initiate_conversation: bool,
    can_invite: bool,
    can_manage_avatar_decorations: bool,
    can_manage_custom_emojis: bool,
    can_public_note: bool,
    can_purge_account: bool,
    can_schedule_note: bool,
    can_search_notes: bool,
    can_update_avatar: bool,
    can_update_banner: bool,
    can_update_content: bool,
    can_use_drive_file_in_sound_settings: bool,
    can_use_reaction: bool,
    can_use_translator: bool,
    clip_limit: usize,
    drive_capacity_mb: usize,
    gtl_available: bool,
    invite_expiration_time: usize,
    invite_limit: usize,
    invite_limit_cycle: usize,
    ltl_available: bool,
    mention_limit: usize,
    mutual_link_limit: usize,
    mutual_link_section_limit: usize,
    note_each_clips_limit: usize,
    pin_limit: usize,
    rate_limit_factor: usize,
    schedule_note_limit: usize,
    schedule_note_max_days: usize,
    skip_nsfw_detection: bool,
    user_each_user_lists_limit: usize,
    user_list_limit: usize,
    webhook_limit: usize,
    word_mute_limit: usize,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct RoleInfo {
    color: Option<String>,
    description: String,
    display_order: i32,
    icon_url: Option<String>,
    id: String,
    is_administrator: bool,
    is_moderator: bool,
    name: String,
}
