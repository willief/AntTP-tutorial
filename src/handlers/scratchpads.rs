// src/handlers/scratchpads.rs
//! Scratchpad handlers - Public and Private mutable data
//!
//! For 1st Year CS Students:
//! Scratchpads are like notepads:
//! - PUBLIC: Everyone can read (like a bulletin board)
//! - PRIVATE: Only you can read (like a diary with a lock)

use actix_web::{web, HttpRequest, HttpResponse};
use base64::Engine; // Need this to use encode/decode methods

use crate::models::{
    ErrorResponse, ScratchpadData, ScratchpadRequest, ScratchpadResponse,
    ScratchpadUpdateRequest, StoreType,
};
use crate::services::NetworkService;

// ============================================================================
// PUBLIC SCRATCHPAD
// ============================================================================

/// POST /anttp-0/public_scratchpad - Create public scratchpad
///
/// For Students:
/// Anyone can read this! Like a public message board
pub async fn create_public_scratchpad(
    req: HttpRequest,
    body: web::Json<ScratchpadRequest>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    log::info!("📝 Creating public scratchpad: {}", body.name);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    // Validate Base64
    if base64::engine::general_purpose::STANDARD
        .decode(&body.content)
        .is_err()
    {
        log::error!("❌ Invalid Base64");
        return HttpResponse::BadRequest().json(ErrorResponse::new("Invalid Base64 encoding"));
    }

    match network
        .store_public_scratchpad(&body.name, &body.content, use_network)
        .await
    {
        Ok(address) => {
            log::info!("✅ Public scratchpad created: {}", address);
            HttpResponse::Ok().json(ScratchpadResponse { address })
        }
        Err(e) => {
            log::error!("❌ Failed to create scratchpad: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to create scratchpad", e.to_string()))
        }
    }
}

/// PUT /anttp-0/public_scratchpad/{address}/{name} - Update public scratchpad
///
/// For Students:
/// Updates the public message on the board
pub async fn update_public_scratchpad(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    body: web::Json<ScratchpadUpdateRequest>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    let (address, name) = path.into_inner();
    log::info!("📝 Updating public scratchpad: {}/{}", address, name);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    // Validate Base64
    if base64::engine::general_purpose::STANDARD
        .decode(&body.content)
        .is_err()
    {
        return HttpResponse::BadRequest().json(ErrorResponse::new("Invalid Base64 encoding"));
    }

    match network
        .update_public_scratchpad(&address, &name, &body.content, use_network)
        .await
    {
        Ok(_) => {
            log::info!("✅ Public scratchpad updated");
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "Scratchpad updated"
            }))
        }
        Err(e) => {
            log::error!("❌ Failed to update scratchpad: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to update scratchpad", e.to_string()))
        }
    }
}

/// GET /anttp-0/public_scratchpad/{address} - Get public scratchpad
///
/// For Students:
/// Read the public message!
pub async fn get_public_scratchpad(
    req: HttpRequest,
    path: web::Path<String>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    let address = path.into_inner();
    log::info!("📖 Getting public scratchpad: {}", address);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network.get_public_scratchpad(&address, use_network).await {
        Ok(content) => {
            log::info!("✅ Public scratchpad retrieved");
            HttpResponse::Ok().json(ScratchpadData { content })
        }
        Err(e) => {
            log::error!("❌ Scratchpad not found: {}", e);
            HttpResponse::NotFound()
                .json(ErrorResponse::with_details("Scratchpad not found", e.to_string()))
        }
    }
}

// ============================================================================
// PRIVATE SCRATCHPAD
// ============================================================================

/// POST /anttp-0/private_scratchpad - Create private scratchpad
///
/// For Students:
/// Encrypted! Only you can read it (if you have the name/key)
pub async fn create_private_scratchpad(
    req: HttpRequest,
    body: web::Json<ScratchpadRequest>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    log::info!("📝 Creating private scratchpad: {}", body.name);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    // Validate Base64
    if base64::engine::general_purpose::STANDARD
        .decode(&body.content)
        .is_err()
    {
        return HttpResponse::BadRequest().json(ErrorResponse::new("Invalid Base64 encoding"));
    }

    match network
        .store_private_scratchpad(&body.name, &body.content, use_network)
        .await
    {
        Ok(address) => {
            log::info!("✅ Private scratchpad created: {}", address);
            HttpResponse::Ok().json(ScratchpadResponse { address })
        }
        Err(e) => {
            log::error!("❌ Failed to create private scratchpad: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to create scratchpad", e.to_string()))
        }
    }
}

/// PUT /anttp-0/private_scratchpad/{address}/{name} - Update private scratchpad
pub async fn update_private_scratchpad(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    body: web::Json<ScratchpadUpdateRequest>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    let (address, name) = path.into_inner();
    log::info!("📝 Updating private scratchpad: {}/{}", address, name);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    if base64::engine::general_purpose::STANDARD
        .decode(&body.content)
        .is_err()
    {
        return HttpResponse::BadRequest().json(ErrorResponse::new("Invalid Base64 encoding"));
    }

    match network
        .update_private_scratchpad(&address, &name, &body.content, use_network)
        .await
    {
        Ok(_) => {
            log::info!("✅ Private scratchpad updated");
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "Scratchpad updated"
            }))
        }
        Err(e) => {
            log::error!("❌ Failed to update scratchpad: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to update scratchpad", e.to_string()))
        }
    }
}

/// GET /anttp-0/private_scratchpad/{address}/{name} - Get private scratchpad
///
/// For Students:
/// You need the name (like a password) to decrypt it!
pub async fn get_private_scratchpad(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    let (address, name) = path.into_inner();
    log::info!("📖 Getting private scratchpad: {}/{}", address, name);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network
        .get_private_scratchpad(&address, &name, use_network)
        .await
    {
        Ok(content) => {
            log::info!("✅ Private scratchpad retrieved");
            HttpResponse::Ok().json(ScratchpadData { content })
        }
        Err(e) => {
            log::error!("❌ Scratchpad not found: {}", e);
            HttpResponse::NotFound()
                .json(ErrorResponse::with_details("Scratchpad not found", e.to_string()))
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
