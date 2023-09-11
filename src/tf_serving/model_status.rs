use serde::{Deserialize, Serialize};

use std::error::Error as StdError;
use std::fmt;
use anyhow::{anyhow, Context, Result};

// 定义一个结构体，用于表示响应的Json数据
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    // 使用serde(rename = "...")属性来指定Json名称
    #[serde(rename = "model_version_status")]
    model_version_status: Vec<ModelVersionStatus>,
}

// 定义一个结构体，用于表示model_version_status数组中的元素
#[derive(Serialize, Deserialize, Debug)]
struct ModelVersionStatus {
    state: String,
    status: Status,
    version: String,
}

// 定义一个结构体，用于表示status对象
#[derive(Serialize, Deserialize, Debug)]
struct Status {
    #[serde(rename = "error_code")]
    error_code: String,
    #[serde(rename = "error_message")]
    error_message: String,
}


// 定义一个异步函数，接受模型名称和版本，以及可选的标签
#[allow(dead_code)]
async fn get_response(
    url: &str,
    model_name: &str,
    version: &str,
    label: Option<&str>,
) -> Result<Response> {
    // 构造url，根据是否有标签来决定是否添加/labels/${LABEL}部分
    let url = match label {
        Some(label) => format!(
            "{}/models/{}/versions/{}/labels/{}",
            url, model_name, version, label
        ),
        None => format!("{}/models/{}/versions/{}", url, model_name, version),
    };

    // 发送GET请求，并等待响应
    let response = reqwest::get(&url).await?;

    // 判断响应是否为成功的状态码
    if response.status().is_success() {
        // 将响应的数据转换为Response类型
        let response: Response = response.json().await?;

        // 返回成功的结果，包含Response对象
        Ok(response)
    } else {
        // 返回自定义错误信息，并附加状态码和url
        Err(anyhow!(
            "Failed to get response for model {} version {} from {}: status code {}",
            model_name,
            version,
            url,
            response.status()
        ))
    }
}

// 定义一个函数，接受模型名称和版本，以及可选的标签
#[allow(dead_code)]
async fn get_model_status(url: &str, model_name: &str, version: &str, label: Option<&str>) -> Result<()> {
    // 调用异步函数get_response，并等待结果
    let response = get_response(url, model_name, version, label)
        .await // 等待异步函数完成
        .context(format!(
            // 添加上下文信息
            "Failed to get response for model {} version {}",
            model_name, version
        ))?;

    // 检查响应是否为符合结构体
    if let Some(model_version_status) = response.model_version_status.get(0) {
        // 检查status.error_code是否为OK
        if model_version_status.status.error_code == "OK" {
            // 返回成功的结果
            Ok(())
        } else {
            // 返回自定义错误，并附加错误码和消息
            Err(anyhow::Error::new(ModelStatusError {
                // 使用ModelStatusError结构体来创建一个anyhow::Error类型的值，并传递错误码和消息作为参数
                error_code: model_version_status.status.error_code.clone(),
                error_message: model_version_status.status.error_message.clone(),
            }))
        }
    } else {
        // 返回自定义错误，并添加上下文信息
        Err(anyhow!("Invalid response")).context(format!(
            "Expected model_version_status array to have at least one element for model {} version {}",
            model_name, version
        ))
    }
}


// 定义一个结构体ModelStatusError，用于表示model_status的错误
#[derive(Debug)]
struct ModelStatusError {
    // 添加一些字段，用于保存错误信息
    error_code: String,
    error_message: String,
}

// 为结构体实现Display trait，用于定义结构体的显示格式
impl fmt::Display for ModelStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用write!宏来将结构体的字段写入给定的格式化器
        // 你可以根据你的需求来自定义显示格式
        write!(
            f,
            "Model status error: {} - {}",
            self.error_code, self.error_message
        )
    }
}

// 为结构体实现Error trait，用于表示这是一个错误类型
impl StdError for ModelStatusError {
    // 你可以根据你的需求来覆盖或者使用默认实现的方法
    // 例如，你可以使用description方法来返回一个简短的错误描述
    fn description(&self) -> &str {
        "Model status error"
    }
}

# [cfg (test)]
mod tests {
    use super::*;
    use mockito::{mock};

