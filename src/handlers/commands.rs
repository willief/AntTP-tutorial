// src/handlers/commands.rs
//! Commands handler - System commands
//!
//! For 1st Year CS Students:
//! Commands are like a help menu!
//! Shows what operations are available
//! Like typing "help" in a command line

use actix_web::{HttpRequest, HttpResponse};

use crate::models::StoreType;

/// GET /anttp-0/command - Get available commands
///
/// For Students:
/// Returns a list of all available AntTP operations
/// Like a "help" menu!
pub async fn get_commands(req: HttpRequest) -> HttpResponse {
    log::info!("ℹ️ Getting commands list");

    let store_type = get_store_type(&req);

    let commands = serde_json::json!({
        "storage_type": format!("{:?}", store_type),
        "available_commands": [
            {
                "name": "chunk",
                "methods": ["POST", "GET"],
                "description": "Store and retrieve immutable data",
                "endpoints": [
                    "POST /anttp-0/chunk",
                    "GET /anttp-0/chunk/{address}",
                    "POST /anttp-0/binary/chunk",
                    "GET /anttp-0/binary/chunk/{address}"
                ]
            },
            {
                "name": "register",
                "methods": ["POST", "PUT", "GET"],
                "description": "Mutable key-value storage with history",
                "endpoints": [
                    "POST /anttp-0/register",
                    "PUT /anttp-0/register/{address}",
                    "GET /anttp-0/register/{address}",
                    "GET /anttp-0/register_history/{address}"
                ]
            },
            {
                "name": "pointer",
                "methods": ["POST", "PUT", "GET"],
                "description": "Mutable address references",
                "endpoints": [
                    "POST /anttp-0/pointer",
                    "PUT /anttp-0/pointer/{address}",
                    "GET /anttp-0/pointer/{address}"
                ]
            },
            {
                "name": "scratchpad",
                "methods": ["POST", "PUT", "GET"],
                "description": "Public and private mutable data",
                "endpoints": [
                    "POST /anttp-0/public_scratchpad",
                    "PUT /anttp-0/public_scratchpad/{address}/{name}",
                    "GET /anttp-0/public_scratchpad/{address}",
                    "POST /anttp-0/private_scratchpad",
                    "PUT /anttp-0/private_scratchpad/{address}/{name}",
                    "GET /anttp-0/private_scratchpad/{address}/{name}"
                ]
            },
            {
                "name": "archive",
                "methods": ["POST", "GET"],
                "description": "File collections (multipart upload)",
                "endpoints": [
                    "POST /anttp-0/multipart/public_archive",
                    "POST /anttp-0/multipart/public_archive/{path}",
                    "GET /anttp-0/public_archive/{address}",
                    "GET /anttp-0/public_archive/{address}/{path}"
                ]
            },
            {
                "name": "tarchive",
                "methods": ["POST"],
                "description": "Tar-based archives",
                "endpoints": [
                    "POST /anttp-0/multipart/tarchive"
                ]
            },
            {
                "name": "graph",
                "methods": ["POST", "GET"],
                "description": "Graph data structures",
                "endpoints": [
                    "POST /anttp-0/graph_entry",
                    "GET /anttp-0/graph_entry/{address}"
                ]
            },
            {
                "name": "pnr",
                "methods": ["POST", "PUT", "GET", "PATCH"],
                "description": "Pointer Name Registry (DNS-like)",
                "endpoints": [
                    "POST /anttp-0/pnr",
                    "PUT /anttp-0/pnr/{name}",
                    "GET /anttp-0/pnr/{name}",
                    "PATCH /anttp-0/pnr/{name}"
                ]
            },
            {
                "name": "key_value",
                "methods": ["POST", "GET"],
                "description": "Object storage with buckets",
                "endpoints": [
                    "POST /anttp-0/key_value",
                    "GET /anttp-0/key_value/{bucket}/{object}"
                ]
            },
            {
                "name": "public_data",
                "methods": ["POST", "GET"],
                "description": "Simple binary storage",
                "endpoints": [
                    "POST /anttp-0/binary/public_data",
                    "GET /anttp-0/binary/public_data/{address}"
                ]
            }
        ],
        "total_endpoints": 37,
        "version": env!("CARGO_PKG_VERSION")
    });

    HttpResponse::Ok().json(commands)
}

/// Helper: Extract store type from x-store-type header
fn get_store_type(req: &HttpRequest) -> StoreType {
    req.headers()
        .get("x-store-type")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse().ok())
        .unwrap_or_default()
}
