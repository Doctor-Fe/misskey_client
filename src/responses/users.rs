use std::{fmt::{Debug, Display}, str::FromStr};

use derive_getters::Getters;
use serde_derive::{Deserialize, Serialize};

use crate::{errors::InvalidEnumString, UnknownValue};

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
