use http::Uri;
use itertools::Itertools;
use std::{collections::HashSet, fmt::Display, str::FromStr};
use serde_derive::Deserialize;
use uuid::Uuid;

use crate::{errors::InvalidEnumString, MisskeyClientRequest, MisskeyHttpClient};
use crate::responses::notes::DetailedUserInfo;

pub struct MiAuth<T> {
    pub(crate) client: MisskeyHttpClient<T>,
    uri: Uri,
    pub(crate) info: MiAuthInfo,
}

impl<T> MiAuth<T> {
    pub fn get_uri(&self) -> &Uri {
        &self.uri
    }
}


#[derive(Clone, Copy, Debug)]
pub(crate) struct MiAuthInfo(Uuid);

impl MisskeyClientRequest for MiAuthInfo {
    type Response = MiAuthServerResponse;

    fn endpoint(&self) -> String {
        format!("/api/miauth/{}/check", self.0)
    }

    fn content_type(&self) -> Option<String> {
        None
    }

    fn body(&self, _: Option<&str>) -> String {
        String::new()
    }
}

pub enum MiAuthStatus<T> {
    Pending(MiAuth<T>),
    Succeed(MisskeyHttpClient<T>, DetailedUserInfo),
}

pub struct MiAuthBuilder<T> {
    client: MisskeyHttpClient<T>,
    uuid: Uuid,
    uri: Option<String>,
    name: Option<String>,
    icon: Option<String>,
    callback: Option<String>,
    permission: HashSet<Permission>,
}

impl<T> MiAuthBuilder<T> {
    pub(crate) fn new(client: MisskeyHttpClient<T>) -> Self {
        Self {
            client,
            uuid: Uuid::new_v4(),
            uri: None,
            name: None,
            icon: None,
            callback: None,
            permission: HashSet::new(),
        }
    }

    pub fn set_app_name(self, name: String) -> Self {
        Self {
            name: Some(name),
            .. self
        }
    }

    pub fn set_app_icon(self, icon: impl Into<String>) -> Self {
        Self {
            icon: Some(icon.into()),
            .. self
        }
    }

    pub fn set_callback_uri(self, uri: impl Into<String>) -> Self {
        Self {
            uri: Some(uri.into()),
            .. self
        }
    }

    pub fn require(mut self, permission: Permission) -> Self {
        self.permission.insert(permission);
        self
    }

    pub fn require_all(mut self, permissions: impl IntoIterator<Item = Permission>) -> Self {
        for p in permissions {
            self.permission.insert(p);
        }
        self
    }

