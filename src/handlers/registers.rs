// src/handlers/registers.rs
//! Register handlers - Mutable key-value storage with history
//!
//! For 1st Year CS Students:
//! Registers are like a notebook where you can:
//! - Write new entries
//! - Update existing entries
//! - See the history of all changes!

use actix_web::{web, HttpRequest, HttpResponse};

use crate::models::{
    ErrorResponse, RegisterData, RegisterRequest, RegisterResponse,
    StoreType,
};
use crate::services::NetworkService;

/// POST /anttp-0/register - Create register
///
/// For Students:
/// Creates a new mutable register with hex-encoded content
pub async fn create_register(
    req: HttpRequest,
    body: web::Json<RegisterRequest>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    log::info!("📝 Creating register: {}", body.name);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    // Validate hex encoding
    if hex::decode(&body.content).is_err() {
        log::error!("❌ Invalid hex encoding");
        return HttpResponse::BadRequest().json(ErrorResponse::new(
            "Content must be hex-encoded (not Base64!)",
        ));
    }

    match network
        .store_register(&body.name, &body.content, use_network)
        .await
    {
        Ok(address) => {
            log::info!("✅ Register created: {}", address);
            HttpResponse::Ok().json(RegisterResponse { address })
        }
        Err(e) => {
            log::error!("❌ Failed to create register: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to create register", e.to_string()))
        }
    }
}

/// PUT /anttp-0/register/{address} - Update register
///
/// For Students:
/// Updates an existing register - adds a new entry to the history
pub async fn update_register(
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<RegisterRequest>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    let address = path.into_inner();
    log::info!("📝 Updating register: {}", address);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    // Validate hex encoding
    if hex::decode(&body.content).is_err() {
        log::error!("❌ Invalid hex encoding");
        return HttpResponse::BadRequest()
            .json(ErrorResponse::new("Content must be hex-encoded"));
    }

    match network
        .update_register(&address, &body.name, &body.content, use_network)
        .await
    {
        Ok(_) => {
            log::info!("✅ Register updated: {}", address);
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "Register updated"
            }))
        }
        Err(e) => {
            log::error!("❌ Failed to update register: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to update register", e.to_string()))
        }
    }
}

/// GET /anttp-0/register/{address} - Get register current value
///
/// For Students:
/// Gets the latest value from the register
pub async fn get_register(
    req: HttpRequest,
    path: web::Path<String>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    let address = path.into_inner();
    log::info!("📖 Getting register: {}", address);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network.get_register(&address, use_network).await {
        Ok(content) => {
            log::info!("✅ Register retrieved");
            HttpResponse::Ok().json(RegisterData { content })
        }
        Err(e) => {
            log::error!("❌ Register not found: {}", e);
            HttpResponse::NotFound()
                .json(ErrorResponse::with_details("Register not found", e.to_string()))
        }
    }
}

/// GET /anttp-0/register_history/{address} - Get register history
///
/// For Students:
/// Gets ALL the values this register has ever had!
/// Like seeing all the edits to a Google Doc
pub async fn get_register_history(
    req: HttpRequest,
    path: web::Path<String>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    let address = path.into_inner();
    log::info!("📜 Getting register history: {}", address);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network.get_register_history(&address, use_network).await {
        Ok(history) => {
            log::info!("✅ Register history retrieved ({} entries)", history.len());
            HttpResponse::Ok().json(history)
        }
        Err(e) => {
            log::error!("❌ Failed to get history: {}", e);
            HttpResponse::NotFound()
                .json(ErrorResponse::with_details("Register not found", e.to_string()))
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
