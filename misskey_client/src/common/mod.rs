//! リクエストとレスポンスの両方に使用する構造体・列挙型

mod charts;
mod notes;
mod notifications;

use std::{fmt::Display, str::FromStr};

pub use charts::ChartSpan;
pub use notes::NoteVisibility;
pub use notifications::NotificationType;
use serde_derive::{Deserialize, Serialize};

use crate::errors::InvalidEnumString;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum StatusVisibility {
    Followers,
    Private,
    Public,
}

impl FromStr for StatusVisibility {
    type Err = InvalidEnumString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use StatusVisibility::*;
        Ok(match s {
            "followers" => Followers,
            "private" => Private,
            "public" => Public,
            _ => return Err(InvalidEnumString)
        })
    }
}

impl Display for StatusVisibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use StatusVisibility::*;
        f.write_str(match self {
            Followers => "followers",
            Private => "private",
            Public => "public",
        })
    }
}

