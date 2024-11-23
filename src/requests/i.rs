pub mod notifications;

use serde_derive::Serialize;

use crate::{responses::users::LiteUserInfo, MisskeyClientRequest};

#[derive(Debug, Serialize)]
pub struct GetSelfData;

impl MisskeyClientRequest for GetSelfData {
    type Response = LiteUserInfo;
    const ENDPOINT: &'static str = "/api/i";
}
