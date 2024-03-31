#[cfg(test)]
mod client_tests {
    use crate::client::WappuClient;

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
        let result = client
            .put("https://httpbin.org/put", "body content", None)
            .await;
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
        assert!(matches!(result, Ok(_)));
    }
}

#[cfg(test)]
mod client_error_tests {
    use reqwest::StatusCode;
    use tokio::time::{timeout, Duration};

    use crate::{headers, query_params, client::WappuClient, client::WappuError};

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
                && matches!(result.unwrap(), Err(WappuError::UnexpectedStatusCode(code, _)) if code == StatusCode::NOT_FOUND)
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
                && matches!(result.unwrap(), Err(WappuError::UnexpectedStatusCode(code, _)) if code == StatusCode::NOT_FOUND)
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
        let result = timeout(
            Duration::from_secs(2),
            client.delete("http://10.255.255.1", None),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_wappu_client_head_network_error() {
        let client = WappuClient::new();
        let result = timeout(
            Duration::from_secs(2),
            client.head("http://10.255.255.1", None),
        )
        .await;
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

        // Expecting an unexpected status code error with the correct status code and response text
        match result {
            Err(WappuError::UnexpectedStatusCode(code, text)) => {
                assert!(text.contains("Not Found"));
                assert!(matches!(code, StatusCode::INTERNAL_SERVER_ERROR));
            }
            Ok(_) => {
                panic!("Expected WappuError::UnexpectedStatusCode, but got Ok");
            }
            Err(err) => {
                panic!(
                    "Expected WappuError::UnexpectedStatusCode, but got other error: {}",
                    err
                );
            }
        }
    }

    #[tokio::test]
    async fn test_wappu_client_delete_unexpected_status_code() {
        let client = WappuClient::new();
        let result = client.delete("https://httpbin.org/status/404", None).await;
        assert!(
            matches!(result, Err(WappuError::UnexpectedStatusCode(code, _)) if code == StatusCode::NOT_FOUND)
        );
    }

    #[tokio::test]
    async fn test_wappu_client_head_unexpected_status_code() {
        let client = WappuClient::new();
        let result = client.head("http://example.com/notfound", None).await;
        assert!(
            matches!(result, Err(WappuError::UnexpectedStatusCode(code, _)) if code == StatusCode::NOT_FOUND)
        );
    }

    #[tokio::test]
    async fn test_wappu_client_patch_unexpected_status_code() {
        let client = WappuClient::new();
        let result = client
            .patch("https://httpbin.org/status/404", "body content", None)
            .await;
        assert!(
            matches!(result, Err(WappuError::UnexpectedStatusCode(code, _)) if code == StatusCode::NOT_FOUND)
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

        assert!(result
            .text()
            .contains("\"User-Agent\": \"WappuClient/1.0\""));
    }

    #[tokio::test]
    async fn test_headers_macro() {
        let client = WappuClient::new();
        let headers = headers! {
            "User-Agent" => "WappuClient/1.0",
            "X-Custom-Header" => "CustomValue",
        };

        // httpbin.org/headers echoes back the headers it receives in a JSON format
        let result = client
            .get("https://httpbin.org/headers", Some(headers))
            .await
            .unwrap();

        assert!(result
            .text()
            .contains("\"User-Agent\": \"WappuClient/1.0\""));
        assert!(result
            .text()
            .contains("\"X-Custom-Header\": \"CustomValue\""));
    }

    #[tokio::test]
    async fn test_client_headers() {
        let client = WappuClient::new();
        let headers = headers! {
            "User-Agent" => "WappuClient/1.0",
            "X-Custom-Header" => "CustomValue",
        };

        let result = client
            .get("https://httpbin.org/headers", Some(headers))
            .await
            .unwrap();

        println!("{:?}", result.headers());
        assert!(result.headers().contains_key("Date"));
    }
    #[tokio::test]
    async fn test_client_cookies() {
        let client = WappuClient::new();
        let result = client.get("https://discord.com", None).await.unwrap();
        assert!(result.cookies().contains_key("__sdcfduid"));
    }

    #[tokio::test]
    async fn test_wappu_client_get_with_query_params() {
        let client =
            WappuClient::new().query_params(vec![("test".to_string(), "true".to_string())]);
        let result = client.get("https://httpbin.org/get", None).await.unwrap();
        let body = result.text();

        // httpbin returns query parameters in the response body as JSON
        assert!(body.contains(r#""test": "true""#));
    }

    #[tokio::test]
    async fn test_wappu_client_get_with_macro_query_params() {
        let params = query_params! {
            "macroTest" => "passed",
            "anotherParam" => "12345",
        };
        let client = WappuClient::new().query_params(params);
        let result = client.get("https://httpbin.org/get", None).await.unwrap();
        let body = result.text();

        // Verify that the response body contains the query parameters set by the macro
        assert!(body.contains(r#""macroTest": "passed""#));
        assert!(body.contains(r#""anotherParam": "12345""#));
    }
}
