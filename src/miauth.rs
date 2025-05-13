use http::{header, Request, Uri, Version};
use itertools::Itertools;
use std::{collections::HashSet, fmt::Display, io::{Read, Write}};
use serde_derive::Deserialize;
use uuid::Uuid;

use crate::errors::MisskeyConnectionResult;
use crate::{responses::notes::DetailedUserInfo, MisskeyHttpClient};

pub struct MiAuth<T> where T: Read + Write {
    client: MisskeyHttpClient<T>,
    uuid: Uuid,
    uri: Uri,
}

impl<T> MiAuth<T> where T: Read + Write {
    pub fn check(mut self) -> MisskeyConnectionResult<MiAuthStatus<T>> {
        let req = Request::post(format!("/api/miauth/{}/check", self.uuid))
            .version(Version::HTTP_11)
            .header(header::ACCEPT_CHARSET, "UTF-8")
            .header(header::ACCEPT_ENCODING, "identity")
            .header(header::CONNECTION, "keep-alive")
            .header(header::HOST, self.client.authority.host())
            .body([])?;

        let response = self.client.internal_request(req)?;
        match serde_json::from_str::<MiAuthServerResponse>(response.body())? {
            MiAuthServerResponse { ok: true, token: Some(token), user: Some(user) } => Ok(MiAuthStatus::Succeed(self.client.login(token), user)),
            MiAuthServerResponse { ok: false, token: None, user: None } => Ok(MiAuthStatus::Pending(self)),
            _ => Ok(MiAuthStatus::Pending(self)), // TODO 形式に沿わない応答についての検討
        }
    }

    pub fn get_uri(&self) -> &Uri {
        &self.uri
    }
}

pub enum MiAuthStatus<T> where T: Read + Write {
    Pending(MiAuth<T>),
    Succeed(MisskeyHttpClient<T>, DetailedUserInfo),
}

pub struct MiAuthBuilder<T> where T: Read + Write {
    client: MisskeyHttpClient<T>,
    uuid: Uuid,
    uri: Option<String>,
    name: Option<String>,
    icon: Option<String>,
    callback: Option<String>,
    permission: HashSet<Permission>,
}

impl<T> MiAuthBuilder<T> where T: Read + Write {
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
        return MiAuth {client, uuid, uri}
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MiAuthServerResponse {
    ok: bool,
    token: Option<String>,
    user: Option<DetailedUserInfo>,
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
        let text = match self {
            Permission::ReadAccount => "read:account",
            Permission::WriteAccount => "write:account",
            Permission::ReadBlocks => "read:blocks",
            Permission::WriteBlocks => "write:blocks",
            Permission::ReadDrive => "read:drive",
            Permission::WriteDrive => "write:drive",
            Permission::ReadFavorites => "read:favorites",
            Permission::WriteFavorites => "write:favorites",
            Permission::ReadFollowing => "read:following",
            Permission::WriteFollowing => "write:following",
            Permission::ReadMessaging => "read:messaging",
            Permission::WriteMessaging => "write:messaging",
            Permission::ReadMutes => "read:mutes",
            Permission::WriteMutes => "write:mutes",
            Permission::WriteNotes => "write:notes",
            Permission::ReadNotifications => "read:notifications",
            Permission::WriteNotifications => "write:notifications",
            Permission::ReadReactions => "read:reactions",
            Permission::WriteReactions => "write:reactions",
            Permission::WriteVotes => "write:votes",
            Permission::ReadPages => "read:pages",
            Permission::WritePages => "write:pages",
            Permission::WritePageLikes => "write:page-likes",
            Permission::ReadPageLikes => "read:page-likes",
            Permission::ReadUserGroups => "read:user-groups",
            Permission::WriteUserGroups => "write:user-groups",
            Permission::ReadChannels => "read:channels",
            Permission::WriteChannels => "write:channels",
            Permission::ReadGallery => "read:gallery",
            Permission::WriteGallery => "write:gallery",
            Permission::ReadGalleryLikes => "read:gallery-likes",
            Permission::WriteGalleryLikes => "write:gallery-likes",
            Permission::ReadFlash => "read:flash",
            Permission::WriteFlash => "write:flash",
            Permission::ReadFlashLikes => "read:flash-likes",
            Permission::WriteFlashLikes => "write:flash-likes",
            Permission::ReadAdminAbuseUserReports => "read:admin:abuse-user-reports",
            Permission::WriteAdminDeleteAccount => "write:admin:delete-account",
            Permission::WriteAdminDeleteAllFilesOfAUser => "write:admin:delete-all-files-of-a-user",
            Permission::ReadAdminIndexStats => "read:admin:index-stats",
            Permission::ReadAdminTableStats => "read:admin:table-stats",
            Permission::ReadAdminUserIps => "read:admin:user-ips",
            Permission::ReadAdminMeta => "read:admin:meta",
            Permission::WriteAdminResetPassword => "write:admin:reset-password",
            Permission::WriteAdminResolveAbuseUserReport => "write:admin:resolve-abuse-user-report",
            Permission::WriteAdminSendEmail => "write:admin:send-email",
            Permission::ReadAdminServerInfo => "read:admin:server-info",
            Permission::ReadAdminShowModerationLog => "read:admin:show-moderation-log",
            Permission::ReadAdminShowUser => "read:admin:show-user",
            Permission::WriteAdminSuspendUser => "write:admin:suspend-user",
            Permission::WriteAdminUnsetUserAvatar => "write:admin:unset-user-avatar",
            Permission::WriteAdminUnsetUserBanner => "write:admin:unset-user-banner",
            Permission::WriteAdminUnsuspendUser => "write:admin:unsuspend-user",
            Permission::WriteAdminMeta => "write:admin:meta",
            Permission::WriteAdminUsernote => "write:admin:user-note",
            Permission::WriteAdminRoles => "write:admin:roles",
            Permission::ReadAdminRoles => "read:admin:roles",
            Permission::WriteAdminRelays => "write:admin:relays",
            Permission::ReadAdminRelays => "read:admin:relays",
            Permission::WriteAdminInviteCodes => "write:admin:invite-codes",
            Permission::ReadAdminInviteCodes => "read:admin:invite-codes",
            Permission::WriteAdminAnnouncements => "write:admin:announcements",
            Permission::ReadAdminAnnouncements => "read:admin:announcements",
            Permission::WriteAdminAvatarDecorations => "write:admin:avatar-decorations",
            Permission::ReadAdminAvatarDecorations => "read:admin:avatar-decorations",
            Permission::WriteAdminFederation => "write:admin:federation",
            Permission::WriteAdminAccount => "write:admin:account",
            Permission::ReadAdminAccount => "read:admin:account",
            Permission::WriteAdminEmoji => "write:admin:emoji",
            Permission::ReadAdminEmoji => "read:admin:emoji",
            Permission::WriteAdminQueue => "write:admin:queue",
            Permission::ReadAdminQueue => "read:admin:queue",
            Permission::WriteAdminPromo => "write:admin:promo",
            Permission::WriteAdminDrive => "write:admin:drive",
            Permission::ReadAdminDrive => "read:admin:drive",
            Permission::WriteAdminAd => "write:admin:ad",
            Permission::ReadAdminAd => "read:admin:ad",
            Permission::WriteInviteCodes => "write:invite-codes",
            Permission::ReadInviteCodes => "read:invite-codes",
            Permission::WriteClipFavorite => "write:clip-favorite",
            Permission::ReadClipFavorite => "read:clip-favorite",
            Permission::ReadFederation => "read:federation",
            Permission::WriteReportAbuse => "write:report-abuse",
        };
        f.write_str(text)
    }
}
