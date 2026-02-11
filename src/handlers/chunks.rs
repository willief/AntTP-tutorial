// src/handlers/chunks.rs
//! Chunk handlers - Immutable data storage
//!
//! For 1st Year CS Students:
//! Handlers are like restaurant workers - they take orders (HTTP requests)
//! and give you food (HTTP responses)!

use actix_web::{web, HttpRequest, HttpResponse};
use base64::Engine; // Need this to use encode/decode methods
use bytes::Bytes;

use crate::models::{ChunkData, ChunkRequest, ChunkResponse, ErrorResponse, StoreType};
use crate::services::NetworkService;

/// POST /anttp-0/chunk - Create chunk (JSON)
///
/// For Students:
/// This takes Base64-encoded content and stores it as a chunk
pub async fn create_chunk(
    req: HttpRequest,
    body: web::Json<ChunkRequest>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    log::info!("📝 Creating chunk (JSON)");

    // Get storage type from header
    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    // Decode Base64 content
    let content_bytes = match base64::engine::general_purpose::STANDARD.decode(&body.content) {
        Ok(bytes) => Bytes::from(bytes),
        Err(e) => {
            log::error!("❌ Invalid Base64: {}", e);
            return HttpResponse::BadRequest().json(ErrorResponse::new(format!(
                "Invalid Base64 encoding: {}",
                e
            )));
        }
    };

    // Store chunk
    match network.store_chunk(content_bytes, use_network).await {
        Ok(address) => {
            log::info!("✅ Chunk created: {}", address);
            HttpResponse::Ok().json(ChunkResponse { address })
        }
        Err(e) => {
            log::error!("❌ Failed to store chunk: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to store chunk", e.to_string()))
        }
    }
}

/// POST /anttp-0/binary/chunk - Create chunk (Binary)
///
/// For Students:
/// This takes raw bytes (like an image) and stores it
pub async fn create_chunk_binary(
    req: HttpRequest,
    body: Bytes,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    log::info!("📝 Creating chunk (Binary, {} bytes)", body.len());

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network.store_chunk(body, use_network).await {
        Ok(address) => {
            log::info!("✅ Binary chunk created: {}", address);
            HttpResponse::Ok().json(ChunkResponse { address })
        }
        Err(e) => {
            log::error!("❌ Failed to store binary chunk: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to store chunk", e.to_string()))
        }
    }
}

/// GET /anttp-0/chunk/{address} - Get chunk (JSON)
///
/// For Students:
/// Retrieves a chunk and returns it as Base64
pub async fn get_chunk(
    req: HttpRequest,
    address: web::Path<String>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    log::info!("📖 Retrieving chunk: {}", address);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network.get_chunk(&address, use_network).await {
        Ok(data) => {
            // Encode to Base64
            let content = base64::engine::general_purpose::STANDARD.encode(&data);
            log::info!("✅ Chunk retrieved ({} bytes)", data.len());
            HttpResponse::Ok().json(ChunkData { content })
        }
        Err(e) => {
            log::error!("❌ Chunk not found: {}", e);
            HttpResponse::NotFound()
                .json(ErrorResponse::with_details("Chunk not found", e.to_string()))
        }
    }
}

/// GET /anttp-0/binary/chunk/{address} - Get chunk (Binary)
///
/// For Students:
/// Retrieves a chunk and returns raw bytes
pub async fn get_chunk_binary(
    req: HttpRequest,
    address: web::Path<String>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    log::info!("📖 Retrieving binary chunk: {}", address);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network.get_chunk(&address, use_network).await {
        Ok(data) => {
            log::info!("✅ Binary chunk retrieved ({} bytes)", data.len());
            HttpResponse::Ok()
                .content_type("application/octet-stream")
                .body(data)
        }
        Err(e) => {
            log::error!("❌ Chunk not found: {}", e);
            HttpResponse::NotFound()
                .json(ErrorResponse::with_details("Chunk not found", e.to_string()))
        }
    }
}

/// Helper: Extract store type from x-store-type header
fn get_store_type(req: &HttpRequest) -> StoreType {
    req.headers()
        .get("x-store-type")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or_default()
}
