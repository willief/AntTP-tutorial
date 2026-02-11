// src/handlers/archives.rs
//! Archive handlers - File collections with multipart upload
//!
//! For 1st Year CS Students:
//! Archives are like ZIP files!
//! - Upload multiple files at once
//! - They stay organized together
//! - Like sending a folder through email

use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use bytes::Bytes;
use futures::StreamExt;
use std::path::PathBuf;

use crate::models::{ArchiveFile, ArchiveResponse, ErrorResponse, StoreType};
use crate::services::NetworkService;

/// POST /anttp-0/multipart/public_archive - Create archive
///
/// For Students:
/// Upload multiple files as a single archive
/// Uses multipart/form-data (like uploading files in a web form)
pub async fn create_archive(
    req: HttpRequest,
    mut payload: Multipart,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    log::info!("📦 Creating archive from multipart upload");

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

        // Get filename from Content-Disposition
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
        log::error!("❌ No files in archive");
        return HttpResponse::BadRequest().json(ErrorResponse::new("No files provided"));
    }

    log::info!("📦 Archive contains {} files", files.len());

    // Store archive
    match network.store_archive(files, use_network).await {
        Ok(address) => {
            log::info!("✅ Archive created: {}", address);
            HttpResponse::Ok().json(ArchiveResponse { address })
        }
        Err(e) => {
            log::error!("❌ Failed to create archive: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to create archive", e.to_string()))
        }
    }
}

/// POST /anttp-0/multipart/public_archive/{path:.*} - Create archive with path
///
/// For Students:
/// Create archive at a specific path (like creating a subfolder)
pub async fn create_archive_with_path(
    req: HttpRequest,
    path: web::Path<String>,
    mut payload: Multipart,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    let archive_path = path.into_inner();
    log::info!("📦 Creating archive at path: {}", archive_path);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    // Parse multipart form data
    let mut files = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(field) => field,
            Err(e) => {
                return HttpResponse::BadRequest()
                    .json(ErrorResponse::new(format!("Invalid multipart data: {}", e)));
            }
        };

        let filename = field
            .content_disposition()
            .get_filename()
            .unwrap_or("unnamed")
            .to_string();

        // Prepend the archive path to filename
        let full_path = PathBuf::from(&archive_path).join(&filename);

        let mut content = Vec::new();
        while let Some(chunk) = field.next().await {
            let chunk = match chunk {
                Ok(data) => data,
                Err(e) => {
                    return HttpResponse::BadRequest().json(ErrorResponse::new(format!(
                        "Failed to read file data: {}",
                        e
                    )));
                }
            };
            content.extend_from_slice(&chunk);
        }

        files.push((full_path, Bytes::from(content)));
    }

    if files.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse::new("No files provided"));
    }

    // Store archive
    match network.store_archive(files, use_network).await {
        Ok(address) => {
            log::info!("✅ Archive created at path: {}", address);
            HttpResponse::Ok().json(ArchiveResponse { address })
        }
        Err(e) => {
            log::error!("❌ Failed to create archive: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to create archive", e.to_string()))
        }
    }
}

/// GET /anttp-0/public_archive/{address} - Get archive root
///
/// For Students:
/// Lists all files in the archive (like "ls" command)
pub async fn get_archive_root(
    req: HttpRequest,
    path: web::Path<String>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    let address = path.into_inner();
    log::info!("📖 Getting archive: {}", address);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network.get_archive(&address, use_network).await {
        Ok(files) => {
            log::info!("✅ Archive retrieved ({} files)", files.len());

            // Convert to response format
            let file_list: Vec<ArchiveFile> = files
                .into_iter()
                .map(|(path, content)| ArchiveFile {
                    path: path.to_string_lossy().to_string(),
                    content: content.to_vec(),
                })
                .collect();

            HttpResponse::Ok().json(serde_json::json!({
                "address": address,
                "files": file_list
            }))
        }
        Err(e) => {
            log::error!("❌ Archive not found: {}", e);
            HttpResponse::NotFound()
                .json(ErrorResponse::with_details("Archive not found", e.to_string()))
        }
    }
}

/// GET /anttp-0/public_archive/{address}/{path:.*} - Get specific file from archive
///
/// For Students:
/// Gets one specific file from the archive
pub async fn get_archive_file(
    req: HttpRequest,
    path: web::Path<(String, String)>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    let (address, file_path) = path.into_inner();
    log::info!("📖 Getting file from archive: {}/{}", address, file_path);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network.get_archive(&address, use_network).await {
        Ok(files) => {
            // Find the requested file
            for (path, content) in files {
                if path.to_string_lossy() == file_path {
                    log::info!("✅ File found ({} bytes)", content.len());
                    return HttpResponse::Ok()
                        .content_type("application/octet-stream")
                        .body(content);
                }
            }

            log::error!("❌ File not found in archive: {}", file_path);
            HttpResponse::NotFound()
                .json(ErrorResponse::new(format!("File not found: {}", file_path)))
        }
        Err(e) => {
            log::error!("❌ Archive not found: {}", e);
            HttpResponse::NotFound()
                .json(ErrorResponse::with_details("Archive not found", e.to_string()))
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
