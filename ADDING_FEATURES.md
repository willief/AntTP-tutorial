# 🎓 Adding AntTP Features - TDD Tutorial

## Based on Your Python Tutorial Project

This guide shows how to add the remaining AntTP features from your Python tutorial (`willief/AntTP-tutorial`) to the Rust backend, following the **exact same TDD methodology** you already have working with Chunks.

---

## 📊 Feature Status

Based on analysis of your Python project at `github.com/willief/AntTP-tutorial`:

| Feature | Python Tutorial | Rust Backend | Priority |
|---------|----------------|--------------|----------|
| **Chunks** | ✅ Complete | ✅ Complete | - |
| **Scratchpads** | ✅ Complete | ❌ TODO | **HIGH** |
| **Registers** | ✅ Complete | ❌ TODO | **HIGH** |
| **Archives** | ✅ Complete | ❌ TODO | MEDIUM |
| **Graph** | ✅ Complete | ❌ TODO | MEDIUM |
| **Pointers/PNR** | ✅ Complete | ❌ TODO | MEDIUM |
| **System Ops** | ✅ Complete | ❌ TODO | LOW |

---

## 🎯 Tutorial Structure

For each feature, we'll follow the **exact same pattern** you see with Chunks:

1. **RED Phase**: Write failing tests
2. **GREEN Phase**: Implement minimal code to pass
3. **REFACTOR Phase**: Improve code quality
4. **INTEGRATE**: Add handlers and routes

This tutorial shows the complete workflow for **Scratchpads**, then provides templates for the remaining features.

---

## 📝 Feature 1: Scratchpads (Complete Tutorial)

Scratchpads are **mutable, volatile data** on Autonomi. They come in two flavors:
- **Public scratchpads**: Unencrypted, anyone can read
- **Private scratchpads**: Encrypted, only owner can read

### Step 1: Add Models (ALREADY DONE! ✅)

The models are already added to `src/models/mod.rs`:

```rust
/// Scratchpad request
pub struct ScratchpadRequest {
    pub content: String,          // Base64 encoded
    pub scratchpad_type: String,  // "public" or "private"
}

/// Scratchpad response
pub struct ScratchpadResponse {
    pub address: String,
    pub scratchpad_type: String,
}

/// Scratchpad data
pub struct ScratchpadData {
    pub content: String,
    pub counter: Option<u64>,
}
```

### Step 2: RED Phase - Write Tests First

Create `src/services/scratchpad_service_tests.rs`:

```rust
#[cfg(test)]
mod scratchpad_service_tests {
    use crate::services::scratchpad_service::{ScratchpadService, ScratchpadError};
    use crate::models::{ScratchpadResponse, ScratchpadData};
    use base64;
    
    #[tokio::test]
    async fn test_validate_scratchpad_type_public() {
        let service = ScratchpadService::new("http://localhost:18888");
        let result = service.validate_scratchpad_type("public");
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_validate_scratchpad_type_private() {
        let service = ScratchpadService::new("http://localhost:18888");
        let result = service.validate_scratchpad_type("private");
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_validate_scratchpad_type_invalid() {
        let service = ScratchpadService::new("http://localhost:18888");
        let result = service.validate_scratchpad_type("invalid");
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_validate_base64() {
        let service = ScratchpadService::new("http://localhost:18888");
        let valid = base64::encode("Hello, Scratchpad!");
        assert!(service.validate_base64(&valid).is_ok());
    }
    
    #[tokio::test]
    async fn test_validate_base64_invalid() {
        let service = ScratchpadService::new("http://localhost:18888");
        let invalid = "Not Base64!!!";
        assert!(service.validate_base64(invalid).is_err());
    }
    
    #[tokio::test]
    #[ignore] // Requires AntTP
    async fn test_create_public_scratchpad() {
        let service = ScratchpadService::new("http://localhost:18888");
        let content = base64::encode("Test scratchpad content");
        
        let result = service.create_scratchpad(&content, "public").await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.address.is_empty());
        assert_eq!(response.scratchpad_type, "public");
    }
    
    #[tokio::test]
    #[ignore] // Requires AntTP
    async fn test_update_scratchpad() {
        let service = ScratchpadService::new("http://localhost:18888");
        
        // Create first
        let content1 = base64::encode("Initial content");
        let create_result = service.create_scratchpad(&content1, "public").await;
        assert!(create_result.is_ok());
        let address = create_result.unwrap().address;
        
        // Update
        let content2 = base64::encode("Updated content");
        let update_result = service.update_scratchpad(&address, &content2).await;
        assert!(update_result.is_ok());
    }
    
    #[tokio::test]
    #[ignore] // Requires AntTP
    async fn test_get_scratchpad() {
        let service = ScratchpadService::new("http://localhost:18888");
        
        // Create
        let content = base64::encode("Retrievable content");
        let create_result = service.create_scratchpad(&content, "public").await;
        let address = create_result.unwrap().address;
        
        // Get
        let get_result = service.get_scratchpad(&address).await;
        assert!(get_result.is_ok());
        
        let data = get_result.unwrap();
        assert_eq!(data.content, content);
    }
}
```

