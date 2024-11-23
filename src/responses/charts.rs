use derive_getters::Getters;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Getters)]
#[serde(rename_all = "camelCase")]
pub struct ActiveUserInfo {
    read_write: Vec<i32>,
    read: Vec<i32>,
    write: Vec<i32>,
    registered_within_week: Vec<i32>,
    registered_within_month: Vec<i32>,
    registered_within_year: Vec<i32>,
    registered_outside_week: Vec<i32>,
    registered_outside_month: Vec<i32>,
    registered_outside_year: Vec<i32>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChartSpan {
    Day,
    Hour,
}
