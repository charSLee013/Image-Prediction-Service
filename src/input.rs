use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "Image Prediction Service")]
pub struct Opts {
    /// The file to read the model information from.
    #[structopt(short = "c", long, default_value = "config.yaml")]
    pub config: String,

    /// The IP address and port to bind to.
    #[structopt(short = "a", long, default_value = "0.0.0.0:1301")]
    pub addr: String,

    /// The TensorFlow Serving RESTful API address.
    #[structopt(long, default_value = "http://localhost:8501/v1")]
    pub tensorflow_api_addr: String,
}

impl Default for Opts {
    fn default() -> Self {
        Opts {
            config: "config.yaml".to_string(),
            addr: "0.0.0.0:1301".to_string(),
            tensorflow_api_addr: "http://localhost:8501/v1".to_string(),
        }
    }
}

pub fn read_opts() -> Result<Opts, Box<dyn std::error::Error>> {
    let mut opts = Opts::from_args();

    // Check if the ADDR environment variable is set
    if let Ok(addr_env) = std::env::var("ADDR") {
        // Check if the value is in IP:PORT format
        if let Some((ip, port)) = split_ip_port(&addr_env) {
            // Update the addr field in the configuration
            opts.addr = format!("{}:{}", ip, port);
        }
    }

    // Check if tensorflow_api_addr starts with "http://" or "https://"
    if !opts.tensorflow_api_addr.starts_with("http://")
        && !opts.tensorflow_api_addr.starts_with("https://")
    {
        return Err(format!("Invalid tensorflow_api_addr: {}", opts.tensorflow_api_addr).into());
    }

    Ok(opts)
}

// Helper function to split IP and PORT from a string in IP:PORT format
fn split_ip_port(s: &str) -> Option<(&str, &str)> {
    if let Some(idx) = s.find(':') {
        let (ip, port) = s.split_at(idx);
        Some((ip, &port[1..]))
    } else {
        None
    }
}