    // 测试get_response函数能够正确地发送GET请求，并返回Response类型的结果
    #[tokio::test]
    async fn test_get_response() {
        // 创建一个临时的服务器，模拟响应数据
        let _m = mock("GET", "/models/foo/versions/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "model_version_status": [
                        {
                            "state": "AVAILABLE",
                            "status": {
                                "error_code": "OK",
                                "error_message": ""
                            },
                            "version": "1"
                        }
                    ]
                }"#,
            )
            .create();
        
        // 调用get_response函数，并传入模型名称和版本
        let response =get_response(
            &mockito::server_url(),
            "foo",
            "1",
            None,
        ).await.unwrap();

        // 检查响应是否符合预期
        assert_eq!(response.model_version_status.len(), 1);
        assert_eq!(response.model_version_status[0].state, "AVAILABLE");
        assert_eq!(response.model_version_status[0].status.error_code, "OK");
        assert_eq!(response.model_version_status[0].status.error_message, "");
        assert_eq!(response.model_version_status[0].version, "1");
    }

    // 测试get_model_status函数能够正确地检查响应数据，并返回成功或失败的结果
    #[tokio::test]
    async fn test_get_model_status() {
        // 创建两个临时的服务器，分别模拟成功和失败的响应数据
        let _m1 = mock("GET", "/models/foo/versions/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "model_version_status": [
                        {
                            "state": "AVAILABLE",
                            "status": {
                                "error_code": "OK",
                                "error_message": ""
                            },
                            "version": "1"
                        }
                    ]
                }"#,
            )
            .create();

        let _m2 = mock("GET", "/models/bar/versions/2")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "model_version_status": [
                        {
                            "state": "UNAVAILABLE",
                            "status": {
                                "error_code": "NOT_FOUND",
                                "error_message": "Model not found"
                            },
                            "version": "2"
                        }
                    ]
                }"#,
            )
            .create();

        // 调用get_model_status函数，并传入模型名称和版本
        let result1 = get_model_status(
            &mockito::server_url(),
            "foo",
            "1",
            None,
        ).await;

        let result2 = get_model_status(
            &mockito::server_url(),
            "bar",
            "2",
            None,
        ).await;

        // 检查结果是否符合预期
        assert!(result1.is_ok());
        assert!(result2.is_err());
        assert_eq!(
            result2.unwrap_err().to_string(),
            "Model status error: NOT_FOUND - Model not found"
        );
    }

    // 测试当请求成功但返回无效的JSON数据时，是否返回错误信息和上下文
    #[tokio::test]
    async fn test_get_response_invalid_json() {
        // 创建一个临时的服务器，模拟响应数据
        let _m = mock("GET", "/models/foo/versions/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "model_version_status": [
                        {
                            "state": "AVAILABLE",
                            "status": {
                                "error_code": "OK",
                                "error_message": ""
                            },
                            "version": "1"
                        }
                    ]
                "#, // 缺少一个右括号
            )
            .create();

        // 调用get_response函数，并传入模型名称和版本
        let response = get_response(
            &mockito::server_url(),
            "foo",
            "1",
            None,
        ).await;

        // 检查结果是否为错误
        assert!(response.is_err());
        assert_eq!(
            response.unwrap_err().to_string(),
            "error decoding response body: EOF while parsing an object at line 12 column 16"
        );
    }

    // 测试当请求失败或返回错误的状态码时，是否返回错误信息和上下文
    #[tokio::test]
    async fn test_get_response_error_status() {
        // 创建一个临时的服务器，模拟响应数据
        let _m = mock("GET", "/models/foo/versions/1")
            .with_status(404)
            .with_header("content-type", "text/plain")
            .with_body("Not Found")
            .create();

        let url = &mockito::server_url();
        // 调用get_response函数，并传入模型名称和版本
        let response = get_response(
            url,
            "foo",
            "1",
            None,
        ).await;

        // 检查结果是否为错误
        assert!(response.is_err());
        assert_eq!(
            response.unwrap_err().to_string(),
            format!("Failed to get response for model foo version 1 from {}/models/foo/versions/1: status code 404 Not Found",
        url)
        );
    }

    // 测试当响应的JSON数据中没有model_version_status数组或数组为空时，是否返回错误信息和上下文
    #[tokio::test]
    async fn test_get_model_status_no_array() {
        // 创建两个临时的服务器，分别模拟没有数组和数组为空的响应数据
        let _m1 = mock("GET", "/models/foo/versions/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "model_version_status": null
                }"#,
            )
            .create();

        let _m2 = mock("GET", "/models/bar/versions/2")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "model_version_status": []
                }"#,
            )
            .create();

        // 调用get_model_status函数，并传入模型名称和版本
        let result1 = get_model_status(
            &mockito::server_url(),
            "foo",
            "1",
            None,
        ).await;

        let result2 = get_model_status(
            &mockito::server_url(),
            "bar",
            "2",
            None,
        ).await;

        // 检查结果是否为错误，并包含上下文信息
        assert!(result1.is_err());
        assert!(result2.is_err());
        assert_eq!(
            result1.unwrap_err().to_string(),
            "Failed to get response for model foo version 1"
        );
        assert_eq!(
            result2.unwrap_err().to_string(),
            "Expected model_version_status array to have at least one element for model bar version 2"
        );
    }

    // 测试当响应的JSON数据中有model_version_status数组但status.error_code不为OK时，是否返回自定义错误信息和码和消息
    #[tokio::test]
    async fn test_get_model_status_error_code() {
        // 创建一个临时的服务器，模拟响应数据
        let _m = mock("GET", "/models/foo/versions/1")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "model_version_status": [
                        {
                            "state": "UNAVAILABLE",
                            "status": {
                                "error_code": "NOT_FOUND",
                                "error_message": "Model not found"
                            },
                            "version": "1"
                        }
                    ]
                }"#,
            )
            .create();

        // 调用get_model_status函数，并传入模型名称和版本
        let result = get_model_status(
            &mockito::server_url(),
            "foo",
            "1",
            None,
        ).await;

        // 检查结果是否为自定义错误，并包含错误码和消息
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Model status error: NOT_FOUND - Model not found"
        );
    }
}
