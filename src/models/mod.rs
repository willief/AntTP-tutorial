use serde::{Deserialize, Serialize};

/// Request model for creating a chunk
/// Equivalent to Python's ChunkCreate model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkRequest {
    /// Base64 encoded content to store
    pub content: String,
    
    /// Storage type: "network", "disk", or "memory"
    #[serde(default = "default_storage_type")]
    pub storage_type: String,
}

fn default_storage_type() -> String {
    "network".to_string()
}

/// Response model after creating a chunk
/// Equivalent to Python's ChunkResponse model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkResponse {
    /// The unique address (XOR address) of the chunk
    pub address: String,
    
    /// Storage type used
    pub storage_type: String,
}

/// Model for chunk data retrieval
/// Equivalent to Python's ChunkData model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkData {
    /// Base64 encoded content
    pub content: String,
}

/// Health check response
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
}

/// Error response model
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub detail: Option<String>,
}

impl ErrorResponse {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            detail: None,
        }
    }
    
    pub fn with_detail(error: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            detail: Some(detail.into()),
        }
    }
}

// ==================== SCRATCHPAD MODELS ====================

/// Request model for creating/updating a scratchpad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScratchpadRequest {
    /// Base64 encoded content
    pub content: String,
    
    /// Scratchpad type: "public" or "private"
    #[serde(default = "default_scratchpad_type")]
    pub scratchpad_type: String,
}

fn default_scratchpad_type() -> String {
    "public".to_string()
}

/// Response after creating a scratchpad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScratchpadResponse {
    /// The scratchpad address
    pub address: String,
    
    /// Type of scratchpad
    pub scratchpad_type: String,
}

/// Scratchpad data retrieval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScratchpadData {
    /// Base64 encoded content
    pub content: String,
    
    /// Counter/version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counter: Option<u64>,
}

// ==================== REGISTER MODELS ====================

/// Request to create/update a register
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    /// Base64 encoded content
    pub content: String,
    
    /// Register name/key
    pub name: String,
}

/// Register response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterResponse {
    /// Register address
    pub address: String,
    
    /// Register name
    pub name: String,
    
    /// Current version
    pub version: u64,
}

/// Register data with history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterData {
    /// Current value (base64)
    pub content: String,
    
    /// Version number
    pub version: u64,
    
    /// Full history (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<RegisterEntry>>,
}

/// Single register history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterEntry {
    pub content: String,
    pub version: u64,
    pub timestamp: Option<String>,
}

// ==================== POINTER MODELS ====================

/// Create pointer request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointerRequest {
    /// Target XOR address
    pub target: String,
    
    /// Pointer name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Pointer response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointerResponse {
    /// Pointer address
    pub address: String,
    
    /// Target address
    pub target: String,
}

/// Pointer data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointerData {
    pub target: String,
    pub counter: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_chunk_request_serialization() {
        let request = ChunkRequest {
            content: "SGVsbG8=".to_string(),
            storage_type: "network".to_string(),
        };
        
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("SGVsbG8="));
        assert!(json.contains("network"));
    }
    
    #[test]
    fn test_chunk_request_default_storage_type() {
        let json = r#"{"content":"SGVsbG8="}"#;
        let request: ChunkRequest = serde_json::from_str(json).unwrap();
        
        assert_eq!(request.storage_type, "network");
    }
    
    #[test]
    fn test_chunk_response_deserialization() {
        let json = r#"{"address":"abc123","storage_type":"network"}"#;
        let response: ChunkResponse = serde_json::from_str(json).unwrap();
        
        assert_eq!(response.address, "abc123");
        assert_eq!(response.storage_type, "network");
    }
    
    #[test]
    fn test_error_response_creation() {
        let error = ErrorResponse::new("Test error");
        assert_eq!(error.error, "Test error");
        assert!(error.detail.is_none());
        
        let error_with_detail = ErrorResponse::with_detail("Error", "Details here");
        assert_eq!(error_with_detail.error, "Error");
        assert_eq!(error_with_detail.detail.unwrap(), "Details here");
    }
}
