use crate::responses::users::LiteUserInfo;

pub trait NoteId {
    fn to_note_id(self) -> String;
}

impl NoteId for String {
    fn to_note_id(self) -> String {
        self
    }
}

impl NoteId for &str {
    fn to_note_id(self) -> String {
        self.to_string()
    }
}

impl<T> NoteId for &T where T: NoteId + Clone {
    fn to_note_id(self) -> String {
        self.clone().to_note_id()
    }
}

pub trait UserId {
    fn to_user_id(self) -> String;
}

impl UserId for String {
    fn to_user_id(self) -> String {
        self
    }
}

impl UserId for &str {
    fn to_user_id(self) -> String {
        self.to_string()
    }
}

impl UserId for LiteUserInfo {
    fn to_user_id(self) -> String {
        self.id().to_string()
    }
}

impl<T> UserId for &T where T: UserId + Clone {
    fn to_user_id(self) -> String {
        self.clone().to_user_id()
    }
}

pub trait ChannelId {
    fn to_channel_id(self) -> String;
}

impl ChannelId for String {
    fn to_channel_id(self) -> String {
        self
    }
}

impl ChannelId for &str {
    fn to_channel_id(self) -> String {
        self.to_string()
    }
}

impl<T> ChannelId for &T where T: ChannelId + Clone {
    fn to_channel_id(self) -> String {
        self.clone().to_channel_id()
    }
}
