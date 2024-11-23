use serde_derive::Serialize;

use crate::{responses::charts::{ActiveUserInfo, ChartSpan}, MisskeyClientRequest};

#[derive(Debug, Serialize)]
pub struct GetActiveUsers {
    span: ChartSpan,
    limit: usize,
    offset: Option<usize>,
}

impl MisskeyClientRequest for GetActiveUsers {
    const ENDPOINT: &'static str = "/api/charts/active-users";

    type Response = ActiveUserInfo;
}
