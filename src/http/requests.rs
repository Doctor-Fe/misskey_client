use std::{collections::BTreeMap, fmt::Display, io::{self, Write}};

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
            body: vec![],
        }
    }

    pub fn header(mut self, key: impl ToString, value: impl ToString) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn body(self, body: Vec<u8>) -> Self {
        Self {
            body,
            .. self
        }
    }

    pub fn write_to(&self, stream: &mut dyn Write) -> io::Result<()> {
        write!(stream, "{} {} {}\r\n", self.method, self.uri, self.version)?;
        for i in &self.headers {
            write!(stream, "{}: {}\r\n", i.0, format!("{:?}", i.1).trim_matches('"'))?;
        }
        write!(stream, "\r\n")?;
        stream.write_all(&self.body)?;
        Ok(())
    }
}

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
            Self::Head => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Trace => "TRACE",
        })
    }
}
