// src/main.rs
//! AntTP-Compatible Rust Backend
//!
//! For 1st Year CS Students:
//! This is the main program that starts the web server.
//! It's like opening a restaurant - we set up tables (routes)
//! and hire staff (handlers) to serve customers (HTTP requests)!

mod handlers;
mod models;
mod services;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use services::NetworkService;
use std::sync::Arc;

/// Health check endpoint
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "AntTP-compatible Rust backend"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    log::info!("🦀 ════════════════════════════════════════════════════════");
    log::info!("🦀  AntTP-Compatible Rust Backend");
    log::info!("🦀  Version: {}", env!("CARGO_PKG_VERSION"));
    log::info!("🦀 ════════════════════════════════════════════════════════");

    // Initialize network service
    log::info!("🌐 Initializing network service...");
    let network_service = match NetworkService::new().await {
        Ok(service) => {
            log::info!("✅ Network service initialized");
            Arc::new(service)
        }
        Err(e) => {
            log::error!("❌ Failed to initialize network service: {}", e);
            log::error!("   Make sure WALLET_PRIVATE_KEY is set!");
            log::warn!("⚠️  Continuing with limited functionality...");
            // In production, you might want to exit here
            // For tutorial purposes, we allow it to continue
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Network init failed: {}", e),
            ));
        }
    };

    // Get server configuration from environment
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "18888".to_string())
        .parse::<u16>()
        .expect("SERVER_PORT must be a valid port number");

    let bind_address = format!("{}:{}", host, port);

    log::info!("🚀 Starting HTTP server...");
    log::info!("📍 Listening on: http://{}", bind_address);
    log::info!("");
    log::info!("📋 Available endpoints:");
    log::info!("   Health:              GET  /health");
    log::info!("");
    log::info!("   Chunks (JSON):       POST /anttp-0/chunk");
    log::info!("   Chunks (JSON):       GET  /anttp-0/chunk/{{address}}");
    log::info!("   Chunks (Binary):     POST /anttp-0/binary/chunk");
    log::info!("   Chunks (Binary):     GET  /anttp-0/binary/chunk/{{address}}");
    log::info!("");
    log::info!("   Registers:           POST /anttp-0/register");
    log::info!("   Registers:           PUT  /anttp-0/register/{{address}}");
    log::info!("   Registers:           GET  /anttp-0/register/{{address}}");
    log::info!("   Register History:    GET  /anttp-0/register_history/{{address}}");
    log::info!("");
    log::info!("   Pointers:            POST /anttp-0/pointer");
    log::info!("   Pointers:            PUT  /anttp-0/pointer/{{address}}");
    log::info!("   Pointers:            GET  /anttp-0/pointer/{{address}}");
    log::info!("");
    log::info!("   Public Scratchpad:   POST /anttp-0/public_scratchpad");
    log::info!("   Public Scratchpad:   PUT  /anttp-0/public_scratchpad/{{address}}/{{name}}");
    log::info!("   Public Scratchpad:   GET  /anttp-0/public_scratchpad/{{address}}");
    log::info!("");
    log::info!("   Private Scratchpad:  POST /anttp-0/private_scratchpad");
    log::info!("   Private Scratchpad:  PUT  /anttp-0/private_scratchpad/{{address}}/{{name}}");
    log::info!("   Private Scratchpad:  GET  /anttp-0/private_scratchpad/{{address}}/{{name}}");
    log::info!("");
    log::info!("   Archives (Multipart):POST /anttp-0/multipart/public_archive");
    log::info!("   Archives (Path):     POST /anttp-0/multipart/public_archive/{{path}}");
    log::info!("   Archives (Root):     GET  /anttp-0/public_archive/{{address}}");
    log::info!("   Archives (File):     GET  /anttp-0/public_archive/{{address}}/{{path}}");
    log::info!("");
    log::info!("   Tarchive:            POST /anttp-0/multipart/tarchive");
    log::info!("");
    log::info!("   Graph Entry:         POST /anttp-0/graph_entry");
    log::info!("   Graph Entry:         GET  /anttp-0/graph_entry/{{address}}");
    log::info!("");
    log::info!("   PNR:                 POST /anttp-0/pnr");
    log::info!("   PNR:                 PUT  /anttp-0/pnr/{{name}}");
    log::info!("   PNR:                 GET  /anttp-0/pnr/{{name}}");
    log::info!("   PNR:                 PATCH /anttp-0/pnr/{{name}}");
    log::info!("");
    log::info!("   Key/Value:           POST /anttp-0/key_value");
    log::info!("   Key/Value:           GET  /anttp-0/key_value/{{bucket}}/{{object}}");
    log::info!("");
    log::info!("   Public Data:         POST /anttp-0/binary/public_data");
    log::info!("   Public Data:         GET  /anttp-0/binary/public_data/{{address}}");
    log::info!("");
    log::info!("   Commands:            GET  /anttp-0/command");
    log::info!("");
    log::info!("🧪 Test with:");
    log::info!("   curl -X POST http://{}:{}/anttp-0/chunk \\", host, port);
    log::info!("     -H 'Content-Type: application/json' \\");
    log::info!("     -H 'x-store-type: memory' \\");
    log::info!("     -d '{{\"content\":\"SGVsbG8gV29ybGQh\"}}'");
    log::info!("");
    log::info!("🦀 ════════════════════════════════════════════════════════");

    // Start HTTP server
    let service_data = web::Data::from(network_service);
    
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            // Add middleware
            .wrap(cors)
            .wrap(middleware::Logger::default())
            // Share network service across all requests
            .app_data(service_data.clone())
            // Health check (no prefix)
            .route("/health", web::get().to(health_check))
            // ========================================
            // CHUNK ENDPOINTS - /anttp-0/chunk
            // ========================================
            .route("/anttp-0/chunk", web::post().to(handlers::create_chunk))
            .route(
                "/anttp-0/chunk/{address}",
                web::get().to(handlers::get_chunk),
            )
            .route(
                "/anttp-0/binary/chunk",
                web::post().to(handlers::create_chunk_binary),
            )
            .route(
                "/anttp-0/binary/chunk/{address}",
                web::get().to(handlers::get_chunk_binary),
            )
            // ========================================
            // REGISTER ENDPOINTS - /anttp-0/register
            // ========================================
            .route("/anttp-0/register", web::post().to(handlers::create_register))
            .route(
                "/anttp-0/register/{address}",
                web::get().to(handlers::get_register),
            )
            .route(
                "/anttp-0/register/{address}",
                web::put().to(handlers::update_register),
            )
            .route(
                "/anttp-0/register_history/{address}",
                web::get().to(handlers::get_register_history),
            )
            // ========================================
            // POINTER ENDPOINTS - /anttp-0/pointer
            // ========================================
            .route("/anttp-0/pointer", web::post().to(handlers::create_pointer))
            .route(
                "/anttp-0/pointer/{address}",
                web::get().to(handlers::get_pointer),
            )
            .route(
                "/anttp-0/pointer/{address}",
                web::put().to(handlers::update_pointer),
            )
            // ========================================
            // PUBLIC SCRATCHPAD - /anttp-0/public_scratchpad
            // ========================================
            .route(
                "/anttp-0/public_scratchpad",
                web::post().to(handlers::create_public_scratchpad),
            )
            .route(
                "/anttp-0/public_scratchpad/{address}/{name}",
                web::put().to(handlers::update_public_scratchpad),
            )
            .route(
                "/anttp-0/public_scratchpad/{address}",
                web::get().to(handlers::get_public_scratchpad),
            )
            // ========================================
            // PRIVATE SCRATCHPAD - /anttp-0/private_scratchpad
            // ========================================
            .route(
                "/anttp-0/private_scratchpad",
                web::post().to(handlers::create_private_scratchpad),
            )
            .route(
                "/anttp-0/private_scratchpad/{address}/{name}",
                web::put().to(handlers::update_private_scratchpad),
            )
            .route(
                "/anttp-0/private_scratchpad/{address}/{name}",
                web::get().to(handlers::get_private_scratchpad),
            )
            // ========================================
            // ARCHIVE ENDPOINTS - /anttp-0/multipart/public_archive
            // ========================================
            .route(
                "/anttp-0/multipart/public_archive",
                web::post().to(handlers::create_archive),
            )
            .route(
                "/anttp-0/multipart/public_archive/{path:.*}",
                web::post().to(handlers::create_archive_with_path),
            )
            .route(
                "/anttp-0/public_archive/{address}",
                web::get().to(handlers::get_archive_root),
            )
            .route(
                "/anttp-0/public_archive/{address}/{path:.*}",
                web::get().to(handlers::get_archive_file),
            )
            // ========================================
            // TARCHIVE ENDPOINT - /anttp-0/multipart/tarchive
            // ========================================
            .route(
                "/anttp-0/multipart/tarchive",
                web::post().to(handlers::create_tarchive),
            )
            // ========================================
            // GRAPH ENDPOINTS - /anttp-0/graph_entry
            // ========================================
            .route(
                "/anttp-0/graph_entry",
                web::post().to(handlers::create_graph_entry),
            )
            .route(
                "/anttp-0/graph_entry/{address}",
                web::get().to(handlers::get_graph_entry),
            )
            // ========================================
            // PNR ENDPOINTS - /anttp-0/pnr
            // ========================================
            .route("/anttp-0/pnr", web::post().to(handlers::create_pnr))
            .route(
                "/anttp-0/pnr/{name}",
                web::put().to(handlers::update_pnr),
            )
            .route("/anttp-0/pnr/{name}", web::get().to(handlers::get_pnr))
            .route(
                "/anttp-0/pnr/{name}",
                web::patch().to(handlers::append_pnr),
            )
            // ========================================
            // KEY/VALUE ENDPOINTS - /anttp-0/key_value
            // ========================================
            .route(
                "/anttp-0/key_value",
                web::post().to(handlers::create_key_value),
            )
            .route(
                "/anttp-0/key_value/{bucket}/{object}",
                web::get().to(handlers::get_key_value),
            )
            // ========================================
            // PUBLIC DATA ENDPOINTS - /anttp-0/binary/public_data
            // ========================================
            .route(
                "/anttp-0/binary/public_data",
                web::post().to(handlers::create_public_data),
            )
            .route(
                "/anttp-0/binary/public_data/{address}",
                web::get().to(handlers::get_public_data),
            )
            // ========================================
            // COMMANDS ENDPOINT - /anttp-0/command
            // ========================================
            .route("/anttp-0/command", web::get().to(handlers::get_commands))
    })
    .bind(&bind_address)?
    .run()
    .await
}
