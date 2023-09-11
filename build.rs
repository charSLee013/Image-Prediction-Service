use std::fs;

static OUT_DIR: &str = "src/proto-gen";

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // generate to custom folder
    fs::create_dir_all(OUT_DIR).unwrap();

    tonic_build::configure()
    .build_client(true)
    .build_server(true)
    .out_dir(OUT_DIR)
    .compile(&["proto/public/image_predction_service.proto"], &["proto/"])?;
    Ok(())
}