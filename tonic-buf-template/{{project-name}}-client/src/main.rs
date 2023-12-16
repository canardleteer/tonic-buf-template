use clap::Parser;

use tonic::Request;
use tracing::{instrument, warn};

use {{crate_name}}_bindings::svc_v1alpha1_decl::{
    simple_timestamp_service_client::SimpleTimestampServiceClient, WhatTimeIsItRequest,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    // Server Arguments
    #[clap(
        help_heading = "client",
        short = 'a',
        long,
        default_value = "127.0.0.1",
        env = "TARGET_SERVER_ADDR"
    )]
    service_addr: String,

    #[clap(
        help_heading = "server",
        short = 'p',
        long,
        default_value = "50051",
        help_heading = "client",
        env = "TARGET_SERVER_PORT"
    )]
    service_port: u16,
}

#[tokio::main]
#[instrument(level = "info")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let mut client = SimpleTimestampServiceClient::connect(format!(
        "http://{}:{}",
        args.service_addr, args.service_port
    ))
    .await?;

    let rsp = client
        .what_time_is_it(Request::new(WhatTimeIsItRequest {}))
        .await?;

    println!(
        "Response from service was: {}",
        rsp.get_ref().seconds_since_epoch
    );

    Ok(())
}
