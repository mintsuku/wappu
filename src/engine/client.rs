use reqwest;
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

    pub async fn get(&self, url: &str) -> Result<String, WappuError> {
        let response = self.client.get(url).send().await?;
        if !response.status().is_success() {
            return Err(WappuError::UnexpectedStatusCode(response.status()));
        }
        let body = response.text().await?;
        Ok(body)
    }

    pub async fn post(
        &self, url: &str, body: &str,
    ) -> Result<String, WappuError> {
        let response = self.client.post(url).body(body.to_string()).send().await?;
        if !response.status().is_success() {
            return Err(WappuError::UnexpectedStatusCode(response.status()));
        }
        let body = response.text().await?;
        Ok(body)
    }

    pub async fn put (
        &self, url: &str, body: &str,
    ) -> Result<String, WappuError> {
        let response = self.client.put(url).body(body.to_string()).send().await?;
        if !response.status().is_success() {
            return Err(WappuError::UnexpectedStatusCode(response.status()));
        }
        let body = response.text().await?;
        Ok(body)
    }

    pub async fn delete (
        &self, url: &str,
    ) -> Result<String, WappuError> {
        let response = self.client.delete(url).send().await?;
        if !response.status().is_success() {
            return Err(WappuError::UnexpectedStatusCode(response.status()));
        }
        let body = response.text().await?;
        Ok(body)
    }

    pub async fn patch (
        &self, url: &str, body: &str,
    ) -> Result<String, WappuError> {
        let response = self.client.patch(url).body(body.to_string()).send().await?;
        if !response.status().is_success() {
            return Err(WappuError::UnexpectedStatusCode(response.status()));
        }
        let body = response.text().await?;
        Ok(body)
    }

    pub async fn head (
        &self, url: &str,
    ) -> Result<String, WappuError> {
        let response = self.client.head(url).send().await?;
        if !response.status().is_success() {
            return Err(WappuError::UnexpectedStatusCode(response.status()));
        }
        let body = response.text().await?;
        Ok(body)
    }
}
