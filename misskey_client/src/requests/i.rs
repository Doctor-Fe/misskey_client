pub mod notifications;

use misskey_client_macroes::ConstParamJsonRequest;
use serde_derive::Serialize;

use crate::responses::users::LiteUserInfo;

#[derive(Debug, Serialize, ConstParamJsonRequest)]
#[misskey_client(endpoint = "/i", response = LiteUserInfo)]
pub struct GetSelfData;
