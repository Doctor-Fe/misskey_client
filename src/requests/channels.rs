use serde_derive::Serialize;

use crate::{responses::channels::ChannelInfo, FixedEndpointJsonRequest};

#[derive(Debug, Serialize)]
pub struct GetFavoriteChannels;

impl FixedEndpointJsonRequest for GetFavoriteChannels {
    const ENDPOINT: &'static str = "/channels/my-favorites";

    type Response = Vec<ChannelInfo>;
}
