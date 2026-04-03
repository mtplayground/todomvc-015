use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::EnvFilter;

use backend::db;
use backend::routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let pool = db::init_db()?;

    let dist_dir = std::env::var("DIST_DIR").unwrap_or_else(|_| "dist".to_string());

    let serve_dir =
        ServeDir::new(&dist_dir).fallback(ServeFile::new(format!("{}/index.html", dist_dir)));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .merge(routes::api_router(pool))
        .layer(cors)
        .fallback_service(serve_dir);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
