
use axum::{routing::get, Router};

use super::rate_converter_handler::handler as rate_conversion_handler;

async fn route_dispatecher() -> Router {
    Router::new()
        .route("/convert", get(rate_conversion_handler))
}

pub async fn start_server() {

    let app = route_dispatecher().await;
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}