use http::{header, Request, Response, Version};
use itertools::Itertools;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{errors::MisskeyConnectionResult, miauth::{MiAuth, MiAuthServerResponse, MiAuthStatus}, MisskeyClientRequest, MisskeyHttpClient, ServerErrorResponse};

impl<T> MisskeyHttpClient<T> where T: AsyncReadExt + AsyncWriteExt + Unpin {
    pub async fn request<R>(&mut self, request: &R) -> MisskeyConnectionResult<Response<R::Response>> where R: MisskeyClientRequest {
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
        let req = req.body(data.bytes())?;

        let (parts, body) = self.internal_request(req).await?.into_parts();

        match serde_json::from_str::<R::Response>(&body) {
            Ok(result) => Ok(Response::from_parts(parts, result)),
            Err(_e) => {
                Err(serde_json::from_str::<ServerErrorResponse>(&body)?.error.into())
            }
        }
    }

    async fn internal_request<R>(&mut self, request: Request<R>) -> MisskeyConnectionResult<Response<String>> where R: IntoIterator<Item = u8> {
        let (parts, body) = request.into_parts();
        let request_bin: Vec<u8> = format!("{} {} {:?}\r\n", parts.method, parts.uri, parts.version).bytes()
            .chain(parts.headers.into_iter().filter_map(|a| a.0.map(|b| (b, a.1))).map(|a| a.0.as_str().bytes().chain(*b": ").chain(a.1.as_bytes().into_iter().map(|a| *a)).chain([b'\r', b'\n']).collect::<Vec<_>>()).flatten())
            .chain(*b"\r\n")
            .chain(body)
            .collect();

        self.stream.write(&request_bin).await?;
        self.stream.flush().await?;
        const BUFF_SIZE: usize = 1;
        let mut result = Vec::new();
        let mut buff: [u8; BUFF_SIZE] = [0; BUFF_SIZE];
        while !result.ends_with(b"\r\n\r\n") {
            let size = self.stream.read(&mut buff).await?;
            for i in 0..size {
                result.push(buff[i]);
            }
        }
        let headers = String::from_utf8_lossy(&result);
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
            let splitted: Vec<&str> = i.split(':').collect();
            let key = splitted[0].trim().to_ascii_lowercase();
            let value = splitted.into_iter().skip(1).join(":").trim().to_string();
            if key == "content-length" {
                length = value.parse::<usize>().ok()
            }
            response = response.header(key, value);
        }
        let length = length.unwrap_or(0);
        let buff = if length > 0 {
            let mut buff = vec![0; length];
            self.stream.read_exact(&mut buff).await?;
            buff
        } else {
            Vec::with_capacity(0)
        };
        return Ok(response.body(String::from_utf8(buff)?)?);
    }
}

impl<T> MiAuth<T> where T: AsyncReadExt + AsyncWriteExt + Unpin {
    pub async fn check(mut self) -> MisskeyConnectionResult<MiAuthStatus<T>> {
        let response = self.client.request(&self.info).await?;
        match response.into_body() {
            MiAuthServerResponse { ok: true, token: Some(token), user: Some(user) } => Ok(MiAuthStatus::Succeed(self.client.login(token), user)),
            MiAuthServerResponse { ok: false, token: None, user: None } => Ok(MiAuthStatus::Pending(self)),
            _ => Ok(MiAuthStatus::Pending(self)), // TODO 形式に沿わない応答についての検討
        }
    }
}
