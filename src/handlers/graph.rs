// src/handlers/graph.rs
//! Graph handlers - Graph data structure storage
//!
//! For 1st Year CS Students:
//! Graphs are like networks or webs of connected data!
//! Think: Social network (friends), Road map (cities), Family tree

use actix_web::{web, HttpRequest, HttpResponse};

use crate::models::{ErrorResponse, GraphEntryRequest, StoreType};
use crate::services::NetworkService;

/// POST /anttp-0/graph_entry - Create graph entry
///
/// For Students:
/// Store a node in a graph data structure
/// Graphs connect pieces of data together (like Facebook friends)
pub async fn create_graph_entry(
    req: HttpRequest,
    body: web::Json<GraphEntryRequest>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    log::info!("🕸️ Creating graph entry: {}", body.name);

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
        .store_graph_entry(&body.name, &body.content, use_network)
        .await
    {
        Ok(address) => {
            log::info!("✅ Graph entry created: {}", address);
            HttpResponse::Ok().json(serde_json::json!({
                "address": address
            }))
        }
        Err(e) => {
            log::error!("❌ Failed to create graph entry: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::with_details("Failed to create graph entry", e.to_string()))
        }
    }
}

/// GET /anttp-0/graph_entry/{address} - Get graph entry
///
/// For Students:
/// Retrieve a node from the graph
pub async fn get_graph_entry(
    req: HttpRequest,
    path: web::Path<String>,
    network: web::Data<NetworkService>,
) -> HttpResponse {
    let address = path.into_inner();
    log::info!("🕸️ Getting graph entry: {}", address);

    let store_type = get_store_type(&req);
    let use_network = store_type == StoreType::Network;

    match network.get_graph_entry(&address, use_network).await {
        Ok((name, content)) => {
            log::info!("✅ Graph entry retrieved");
            HttpResponse::Ok().json(serde_json::json!({
                "name": name,
                "content": content
            }))
        }
        Err(e) => {
            log::error!("❌ Graph entry not found: {}", e);
            HttpResponse::NotFound()
                .json(ErrorResponse::with_details("Graph entry not found", e.to_string()))
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
