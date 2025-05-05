use serde_derive::Serialize;

use crate::{responses::channels::ChannelInfo, FixedEndpointMisskeyClientRequest};

#[derive(Debug, Serialize)]
pub struct GetFavoriteChannels;

impl FixedEndpointMisskeyClientRequest for GetFavoriteChannels {
    const ENDPOINT: &'static str = "/channels/my-favorites";

    type Response = Vec<ChannelInfo>;
}
