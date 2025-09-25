use http::{header, Request, Response, StatusCode, Version};

use crate::{errors::{MisskeyConnectionError, MisskeyConnectionResult}, MisskeyClientRequest, MisskeyHttpClient, ServerErrorResponse};

#[cfg(not(feature = "async"))]
mod sync;
#[cfg(feature = "async")]
mod r#async;

impl<T> MisskeyHttpClient<T> {
    fn gen_request<R>(&mut self, request: &R) -> MisskeyConnectionResult<Vec<u8>> where R: MisskeyClientRequest {
        let data = request.body(self.access_token.as_deref()).to_string();
        let length = data.as_bytes().len();
        let mut req = Request::post(format!("/api{}", request.endpoint().to_string()))
            .version(Version::HTTP_11)
            .header(header::ACCEPT_CHARSET, "UTF-8")
            .header(header::ACCEPT_ENCODING, "identity")
            .header(header::CONNECTION, "keep-alive")
            .header(header::CONTENT_LENGTH, length)
            .header(header::HOST, self.authority.host());
        if let Some(content_type) = request.content_type() {
            req = req.header(header::CONTENT_TYPE, format!("{}; Charset=UTF-8", content_type.to_string()));
        }
        let (parts, body) = req.body(data.into_bytes())?.into_parts();
        Ok(format!("{} {} {:?}\r\n", parts.method, parts.uri, parts.version).bytes()
        .chain(parts.headers.into_iter().filter_map(|a| a.0.map(|b| (b, a.1))).map(|a| a.0.as_str().bytes().chain(*b": ").chain(a.1.as_bytes().into_iter().map(|a| *a)).chain([b'\r', b'\n']).collect::<Vec<_>>()).flatten())
        .chain(*b"\r\n")
        .chain(body)
        .collect())
    }

    fn gen_header(&self, headers: &str) -> MisskeyConnectionResult<(http::response::Builder, usize)> {
        let mut headers = headers.split('\n').map(|a| a.trim()).take_while(|a| !a.is_empty());
        let mut first = headers.next().unwrap().split(' ').peekable();
        let version = first.next().unwrap();
        let code = first.next().unwrap();
        let mut response = Response::builder().version(match version {
            "HTTP/0.9" => Version::HTTP_09,
            "HTTP/1.0" => Version::HTTP_10,
            "HTTP/1.1" => Version::HTTP_11,
            "HTTP/2" => Version::HTTP_2,
            "HTTP/3" => Version::HTTP_3,
            _ => unimplemented!(),
        }).status(code);
        let mut message = String::new();
        while let Some(a) = first.next() {
            message.push_str(a);
            if first.peek().is_some() {
                message.push(' ');
            }
        }
        let mut length: Option<usize> = None;
        for i in headers {
            let (key, value) = i.split_once(':').unwrap_or((i, ""));
            let (key, value) = (key.trim(), value.trim());
            if key == "content-length" {
                length = value.parse::<usize>().ok()
            }
            response = response.header(key, value);
        }
        let length = length.unwrap_or(0);
        Ok((response, length))
    }

    fn gen_result<R>(&self, request: &R, response: http::response::Builder, body: Vec<u8>) -> MisskeyConnectionResult<Response<Option<R::Response>>> where R: MisskeyClientRequest {
        let body = String::from_utf8(body)?;
        let (parts, _) = response.body(())?.into_parts();
        match serde_json::from_str::<R::Response>(&body) {
            Ok(result) => Ok(Response::from_parts(parts, Some(result))),
            Err(_e) if _e.is_eof() && request.can_be_empty() && parts.status == StatusCode::NO_CONTENT => Ok(Response::from_parts(parts, None)),
            Err(_e) => {
                match serde_json::from_str::<ServerErrorResponse>(&body) {
                    Ok(a) => Err(a.into()),
                    Err(e) => Err(MisskeyConnectionError::SerdeError { parent_error: _e, error: e, raw_string: body }),
                }
            },
        }
    }
}