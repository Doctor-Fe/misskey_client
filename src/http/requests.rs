use std::{collections::BTreeMap, fmt::Display};

use itertools::Itertools;

pub(crate) struct HttpRequest {
    version: String,
    uri: String,
    method: Method,
    headers: BTreeMap<String, String>,
    body: Vec<u8>,
}

impl HttpRequest {
    pub fn new(method: Method, uri: impl ToString, version: impl ToString) -> Self {
        Self {
            version: version.to_string(),
            uri: uri.to_string(),
            method,
            headers: BTreeMap::new(),
            body: Vec::with_capacity(0),
        }
    }

    pub fn header(mut self, key: impl ToString, value: impl ToString) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn body(self, body: impl Iterator<Item = u8>) -> Self {
        Self {
            body: body.collect(),
            .. self
        }
    }
}

impl IntoIterator for HttpRequest {
    type Item = u8;

    type IntoIter = std::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        let mut s = format!("{} {} {}\r\n{}\r\n\r\n", self.method, self.uri, self.version, self.headers.into_iter().map(|a| format!("{}: {}", a.0, a.1)).join("\r\n")).into_bytes();
        for i in self.body {
            s.push(i);
        }
        return s.into_iter();
    }
}

#[allow(unused)]
pub(crate) enum Method {
    Delete,
    Get,
    Head,
    Post,
    Put,
    Trace,
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Delete => "DELETE",
            Self::Get => "GET",
            Self::Head => "HEAD",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Trace => "TRACE",
        })
    }
}
