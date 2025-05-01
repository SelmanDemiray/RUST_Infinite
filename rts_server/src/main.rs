use axum::{routing::get, Router, Extension};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenvy::dotenv;
use std::env;

mod auth;
mod ws;
mod db;
mod error;
// mod game_state; // Add later for managing active game simulation
// mod admin; // Add later for admin routes

// Shared application state
#[derive(Clone)]
pub struct AppState {
    db_pool: PgPool,
    // Add more shared state: e.g., Concurrent map of connected clients, game world state manager
    // connected_clients: Arc<DashMap<PlayerId, Sender<ServerMessage>>>,
    // game_world: Arc<Mutex<GameWorld>>, // Or more sophisticated concurrent structure
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file

    // Initialize tracing (logging)
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "rts_server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Database setup
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPool::connect(&database_url).await?;
    tracing::info!("Database pool connected");

    // Run migrations (optional, often done separately)
    // sqlx::migrate!("./migrations").run(&db_pool).await?;
    // tracing::info!("Database migrations applied");

    let app_state = AppState { db_pool };

    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any) // Configure restrictively in production!
        .allow_methods(Any)
        .allow_headers(Any);

    // Build application routes
    let app = Router::new()
        .route("/", get(|| async { "RTS Server Running" }))
        // Authentication routes
        .route("/api/register", axum::routing::post(auth::register_handler))
        .route("/api/login", axum::routing::post(auth::login_handler))
        // WebSocket route
        .route("/ws", get(ws::websocket_handler))
        // --- Add Admin Routes ---
        // .nest("/admin", admin::admin_routes())
        .layer(Extension(app_state))
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // Run the server
    let addr_str = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    let addr: SocketAddr = addr_str.parse()?;
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
