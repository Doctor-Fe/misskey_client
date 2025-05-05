pub mod notifications;

use serde_derive::Serialize;

use crate::{responses::users::LiteUserInfo, FixedEndpointMisskeyClientRequest};

#[derive(Debug, Serialize)]
pub struct GetSelfData;

impl FixedEndpointMisskeyClientRequest for GetSelfData {
    type Response = LiteUserInfo;
    const ENDPOINT: &'static str = "/i";
}
