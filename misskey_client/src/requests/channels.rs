use misskey_client_macroes::FixedEndpointJsonRequest;
use serde_derive::Serialize;

use crate::responses::channels::ChannelInfo;

#[derive(Debug, Serialize, FixedEndpointJsonRequest)]
#[misskey_client(endpoint = "/channels/my-favorites", response = Vec<ChannelInfo>)]
pub struct GetFavoriteChannels;
