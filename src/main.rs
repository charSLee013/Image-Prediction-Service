mod config;
mod input;
mod logger;
mod pb;
mod service;
mod tf_serving;
use std::{collections::HashMap, sync::Arc};

use config::{read_config_from_path, Model};
use input::read_opts;
use log::{debug, error, info, warn};
use logger::init_logging;
use pb::image_prediction_pb::image_prediction_server::ImagePredictionServer;
use service::ImagePredictionService;
use tonic::transport::Server;

use crate::input::Opts;

fn main() {
    // 创建日志记录器
    if let Err(e) = init_logging() {
        panic!("cannot init logger: {:?}", e);
    }

    // get the config file name from the command line arguments

    let opts: Opts = match read_opts() {
        Ok(o) => o,
        Err(e) => {
            error!("cannot parser command line: {:?}", e);
            std::process::exit(1);
        }
    };
    debug!("{:?}", opts);

    // pass the config file name to the read_config function
    let model_map: HashMap<String, Model> = match read_config_from_path(&opts.config) {
        Ok(t) => t,
        Err(e) => {
            if e.to_string().contains("No such file") {
                error!("Cannot find config file {} , please check if the input parameters and local configuration file are set correctly.",opts.config);
            } else {
                error!("{}", e.to_string());
            }
            std::process::exit(1);
        }
    };

    // check model is empty
    if model_map.is_empty() {
        warn!("Cannot find any model info from {}", &opts.config);
        println!("Cannot find any model info from {}", &opts.config);
        std::process::exit(1);
    } else {
        info!("model info: {:?}", model_map);
    }
    // start the gRPC server and use the model map
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .global_queue_interval(31)
        .build()
        .unwrap();

    let image_predction = ImagePredictionService {
        models: Arc::new(model_map),
        tf_serving_url: Arc::new(opts.tensorflow_api_addr),
    };

    rt.block_on(start_gpc_server(&opts.addr, image_predction))
        .unwrap();
}

pub async fn start_gpc_server(
    addr: &str,
    service: ImagePredictionService,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("ImagePredictionServer listening on: {}", addr);

    Server::builder()
        .add_service(ImagePredictionServer::new(service))
        .serve(addr.parse().unwrap())
        .await?;

    Ok(())
}
