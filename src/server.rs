use tonic::{transport::Server, Request, Response, Status};
use ::log::{debug, error, info};

use {{proto_name}}::*;

mod log;

pub mod {{proto_name}} {
    tonic::include_proto!("{{proto_name}}");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log(env::var_os("DEBUG").is_some());
    debug!("Debug mode activated!");

    return Ok(());
}
