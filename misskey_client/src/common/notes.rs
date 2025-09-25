use std::{fmt::Display, str::FromStr};

use serde_derive::{Deserialize, Serialize};

use crate::errors::InvalidEnumString;

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
        use NoteVisibility::*;
        f.write_str(match self {
            Public => "public",
            Home => "home",
            Followers => "followers",
            Specified => "specified",
        })
    }
}

impl FromStr for NoteVisibility {
    type Err = InvalidEnumString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use NoteVisibility::*;
        Ok(match s {
            "public" => Public,
            "home" => Home,
            "followers" => Followers,
            "specified" => Specified,
            _ => return Err(InvalidEnumString),
        })
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ReactionAcceptance {
    /// いいねのみを受け付けます。
    LikeOnly,
    /// リモートユーザーからはいいねのみを受け付けます。
    LikeOnlyForRemote,
    /// センシティブフラグの立っていないリアクションのみを受け付けます。
    NonSensitiveOnly,
    /// ローカルユーザーからはセンシティブフラグの立っていないリアクションのみを受け付けます。<br />
    /// リモートユーザーからはいいねのみを受け付けます。
    NonSensitiveOnlyForLocalLikeOnlyForRemote,
}
