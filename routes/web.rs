use axum::{routing::get, Router};

pub fn init_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Welcome to Pakem Framework!" }))
        // ─── Protected Routes (do not remove this comment)
        
        // Debug (do not remove this comment)
}