    pub fn build(self) -> MiAuth<T> {
        let Self {callback, client, icon, name, permission, uri, uuid} = self;
        let mut list = Vec::with_capacity(5);
        if !permission.is_empty() {
            list.push(format!("permission={}", permission.into_iter().join(",")));
        }
        for value in [("callback", callback), ("icon", icon), ("name", name), ("uri", uri)]
            .into_iter()
            .filter_map(|a| a.1.map(|b| format!("{}={}", a.0, b)))
        {
            list.push(value);
        }
        let uri = Uri::builder().authority(client.authority.clone())
            .path_and_query(format!("/miauth/{}{}{}",
                    uuid,
                    if list.is_empty() {""} else {"?"},
                    list.into_iter().join("&")
                )
            )
            .build()
            .expect("Internal uri error.");
        return MiAuth {client, info: MiAuthInfo(uuid), uri}
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct MiAuthServerResponse {
    pub(crate) ok: bool,
    pub(crate) token: Option<String>,
    pub(crate) user: Option<DetailedUserInfo>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Permission {
    ReadAccount,
    WriteAccount,
    ReadBlocks,
    WriteBlocks,
    ReadDrive,
    WriteDrive,
    ReadFavorites,
    WriteFavorites,
    ReadFollowing,
    WriteFollowing,
    ReadMessaging,
    WriteMessaging,
    ReadMutes,
    WriteMutes,
    WriteNotes,
    ReadNotifications,
    WriteNotifications,
    ReadReactions,
    WriteReactions,
    WriteVotes,
    ReadPages,
    WritePages,
    WritePageLikes,
    ReadPageLikes,
    ReadUserGroups,
    WriteUserGroups,
    ReadChannels,
    WriteChannels,
    ReadGallery,
    WriteGallery,
    ReadGalleryLikes,
    WriteGalleryLikes,
    ReadFlash,
    WriteFlash,
    ReadFlashLikes,
    WriteFlashLikes,
    ReadAdminAbuseUserReports,
    WriteAdminDeleteAccount,
    WriteAdminDeleteAllFilesOfAUser,
    ReadAdminIndexStats,
    ReadAdminTableStats,
    ReadAdminUserIps,
    ReadAdminMeta,
    WriteAdminResetPassword,
    WriteAdminResolveAbuseUserReport,
    WriteAdminSendEmail,
    ReadAdminServerInfo,
    ReadAdminShowModerationLog,
    ReadAdminShowUser,
    WriteAdminSuspendUser,
    WriteAdminUnsetUserAvatar,
    WriteAdminUnsetUserBanner,
    WriteAdminUnsuspendUser,
    WriteAdminMeta,
    WriteAdminUsernote,
    WriteAdminRoles,
    ReadAdminRoles,
    WriteAdminRelays,
    ReadAdminRelays,
    WriteAdminInviteCodes,
    ReadAdminInviteCodes,
    WriteAdminAnnouncements,
    ReadAdminAnnouncements,
    WriteAdminAvatarDecorations,
    ReadAdminAvatarDecorations,
    WriteAdminFederation,
    WriteAdminAccount,
    ReadAdminAccount,
    WriteAdminEmoji,
    ReadAdminEmoji,
    WriteAdminQueue,
    ReadAdminQueue,
    WriteAdminPromo,
    WriteAdminDrive,
    ReadAdminDrive,
    WriteAdminAd,
    ReadAdminAd,
    WriteInviteCodes,
    ReadInviteCodes,
    WriteClipFavorite,
    ReadClipFavorite,
    ReadFederation,
    WriteReportAbuse,
}

impl Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Permission::*;
        let text = match self {
            ReadAccount => "read:account",
            WriteAccount => "write:account",
            ReadBlocks => "read:blocks",
            WriteBlocks => "write:blocks",
            ReadDrive => "read:drive",
            WriteDrive => "write:drive",
            ReadFavorites => "read:favorites",
            WriteFavorites => "write:favorites",
            ReadFollowing => "read:following",
            WriteFollowing => "write:following",
            ReadMessaging => "read:messaging",
            WriteMessaging => "write:messaging",
            ReadMutes => "read:mutes",
            WriteMutes => "write:mutes",
            WriteNotes => "write:notes",
            ReadNotifications => "read:notifications",
            WriteNotifications => "write:notifications",
            ReadReactions => "read:reactions",
            WriteReactions => "write:reactions",
            WriteVotes => "write:votes",
            ReadPages => "read:pages",
            WritePages => "write:pages",
            WritePageLikes => "write:page-likes",
            ReadPageLikes => "read:page-likes",
            ReadUserGroups => "read:user-groups",
            WriteUserGroups => "write:user-groups",
            ReadChannels => "read:channels",
            WriteChannels => "write:channels",
            ReadGallery => "read:gallery",
            WriteGallery => "write:gallery",
            ReadGalleryLikes => "read:gallery-likes",
            WriteGalleryLikes => "write:gallery-likes",
            ReadFlash => "read:flash",
            WriteFlash => "write:flash",
            ReadFlashLikes => "read:flash-likes",
            WriteFlashLikes => "write:flash-likes",
            ReadAdminAbuseUserReports => "read:admin:abuse-user-reports",
            WriteAdminDeleteAccount => "write:admin:delete-account",
            WriteAdminDeleteAllFilesOfAUser => "write:admin:delete-all-files-of-a-user",
            ReadAdminIndexStats => "read:admin:index-stats",
            ReadAdminTableStats => "read:admin:table-stats",
            ReadAdminUserIps => "read:admin:user-ips",
            ReadAdminMeta => "read:admin:meta",
            WriteAdminResetPassword => "write:admin:reset-password",
            WriteAdminResolveAbuseUserReport => "write:admin:resolve-abuse-user-report",
            WriteAdminSendEmail => "write:admin:send-email",
            ReadAdminServerInfo => "read:admin:server-info",
            ReadAdminShowModerationLog => "read:admin:show-moderation-log",
            ReadAdminShowUser => "read:admin:show-user",
            WriteAdminSuspendUser => "write:admin:suspend-user",
            WriteAdminUnsetUserAvatar => "write:admin:unset-user-avatar",
            WriteAdminUnsetUserBanner => "write:admin:unset-user-banner",
            WriteAdminUnsuspendUser => "write:admin:unsuspend-user",
            WriteAdminMeta => "write:admin:meta",
            WriteAdminUsernote => "write:admin:user-note",
            WriteAdminRoles => "write:admin:roles",
            ReadAdminRoles => "read:admin:roles",
            WriteAdminRelays => "write:admin:relays",
            ReadAdminRelays => "read:admin:relays",
            WriteAdminInviteCodes => "write:admin:invite-codes",
            ReadAdminInviteCodes => "read:admin:invite-codes",
            WriteAdminAnnouncements => "write:admin:announcements",
            ReadAdminAnnouncements => "read:admin:announcements",
            WriteAdminAvatarDecorations => "write:admin:avatar-decorations",
            ReadAdminAvatarDecorations => "read:admin:avatar-decorations",
            WriteAdminFederation => "write:admin:federation",
            WriteAdminAccount => "write:admin:account",
            ReadAdminAccount => "read:admin:account",
            WriteAdminEmoji => "write:admin:emoji",
            ReadAdminEmoji => "read:admin:emoji",
            WriteAdminQueue => "write:admin:queue",
            ReadAdminQueue => "read:admin:queue",
            WriteAdminPromo => "write:admin:promo",
            WriteAdminDrive => "write:admin:drive",
            ReadAdminDrive => "read:admin:drive",
            WriteAdminAd => "write:admin:ad",
            ReadAdminAd => "read:admin:ad",
            WriteInviteCodes => "write:invite-codes",
            ReadInviteCodes => "read:invite-codes",
            WriteClipFavorite => "write:clip-favorite",
            ReadClipFavorite => "read:clip-favorite",
            ReadFederation => "read:federation",
            WriteReportAbuse => "write:report-abuse",
        };
        f.write_str(text)
    }
}

impl FromStr for Permission {
    type Err = InvalidEnumString;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Permission::*;
        Ok(match s {
            "read:account" => ReadAccount,
            "write:account" => WriteAccount,
            "read:blocks" => ReadBlocks,
            "write:blocks" => WriteBlocks,
            "read:drive" => ReadDrive,
            "write:drive" => WriteDrive,
            "read:favorites" => ReadFavorites,
            "write:favorites" => WriteFavorites,
            "read:following" => ReadFollowing,
            "write:following" => WriteFollowing,
            "read:messaging" => ReadMessaging,
            "write:messaging" => WriteMessaging,
            "read:mutes" => ReadMutes,
            "write:mutes" => WriteMutes,
            "write:notes" => WriteNotes,
            "read:notifications" => ReadNotifications,
            "write:notifications" => WriteNotifications,
            "read:reactions" => ReadReactions,
            "write:reactions" => WriteReactions,
            "write:votes" => WriteVotes,
            "read:pages" => ReadPages,
            "write:pages" => WritePages,
            "write:page-likes" => WritePageLikes,
            "read:page-likes" => ReadPageLikes,
            "read:user-groups" => ReadUserGroups,
            "write:user-groups" => WriteUserGroups,
            "read:channels" => ReadChannels,
            "write:channels" => WriteChannels,
            "read:gallery" => ReadGallery,
            "write:gallery" => WriteGallery,
            "read:gallery-likes" => ReadGalleryLikes,
            "write:gallery-likes" => WriteGalleryLikes,
            "read:flash" => ReadFlash,
            "write:flash" => WriteFlash,
            "read:flash-likes" => ReadFlashLikes,
            "write:flash-likes" => WriteFlashLikes,
            "read:admin:abuse-user-reports" => ReadAdminAbuseUserReports,
            "write:admin:delete-account" => WriteAdminDeleteAccount,
            "write:admin:delete-all-files-of-a-user" => WriteAdminDeleteAllFilesOfAUser,
            "read:admin:index-stats" => ReadAdminIndexStats,
            "read:admin:table-stats" => ReadAdminTableStats,
            "read:admin:user-ips" => ReadAdminUserIps,
            "read:admin:meta" => ReadAdminMeta,
            "write:admin:reset-password" => WriteAdminResetPassword,
            "write:admin:resolve-abuse-user-report" => WriteAdminResolveAbuseUserReport,
            "write:admin:send-email" => WriteAdminSendEmail,
            "read:admin:server-info" => ReadAdminServerInfo,
            "read:admin:show-moderation-log" => ReadAdminShowModerationLog,
            "read:admin:show-user" => ReadAdminShowUser,
            "write:admin:suspend-user" => WriteAdminSuspendUser,
            "write:admin:unset-user-avatar" => WriteAdminUnsetUserAvatar,
            "write:admin:unset-user-banner" => WriteAdminUnsetUserBanner,
            "write:admin:unsuspend-user" => WriteAdminUnsuspendUser,
            "write:admin:meta" => WriteAdminMeta,
            "write:admin:user-note" => WriteAdminUsernote,
            "write:admin:roles" => WriteAdminRoles,
            "read:admin:roles" => ReadAdminRoles,
            "write:admin:relays" => WriteAdminRelays,
            "read:admin:relays" => ReadAdminRelays,
            "write:admin:invite-codes" => WriteAdminInviteCodes,
            "read:admin:invite-codes" => ReadAdminInviteCodes,
            "write:admin:announcements" => WriteAdminAnnouncements,
            "read:admin:announcements" => ReadAdminAnnouncements,
            "write:admin:avatar-decorations" => WriteAdminAvatarDecorations,
            "read:admin:avatar-decorations" => ReadAdminAvatarDecorations,
            "write:admin:federation" => WriteAdminFederation,
            "write:admin:account" => WriteAdminAccount,
            "read:admin:account" => ReadAdminAccount,
            "write:admin:emoji" => WriteAdminEmoji,
            "read:admin:emoji" => ReadAdminEmoji,
            "write:admin:queue" => WriteAdminQueue,
            "read:admin:queue" => ReadAdminQueue,
            "write:admin:promo" => WriteAdminPromo,
            "write:admin:drive" => WriteAdminDrive,
            "read:admin:drive" => ReadAdminDrive,
            "write:admin:ad" => WriteAdminAd,
            "read:admin:ad" => ReadAdminAd,
            "write:invite-codes" => WriteInviteCodes,
            "read:invite-codes" => ReadInviteCodes,
            "write:clip-favorite" => WriteClipFavorite,
            "read:clip-favorite" => ReadClipFavorite,
            "read:federation" => ReadFederation,
            "write:report-abuse" => WriteReportAbuse,
            _ => return Err(InvalidEnumString),
        })
    }
}
