// src/handlers/publicdata.rs
//! Public Data handlers - Simple binary storage
//!
//! For 1st Year CS Students:
//! Public Data is like chunks but simpler!
//! Just store raw bytes, no encoding needed
//! Perfect for images, videos, binary files

use actix_web::{web, HttpRequest, HttpResponse};
use bytes::Bytes;

use crate::models::{ErrorResponse, StoreType};
use crate::services::NetworkService;

/// POST /anttp-0/binary/public_data - Create public data
///
/// For Students:
/// Store raw binary data (like an image or video)
/// No encoding! Just send the raw bytes
pub async fn create_public_data(
    req: HttpRequest,
    body: Bytes,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    log::info!("📤 Creating public data ({} bytes)", body.len());

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network.store_public_data(body, use_network).await {
        Ok(address) => {
            log::info!("✅ Public data created: {}", address);
            HttpResponse::Ok().json(serde_json::json!({
                "address": address
            }))
        }
        Err(e) => {
            log::error!("❌ Failed to create public data: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to create public data", e.to_string()))
        }
    }
}

/// GET /anttp-0/binary/public_data/{address} - Get public data
///
/// For Students:
/// Retrieve raw binary data
/// Returns the exact bytes you stored!
pub async fn get_public_data(
    req: HttpRequest,
    path: web::Path<String>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    let address = path.into_inner();
    log::info!("📥 Getting public data: {}", address);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network.get_public_data(&address, use_network).await {
        Ok(data) => {
            log::info!("✅ Public data retrieved ({} bytes)", data.len());
            HttpResponse::Ok()
                .content_type("application/octet-stream")
                .body(data)
        }
        Err(e) => {
            log::error!("❌ Public data not found: {}", e);
            HttpResponse::NotFound()
                .json(ErrorResponse::with_details("Public data not found", e.to_string()))
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
