use breachlet_core::tracing::configure_tracing;
use breachlet_infra::config::BackendConfig;
use tracing::info;

#[tokio::main]
async fn main() {
    configure_tracing();
    info!("Starting Breachlet Backend Server...");

    let config = BackendConfig::load();
    info!("{:?}", config)
}
