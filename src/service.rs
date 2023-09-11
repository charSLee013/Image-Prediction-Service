use super::pb::image_prediction_pb;

use super::tf_serving::predict_service::predict as tf_predict;
use base64_simd::URL_SAFE;
use image_prediction_pb::image_prediction_server::ImagePrediction;
use image_prediction_pb::{ImagePredictionRequest, ImageVectorResponse};
use log::{debug, error};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task;
use tokio::time::Instant;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tonic::{Request, Response, Status};

use crate::config::Model;

// This is the service that implements the ImagePrediction trait
pub struct ImagePredictionService {
    // Add a field to store the available model names
    pub models: Arc<HashMap<String, Model>>,
    pub tf_serving_url: Arc<String>,
}
#[tonic::async_trait]
impl ImagePrediction for ImagePredictionService {
    type PredictStream = ReceiverStream<Result<ImageVectorResponse, Status>>;

    async fn predict(
        &self,
        request: Request<tonic::Streaming<ImagePredictionRequest>>,
    ) -> Result<Response<Self::PredictStream>, Status> {
        // 创建一个多生产者单消费者通道，用于发送响应
        let (tx, rx) = tokio::sync::mpsc::channel(1024);

        // Get the stream of image requests from the client
        let mut stream = request.into_inner();

        while let Some(image_request) = stream.next().await {
            let tx = tx.clone();
            let image_request = image_request?;

            // Get the model name from the image request
            let model_name = image_request.model;

            let models = Arc::clone(&self.models);
            // Check if the model name is in the field of the service
            let req_model = match models.get(&model_name) {
                Some(o) => o,
                None => {
                    // Return an error status with a message
                    let err = Status::invalid_argument(format!(
                        "The model name {} does not exist",
                        model_name
                    ));
                    let _ = tx.send(Err(err)).await;
                    continue;
                }
            }
            .clone();

            // clone the data before the async block
            let tf_serving_url = Arc::clone(&self.tf_serving_url);

            task::spawn(async move {
                // record start time
                let start_time = Instant::now(); // 记录开始时间
                                                 // Get the image data from the image request
                let image_data = image_request.image;

                // Get the id from the image request
                let res_id = image_request.id;

                // Encode image data with base64
                let img_base64 = URL_SAFE.encode_to_string(&image_data[..]);

                // send prection request to tensorflow serving
                let binding = req_model.version.to_string();
                let img_vector = tf_predict(
                    &tf_serving_url,
                    &req_model.name,
                    &binding,
                    &req_model.input_name,
                    &img_base64,
                )
                .await
                .unwrap();

                let img_vector = img_vector.get(0).unwrap_or(&vec![]).clone();

                let resp = ImageVectorResponse {
                    vector: img_vector, // use img_vector if it's Ok, otherwise use a empty value
                    id: res_id,
                };

                let elapsed_time = Instant::now().duration_since(start_time).as_secs_f32();
                debug!(
                    "recv image_data len: {}\t order id : {} \t encode base64 len: {} \t executed in: {:.2} s",
                    image_data.len(),
                    res_id,
                    img_base64.len(),
                    elapsed_time,
                );

                if let Err(err) = tx.send(Ok(resp)).await {
                    error!("Error sending response: {:?}", err);
                }
            });
        }

        // 返回带有响应结果的流式响应对象
        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(
            rx,
        )))
    }
}
