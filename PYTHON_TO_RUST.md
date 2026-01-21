# 🔄 Python → Rust Migration Guide

## Side-by-Side Comparison

This guide shows how Python FastAPI code translates to Rust Actix-Web.

---

## 1. Dependencies

### Python (`requirements.txt`)
```python
fastapi==0.104.1
uvicorn[standard]==0.24.0
httpx==0.25.1
aiohttp==3.9.1
pytest==7.4.3
pytest-asyncio==0.21.1
```

### Rust (`Cargo.toml`)
```toml
[dependencies]
actix-web = "4.5"           # Like FastAPI
tokio = { version = "1.36", features = ["full"] }  # Like asyncio
reqwest = { version = "0.11", features = ["json"] } # Like httpx
serde = { version = "1.0", features = ["derive"] }  # JSON serialization

[dev-dependencies]
tokio-test = "0.4"          # Like pytest-asyncio
```

---

## 2. Data Models

### Python (Pydantic)
```python
from pydantic import BaseModel

class ChunkRequest(BaseModel):
    content: str
    storage_type: str = "network"

class ChunkResponse(BaseModel):
    address: str
    storage_type: str
```

### Rust (Serde)
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkRequest {
    pub content: String,
    
    #[serde(default = "default_storage_type")]
    pub storage_type: String,
}

fn default_storage_type() -> String {
    "network".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkResponse {
    pub address: String,
    pub storage_type: String,
}
```

**Key Differences:**
- `class` → `struct`
- `BaseModel` → `#[derive(Serialize, Deserialize)]`
- `str` → `String`
- Default values via attribute

---

## 3. Error Handling

### Python
```python
class ChunkError(Exception):
    pass

def validate_base64(content: str) -> None:
    try:
        base64.b64decode(content)
    except Exception:
        raise ChunkError("Invalid base64")
```

### Rust
```rust
#[derive(Debug)]
pub enum ChunkError {
    InvalidBase64,
    NetworkError(String),
}

impl std::error::Error for ChunkError {}

pub fn validate_base64(&self, content: &str) -> Result<(), ChunkError> {
    base64::decode(content)
        .map(|_| ())
        .map_err(|_| ChunkError::InvalidBase64)
}
```

**Key Differences:**
- No exceptions! Use `Result<T, E>`
- Enums for error types
- Explicit error handling (no try/except)

---

## 4. Async Functions

### Python
```python
async def create_chunk(
    content: str,
    storage_type: str
) -> ChunkResponse:
    async with httpx.AsyncClient() as client:
        response = await client.post(url, json={
            "content": content,
            "storage_type": storage_type
        })
        return response.json()
```

### Rust
```rust
pub async fn create_chunk(
    &self,
    content: &str,
    storage_type: &str,
) -> Result<ChunkResponse, ChunkError> {
    let response = self.client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| ChunkError::NetworkError(e.to_string()))?;
    
    let chunk_response: ChunkResponse = response
        .json()
        .await
        .map_err(|_| ChunkError::NetworkError("Parse error".to_string()))?;
    
    Ok(chunk_response)
}
```

**Key Differences:**
- `async` keyword in same place
- `await` is `.await` (suffix, not prefix)
- Must handle errors with `?` operator
- `&self` for methods (borrowing)

---

## 5. HTTP Endpoints

### Python (FastAPI)
```python
from fastapi import FastAPI, HTTPException

app = FastAPI()

@app.post("/chunks")
async def create_chunk(request: ChunkRequest):
    try:
        result = await service.create_chunk(
            request.content,
            request.storage_type
        )
        return result
    except ChunkError as e:
        raise HTTPException(status_code=400, detail=str(e))
```

### Rust (Actix-Web)
```rust
use actix_web::{web, HttpResponse, Responder};

pub async fn create_chunk(
    chunk_req: web::Json<ChunkRequest>,
    service: web::Data<ChunkService>,
) -> impl Responder {
    match service.create_chunk(&chunk_req.content, &chunk_req.storage_type).await {
        Ok(chunk_response) => {
            HttpResponse::Ok().json(chunk_response)
        }
        Err(ChunkError::InvalidBase64) => {
            HttpResponse::BadRequest()
                .json(ErrorResponse::new("Invalid base64"))
        }
        _ => {
            HttpResponse::InternalServerError()
                .json(ErrorResponse::new("Server error"))
        }
    }
}
```

**Key Differences:**
- No decorator! Routes registered separately
- Pattern matching instead of try/except
- `web::Json<T>` for request body
- `web::Data<T>` for dependency injection

---

## 6. Application Setup

### Python
```python
from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
)

@app.get("/health")
async def health():
    return {"status": "healthy"}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
```

### Rust
```rust
use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method();
        
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .route("/health", web::get().to(health_check))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "healthy".to_string(),
    })
}
```

**Key Differences:**
- `#[actix_web::main]` instead of `if __name__ == "__main__"`
- Routes registered with `.route()`
- Middleware via `.wrap()`
- Must return `Result<T, E>`

---

## 7. Testing

### Python (pytest)
```python
import pytest

@pytest.mark.asyncio
async def test_create_chunk():
    service = ChunkService("http://localhost:18888")
    content = base64.b64encode(b"Hello").decode()
    
    result = await service.create_chunk(content, "network")
    
    assert result.address is not None
    assert result.storage_type == "network"
```

### Rust (cargo test)
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_create_chunk() {
        let service = ChunkService::new("http://localhost:18888");
        let content = base64::encode("Hello");
        
        let result = service.create_chunk(&content, "network").await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.storage_type, "network");
    }
}
```

**Key Differences:**
- `#[tokio::test]` instead of `@pytest.mark.asyncio`
- `#[cfg(test)]` to conditionally compile tests
- `assert!()` macros instead of `assert`
- `unwrap()` to get value from `Result`

