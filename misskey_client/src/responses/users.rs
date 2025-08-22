use std::{fmt::{Debug, Display}, str::FromStr};

use chrono::{DateTime, Utc};
use derive_getters::Getters;
use serde_derive::{Deserialize, Serialize};

use crate::{common::StatusVisibility, errors::InvalidEnumString, responses::notes::{AchievementInfo, FieldInfo, NoteInfo, RoleInfo, UserPolicies}, UnknownValue};

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct LiteUserInfo {
    id: String,
    name: Option<String>,
    username: String,
    host: Option<String>,
    avatar_url: Option<String>,
    avatar_blur_hash: Option<String>,
    avatar_decorations: Vec<AvatarDecorationInfo>,
    is_bot: bool,
    is_cat: bool,
    instance: Option<InstanceInfo>,
    emojis: UnknownValue, // 用途不明
    online_status: OnlineStatus,
    #[serde(default)] badge_roles: Vec<BadgeRoleInfo>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct InstanceInfo {
    name: Option<String>,
    software_name: Option<String>,
    software_version: Option<String>,
    icon_url: Option<String>,
    favicon_url: Option<String>,
    theme_color: Option<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OnlineStatus {
    Active,
    Offline,
    Online,
    Unknown,
}

impl Display for OnlineStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Active => "active",
            Self::Offline => "offline",
            Self::Online => "online",
            Self::Unknown => "unknown",
        })
    }
}

impl FromStr for OnlineStatus {
    type Err = InvalidEnumString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "active" => Self::Active,
            "offline" => Self::Offline,
            "online" => Self::Online,
            "unknown" => Self::Unknown,
            _ => return Err(InvalidEnumString),
        })
    }
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct AvatarDecorationInfo {
    id: String,
    #[serde(default)] angle: f64,
    #[serde(default)] flip_h: bool,
    url: String,
    #[serde(default)] offset_x: f64,
    #[serde(default)] offset_y: f64,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct BadgeRoleInfo {
    name: String,
    icon_url: Option<String>,
    display_order: i32,
    behavior: Option<String>,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct RelationInfo {
    id: String,
    is_following: bool,
    has_pending_follow_request_from_you: bool,
    has_pending_follow_request_to_you: bool,
    is_followed: bool,
    is_blocking: bool,
    is_blocked: bool,
    is_muted: bool,
    is_renote_muted: bool,
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
    followers_visibility: StatusVisibility,
    following_count: usize,
    following_visibility: StatusVisibility,
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
