use log::{info};

use log4rs::encode::{json::JsonEncoder}; // 引入json模块
use log4rs::config::{Appender, Config, Logger};

use log::LevelFilter;
use log4rs::append::rolling_file::{
    policy::compound::{
        CompoundPolicy, roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger,
    },
    RollingFileAppender,
};
use std::env; // 引入env模块

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(log_file) = env::var("LOG_OUTPUT") {
        // 如果环境变量LOG_OUTPUT设置了，日志则写入文件并滚动。
        let window_size = 7;
        let size_limit = 10 * 1024 * 1024;
        let log_dir = env::var("LOG_DIR").unwrap_or_else(|_| ".".to_string()); // 如果你希望，可以从环境变量中读取日志文件夹位置

        let roller = FixedWindowRoller::builder().build(&format!("{}/debug.{{}}.log", log_dir), window_size)?;
        let trigger = SizeTrigger::new(size_limit);
        let roll_strategy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));
        let file_appender = RollingFileAppender::builder()
            .encoder(Box::new(JsonEncoder::new())) // 使用Json编码器
            .build(format!("{}/debug.log", log_dir), Box::new(roll_strategy))?;

        let config = Config::builder()
            .appender(Appender::builder().build("default", Box::new(file_appender)))
            .logger(Logger::builder().build(env! ("CARGO_PKG_NAME"), log::LevelFilter::Warn)) // 使用env! ()宏获取Cargo.toml中的App名称，并作为记录器名称的前缀
            .build(log4rs::config::runtime::Root::builder().appender("default").build(LevelFilter::Debug))?;

        log4rs::config::init_config(config)?;

        info!("Logger initialized with file output. Log file is {}", log_file);
    } else {
        // dev model log
        env_logger::init();
        info!("Logger initialized with stdout.");    }

    Ok(())
}