---

## 8. Configuration

### Python
```python
from pydantic_settings import BaseSettings

class Settings(BaseSettings):
    anttp_base_url: str = "http://localhost:18888"
    log_level: str = "info"
    
    class Config:
        env_file = ".env"

settings = Settings()
```

### Rust
```rust
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub anttp_url: String,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            anttp_url: env::var("ANTTP_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:18888".to_string()),
            log_level: env::var("LOG_LEVEL")
                .unwrap_or_else(|_| "info".to_string()),
        }
    }
}
```

**Key Differences:**
- Manual env var reading
- `unwrap_or_else()` for defaults
- No automatic `.env` loading (use `dotenv` crate)

---

## 9. Type System Comparison

| Python | Rust | Notes |
|--------|------|-------|
| `str` | `&str` or `String` | Rust has two string types |
| `int` | `i32`, `i64`, `usize` | Rust has sized integers |
| `float` | `f32`, `f64` | Rust has sized floats |
| `bool` | `bool` | Same! |
| `List[T]` | `Vec<T>` | Rust vector |
| `Dict[K, V]` | `HashMap<K, V>` | Rust hash map |
| `Optional[T]` | `Option<T>` | Rust option type |
| `None` | `None` | Inside `Option<T>` |

---

## 10. Ownership & Borrowing (New Concept!)

Rust's unique feature - prevents memory bugs at compile time:

```rust
// Ownership: Each value has one owner
let s1 = String::from("hello");
let s2 = s1;  // s1 moved to s2, s1 no longer valid
// println!("{}", s1);  // ❌ Error! s1 was moved

// Borrowing: Multiple references allowed
let s1 = String::from("hello");
let s2 = &s1;  // Borrow s1
let s3 = &s1;  // Can have multiple borrows
println!("{} {} {}", s1, s2, s3);  // ✅ All valid!

// In functions:
fn takes_ownership(s: String) { }  // Takes ownership
fn borrows(s: &String) { }         // Just borrows

let s = String::from("hello");
borrows(&s);      // ✅ s still valid after
takes_ownership(s); // ❌ s no longer valid after
```

This is Python's biggest difference from Rust!

---

## Summary Table

| Feature | Python | Rust |
|---------|--------|------|
| Type System | Dynamic | Static |
| Memory Safety | Runtime (GC) | Compile-time (ownership) |
| Performance | Slower | Faster |
| Error Handling | Exceptions | `Result<T, E>` |
| Async Runtime | asyncio | Tokio |
| Web Framework | FastAPI | Actix-Web |
| Testing | pytest | cargo test |
| Package Manager | pip | cargo |

---

## When to Use Each?

**Use Python when:**
- Rapid prototyping
- Data science / ML
- Scripting
- Dynamic requirements

**Use Rust when:**
- Performance critical
- Memory safety critical
- System programming
- Long-running services

**For this project:**
We use Rust for better performance and type safety while maintaining the same API! 🦀
