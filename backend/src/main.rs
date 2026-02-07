use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

// ============================================================================
// DATA STRUCTURES
// ============================================================================

// Chunks - Immutable content-addressed storage
#[derive(Debug, Serialize, Deserialize, Clone)]
struct ChunkData {
    content: String,
    address: String,
    size: usize,
}

// Files - Large data split into chunks with metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
struct FileData {
    name: String,
    data_map: String,  // Address to the datamap
    size: usize,
    content_type: String,
    created_at: String,
}

// Registers - Versioned key-value storage with CRDT conflict resolution
#[derive(Debug, Serialize, Deserialize, Clone)]
struct RegisterEntry {
    key: String,
    value: String,
    version: u32,
    merkle_reg: Vec<String>,  // Merkle tree for conflict resolution
}

// Pointers - Mutable references to other data
#[derive(Debug, Serialize, Deserialize, Clone)]
struct PointerData {
    name: String,
    target: String,      // Address being pointed to
    counter: u64,        // Version counter
    owner: String,       // Owner's public key
}

// Archives - Collections of files with metadata
#[derive(Debug, Serialize, Deserialize, Clone)]
struct ArchiveData {
    name: String,
    files: Vec<ArchiveFile>,
    metadata: HashMap<String, String>,
    address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ArchiveFile {
    path: String,
    data_map: String,
    size: usize,
}

// PNR (Personal Name Resolution) - Human-readable names
#[derive(Debug, Serialize, Deserialize, Clone)]
struct PnrEntry {
    name: String,
    target: String,      // Can point to any address type
    record_type: String, // "chunk", "file", "register", "archive", etc.
}

// ============================================================================
// REQUEST/RESPONSE TYPES
// ============================================================================

#[derive(Debug, Deserialize)]
struct ChunkRequest {
    content: String,
}

#[derive(Debug, Deserialize)]
struct FileUploadRequest {
    name: String,
    content: String,
    content_type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    key: String,
    value: String,
}

#[derive(Debug, Deserialize)]
struct PointerRequest {
    name: String,
    target: String,
}

#[derive(Debug, Deserialize)]
struct ArchiveRequest {
    name: String,
    files: Vec<ArchiveFileRequest>,
    metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct ArchiveFileRequest {
    path: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct PnrRequest {
    name: String,
    target: String,
    record_type: String,
}

// ============================================================================
// APPLICATION STATE
// ============================================================================

struct AppState {
    chunks: Mutex<Vec<ChunkData>>,
    files: Mutex<Vec<FileData>>,
    registers: Mutex<Vec<RegisterEntry>>,
    pointers: Mutex<Vec<PointerData>>,
    archives: Mutex<Vec<ArchiveData>>,
    pnr_entries: Mutex<Vec<PnrEntry>>,
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

fn generate_address(prefix: &str) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{}_{:x}", prefix, timestamp)
}

fn get_current_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    format!("{}", duration.as_secs())
}

// ============================================================================
// HEALTH CHECK
// ============================================================================

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "AntTP Tutorial Backend is running",
        "primitives": ["chunks", "files", "registers", "pointers", "archives", "pnr"]
    }))
}

// ============================================================================
// CHUNKS ENDPOINTS
// ============================================================================

async fn store_chunk(
    data: web::Data<AppState>,
    req: web::Json<ChunkRequest>,
) -> impl Responder {
    let address = generate_address("chunk");
    let size = req.content.len();
    
    let chunk = ChunkData {
        content: req.content.clone(),
        address: address.clone(),
        size,
    };
    
    data.chunks.lock().unwrap().push(chunk.clone());
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "address": address,
        "size": size,
        "message": "Chunk stored successfully"
    }))
}

async fn get_chunk(
    data: web::Data<AppState>,
    address: web::Path<String>,
) -> impl Responder {
    let chunks = data.chunks.lock().unwrap();
    
    if let Some(chunk) = chunks.iter().find(|c| c.address == *address) {
        HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "chunk": chunk
        }))
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "Chunk not found"
        }))
    }
}

async fn list_chunks(data: web::Data<AppState>) -> impl Responder {
    let chunks = data.chunks.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "count": chunks.len(),
        "chunks": chunks.clone()
    }))
}

// ============================================================================
// FILES ENDPOINTS
// ============================================================================

