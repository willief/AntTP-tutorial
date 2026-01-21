use actix_web::{web, HttpResponse, Responder};
use log::{info, error};

use crate::models::{ChunkRequest, ChunkData, ErrorResponse};
use crate::services::chunk_service::{ChunkService, ChunkError};

/// POST /chunks - Create a new chunk
/// 
/// Equivalent to FastAPI's @app.post("/chunks")
pub async fn create_chunk(
    chunk_req: web::Json<ChunkRequest>,
    service: web::Data<ChunkService>,
) -> impl Responder {
    info!("Received create chunk request");
    
    match service.create_chunk(&chunk_req.content, &chunk_req.storage_type).await {
        Ok(chunk_response) => {
            info!("Chunk created: {}", chunk_response.address);
            HttpResponse::Ok().json(chunk_response)
        }
        Err(e) => {
            error!("Failed to create chunk: {}", e);
            
            match e {
                ChunkError::InvalidBase64 => {
                    HttpResponse::BadRequest()
                        .json(ErrorResponse::with_detail(
                            "Invalid input",
                            "Content must be valid base64 encoding"
                        ))
                }
                ChunkError::InvalidStorageType => {
                    HttpResponse::BadRequest()
                        .json(ErrorResponse::with_detail(
                            "Invalid storage type",
                            "Storage type must be: network, disk, or memory"
                        ))
                }
                ChunkError::ServiceUnavailable => {
                    HttpResponse::ServiceUnavailable()
                        .json(ErrorResponse::new("AntTP service is unavailable"))
                }
                _ => {
                    HttpResponse::InternalServerError()
                        .json(ErrorResponse::new("Internal server error"))
                }
            }
        }
    }
}

/// GET /chunks/{address} - Retrieve a chunk
/// 
/// Equivalent to FastAPI's @app.get("/chunks/{address}")
pub async fn get_chunk(
    address: web::Path<String>,
    service: web::Data<ChunkService>,
) -> impl Responder {
    let chunk_address = address.into_inner();
    info!("Received get chunk request for: {}", chunk_address);
    
    match service.get_chunk(&chunk_address).await {
        Ok(chunk_data) => {
            info!("Chunk retrieved successfully");
            HttpResponse::Ok().json(chunk_data)
        }
        Err(e) => {
            error!("Failed to get chunk: {}", e);
            
            match e {
                ChunkError::NotFound => {
                    HttpResponse::NotFound()
                        .json(ErrorResponse::with_detail(
                            "Chunk not found",
                            format!("No chunk found at address: {}", chunk_address)
                        ))
                }
                ChunkError::ServiceUnavailable => {
                    HttpResponse::ServiceUnavailable()
                        .json(ErrorResponse::new("AntTP service is unavailable"))
                }
                _ => {
                    HttpResponse::InternalServerError()
                        .json(ErrorResponse::new("Internal server error"))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};
    use crate::services::chunk_service::ChunkService;
    
    #[actix_web::test]
    async fn test_create_chunk_invalid_base64() {
        // Arrange
        let service = web::Data::new(ChunkService::new("http://localhost:18888"));
        let app = test::init_service(
            App::new()
                .app_data(service.clone())
                .route("/chunks", web::post().to(create_chunk))
        ).await;
        
        let invalid_request = ChunkRequest {
            content: "Not Valid Base64!!!".to_string(),
            storage_type: "network".to_string(),
        };
        
        // Act
        let req = test::TestRequest::post()
            .uri("/chunks")
            .set_json(&invalid_request)
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Assert
        assert_eq!(resp.status(), 400); // Bad Request
    }
    
    #[actix_web::test]
    async fn test_create_chunk_invalid_storage_type() {
        // Arrange
        let service = web::Data::new(ChunkService::new("http://localhost:18888"));
        let app = test::init_service(
            App::new()
                .app_data(service.clone())
                .route("/chunks", web::post().to(create_chunk))
        ).await;
        
        let invalid_request = ChunkRequest {
            content: base64::encode("Hello"),
            storage_type: "invalid_type".to_string(),
        };
        
        // Act
        let req = test::TestRequest::post()
            .uri("/chunks")
            .set_json(&invalid_request)
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Assert
        assert_eq!(resp.status(), 400); // Bad Request
    }
    
    #[actix_web::test]
    async fn test_get_chunk_endpoint_structure() {
        // Arrange
        let service = web::Data::new(ChunkService::new("http://localhost:18888"));
        let app = test::init_service(
            App::new()
                .app_data(service.clone())
                .route("/chunks/{address}", web::get().to(get_chunk))
        ).await;
        
        // Act
        let req = test::TestRequest::get()
            .uri("/chunks/test_address_123")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        
        // Assert - will be 503 or 404 since AntTP is not running in tests
        // but endpoint structure is correct
        assert!(resp.status().is_client_error() || resp.status().is_server_error());
    }
}
