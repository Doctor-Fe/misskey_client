use http::Response;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::{errors::MisskeyConnectionResult, miauth::{MiAuth, MiAuthServerResponse, MiAuthStatus}, MisskeyClientRequest, MisskeyHttpClient};

impl<T> MisskeyHttpClient<T> where T: AsyncReadExt + AsyncWriteExt + Unpin {
    pub async fn request<R>(&mut self, request: &R) -> MisskeyConnectionResult<Response<Option<R::Response>>> where R: MisskeyClientRequest {
        let request_bin = self.gen_request(request)?;
        self.stream.write(&request_bin).await?;
        self.stream.flush().await?;

        let mut buff = [0; 1];
        let mut result = Vec::new();
        while !result.ends_with(b"\r\n\r\n") {
            let size = self.stream.read(&mut buff).await?;
            for i in 0..size {
                result.push(buff[i]);
            }
        }

        let (response, length) = self.gen_header(String::from_utf8_lossy(&result).as_ref())?;
        
        let body = if length > 0 {
            let mut buff = vec![0; length];
            self.stream.read_exact(&mut buff).await?;
            buff
        } else {
            Vec::with_capacity(0)
        };

        self.gen_result(request, response, body)
    }
}

impl<T> MiAuth<T> where T: AsyncReadExt + AsyncWriteExt + Unpin {
    pub async fn check(mut self) -> MisskeyConnectionResult<MiAuthStatus<T>> {
        let response = self.client.request(&self.info).await?;
        match response.into_body() {
            Some(MiAuthServerResponse { ok: true, token: Some(token), user: Some(user) }) => Ok(MiAuthStatus::Succeed(self.client.login(token), user)),
            Some(MiAuthServerResponse { ok: false, token: None, user: None }) => Ok(MiAuthStatus::Pending(self)),
            _ => Ok(MiAuthStatus::Pending(self)), // TODO 形式に沿わない応答についての検討
        }
    }
}
