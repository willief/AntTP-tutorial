// src/handlers/tarchive.rs
//! Tarchive handlers - Tar-based archive format
//!
//! For 1st Year CS Students:
//! Tarchive is like a regular archive but uses TAR format
//! TAR = Tape Archive (old Unix format for bundling files)
//! Think: .tar.gz files you download from GitHub!

use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use bytes::Bytes;
use futures::StreamExt;
use std::path::PathBuf;

use crate::models::{ArchiveResponse, ErrorResponse, StoreType};
use crate::services::NetworkService;

/// POST /anttp-0/multipart/tarchive - Create tarchive
///
/// For Students:
/// Like regular archives but stored in TAR format
/// TAR = Tape Archive (bundling multiple files into one)
pub async fn create_tarchive(
    req: HttpRequest,
    mut payload: Multipart,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    log::info!("📦 Creating tarchive from multipart upload");

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    // Parse multipart form data
    let mut files = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(e) => {
                log::error!("❌ Failed to read multipart field: {}", e);
                return HttpResponse::BadRequest()
                    .json(ErrorResponse::new(format!("Invalid multipart data: {}", e)));
            }
        };

        // Get filename
        let filename = field
            .content_disposition()
            .get_filename()
            .unwrap_or("unnamed")
            .to_string();

        log::info!("  📄 Reading file: {}", filename);

        // Read file content
        let mut content = Vec::new();
        while let Some(chunk) = field.next().await {
            let chunk = match chunk {
                Ok(data) => data,
                Err(e) => {
                    log::error!("❌ Failed to read chunk: {}", e);
                    return HttpResponse::BadRequest().json(ErrorResponse::new(format!(
                        "Failed to read file data: {}",
                        e
                    )));
                }
            };
            content.extend_from_slice(&chunk);
        }

        files.push((PathBuf::from(filename), Bytes::from(content)));
    }

    if files.is_empty() {
        log::error!("❌ No files in tarchive");
        return HttpResponse::BadRequest().json(ErrorResponse::new("No files provided"));
    }

    log::info!("📦 Tarchive contains {} files", files.len());

    // Store as tarchive (similar to archive but TAR format)
    match network.store_tarchive(files, use_network).await {
        Ok(address) => {
            log::info!("✅ Tarchive created: {}", address);
            HttpResponse::Ok().json(ArchiveResponse { address })
        }
        Err(e) => {
            log::error!("❌ Failed to create tarchive: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to create tarchive", e.to_string()))
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
