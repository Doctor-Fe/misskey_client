use serde_derive::Serialize;

use crate::{responses::channels::ChannelInfo, MisskeyClientRequest};

#[derive(Debug, Serialize)]
pub struct GetFavoriteChannels;

impl MisskeyClientRequest for GetFavoriteChannels {
    const ENDPOINT: &'static str = "/api/channels/my-favorites";

    type Response = Vec<ChannelInfo>;
}
