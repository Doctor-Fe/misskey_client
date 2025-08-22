pub mod notifications;

use misskey_client_macroes::FixedEndpointJsonRequest;
use serde_derive::Serialize;

use crate::responses::users::LiteUserInfo;

#[derive(Debug, Serialize, FixedEndpointJsonRequest)]
#[misskey_client(endpoint = "/i", response = LiteUserInfo)]
pub struct GetSelfData;
