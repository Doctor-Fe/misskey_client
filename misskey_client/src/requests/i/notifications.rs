use std::collections::HashSet;

use misskey_client_macroes::FixedEndpointJsonRequest;
use serde_derive::Serialize;

use crate::{common::NotificationType, responses::notifications::NotificationInfo};

#[derive(Debug, Serialize, FixedEndpointJsonRequest)]
#[misskey_client(endpoint = "/i/notifications", response = Vec<NotificationInfo>)]
#[serde(rename_all = "camelCase")]
pub struct GetNotifications {
    limit: usize,
    #[serde(skip_serializing_if = "Option::is_none")] since_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] until_id: Option<String>,
    mark_as_read: bool,
    include_types: HashSet<NotificationType>,
    exclude_types: HashSet<NotificationType>,
}

impl GetNotifications {
    pub fn new() -> Self {
        Self {
            limit: 10,
            since_id: None,
            until_id: None,
            mark_as_read: true,
            include_types: HashSet::new(),
            exclude_types: HashSet::new(),
        }
    }

    pub fn limit(self, limit: usize) -> Self {
        Self {
            limit,
            .. self
        }
    }

    pub fn since(self, since_id: String) -> Self {
        Self {
            since_id: Some(since_id),
            .. self
        }
    }

    pub fn until(self, until_id: String) -> Self {
        Self {
            until_id: Some(until_id),
            .. self
        }
    }

    pub fn mark_as_read(self, mark_as_read: bool) -> Self {
        Self {
            mark_as_read,
            .. self
        }
    }

    pub fn include(mut self, include: NotificationType) -> Self {
        self.include_types.insert(include);
        self
    }

    pub fn exclude(mut self, exclude: NotificationType) -> Self {
        self.exclude_types.insert(exclude);
        self
    }

    pub fn include_all(mut self, include: impl Iterator<Item = NotificationType>) -> Self {
        for i in include {
            self.include_types.insert(i);
        }
        self
    }

    pub fn exclude_all(mut self, exclude: impl Iterator<Item = NotificationType>) -> Self {
        for i in exclude {
            self.exclude_types.insert(i);
        }
        self
    }
}
