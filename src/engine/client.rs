use reqwest::{self, header::{HeaderMap, HeaderValue, SET_COOKIE}, Response, StatusCode};
use std::{collections::HashMap, error::Error};

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

#[macro_export]
macro_rules! query_params {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut params = Vec::new();
        $(
            params.push((String::from($key), String::from($value)));
        )*
        params
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
    query_params: Vec<(String, String)>, // New field to store query parameters
}


impl WappuClient {
    pub fn new() -> Self {
        WappuClient {
            client: reqwest::Client::new(),
            query_params: Vec::new(),
        }
    }

    pub fn query_params(mut self, params: Vec<(String, String)>) -> Self {
        self.query_params = params;
        self
    }
    
    pub async fn get(&self, url: &str, headers: Option<HeaderMap>) -> Result<WappuResponse, WappuError> {
        let request = self.client.get(url);

        let request = if !self.query_params.is_empty() {
            request.query(&self.query_params)
        } else {
            request
        };

        let response = self.send_request(request, headers).await?;
        WappuResponse::from_response(response).await
    }

    pub async fn post(&self, url: &str, body: &str, headers: Option<HeaderMap>) -> Result<WappuResponse, WappuError> {
        let request = self.client.post(url).body(body.to_string());
        
        let request = if !self.query_params.is_empty() {
            request.query(&self.query_params)
        } else {
            request
        };
    
        let response = self.send_request(request, headers).await?;
        WappuResponse::from_response(response).await
    }
    

    pub async fn put(&self, url: &str, body: &str, headers: Option<HeaderMap>) -> Result<WappuResponse, WappuError> {
        let request = self.client.put(url).body(body.to_string());
    
        let request = if !self.query_params.is_empty() {
            request.query(&self.query_params)
        } else {
            request
        };
    
        let response = self.send_request(request, headers).await?;
        WappuResponse::from_response(response).await
    }
    

    pub async fn delete(&self, url: &str, headers: Option<HeaderMap>) -> Result<WappuResponse, WappuError> {
        let request = self.client.delete(url);
    
        let request = if !self.query_params.is_empty() {
            request.query(&self.query_params)
        } else {
            request
        };
    
        let response = self.send_request(request, headers).await?;
        WappuResponse::from_response(response).await
    }
    
    

    pub async fn head(&self, url: &str, headers: Option<HeaderMap>) -> Result<WappuResponse, WappuError> {
        let request = self.client.head(url);
    
        let request = if !self.query_params.is_empty() {
            request.query(&self.query_params)
        } else {
            request
        };
    
        let response = self.send_request(request, headers).await?;
        WappuResponse::from_response(response).await
    }
    

    pub async fn patch(&self, url: &str, body: &str, headers: Option<HeaderMap>) -> Result<WappuResponse, WappuError> {
        let request = self.client.patch(url).body(body.to_string());
    
        let request = if !self.query_params.is_empty() {
            request.query(&self.query_params)
        } else {
            request
        };
    
        let response = self.send_request(request, headers).await?;
        WappuResponse::from_response(response).await
    }
    

    async fn send_request(&self, request: reqwest::RequestBuilder, headers: Option<HeaderMap>) -> Result<Response, WappuError> {
        let mut request = request;
        if let Some(h) = headers {
            request = request.headers(h);
        }
        let response = request.send().await?;
        if !response.status().is_success() {
            return Err(WappuError::UnexpectedStatusCode(response.status()));
        }
        Ok(response)
    }
}


pub struct WappuResponse {
    text: String,
    headers: HeaderMap,
    status_code: StatusCode,
    cookies: HashMap<String, String>, // Cookies represented as a key-value pair for simplicity
}

impl WappuResponse {
    // Creates a new WappuResponse from a reqwest::Response, fetching text, headers, and cookies asynchronously
    async fn from_response(response: Response) -> Result<Self, WappuError> {
        let status_code = response.status();
        let headers = response.headers().clone(); // Clone headers before consuming response

        // Attempt to extract cookies from headers before calling response.text()
        let cookies = headers.get_all(SET_COOKIE)
                             .iter()
                             .filter_map(|header_value| parse_cookie(header_value))
                             .collect();

        let body_text = response.text().await.map_err(WappuError::from)?;

        Ok(WappuResponse { text: body_text, headers, status_code, cookies })
    }

    // Method to get the response text without consuming the response
    pub fn text(&self) -> &str {
        &self.text
    }

    // Method to get headers
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    // Method to get status code
    pub fn status_code(&self) -> StatusCode {
        self.status_code
    }

    // Method to get cookies
    pub fn cookies(&self) -> &HashMap<String, String> {
        &self.cookies
    }
}

// Utility function to parse a cookie from a Set-Cookie header value
fn parse_cookie(header_value: &HeaderValue) -> Option<(String, String)> {
    header_value.to_str().ok().and_then(|cookie_str| {
        let parts: Vec<&str> = cookie_str.splitn(2, '=').collect();
        if parts.len() == 2 {
            Some((parts[0].trim().to_string(), parts[1].split(';').next()?.trim().to_string()))
        } else {
            None
        }
    })
}