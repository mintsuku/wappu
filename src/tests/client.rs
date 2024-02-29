#[cfg(test)]
mod client_tests {
    use crate::WappuClient;

    #[tokio::test]
    async fn test_wappu_client_get() {
        let client = WappuClient::new();
        let result = client.get("https://httpbin.org/get", None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_wappu_client_post() {
        let client = WappuClient::new();
        let result = client
            .post("https://httpbin.org/post", "body content", None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_wappu_client_put() {
        let client = WappuClient::new();
        let result = client.put("https://httpbin.org/put", "body content", None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_wappu_client_delete() {
        let client = WappuClient::new();
        let result = client.delete("https://httpbin.org/delete", None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_wappu_client_patch() {
        let client = WappuClient::new();
        let result = client
            .patch("https://httpbin.org/patch", "body content", None)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_wappu_client_head() {
        let client = WappuClient::new();
        let result = client.head("https://httpbin.org/get", None).await;
        // HEAD requests don't have a response body, so we check for a successful status code instead
        assert!(matches!(result, Ok(_)));
    }
}

#[cfg(test)]
mod client_error_tests {
    use reqwest::StatusCode;
    use tokio::time::{timeout, Duration};

    use crate::{headers, WappuClient, WappuError};

    #[tokio::test]
    async fn test_wappu_client_get_network_error() {
        let client = WappuClient::new();
        let result = timeout(
            Duration::from_secs(10),
            client.get("http://10.255.255.1", None),
        )
        .await;

        // Expecting a timeout error or a network error
        assert!(result.is_err() || matches!(result.unwrap(), Err(WappuError::Network(_))));
    }

    #[tokio::test]
    async fn test_wappu_client_get_unexpected_status_code() {
        let client = WappuClient::new();
        let result = timeout(
            Duration::from_secs(10),
            client.get("http://example.com/notfound", None),
        )
        .await;

        // Expecting an unexpected status code error
        assert!(
            result.is_ok()
                && matches!(result.unwrap(), Err(WappuError::UnexpectedStatusCode(code)) if code == StatusCode::NOT_FOUND)
        );
    }

    #[tokio::test]
    async fn test_wappu_client_post_network_error() {
        let client = WappuClient::new();
        let result = timeout(
            Duration::from_secs(10),
            client.post("http://10.255.255.1", "body content", None),
        )
        .await;

        // Expecting a timeout error or a network error
        assert!(result.is_err() || matches!(result.unwrap(), Err(WappuError::Network(_))));
    }

    #[tokio::test]
    async fn test_wappu_client_post_unexpected_status_code() {
        let client = WappuClient::new();
        let result = timeout(
            Duration::from_secs(10),
            client.post("http://example.com/notfound", "body content", None),
        )
        .await;

        // Expecting an unexpected status code error
        assert!(
            result.is_ok()
                && matches!(result.unwrap(), Err(WappuError::UnexpectedStatusCode(code)) if code == StatusCode::NOT_FOUND)
        );
    }

    #[tokio::test]
    async fn test_wappu_client_put_network_error() {
        let client = WappuClient::new();
        let result = timeout(
            Duration::from_secs(2),
            client.put("http://10.255.255.1", "body content", None),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_wappu_client_delete_network_error() {
        let client = WappuClient::new();
        let result = timeout(Duration::from_secs(2), client.delete("http://10.255.255.1", None)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_wappu_client_head_network_error() {
        let client = WappuClient::new();
        let result = timeout(Duration::from_secs(2), client.head("http://10.255.255.1", None)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_wappu_client_patch_network_error() {
        let client = WappuClient::new();
        let result = timeout(
            Duration::from_secs(2),
            client.patch("http://10.255.255.1", "body content", None),
        )
        .await;
        assert!(result.is_err());
    }

    // Tests for unexpected status code error for each method
    #[tokio::test]
    async fn test_wappu_client_put_unexpected_status_code() {
        let client = WappuClient::new();
        let result = client
            .put("http://example.com/notfound", "body content", None)
            .await;
        assert!(
            matches!(result, Err(WappuError::UnexpectedStatusCode(code)) if code == StatusCode::NOT_FOUND)
        );
    }

    #[tokio::test]
    async fn test_wappu_client_delete_unexpected_status_code() {
        let client = WappuClient::new();
        let result = client.delete("https://httpbin.org/status/404", None).await;
        assert!(
            matches!(result, Err(WappuError::UnexpectedStatusCode(code)) if code == StatusCode::NOT_FOUND)
        );
    }

    #[tokio::test]
    async fn test_wappu_client_head_unexpected_status_code() {
        let client = WappuClient::new();
        let result = client.head("http://example.com/notfound", None).await;
        assert!(
            matches!(result, Err(WappuError::UnexpectedStatusCode(code)) if code == StatusCode::NOT_FOUND)
        );
    }

    #[tokio::test]
    async fn test_wappu_client_patch_unexpected_status_code() {
        let client = WappuClient::new();
        let result = client
            .patch("https://httpbin.org/status/404", "body content", None)
            .await;
        assert!(
            matches!(result, Err(WappuError::UnexpectedStatusCode(code)) if code == StatusCode::NOT_FOUND)
        );
    }
    #[tokio::test]
    async fn test_headers_echoed_back() {
        use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

        let client = WappuClient::new();
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("WappuClient/1.0"));

        // httpbin.org/headers echoes back the headers it receives in a JSON format
        let result = client
            .get("https://httpbin.org/headers", Some(headers))
            .await
            .unwrap();

        assert!(result.contains("\"User-Agent\": \"WappuClient/1.0\""));
    }

    #[tokio::test]
    async fn test_headers_macro() {
        let client = WappuClient::new();
        let headers = headers! {
            "User-Agent" => "WappuClient/1.0",
            "X-Custom-Header" => "CustomValue",
        };

        // httpbin.org/headers echoes back the headers it receives in a JSON format
        let result = client.get("https://httpbin.org/headers", Some(headers)).await.unwrap();

        assert!(result.contains("\"User-Agent\": \"WappuClient/1.0\""));
        assert!(result.contains("\"X-Custom-Header\": \"CustomValue\""));
    }
}
