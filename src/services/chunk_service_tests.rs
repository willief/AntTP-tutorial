// Unit tests for ChunkService
// Following TDD: Write tests FIRST, then implement

#[cfg(test)]
mod chunk_service_tests {
    use crate::services::chunk_service::{ChunkService, ChunkError};
    use crate::models::{ChunkResponse, ChunkData};
    
    #[tokio::test]
    async fn test_validate_base64_valid_content() {
        // Arrange
        let service = ChunkService::new("http://localhost:18888");
        let valid_base64 = base64::encode("Hello, Autonomi!");
        
        // Act
        let result = service.validate_base64(&valid_base64);
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_validate_base64_invalid_content() {
        // Arrange
        let service = ChunkService::new("http://localhost:18888");
        let invalid_base64 = "Not Valid Base64!!!";
        
        // Act
        let result = service.validate_base64(invalid_base64);
        
        // Assert
        assert!(result.is_err());
        match result {
            Err(ChunkError::InvalidBase64) => (),
            _ => panic!("Expected InvalidBase64 error"),
        }
    }
    
    #[tokio::test]
    async fn test_decode_content_success() {
        // Arrange
        let service = ChunkService::new("http://localhost:18888");
        let original_text = "Hello, Autonomi!";
        let encoded = base64::encode(original_text);
        
        // Act
        let result = service.decode_content(&encoded);
        
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), original_text);
    }
    
    #[tokio::test]
    async fn test_storage_type_validation_network() {
        // Arrange
        let service = ChunkService::new("http://localhost:18888");
        
        // Act
        let result = service.validate_storage_type("network");
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_storage_type_validation_invalid() {
        // Arrange
        let service = ChunkService::new("http://localhost:18888");
        
        // Act
        let result = service.validate_storage_type("invalid_type");
        
        // Assert
        assert!(result.is_err());
        match result {
            Err(ChunkError::InvalidStorageType) => (),
            _ => panic!("Expected InvalidStorageType error"),
        }
    }
}

#[cfg(test)]
mod chunk_service_integration_tests {
    use crate::services::chunk_service::ChunkService;
    use base64;
    
    // Note: These tests require AntTP to be running
    // They will be skipped in CI unless AntTP is available
    
    #[tokio::test]
    #[ignore] // Run with: cargo test -- --ignored
    async fn test_create_chunk_network_storage() {
        // Arrange
        let service = ChunkService::new("http://localhost:18888");
        let content = base64::encode("Test chunk content");
        
        // Act
        let result = service.create_chunk(&content, "network").await;
        
        // Assert
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.address.is_empty());
        assert_eq!(response.storage_type, "network");
    }
    
    #[tokio::test]
    #[ignore]
    async fn test_get_chunk_success() {
        // Arrange
        let service = ChunkService::new("http://localhost:18888");
        let content = base64::encode("Test chunk for retrieval");
        
        // First create a chunk
        let create_result = service.create_chunk(&content, "network").await;
        assert!(create_result.is_ok());
        let chunk_address = create_result.unwrap().address;
        
        // Act - retrieve the chunk
        let result = service.get_chunk(&chunk_address).await;
        
        // Assert
        assert!(result.is_ok());
        let chunk_data = result.unwrap();
        assert_eq!(chunk_data.content, content);
    }
}
