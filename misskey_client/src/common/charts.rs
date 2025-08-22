use std::{fmt::Display, str::FromStr};

use serde_derive::{Deserialize, Serialize};

use crate::errors::InvalidEnumString;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChartSpan {
    Day,
    Hour,
}

impl Display for ChartSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ChartSpan::Day => "day",
            ChartSpan::Hour => "hour",
        })
    }
}

impl FromStr for ChartSpan {
    type Err = InvalidEnumString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ChartSpan::*;
        Ok(match s {
            "day" => Day,
            "hour" => Hour,
            _ => return Err(InvalidEnumString),
        })
    }
}
