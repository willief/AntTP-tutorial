// src/handlers/keyvalue.rs
//! Key/Value handlers - Object storage with buckets
//!
//! For 1st Year CS Students:
//! Key/Value storage is like a filing cabinet with drawers!
//! - Bucket = Drawer (e.g., "photos", "documents")
//! - Object = File in the drawer (e.g., "vacation.jpg")
//! Like AWS S3 or Google Cloud Storage!

use actix_web::{web, HttpRequest, HttpResponse};
use base64::Engine; // Need this to use encode/decode methods

use crate::models::{ErrorResponse, KeyValueData, KeyValueRequest, StoreType};
use crate::services::NetworkService;

/// POST /anttp-0/key_value - Create key/value pair
///
/// For Students:
/// Store data in a bucket/object structure
/// Like: bucket="photos", object="vacation.jpg"
pub async fn create_key_value(
    req: HttpRequest,
    body: web::Json<KeyValueRequest>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    log::info!(
        "🗄️ Creating key/value: {}/{}",
        body.bucket,
        body.object
    );

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
        .store_key_value(&body.bucket, &body.object, &body.content, use_network)
        .await
    {
        Ok(address) => {
            log::info!("✅ Key/value created: {}", address);
            HttpResponse::Ok().json(serde_json::json!({
                "address": address,
                "bucket": body.bucket,
                "object": body.object
            }))
        }
        Err(e) => {
            log::error!("❌ Failed to create key/value: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to create key/value", e.to_string()))
        }
    }
}

/// GET /anttp-0/key_value/{bucket}/{object} - Get key/value
///
/// For Students:
/// Retrieve data from bucket/object
pub async fn get_key_value(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    let (bucket, object) = path.into_inner();
    log::info!("🗄️ Getting key/value: {}/{}", bucket, object);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network.get_key_value(&bucket, &object, use_network).await {
        Ok(content) => {
            log::info!("✅ Key/value retrieved");
            HttpResponse::Ok().json(KeyValueData { content })
        }
        Err(e) => {
            log::error!("❌ Key/value not found: {}", e);
            HttpResponse::NotFound()
                .json(ErrorResponse::with_details("Key/value not found", e.to_string()))
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
