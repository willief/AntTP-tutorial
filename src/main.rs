mod config;
mod models;
mod services;
mod handlers;

use actix_web::{web, App, HttpServer, HttpResponse, middleware};
use actix_cors::Cors;
use dotenv::dotenv;
use log::info;

use config::Config;
use models::HealthResponse;
use services::chunk_service::ChunkService;
use handlers::chunk_handler;

/// Health check endpoint
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load .env file
    dotenv().ok();
    
    // Initialize logger
    env_logger::init();
    
    // Load configuration
    let config = Config::from_env();
    
    // Display startup banner
    println!("\n🦀 ════════════════════════════════════════════════════════");
    println!("🦀  Autonomi Rust Backend - Starting");
    println!("🦀 ════════════════════════════════════════════════════════");
    config.display();
    println!("🦀 ════════════════════════════════════════════════════════\n");
    
    // Create service instances
    let chunk_service = web::Data::new(ChunkService::new(&config.anttp_url));
    let bind_address = config.bind_address.clone();
    
    info!("Starting HTTP server on {}", bind_address);
    
    // Start HTTP server
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        App::new()
            // Middleware
            .wrap(middleware::Logger::default())
            .wrap(cors)
            
            // Share service instances
            .app_data(chunk_service.clone())
            
            // Routes
            .route("/health", web::get().to(health_check))
            .route("/chunks", web::post().to(chunk_handler::create_chunk))
            .route("/chunks/{address}", web::get().to(chunk_handler::get_chunk))
    })
    .bind(&bind_address)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    
    #[actix_web::test]
    async fn test_health_check() {
        let resp = health_check().await;
        assert_eq!(resp.status(), 200);
    }
    
    #[actix_web::test]
    async fn test_app_routes() {
        let chunk_service = web::Data::new(ChunkService::new("http://localhost:18888"));
        
        let app = test::init_service(
            App::new()
                .app_data(chunk_service)
                .route("/health", web::get().to(health_check))
                .route("/chunks", web::post().to(chunk_handler::create_chunk))
                .route("/chunks/{address}", web::get().to(chunk_handler::get_chunk))
        ).await;
        
        // Test health endpoint exists
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
