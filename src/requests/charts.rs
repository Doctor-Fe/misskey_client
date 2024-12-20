use serde_derive::{Deserialize, Serialize};

use crate::{responses::charts::ActiveUserInfo, MisskeyClientRequest};

#[derive(Debug, Serialize)]
pub struct GetActiveUsers {
    span: ChartSpan,
    limit: usize,
    offset: Option<usize>,
}

impl MisskeyClientRequest for GetActiveUsers {
    const ENDPOINT: &'static str = "/charts/active-users";

    type Response = ActiveUserInfo;
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ChartSpan {
    Day,
    Hour,
}
