use breachlet_app::{router, state::AppState};
use breachlet_core::{config::BackendConfig, error::AppResult, tracing::configure_tracing};

use anyhow::Context;

use tracing::info;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Configure and Initialise tracing
    configure_tracing();
    info!("Starting Breachlet Backend Server...");

    // Load config
    let config = BackendConfig::load()?;
    info!("Successfully loaded backend config");

    // Initialise the persistent app state
    let app_state = AppState::init(config.database).await?;
    info!("App State successfully initialised");

    // Run database migrations on startup
    info!("Starting database migrations...");
    sqlx::migrate!("../../../migrations").run(&app_state.db_pool).await?;
    info!("Successfully completed database migrations");

    // Build the app router with a persistent app state
    let app = router::build_app_router(app_state).await?;
    info!("Successfully built app router");

    // Create TCP listener for the HTTP server
    let listener = tokio::net::TcpListener::bind(config.server.address())
        .await
        .with_context(|| format!("Failed to bind TCP listener on {}", config.server.address()))?;
    info!("listening on http://{}", &config.server.address());

    // Serve the app via the TCP listener
    axum::serve(listener, app)
        .await
        .with_context(|| "HTTP Server exited with error")?;
    info!("HTTP Server exited with success");

    Ok(())
}