**Run tests** (they will FAIL - that's the RED phase!):
```bash
cargo test scratchpad_service_tests
# Expected: compilation errors - ScratchpadService doesn't exist yet!
```

### Step 3: GREEN Phase - Implement Service

Create `src/services/scratchpad_service.rs`:

```rust
use crate::models::{ScratchpadRequest, ScratchpadResponse, ScratchpadData};
use base64;
use log::{info, warn, error};
use reqwest::Client;
use serde_json::json;

#[derive(Debug, PartialEq)]
pub enum ScratchpadError {
    InvalidBase64,
    InvalidType,
    NetworkError(String),
    NotFound,
    ServiceUnavailable,
}

impl std::fmt::Display for ScratchpadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ScratchpadError::InvalidBase64 => write!(f, "Invalid base64 encoding"),
            ScratchpadError::InvalidType => write!(f, "Invalid scratchpad type"),
            ScratchpadError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ScratchpadError::NotFound => write!(f, "Scratchpad not found"),
            ScratchpadError::ServiceUnavailable => write!(f, "AntTP service unavailable"),
        }
    }
}

impl std::error::Error for ScratchpadError {}

pub struct ScratchpadService {
    anttp_url: String,
    client: Client,
}

impl ScratchpadService {
    pub fn new(anttp_url: &str) -> Self {
        info!("Initializing ScratchpadService with AntTP URL: {}", anttp_url);
        Self {
            anttp_url: anttp_url.to_string(),
            client: Client::new(),
        }
    }
    
    pub fn validate_base64(&self, content: &str) -> Result<(), ScratchpadError> {
        base64::decode(content)
            .map(|_| ())
            .map_err(|_| ScratchpadError::InvalidBase64)
    }
    
    pub fn validate_scratchpad_type(&self, scratchpad_type: &str) -> Result<(), ScratchpadError> {
        match scratchpad_type {
            "public" | "private" => Ok(()),
            _ => {
                warn!("Invalid scratchpad type: {}", scratchpad_type);
                Err(ScratchpadError::InvalidType)
            }
        }
    }
    
    pub async fn create_scratchpad(
        &self,
        content: &str,
        scratchpad_type: &str,
    ) -> Result<ScratchpadResponse, ScratchpadError> {
        self.validate_base64(content)?;
        self.validate_scratchpad_type(scratchpad_type)?;
        
        info!("Creating {} scratchpad", scratchpad_type);
        
        let endpoint = match scratchpad_type {
            "public" => "scratchpad/public",
            "private" => "scratchpad/private",
            _ => return Err(ScratchpadError::InvalidType),
        };
        
        let url = format!("{}/{}", self.anttp_url, endpoint);
        let payload = json!({ "content": content });
        
        let response = self.client
            .post(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to connect to AntTP: {}", e);
                ScratchpadError::ServiceUnavailable
            })?;
        
        if !response.status().is_success() {
            error!("AntTP returned error: {}", response.status());
            return Err(ScratchpadError::NetworkError(format!("HTTP {}", response.status())));
        }
        
        let scratchpad_response: ScratchpadResponse = response
            .json()
            .await
            .map_err(|e| {
                error!("Failed to parse response: {}", e);
                ScratchpadError::NetworkError("Invalid response".to_string())
            })?;
        
        info!("Scratchpad created: {}", scratchpad_response.address);
        Ok(scratchpad_response)
    }
    
    pub async fn update_scratchpad(
        &self,
        address: &str,
        content: &str,
    ) -> Result<(), ScratchpadError> {
        self.validate_base64(content)?;
        
        info!("Updating scratchpad: {}", address);
        
        let url = format!("{}/scratchpad/{}", self.anttp_url, address);
        let payload = json!({ "content": content });
        
        let response = self.client
            .put(&url)
            .json(&payload)
            .send()
            .await
            .map_err(|_| ScratchpadError::ServiceUnavailable)?;
        
        if !response.status().is_success() {
            return Err(ScratchpadError::NetworkError("Update failed".to_string()));
        }
        
        info!("Scratchpad updated successfully");
        Ok(())
    }
    
    pub async fn get_scratchpad(&self, address: &str) -> Result<ScratchpadData, ScratchpadError> {
        info!("Retrieving scratchpad: {}", address);
        
        let url = format!("{}/scratchpad/{}", self.anttp_url, address);
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|_| ScratchpadError::ServiceUnavailable)?;
        
        if !response.status().is_success() {
            return match response.status().as_u16() {
                404 => Err(ScratchpadError::NotFound),
                _ => Err(ScratchpadError::NetworkError("Retrieval failed".to_string())),
            };
        }
        
        let data: ScratchpadData = response
            .json()
            .await
            .map_err(|_| ScratchpadError::NetworkError("Parse error".to_string()))?;
        
        info!("Scratchpad retrieved successfully");
        Ok(data)
    }
}

#[cfg(test)]
#[path = "scratchpad_service_tests.rs"]
mod scratchpad_service_tests;
```

**Update `src/services/mod.rs`**:
```rust
pub mod chunk_service;
pub mod scratchpad_service;  // ADD THIS LINE
```

**Run tests again**:
```bash
cargo test scratchpad_service_tests
# Expected: Unit tests PASS ✅, integration tests need AntTP
```

### Step 4: Add HTTP Handlers

Create `src/handlers/scratchpad_handler.rs`:

```rust
use actix_web::{web, HttpResponse, Responder};
use log::{info, error};

use crate::models::{ScratchpadRequest, ErrorResponse};
use crate::services::scratchpad_service::{ScratchpadService, ScratchpadError};

pub async fn create_scratchpad(
    req: web::Json<ScratchpadRequest>,
    service: web::Data<ScratchpadService>,
) -> impl Responder {
    info!("Received create scratchpad request");
    
    match service.create_scratchpad(&req.content, &req.scratchpad_type).await {
        Ok(response) => {
            info!("Scratchpad created: {}", response.address);
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            error!("Failed to create scratchpad: {}", e);
            match e {
                ScratchpadError::InvalidBase64 => {
                    HttpResponse::BadRequest()
                        .json(ErrorResponse::new("Invalid base64 encoding"))
                }
                ScratchpadError::InvalidType => {
                    HttpResponse::BadRequest()
                        .json(ErrorResponse::new("Invalid scratchpad type"))
                }
                ScratchpadError::ServiceUnavailable => {
                    HttpResponse::ServiceUnavailable()
                        .json(ErrorResponse::new("AntTP unavailable"))
                }
                _ => HttpResponse::InternalServerError()
                    .json(ErrorResponse::new("Internal error")),
            }
        }
    }
}

pub async fn update_scratchpad(
    address: web::Path<String>,
    req: web::Json<ScratchpadRequest>,
    service: web::Data<ScratchpadService>,
) -> impl Responder {
    info!("Received update scratchpad request");
    
    match service.update_scratchpad(&address, &req.content).await {
        Ok(_) => {
            info!("Scratchpad updated successfully");
            HttpResponse::Ok().json(serde_json::json!({ "status": "updated" }))
        }
        Err(e) => {
            error!("Failed to update: {}", e);
            HttpResponse::BadRequest().json(ErrorResponse::new(e.to_string()))
        }
    }
}

pub async fn get_scratchpad(
    address: web::Path<String>,
    service: web::Data<ScratchpadService>,
) -> impl Responder {
    info!("Received get scratchpad request");
    
    match service.get_scratchpad(&address).await {
        Ok(data) => {
            info!("Scratchpad retrieved");
            HttpResponse::Ok().json(data)
        }
        Err(ScratchpadError::NotFound) => {
            HttpResponse::NotFound()
                .json(ErrorResponse::new("Scratchpad not found"))
        }
        Err(e) => {
            error!("Failed to get: {}", e);
            HttpResponse::InternalServerError()
                .json(ErrorResponse::new("Internal error"))
        }
    }
}
```

**Update `src/handlers/mod.rs`**:
```rust
pub mod chunk_handler;
pub mod scratchpad_handler;  // ADD THIS
```

### Step 5: Register Routes

Update `src/main.rs`:

```rust
use services::scratchpad_service::ScratchpadService;  // Add import
use handlers::scratchpad_handler;  // Add import

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ... existing setup ...
    
    let chunk_service = web::Data::new(ChunkService::new(&config.anttp_url));
    let scratchpad_service = web::Data::new(ScratchpadService::new(&config.anttp_url));  // ADD THIS
    
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(chunk_service.clone())
            .app_data(scratchpad_service.clone())  // ADD THIS
            
            // Chunk routes
            .route("/chunks", web::post().to(chunk_handler::create_chunk))
            .route("/chunks/{address}", web::get().to(chunk_handler::get_chunk))
            
            // Scratchpad routes - ADD THESE
            .route("/scratchpads", web::post().to(scratchpad_handler::create_scratchpad))
            .route("/scratchpads/{address}", web::put().to(scratchpad_handler::update_scratchpad))
            .route("/scratchpads/{address}", web::get().to(scratchpad_handler::get_scratchpad))
            
            .route("/health", web::get().to(health_check))
    })
    .bind(&bind_address)?
    .run()
    .await
}
```

### Step 6: Test Full Integration

```bash
# Start services
./start.sh

# Test create public scratchpad
curl -X POST http://localhost:8000/scratchpads \
  -H "Content-Type: application/json" \
  -d '{
    "content": "SGVsbG8gU2NyYXRjaHBhZCE=",
    "scratchpad_type": "public"
  }'

# Response: {"address":"abc123...","scratchpad_type":"public"}

# Test update
curl -X PUT http://localhost:8000/scratchpads/abc123 \
  -H "Content-Type: application/json" \
  -d '{
    "content": "VXBkYXRlZCBjb250ZW50"
  }'

# Test get
curl http://localhost:8000/scratchpads/abc123
```

**Scratchpads are now complete!** ✅

---

## 🎯 Remaining Features - Quick Templates

Now that you've seen the complete Scratchpad workflow, here are templates for the remaining features:

### Feature 2: Registers (Versioned Mutable Data)

**Models**: Already added! See `src/models/mod.rs`

**Service Template** (`src/services/register_service.rs`):
```rust
// Similar to ScratchpadService but with:
// - create_register(name, content)
// - update_register(address, content) 
// - get_register(address) -> RegisterData with version
// - get_register_history(address) -> Vec<RegisterEntry>
```

**Endpoints**:
- `POST /registers` - Create
- `PUT /registers/{address}` - Update
- `GET /registers/{address}` - Get current
- `GET /registers/{address}/history` - Get history

### Feature 3: Pointers (Mutable References)

**Models**: Already added! See `src/models/mod.rs`

**Service Template** (`src/services/pointer_service.rs`):
```rust
// - create_pointer(target) -> PointerResponse
// - update_pointer(address, new_target)
// - get_pointer(address) -> PointerData
// - resolve_pointer(address) -> final target (follow chain)
```

**Endpoints**:
- `POST /pointers` - Create
- `PUT /pointers/{address}` - Update target
- `GET /pointers/{address}` - Get pointer
- `GET /pointers/{address}/resolve` - Resolve full chain

### Feature 4: Archives (File Collections)

**Models Needed**:
```rust
pub struct ArchiveRequest {
    pub files: Vec<ArchiveFile>,
}

pub struct ArchiveFile {
    pub name: String,
    pub content: String,  // base64
}

pub struct ArchiveResponse {
    pub address: String,
    pub file_count: usize,
}
```

**Service**: Handle multi-file uploads as archives

### Feature 5: Graph (Structured Data)

**Models Needed**:
```rust
pub struct GraphEntryRequest {
    pub data: String,  // base64 JSON
    pub links: Vec<String>,  // XOR addresses
}

pub struct GraphEntryResponse {
    pub address: String,
}
```

---

## 📚 Summary - What You Learned

By following this tutorial, you now know how to:

1. ✅ **Follow TDD strictly** - Tests always first
2. ✅ **Structure services** - Error types, validation, API calls
3. ✅ **Create handlers** - HTTP layer with proper error handling
4. ✅ **Register routes** - Wire everything in main.rs
5. ✅ **Test integration** - Full end-to-end with cURL

The pattern is **identical for every feature**:
- RED: Write tests
- GREEN: Implement service
- REFACTOR: Clean up
- INTEGRATE: Add handlers & routes

---

## 🚀 Next Steps

1. **Implement Scratchpads** (shown above)
2. **Use same pattern for Registers**
3. **Continue with Pointers**
4. **Add Archives**
5. **Finish with Graph**

Each feature takes ~1-2 hours following this pattern!

---

## 💡 Pro Tips

1. **Always write tests first** - Even if tempted to code first
2. **Run tests frequently** - cargo watch -x test
3. **Keep services simple** - One feature per service file
4. **Handlers are thin** - Just translate HTTP ↔ Service
5. **Error handling is key** - Make errors descriptive

---

## 📖 Resources

- Your Python tutorial: `github.com/willief/AntTP-tutorial`
- AntTP API docs: `http://localhost:18888/swagger-ui/`
- This Rust project: Complete Chunks implementation as reference

**Happy coding!** 🦀

*Built with Test-Driven Development*
