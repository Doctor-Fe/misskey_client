use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, Utc};
use derive_getters::Getters;
use serde_derive::{Deserialize, Serialize};

use crate::{errors::InvalidEnumString, traits::NoteId, UnknownValue};

use super::{channels::LiteChannelInfo, users::{AvatarDecorationInfo, BadgeRoleInfo, LiteUserInfo, OnlineStatus}};

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
    // files
    #[serde(default)] tags: Vec<String>,
    poll: Option<PollInfo>,
    emojis: Option<UnknownValue>, // 用途不明
    channel_id: Option<String>,
    channel: Option<LiteChannelInfo>,
    local_only: bool,
    reaction_acceptance: Option<String>,
    reaction_emojis: UnknownValue, // 型不明
    reactions: UnknownValue, // 同じく
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

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NoteVisibility {
    Public,
    Home,
    Followers,
    Specified,
}

impl Default for NoteVisibility {
    fn default() -> Self {
        NoteVisibility::Public
    }
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
    type Err = InvalidEnumString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "public" => Self::Public,
            "home" => Self::Home,
            "followers" => Self::Followers,
            "specified" => Self::Specified,
            _ => return Err(InvalidEnumString),
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
pub struct DetailedUserInfo {
    #[serde(default)] achievements: Vec<AchievementInfo>,
    also_known_as: UnknownValue,
    always_mark_nsfw: Option<bool>,
    auto_accept_followed: Option<bool>,
    auto_sensitive: Option<bool>,
    avatar_blurhash: Option<String>,
    #[serde(default)] avatar_decorations: Vec<AvatarDecorationInfo>,
    avatar_id: Option<String>,
    avatar_url: Option<String>, //  TODO Uri に変更を検討
    #[serde(default)] badge_roles: Vec<BadgeRoleInfo>,
    banner_blurhash: Option<String>,
    banner_id: Option<String>,
    banner_url: Option<String>,
    birthday: Option<String>, // TODO 専用の構造体の作成を検討
    #[serde(default)] careful_bot: bool,
    created_at: Option<DateTime<Utc>>,
    description: Option<String>, // TODO null が入ることがあるかのチェック
    #[serde(default)] email_notification_types: Vec<UnknownValue>, // 中身は何か
    emojis: UnknownValue,
    #[serde(default)] fields: Vec<FieldInfo>,
    followers_count: usize,
    followers_visibility: String, // TODO 列挙型を作る
    following_count: usize,
    following_visibility: String, // TODO 列挙型を作る
    has_pending_received_follow_request: Option<bool>,
    has_unread_announcement: Option<bool>,
    has_unread_antenna: Option<bool>,
    has_unread_channel: Option<bool>,
    has_unread_mentions: Option<bool>,
    has_unread_notifications: Option<bool>,
    has_unread_specified_notes: Option<bool>,
    #[serde(default)] hide_online_status: bool,
    host: Option<String>,
    id: String,
    #[serde(default)] inject_featured_note: bool,
    #[serde(default)] is_admin: bool,
    is_bot: bool,
    is_cat: bool,
    #[serde(default)] is_deleted: bool,
    #[serde(default)] is_explorable: bool,
    #[serde(default)] is_limited: bool,
    is_locked: bool,
    #[serde(default)] is_moderator: bool,
    is_silenced: bool,
    is_suspended: bool,
    lang: Option<String>,
    last_fetched_at: Option<UnknownValue>, // TODO DateTime に変更
    location: Option<String>,
    #[serde(default)] logged_in_days: usize,
    memo: Option<String>,
    moved_to: Option<UnknownValue>,
    #[serde(default)] muted_instanced: Vec<String>,
    #[serde(default)] muted_words: Vec<Vec<String>>,
    #[serde(default)] muting_notification_types: Vec<String>, // TODO 列挙型を作る
    #[serde(default)] mutual_link_sections: Vec<String>,
    #[serde(default)] name: Option<String>,
    #[serde(default)] no_crawle: bool,
    notes_count: usize,
    notification_receive_config: Option<UnknownValue>, // TODO 構造体を作る
    online_status: OnlineStatus,
    #[serde(default)] pinned_note_ids: Vec<String>,
    #[serde(default)] pinned_notes: Vec<NoteInfo>,
    pinned_page: Option<UnknownValue>, // TODO 構造体を作る
    pinned_page_id: Option<String>,
    policies: Option<UserPolicies>,
    #[serde(default)] prevent_ai_learning: bool,
    public_reactions: bool,
    receive_announcement_email: Option<bool>,
    #[serde(default)] roles: Vec<RoleInfo>,
    #[serde(default)] security_keys: bool,
    #[serde(default)] two_factor_enabled: bool,
    #[serde(default)] unread_announcements: Vec<UnknownValue>,
    #[serde(default)] unread_notifications_count: usize,
    #[serde(default)] updated_at: Option<DateTime<Utc>>,
    uri: Option<String>,
    url: Option<String>,
    #[serde(default)] user_password_less_login: bool,
    username: String,
    #[serde(default)] verified_links: Vec<UnknownValue>,
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
