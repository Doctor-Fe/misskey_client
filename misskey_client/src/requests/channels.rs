use misskey_client_macroes::ConstParamJsonRequest;
use serde_derive::Serialize;

use crate::responses::channels::ChannelInfo;

#[derive(Debug, Serialize, ConstParamJsonRequest)]
#[misskey_client(endpoint = "/channels/my-favorites", response = Vec<ChannelInfo>)]
pub struct GetFavoriteChannels;
