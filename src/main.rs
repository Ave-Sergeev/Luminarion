use crate::setting::settings::Settings;
use axum::http::{StatusCode, Uri};
use axum::Router;
use tokio::net::TcpListener;
use crate::http::api::*;

mod http;
mod repository;
mod service;
mod setting;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new("config.yaml")?;

    let address = &format!("{}:{}", settings.server.host, settings.server.port);

    let listener = TcpListener::bind(address).await?;

    axum::serve(listener, routes()).await?;

    Ok(())
}

fn routes() -> Router {
    let all_routes = Router::new().nest("/v1", system_routes()).nest("/v1", test_routes());

    Router::new().nest("/api", all_routes).fallback(fallback)
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Handler for route '{uri}' not found!"))
}
