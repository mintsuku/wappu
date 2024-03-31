use serde::{Deserialize, Serialize};
use crate::engine::client::{WappuClient, WappuError};
use std::time::Duration;

#[derive(Serialize)]
struct CreateTaskRequest {
    #[serde(rename = "clientKey")]
    client_key: String,
    task: Task,
}

#[derive(Serialize)]
struct Task {
    #[serde(rename = "type")]
    task_type: String,
    #[serde(rename = "websiteURL")]
    website_url: String,
    #[serde(rename = "websiteKey")]
    website_key: String,
}

#[derive(Deserialize, Debug)]
struct CreateTaskResponse {
    #[serde(rename = "errorId")]
    error_id: i32,
    #[serde(rename = "taskId")]
    task_id: i32,
}

#[derive(Serialize)]
struct GetTaskResultRequest {
    #[serde(rename = "clientKey")]
    client_key: String,
    #[serde(rename = "taskId")]
    task_id: i32,
}

#[derive(Deserialize)]
struct GetTaskResultResponse {
    status: String,
    solution: Option<Solution>,
}

#[derive(Deserialize)]
struct Solution {
    #[serde(rename = "gRecaptchaResponse")]
    g_recaptcha_response: String,
}

pub struct CaptchaClient {
    client: WappuClient,
    api_key: String,
    task_type: String,
}

impl CaptchaClient {
    pub fn new(api_key: String, task_type: String) -> Self {
        CaptchaClient {
            client: WappuClient::new(),
            api_key,
            task_type,
        }
    }

    pub async fn solve_captcha(
        &self,
        website_url: String,
        website_key: String,
    ) -> Result<String, WappuError> {
        let task = Task {
            task_type: self.task_type.clone(),
            website_url,
            website_key,
        };

        let create_task_request = CreateTaskRequest {
            client_key: self.api_key.clone(),
            task,
        };

        let create_task_response: CreateTaskResponse = self
            .client
            .post(
                "https://api.capmonster.cloud/createTask",
                &serde_json::to_string(&create_task_request)?,
                None,
            )
            .await?
            .json()
            .await?;

        if create_task_response.error_id != 0 {
            return Err(WappuError::CapmonsterError(format!(
                "Failed to create task: error ID {}",
                create_task_response.error_id
            )));
        }

        let task_id = create_task_response.task_id;

        loop {
            let get_task_result_request = GetTaskResultRequest {
                client_key: self.api_key.clone(),
                task_id,
            };

            let get_task_result_response: GetTaskResultResponse = self
                .client
                .post(
                    "https://api.capmonster.cloud/getTaskResult",
                    &serde_json::to_string(&get_task_result_request)?,
                    None,
                )
                .await?
                .json()
                .await?;

            match get_task_result_response.status.as_str() {
                "processing" => tokio::time::sleep(Duration::from_secs(5)).await,
                "ready" => {
                    if let Some(solution) = get_task_result_response.solution {
                        return Ok(solution.g_recaptcha_response);
                    } else {
                        return Err(WappuError::CapmonsterError(
                            "No solution found in the response".to_string(),
                        ));
                    }
                }
                _ => {
                    return Err(WappuError::CapmonsterError(format!(
                        "Unexpected status: {}",
                        get_task_result_response.status
                    )))
                }
            }
        }
    }
}