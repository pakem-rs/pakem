use pakem::prelude::*;
use serde_json::json;

//                        WELCOME CONTROLLER
// Controller halaman utama aplikasi Pakem.
// Mendukung Dual-Mode Response:
// 1. Browser Request (Accept: text/html) -> Mengembalikan Dashboard HTML modern ala Laravel.
// 2. API / cURL Request (Accept: application/json atau ?format=json) -> Mengembalikan JSON ala Lumen!

pub struct WelcomeController;

impl WelcomeController {
    /// Halaman Utama ("/") - Dual Mode Response
    pub async fn index(req: Request) -> impl IntoResponse {
        // Cek informasi koneksi database aktif
        let db_info = if let Some(pool) = &req.state.db {
            let driver = format!("{:?}", pool.kind);
            let is_ok = sqlx::query("SELECT 1").execute(&pool.pool).await.is_ok();
            (driver, if is_ok { "connected" } else { "disconnected" })
        } else {
            ("SQLite".to_string(), "not connected")
        };

        // 1. Jika request meminta JSON (via Header Accept: application/json atau ?format=json)
        // Mengembalikan spesifikasi framework ala Lumen microframework!
        let wants_json = req.wants_json() 
            || req.header("accept").map(|a| a.contains("json")).unwrap_or(false);

        if wants_json {
            return axum::Json(json!({
                "name": "Pakem Framework",
                "tagline": "The High-Performance Rust Backend for Modern APIs",
                "version": "1.0.1",
                "rust_edition": "2021",
                "engine": "Axum 0.7 + Tokio Async",
                "environment": "local",
                "database": {
                    "driver": db_info.0,
                    "status": db_info.1
                },
                "documentation": "https://pakem.my.id",
                "endpoints": {
                    "home": "/",
                    "health": "/health",
                    "api_index": "/api",
                    "api_health": "/api/health"
                }
            })).into_response();
        }

        // 2. Jika diakses via Web Browser biasa, render Blade HTML Dashboard
        req.view("welcome")
            .with("title", "Pakem Framework - Look like Laravel, Run at Rust Speed")
            .with("app_name", "Pakem Framework")
            .with("version", "1.0.1")
            .with("rust_edition", "2021")
            .with("db_driver", db_info.0)
            .with("db_status", db_info.1)
            .with("docs_url", "https://pakem.my.id")
            .render(req)
            .await
            .into_response()
    }

    /// Menampilkan Halaman Login ("/login")
    pub async fn login(req: Request) -> impl IntoResponse {
        req.view("auth.login")
            .with("title", "Login - Pakem")
            .render(req)
            .await
            .into_response()
    }

    /// Menampilkan Halaman Register ("/register")
    pub async fn register(req: Request) -> impl IntoResponse {
        req.view("auth.register")
            .with("title", "Register - Pakem")
            .render(req)
            .await
            .into_response()
    }
}