async fn upload_file(
    data: web::Data<AppState>,
    req: web::Json<FileUploadRequest>,
) -> impl Responder {
    // Simulate file chunking and datamap creation
    let data_map = generate_address("datamap");
    let size = req.content.len();
    
    let file = FileData {
        name: req.name.clone(),
        data_map: data_map.clone(),
        size,
        content_type: req.content_type.clone().unwrap_or_else(|| "application/octet-stream".to_string()),
        created_at: get_current_timestamp(),
    };
    
    data.files.lock().unwrap().push(file.clone());
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data_map": data_map,
        "size": size,
        "message": "File uploaded successfully"
    }))
}

async fn get_file(
    data: web::Data<AppState>,
    data_map: web::Path<String>,
) -> impl Responder {
    let files = data.files.lock().unwrap();
    
    if let Some(file) = files.iter().find(|f| f.data_map == *data_map) {
        HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "file": file
        }))
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "File not found"
        }))
    }
}

async fn list_files(data: web::Data<AppState>) -> impl Responder {
    let files = data.files.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "count": files.len(),
        "files": files.clone()
    }))
}

// ============================================================================
// REGISTERS ENDPOINTS
// ============================================================================

async fn set_register(
    data: web::Data<AppState>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    let mut registers = data.registers.lock().unwrap();
    
    if let Some(entry) = registers.iter_mut().find(|r| r.key == req.key) {
        entry.value = req.value.clone();
        entry.version += 1;
        entry.merkle_reg.push(format!("v{}:{}", entry.version, req.value));
        
        HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "register": entry.clone(),
            "message": "Register updated successfully"
        }))
    } else {
        let new_entry = RegisterEntry {
            key: req.key.clone(),
            value: req.value.clone(),
            version: 1,
            merkle_reg: vec![format!("v1:{}", req.value)],
        };
        registers.push(new_entry.clone());
        
        HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "register": new_entry,
            "message": "Register created successfully"
        }))
    }
}

async fn get_register(
    data: web::Data<AppState>,
    key: web::Path<String>,
) -> impl Responder {
    let registers = data.registers.lock().unwrap();
    
    if let Some(entry) = registers.iter().find(|r| r.key == *key) {
        HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "register": entry
        }))
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "Register not found"
        }))
    }
}

async fn list_registers(data: web::Data<AppState>) -> impl Responder {
    let registers = data.registers.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "count": registers.len(),
        "registers": registers.clone()
    }))
}

// ============================================================================
// POINTERS ENDPOINTS
// ============================================================================

async fn create_pointer(
    data: web::Data<AppState>,
    req: web::Json<PointerRequest>,
) -> impl Responder {
    let pointer = PointerData {
        name: req.name.clone(),
        target: req.target.clone(),
        counter: 1,
        owner: "demo_owner_key".to_string(),
    };
    
    data.pointers.lock().unwrap().push(pointer.clone());
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "pointer": pointer,
        "message": "Pointer created successfully"
    }))
}

async fn update_pointer(
    data: web::Data<AppState>,
    name: web::Path<String>,
    req: web::Json<PointerRequest>,
) -> impl Responder {
    let mut pointers = data.pointers.lock().unwrap();
    
    if let Some(pointer) = pointers.iter_mut().find(|p| p.name == *name) {
        pointer.target = req.target.clone();
        pointer.counter += 1;
        
        HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "pointer": pointer.clone(),
            "message": "Pointer updated successfully"
        }))
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "Pointer not found"
        }))
    }
}

async fn get_pointer(
    data: web::Data<AppState>,
    name: web::Path<String>,
) -> impl Responder {
    let pointers = data.pointers.lock().unwrap();
    
    if let Some(pointer) = pointers.iter().find(|p| p.name == *name) {
        HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "pointer": pointer
        }))
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "Pointer not found"
        }))
    }
}

async fn list_pointers(data: web::Data<AppState>) -> impl Responder {
    let pointers = data.pointers.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "count": pointers.len(),
        "pointers": pointers.clone()
    }))
}

// ============================================================================
// ARCHIVES ENDPOINTS
// ============================================================================

