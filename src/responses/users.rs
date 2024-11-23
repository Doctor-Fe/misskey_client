use std::{fmt::Display, str::FromStr};

use derive_getters::Getters;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct LiteUserInfo {
    id: String,
    name: Option<String>,
    username: String,
    host: Option<String>,
    avatar_url: Option<String>,
    avatar_blur_hash: Option<String>,
    avatar_decorations: Vec<DecorationInfo>,
    is_bot: bool,
    is_cat: bool,
    instance: Option<InstanceInfo>,
    emojis: serde_json::Value, // 用途不明
    online_status: OnlineStatus,
    badge_roles: Vec<BadgeRoleInfo>,
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
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "active" => Self::Active,
            "offline" => Self::Offline,
            "online" => Self::Online,
            "unknown" => Self::Unknown,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct DecorationInfo {
    id: String,
    #[serde(default)]
    offset_x: f64,
    #[serde(default)]
    offset_y: f64,
    url: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct BadgeRoleInfo {
    name: String,
    icon_url: Option<String>,
    display_order: i32,
    behavior: Option<String>,
}
