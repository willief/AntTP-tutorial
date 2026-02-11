// src/handlers/pointers.rs
//! Pointer handlers - Mutable address references
//!
//! For 1st Year CS Students:
//! Pointers are like shortcuts or bookmarks!
//! - They point to other addresses
//! - You can update where they point
//! - Like DNS: "mywebsite.com" points to "192.168.1.1"

use actix_web::{web, HttpRequest, HttpResponse};

use crate::models::{ErrorResponse, PointerData, PointerRequest, PointerResponse, StoreType};
use crate::services::NetworkService;

/// POST /anttp-0/pointer - Create pointer
///
/// For Students:
/// Creates a new pointer that points to a chunk/archive/etc
pub async fn create_pointer(
    req: HttpRequest,
    body: web::Json<PointerRequest>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    log::info!("📝 Creating pointer: {} → {}", body.name, body.content);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network
        .store_pointer(&body.name, &body.content, use_network)
        .await
    {
        Ok(address) => {
            log::info!("✅ Pointer created: {}", address);
            HttpResponse::Ok().json(PointerResponse { address })
        }
        Err(e) => {
            log::error!("❌ Failed to create pointer: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to create pointer", e.to_string()))
        }
    }
}

/// PUT /anttp-0/pointer/{address} - Update pointer
///
/// For Students:
/// Changes where the pointer points to (like updating a bookmark!)
pub async fn update_pointer(
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<PointerRequest>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    let address = path.into_inner();
    log::info!("📝 Updating pointer {}: → {}", address, body.content);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network
        .update_pointer(&address, &body.name, &body.content, use_network)
        .await
    {
        Ok(_) => {
            log::info!("✅ Pointer updated: {}", address);
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "Pointer updated"
            }))
        }
        Err(e) => {
            log::error!("❌ Failed to update pointer: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to update pointer", e.to_string()))
        }
    }
}

/// GET /anttp-0/pointer/{address} - Get pointer target
///
/// For Students:
/// Follows the pointer to see where it points!
pub async fn get_pointer(
    req: HttpRequest,
    path: web::Path<String>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    let address = path.into_inner();
    log::info!("📖 Getting pointer: {}", address);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network.get_pointer(&address, use_network).await {
        Ok(target) => {
            log::info!("✅ Pointer retrieved → {}", target);
            HttpResponse::Ok().json(PointerData { content: target })
        }
        Err(e) => {
            log::error!("❌ Pointer not found: {}", e);
            HttpResponse::NotFound()
                .json(ErrorResponse::with_details("Pointer not found", e.to_string()))
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
