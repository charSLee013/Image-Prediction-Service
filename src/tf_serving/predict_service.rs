use anyhow::{anyhow, Result};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// 用于发送Base64编码的图像预测请求并获取图像特征向量
// 因为只能传一张图片，所以结果固定是数量为1的Vec<f32>数组，比如说 vec![vec![0.1,0.2,0.3]]
pub async fn predict(
    url: &str,
    model_name: &str,
    version: &str,
    input_name: &str,
    image_base64: &str,
) -> Result<Vec<Vec<f32>>> {
    // 构造 API URL
    let url = format!("{}/models/{}/versions/{}:predict", url, model_name, version);

    // 构造 POST 请求的 JSON 数据
    let request_data = PredctionRequest {
        instances: vec![HashMap::from([(
            input_name.to_string(),
            image_base64.to_string(),
        )])],
    };

    // 发送 POST 请求，并等待响应
    let response = reqwest::Client::new()
        .post(&url)
        .json(&request_data)
        .send()
        .await?;

    // 检查 HTTP 状态码是否为成功
    if response.status().is_success() {
        // 将响应的 JSON 数据解析为 ModelResponse 对象
        let model_response: PredctionResponse = response.json().await?;

        // 返回预测结果
        Ok(model_response.predictions)
    } else {
        // 返回自定义错误信息，并附加状态码和 URL
        Err(anyhow!(
            "Failed to predict for model {}: status code {}",
            model_name,
            response.status()
        ))
    }
}

// 定义请求的数据结构
#[derive(Serialize, Deserialize, Debug)]
struct PredctionRequest {
    instances: Vec<HashMap<String, String>>,
}

// 定义响应的数据结构
#[derive(Serialize, Deserialize, Debug)]
struct PredctionResponse {
    predictions: Vec<Vec<f32>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    // 测试当请求成功并返回有效的JSON数据时，是否能正确解析预测结果
    #[tokio::test]
    async fn test_predict_success() {
        // 创建一个临时的服务器，模拟响应数据
        let _m = mock("POST", "/models/foo/versions/1:predict")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "predictions": [[0.1, 0.2, 0.3, 0.4]]
                }"#,
            )
            .create();

        // 调用predict函数，并传入模型名称、版本、输入名称和图片Base64编码
        let result = predict(
            &mockito::server_url(),
            "foo",
            "1",
            "b64_input_bytes",
            "some_base64_string",
        )
        .await;

        // 检查结果是否为成功，并包含预测结果
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![vec![0.1, 0.2, 0.3, 0.4]]);
    }

    // 测试当请求失败或返回错误的状态码时，是否返回自定义错误信息和上下文
    #[tokio::test]
    async fn test_predict_error_status() {
        // 创建一个临时的服务器，模拟响应数据
        let _m = mock("POST", "/models/bar/versions/2:predict")
            .with_status(404)
            .with_header("content-type", "text/plain")
            .with_body("Not Found")
            .create();

        // 调用predict函数，并传入模型名称、版本、输入名称和图片Base64编码
        let result = predict(
            &mockito::server_url(),
            "bar",
            "2",
            "b64_input_bytes",
            "some_base64_string",
        )
        .await;

        // 检查结果是否为错误，并包含自定义错误信息和上下文
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Failed to predict for model bar: status code 404 Not Found"
        );
    }

    // 测试当请求成功但返回无效的JSON数据时，是否返回错误信息和上下文
    #[tokio::test]
    async fn test_predict_invalid_json() {
        // 创建一个临时的服务器，模拟响应数据
        let _m = mock("POST", "/models/foo/versions/1:predict")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "predictions": [[0.1, 0.2, 0.3, 0.4]
                }"#, // 缺少一个右括号
            )
            .create();

        // 调用predict函数，并传入模型名称、版本、输入名称和图片Base64编码
        let result = predict(
            &mockito::server_url(),
            "foo",
            "1",
            "b64_input_bytes",
            "some_base64_string",
        )
        .await;

        // 检查结果是否为错误，并包含错误信息和上下文
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "error decoding response body: expected `,` or `]` at line 3 column 17"
        );
    }
}
