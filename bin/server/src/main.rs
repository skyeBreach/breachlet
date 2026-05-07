use breachlet_svc_infra::tracing::configure_tracing;
use tracing::info;

#[tokio::main]
async fn main() {
    configure_tracing();
    info!("Starting Breachlet Backend Server...");
}
