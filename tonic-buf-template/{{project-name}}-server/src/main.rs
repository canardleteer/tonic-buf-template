use clap::Parser;

use std::net::{IpAddr, SocketAddr};
use tonic::transport::Server;
use tracing::{info, instrument, warn};
use tracing_subscriber::{filter::LevelFilter, layer::SubscriberExt, Layer, Registry};

use {{crate_name}}_bindings::{
    svc_v1alpha1_decl::simple_timestamp_service_server::SimpleTimestampServiceServer,
    SVC_V1ALPHA1_FILE_DESCRIPTOR_SET,
};

use {{crate_name}}_service::TimeService;

/// This is generally our Command Line Arguments declaration for the service,
/// nothing fancy here.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    // Server Arguments
    #[clap(
        help_heading = "server",
        short = 'a',
        long,
        default_value = "0.0.0.0",
        env = "SERVER_LISTEN_ADDR"
    )]
    listen_interface: IpAddr,

    #[clap(
        help_heading = "server",
        short = 'p',
        long,
        default_value = "50051",
        help_heading = "server",
        env = "SERVER_LISTEN_PORT"
    )]
    listen_port: u16,
}

#[tokio::main]
#[instrument(level = "info")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // TEMPLATE_FIXME(canardleteer): Build your own.
    setup_logging()?;

    let listen_addr = SocketAddr::new(args.listen_interface, args.listen_port);

    // Build our gRPC reflection service, adding our FileDescriptorSet + the one for Health Check.
    let reflection_svc = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(SVC_V1ALPHA1_FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(tonic_health::pb::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    // Health checks should be fairly precise, and deeply integrated with the
    // service, but in our case, it's trivial, so we'll add a basic one that
    // always reports SERVING while the process is up.
    let (mut health_reporter, health_svc) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<SimpleTimestampServiceServer<TimeService>>()
        .await;

    // This is our actual service, that we intend to expose.
    let svc_v1alpha1 = TimeService::default();

    // This will hold the process alive in a serve loop until there's reason
    // to end it.
    info!("serving gRPC on: {}...", listen_addr);
    Server::builder()
        .add_service(SimpleTimestampServiceServer::new(svc_v1alpha1))
        .add_service(health_svc)
        .add_service(reflection_svc)
        .serve(listen_addr)
        .await?;

    Ok(())
}

/// In general, this should lead to a more common definition, that is uniform for
/// your entire services fleet, wiring up to your observability stack as
/// appropriate. This should come from a crate you build.
pub fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    // We use `RUST_LOG` to control this trivial declaration.
    let text_filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    let text_filter_level = text_filter.max_level_hint();

    // We only intend to ship logs via stdout, in this example.
    let stdout_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_filter(text_filter);

    // Make a telemetry Subscriber, from the overall Tracing system.
    let subscriber = Registry::default().with(stdout_layer);

    // And set this Subscriber, as the global defaul for this application.
    match tracing::subscriber::set_global_default(subscriber) {
        Ok(_) => {
            warn!("Text to stdout Level set to: {:?}", text_filter_level);
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}
