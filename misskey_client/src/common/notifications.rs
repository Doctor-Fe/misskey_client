use std::fmt::Display;

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NotificationType {
    Note,
    Follow,
    Mention,
    Reply,
    Renote,
    Quote,
    Reaction,
    PollEnded,
    ReceiveFollowRequest,
    FollowRequestAccepted,
    RoleAssigned,
    AchievementEarned,
    App,
    Test,
    PollVote,
    GroupInvited,
}

impl Display for NotificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            NotificationType::Note => "note",
            NotificationType::Follow => "follow",
            NotificationType::Mention => "mention",
            NotificationType::Reply => "reply",
            NotificationType::Renote => "renote",
            NotificationType::Quote => "quote",
            NotificationType::Reaction => "reaction",
            NotificationType::PollEnded => "pollEnded",
            NotificationType::ReceiveFollowRequest => "receiveFollowRequest",
            NotificationType::FollowRequestAccepted => "followRequestAccepted",
            NotificationType::RoleAssigned => "roleAssigned",
            NotificationType::AchievementEarned => "achievementEarned",
            NotificationType::App => "app",
            NotificationType::Test => "test",
            NotificationType::PollVote => "pollVote",
            NotificationType::GroupInvited => "groupInvited",
        })
    }
}
