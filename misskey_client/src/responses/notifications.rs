use derive_getters::Getters;
use serde_derive::{Deserialize, Serialize};

use crate::UnknownValue;

use super::{notes::{NoteInfo, UserPolicies}, users::LiteUserInfo};

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct NotificationInfo {
    id: String,
    created_at: String,
    #[serde(flatten)]
    notification: NotificationDetail,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "type")]
pub enum NotificationDetail {
    Note {
        user: LiteUserInfo,
        user_id: String,
        note: NoteInfo,
    },
    Mention {
        user: LiteUserInfo,
        user_id: String,
        note: NoteInfo,
    },
    Reply {
        user: LiteUserInfo,
        user_id: String,
        note: NoteInfo,
    },
    Renote {
        user: LiteUserInfo,
        user_id: String,
        note: NoteInfo,
    },
    Quote {
        user: LiteUserInfo,
        user_id: String,
        note: NoteInfo,
    },
    Reaction {
        user: LiteUserInfo,
        user_id: String,
        note: NoteInfo,
        reaction: String,
    },
    PollEnded {
        user: LiteUserInfo,
        user_id: String,
        note: NoteInfo,
    },
    Follow {
        user: LiteUserInfo,
        user_id: String,
    },
    ReceiveFollowRequest {
        user: LiteUserInfo,
        user_id: String,
    },
    FollowRequestAccepted {
        user: LiteUserInfo,
        user_id: String,
    },
    RoleAssigned {
        role: RoleInfo,
    },
    AchievementEarned {
        achievement: String,
    },
    App {
        body: String,
        header: String,
        icon: String,
    },
    #[serde(rename = "reaction:grouped")]
    ReactionGrouped {
        note: NoteInfo,
        #[serde(default)]
        reactions: Vec<UserReactionPair>,
    },
    #[serde(rename = "renote:grouped")]
    RenoteGrouped {
        note: NoteInfo,
        #[serde(default)]
        users: Vec<LiteUserInfo>,
    },
    Test,
}

#[derive(Debug, Deserialize, Getters)]
pub struct UserReactionPair {
    user: LiteUserInfo,
    reaction: String,
}

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct RoleInfo {
    id: String,
    name: String,
    color: Option<String>,
    icon_url: Option<String>,
    description: String,
    is_moderator: bool,
    is_administrator: bool,
    display_order: i32,
    created_at: String,
    updated_at: String,
    target: RoleTarget,
    cond_formula: UnknownValue,
    is_public: bool,
    is_explorable: bool,
    as_badge: bool,
    badge_behavior: Option<String>,
    can_edit_members_by_moderator: bool,
    policies: UserPolicies,
    users_count: usize,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RoleTarget {
    Manual,
    Conditional,
}

// pub struct RolePolicy {
//     //
// }
