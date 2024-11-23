use serde_derive::Serialize;

use crate::{responses::notes::{CreatedNoteInfo, NoteInfo, NoteVisibility}, MisskeyClientRequest};

#[derive(Debug, Serialize)]
pub struct CreateNote<'a> {
    text: &'a str,
    cw: Option<&'a str>,
    visibility: NoteVisibility,
}

impl<'a> CreateNote<'a> {
    pub fn new(text: &'a str, visibility: NoteVisibility) -> Self {
        Self { text, cw: None, visibility }
    }

    pub fn cw(mut self, cw: &'a str) -> Self {
        self.cw = Some(cw);
        self
    }
}

impl MisskeyClientRequest for CreateNote<'_> {
    const ENDPOINT: &'static str = "/api/notes/create";

    type Response = CreatedNoteInfo;
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchNote<'a> {
    query: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    since_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    until_id: Option<&'a str>,
    limit: usize,
    offset: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    host: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel_id: Option<String>,
}

impl<'a> SearchNote<'a> {
    pub fn new(query: &'a str) -> Self {
        Self {
            query,
            since_id: None,
            until_id: None,
            limit: 10,
            offset: 0,
            host: None,
            user_id: None,
            channel_id: None,
        }
    }

    pub fn since(mut self, since_id: &'a str) -> Self {
        self.since_id = Some(since_id);
        self
    }

    pub fn until(mut self, until_id: &'a str) -> Self {
        self.until_id = Some(until_id);
        self
    }

    pub fn host(mut self, host: &'a str) -> Self {
        self.host = Some(host);
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    pub fn user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    pub fn channel_id(mut self, channel_id: String) -> Self {
        self.channel_id = Some(channel_id);
        self
    }
}

impl MisskeyClientRequest for SearchNote<'_> {
    const ENDPOINT: &'static str = "/api/notes/search";

    type Response = Vec<NoteInfo>;
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteNote {
    note_id: String,
}

impl DeleteNote {
    pub fn new(note_id: String) -> Self {
        Self { note_id }
    }
}

impl MisskeyClientRequest for DeleteNote {
    const ENDPOINT: &'static str = "/api/notes/delete";

    type Response = ();
}