async fn create_archive(
    data: web::Data<AppState>,
    req: web::Json<ArchiveRequest>,
) -> impl Responder {
    let address = generate_address("archive");
    
    let files: Vec<ArchiveFile> = req.files.iter().map(|f| {
        ArchiveFile {
            path: f.path.clone(),
            data_map: generate_address("datamap"),
            size: f.content.len(),
        }
    }).collect();
    
    let archive = ArchiveData {
        name: req.name.clone(),
        files,
        metadata: req.metadata.clone().unwrap_or_default(),
        address: address.clone(),
    };
    
    data.archives.lock().unwrap().push(archive.clone());
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "archive": archive,
        "message": "Archive created successfully"
    }))
}

async fn get_archive(
    data: web::Data<AppState>,
    address: web::Path<String>,
) -> impl Responder {
    let archives = data.archives.lock().unwrap();
    
    if let Some(archive) = archives.iter().find(|a| a.address == *address) {
        HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "archive": archive
        }))
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "Archive not found"
        }))
    }
}

async fn list_archives(data: web::Data<AppState>) -> impl Responder {
    let archives = data.archives.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "count": archives.len(),
        "archives": archives.clone()
    }))
}

// ============================================================================
// PNR ENDPOINTS
// ============================================================================

async fn create_pnr(
    data: web::Data<AppState>,
    req: web::Json<PnrRequest>,
) -> impl Responder {
    let entry = PnrEntry {
        name: req.name.clone(),
        target: req.target.clone(),
        record_type: req.record_type.clone(),
    };
    
    data.pnr_entries.lock().unwrap().push(entry.clone());
    
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "pnr": entry,
        "message": "PNR entry created successfully"
    }))
}

async fn resolve_pnr(
    data: web::Data<AppState>,
    name: web::Path<String>,
) -> impl Responder {
    let entries = data.pnr_entries.lock().unwrap();
    
    if let Some(entry) = entries.iter().find(|e| e.name == *name) {
        HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "pnr": entry
        }))
    } else {
        HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "error": "PNR entry not found"
        }))
    }
}

async fn list_pnr(data: web::Data<AppState>) -> impl Responder {
    let entries = data.pnr_entries.lock().unwrap();
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "count": entries.len(),
        "entries": entries.clone()
    }))
}

// ============================================================================
// MAIN SERVER
// ============================================================================

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    let app_state = web::Data::new(AppState {
        chunks: Mutex::new(Vec::new()),
        files: Mutex::new(Vec::new()),
        registers: Mutex::new(Vec::new()),
        pointers: Mutex::new(Vec::new()),
        archives: Mutex::new(Vec::new()),
        pnr_entries: Mutex::new(Vec::new()),
    });

    let host = std::env::var("BACKEND_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("BACKEND_PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_addr = format!("{}:{}", host, port);

    println!("🚀 AntTP Tutorial Backend starting on {}", bind_addr);

    HttpServer::new(move || {
        let cors = Cors::permissive();
        
        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .route("/health", web::get().to(health_check))
            
            // Chunk routes
            .route("/api/chunks", web::post().to(store_chunk))
            .route("/api/chunks", web::get().to(list_chunks))
            .route("/api/chunks/{address}", web::get().to(get_chunk))
            
            // File routes
            .route("/api/files", web::post().to(upload_file))
            .route("/api/files", web::get().to(list_files))
            .route("/api/files/{data_map}", web::get().to(get_file))
            
            // Register routes
            .route("/api/registers", web::post().to(set_register))
            .route("/api/registers", web::get().to(list_registers))
            .route("/api/registers/{key}", web::get().to(get_register))
            
            // Pointer routes
            .route("/api/pointers", web::post().to(create_pointer))
            .route("/api/pointers", web::get().to(list_pointers))
            .route("/api/pointers/{name}", web::get().to(get_pointer))
            .route("/api/pointers/{name}", web::put().to(update_pointer))
            
            // Archive routes
            .route("/api/archives", web::post().to(create_archive))
            .route("/api/archives", web::get().to(list_archives))
            .route("/api/archives/{address}", web::get().to(get_archive))
            
            // PNR routes
            .route("/api/pnr", web::post().to(create_pnr))
            .route("/api/pnr", web::get().to(list_pnr))
            .route("/api/pnr/{name}", web::get().to(resolve_pnr))
    })
    .bind(&bind_addr)?
    .run()
    .await
}
