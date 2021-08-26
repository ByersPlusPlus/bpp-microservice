use tonic::{transport::Server, Request, Response, Status};
use ::log::{debug, error, info};
use crate::log::setup_log;
use std::env;

use {{proto_name}}::*;

mod log;

pub mod {{proto_name}} {
    tonic::include_proto!("{{proto_name}}");
}

// Implement your proto here
// https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md
// https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_log(env::var_os("DEBUG").is_some());
    debug!("Debug mode activated!");

    return Ok(());
}
