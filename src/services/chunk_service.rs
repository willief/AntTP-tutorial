use crate::models::{ChunkRequest, ChunkResponse, ChunkData};
use base64;
use log::{info, warn, error};
use reqwest::Client;
use serde_json::json;

/// Custom error types for chunk operations
#[derive(Debug, PartialEq)]
pub enum ChunkError {
    InvalidBase64,
    InvalidStorageType,
    NetworkError(String),
    NotFound,
    ServiceUnavailable,
}

impl std::fmt::Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ChunkError::InvalidBase64 => write!(f, "Invalid base64 encoding"),
            ChunkError::InvalidStorageType => write!(f, "Invalid storage type"),
            ChunkError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ChunkError::NotFound => write!(f, "Chunk not found"),
            ChunkError::ServiceUnavailable => write!(f, "AntTP service unavailable"),
        }
    }
}

impl std::error::Error for ChunkError {}

/// Service for chunk operations
/// Handles business logic and communication with AntTP
pub struct ChunkService {
    anttp_url: String,
    client: Client,
}

impl ChunkService {
    /// Create a new ChunkService instance
    pub fn new(anttp_url: &str) -> Self {
        info!("Initializing ChunkService with AntTP URL: {}", anttp_url);
        Self {
            anttp_url: anttp_url.to_string(),
            client: Client::new(),
        }
    }
    
    /// Validate base64 encoding
    pub fn validate_base64(&self, content: &str) -> Result<(), ChunkError> {
        base64::decode(content)
            .map(|_| ())
            .map_err(|_| ChunkError::InvalidBase64)
    }
    
    /// Decode base64 content to string
    pub fn decode_content(&self, content: &str) -> Result<String, ChunkError> {
        let bytes = base64::decode(content)
            .map_err(|_| ChunkError::InvalidBase64)?;
        
        String::from_utf8(bytes)
            .map_err(|_| ChunkError::InvalidBase64)
    }
    
    /// Validate storage type
    pub fn validate_storage_type(&self, storage_type: &str) -> Result<(), ChunkError> {
        match storage_type {
            "network" | "disk" | "memory" => Ok(()),
            _ => {
                warn!("Invalid storage type: {}", storage_type);
                Err(ChunkError::InvalidStorageType)
            }
        }
    }
    
    /// Create a chunk on the Autonomi network via AntTP
    /// 
    /// # Arguments
    /// * `content` - Base64 encoded content
    /// * `storage_type` - Storage type (network/disk/memory)
    /// 
    /// # Returns
    /// * `Ok(ChunkResponse)` - Success with chunk address
    /// * `Err(ChunkError)` - Validation or network error
    pub async fn create_chunk(
        &self,
        content: &str,
        storage_type: &str,
    ) -> Result<ChunkResponse, ChunkError> {
        // Validate inputs
        self.validate_base64(content)?;
        self.validate_storage_type(storage_type)?;
        
        info!("Creating chunk with storage_type: {}", storage_type);
        
        // Prepare request
        let url = format!("{}/chunk", self.anttp_url);
        let payload = json!({
            "content": content,
            "storage_type": storage_type
        });
        
        // Make request to AntTP
        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to connect to AntTP: {}", e);
                ChunkError::ServiceUnavailable
            })?;
        
        // Check response status
        let status = response.status();
        if !status.is_success() {
            error!("AntTP returned error status: {}", status);
            
            return match status.as_u16() {
                404 => Err(ChunkError::NotFound),
                503 => Err(ChunkError::ServiceUnavailable),
                _ => Err(ChunkError::NetworkError(format!("HTTP {}", status))),
            };
        }
        
        // Parse response
        let chunk_response: ChunkResponse = response
            .json()
            .await
            .map_err(|e| {
                error!("Failed to parse AntTP response: {}", e);
                ChunkError::NetworkError("Invalid response format".to_string())
            })?;
        
        info!("Chunk created successfully: {}", chunk_response.address);
        Ok(chunk_response)
    }
    
    /// Retrieve a chunk from the Autonomi network via AntTP
    /// 
    /// # Arguments
    /// * `address` - The chunk's XOR address
    /// 
    /// # Returns
    /// * `Ok(ChunkData)` - Success with chunk content
    /// * `Err(ChunkError)` - Network error or not found
    pub async fn get_chunk(&self, address: &str) -> Result<ChunkData, ChunkError> {
        info!("Retrieving chunk: {}", address);
        
        let url = format!("{}/chunk/{}", self.anttp_url, address);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to connect to AntTP: {}", e);
                ChunkError::ServiceUnavailable
            })?;
        
        let status = response.status();
        if !status.is_success() {
            return match status.as_u16() {
                404 => {
                    warn!("Chunk not found: {}", address);
                    Err(ChunkError::NotFound)
                }
                503 => Err(ChunkError::ServiceUnavailable),
                _ => Err(ChunkError::NetworkError(format!("HTTP {}", status))),
            };
        }
        
        let chunk_data: ChunkData = response
            .json()
            .await
            .map_err(|e| {
                error!("Failed to parse chunk data: {}", e);
                ChunkError::NetworkError("Invalid response format".to_string())
            })?;
        
        info!("Chunk retrieved successfully");
        Ok(chunk_data)
    }
}

#[cfg(test)]
#[path = "chunk_service_tests.rs"]
mod chunk_service_tests;
