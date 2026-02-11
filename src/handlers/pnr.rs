// src/handlers/pnr.rs
//! PNR handlers - Pointer Name Registry (DNS-like system)
//!
//! For 1st Year CS Students:
//! PNR = Pointer Name Registry
//! It's like DNS for the Autonomi network!
//! Instead of "google.com" → "172.217.14.206"
//! You get "my-website" → "abc123...network-address"

use actix_web::{web, HttpRequest, HttpResponse};

use crate::models::{ErrorResponse, PnrRequest, StoreType};
use crate::services::NetworkService;

/// POST /anttp-0/pnr - Create PNR
///
/// For Students:
/// Create a name registry (like registering a domain name)
pub async fn create_pnr(
    req: HttpRequest,
    body: web::Json<PnrRequest>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    log::info!("🌐 Creating PNR: {}", body.name);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network
        .store_pnr(&body.name, &body.records, use_network)
        .await
    {
        Ok(address) => {
            log::info!("✅ PNR created: {}", address);
            HttpResponse::Ok().json(serde_json::json!({
                "address": address,
                "name": body.name
            }))
        }
        Err(e) => {
            log::error!("❌ Failed to create PNR: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to create PNR", e.to_string()))
        }
    }
}

/// PUT /anttp-0/pnr/{name} - Update PNR
///
/// For Students:
/// Update DNS-like records (like changing where a domain points)
pub async fn update_pnr(
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<PnrRequest>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    let name = path.into_inner();
    log::info!("🌐 Updating PNR: {}", name);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network
        .update_pnr(&name, &body.records, use_network)
        .await
    {
        Ok(_) => {
            log::info!("✅ PNR updated: {}", name);
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "PNR updated"
            }))
        }
        Err(e) => {
            log::error!("❌ Failed to update PNR: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to update PNR", e.to_string()))
        }
    }
}

/// GET /anttp-0/pnr/{name} - Get PNR
///
/// For Students:
/// Look up DNS-like records (like "nslookup")
pub async fn get_pnr(
    req: HttpRequest,
    path: web::Path<String>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    let name = path.into_inner();
    log::info!("🌐 Getting PNR: {}", name);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network.get_pnr(&name, use_network).await {
        Ok(records) => {
            log::info!("✅ PNR retrieved");
            HttpResponse::Ok().json(serde_json::json!({
                "name": name,
                "records": records
            }))
        }
        Err(e) => {
            log::error!("❌ PNR not found: {}", e);
            HttpResponse::NotFound()
                .json(ErrorResponse::with_details("PNR not found", e.to_string()))
        }
    }
}

/// PATCH /anttp-0/pnr/{name} - Append to PNR
///
/// For Students:
/// Add more records without replacing existing ones
pub async fn append_pnr(
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<PnrRequest>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    let name = path.into_inner();
    log::info!("🌐 Appending to PNR: {}", name);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network
        .append_pnr(&name, &body.records, use_network)
        .await
    {
        Ok(_) => {
            log::info!("✅ PNR appended: {}", name);
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "Records appended to PNR"
            }))
        }
        Err(e) => {
            log::error!("❌ Failed to append PNR: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to append PNR", e.to_string()))
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
