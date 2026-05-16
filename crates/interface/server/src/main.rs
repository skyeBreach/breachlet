use std::sync::Arc;

use breachlet_app::{
    router::{self, build_router},
    services::user_service::UserService,
    state::AppState,
};
use breachlet_core::{config::BackendConfig, error::AppResult, tracing::configure_tracing};

use anyhow::Context;

use breachlet_infra::postgres::user_repo::PgUserRepository;
use sqlx::postgres::PgPoolOptions;
use tracing::{info, info_span};

#[tokio::main]
async fn main() -> AppResult<()> {
    // Configure and Initialise tracing
    configure_tracing();
    info!("Starting Breachlet Backend Server...");

    // Load config
    let config = BackendConfig::load()?;
    info!("Successfully loaded backend config");

    // Initisalise database pool and create all repositories
    let pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .connect(&config.database.url())
        .await?;

    // Run database migrations on startup
    info!("Starting database migrations...");
    sqlx::migrate!("../../../migrations").run(&pool).await?;
    info!("Successfully completed database migrations");

    // Initialise all repositories
    info!("Initialising required repositories");
    let user_repo = Arc::new(PgUserRepository::new(pool));

    // Initialise the persistent app state
    let app_state = AppState {
        user_service: Arc::new(UserService::new(user_repo)),
    };
    info!("App State successfully initialised");

    // Build the app router with a persistent state
    let app = build_router(app_state).await?;
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
