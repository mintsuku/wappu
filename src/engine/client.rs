use reqwest::{self, header::HeaderMap};
use std::error::Error;

#[derive(Debug)]
pub enum WappuError {
    Network(reqwest::Error),
    UnexpectedStatusCode(reqwest::StatusCode),
}

impl std::fmt::Display for WappuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            WappuError::Network(ref err) => write!(f, "Network error: {}", err),
            WappuError::UnexpectedStatusCode(ref code) => write!(f, "Unexpected status code: {}", code),
        }
    }
}
#[macro_export]
macro_rules! headers {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = reqwest::header::HeaderMap::new();
        $(
            map.insert($key, $value.parse().unwrap());
        )*
        map
    }};
}

impl Error for WappuError {}

impl From<reqwest::Error> for WappuError {
    fn from(err: reqwest::Error) -> WappuError {
        WappuError::Network(err)
    }
}

pub struct WappuClient {
    client: reqwest::Client,
}

impl WappuClient {
    pub fn new() -> Self {
        WappuClient {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get(&self, url: &str, headers: Option<HeaderMap>) -> Result<String, WappuError> {
        let mut request = self.client.get(url);
        if let Some(headers) = headers {
            request = request.headers(headers);
        }
        let response = request.send().await?;
        if !response.status().is_success() {
            return Err(WappuError::UnexpectedStatusCode(response.status()));
        }
        let body = response.text().await?;
        Ok(body)
    }

    pub async fn post(
        &self, url: &str, body: &str, headers: Option<HeaderMap>,
    ) -> Result<String, WappuError> {
        let mut request = self.client.post(url).body(body.to_string());
        if let Some(headers) = headers {
            request = request.headers(headers);
        }
        let response = request.send().await?;
        if !response.status().is_success() {
            return Err(WappuError::UnexpectedStatusCode(response.status()));
        }
        let body = response.text().await?;
        Ok(body)
    }

    pub async fn put(
        &self, url: &str, body: &str, headers: Option<HeaderMap>,
    ) -> Result<String, WappuError> {
        let mut request = self.client.put(url).body(body.to_string());
        if let Some(headers) = headers {
            request = request.headers(headers);
        }
        let response = request.send().await?;
        if !response.status().is_success() {
            return Err(WappuError::UnexpectedStatusCode(response.status()));
        }
        let body = response.text().await?;
        Ok(body)
    }

    pub async fn delete(
        &self, url: &str, headers: Option<HeaderMap>,
    ) -> Result<String, WappuError> {
        let mut request = self.client.delete(url);
        if let Some(headers) = headers {
            request = request.headers(headers);
        }
        let response = request.send().await?;
        if !response.status().is_success() {
            return Err(WappuError::UnexpectedStatusCode(response.status()));
        }
        let body = response.text().await?;
        Ok(body)
    }

    pub async fn patch(
        &self, url: &str, body: &str, headers: Option<HeaderMap>,
    ) -> Result<String, WappuError> {
        let mut request = self.client.patch(url).body(body.to_string());
        if let Some(headers) = headers {
            request = request.headers(headers);
        }
        let response = request.send().await?;
        if !response.status().is_success() {
            return Err(WappuError::UnexpectedStatusCode(response.status()));
        }
        let body = response.text().await?;
        Ok(body)
    }

    pub async fn head(&self, url: &str, headers: Option<HeaderMap>) -> Result<(), WappuError> {
        let mut request = self.client.head(url);
        if let Some(headers) = headers {
            request = request.headers(headers);
        }
        let response = request.send().await?;
        if !response.status().is_success() {
            return Err(WappuError::UnexpectedStatusCode(response.status()));
        }
        Ok(())
    }
}
