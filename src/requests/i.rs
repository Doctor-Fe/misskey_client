pub mod notifications;

use serde_derive::Serialize;

use crate::{responses::users::LiteUserInfo, FixedEndpointJsonRequest};

#[derive(Debug, Serialize)]
pub struct GetSelfData;

impl FixedEndpointJsonRequest for GetSelfData {
    type Response = LiteUserInfo;
    const ENDPOINT: &'static str = "/i";
}
