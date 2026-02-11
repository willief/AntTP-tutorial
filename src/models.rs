// src/models.rs
//! Data models matching AntTP API exactly
//! 
//! For 1st Year CS Students:
//! These are the "shapes" of data we send and receive.
//! Think of them like forms - each field is a blank to fill in!

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// CHUNK - Immutable data storage
// ============================================================================

/// Request to create a chunk (JSON format)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkRequest {
    /// Base64-encoded content
    pub content: String,
}

/// Response after creating a chunk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkResponse {
    /// Network address where chunk is stored
    pub address: String,
}

/// Chunk data for retrieval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkData {
    pub content: String,
}

// ============================================================================
// REGISTER - Mutable key-value with history
// ============================================================================

/// Request to create a register
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    /// Name of the register
    pub name: String,
    /// Hex-encoded content (NOT base64!)
    pub content: String,
}

/// Response after creating a register
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub address: String,
}

/// Register data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterData {
    pub content: String,
}

/// Register history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterHistoryEntry {
    pub content: String,
    pub timestamp: i64,
}

// ============================================================================
// POINTER - Mutable address reference
// ============================================================================

/// Request to create a pointer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointerRequest {
    /// Name of the pointer
    pub name: String,
    /// Target address it points to
    pub content: String,
}

/// Response after creating a pointer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointerResponse {
    pub address: String,
}

/// Pointer data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointerData {
    /// The address this pointer points to
    pub content: String,
}

// ============================================================================
// SCRATCHPAD - Public and Private mutable data
// ============================================================================

/// Request to create a scratchpad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScratchpadRequest {
    /// Name (required for private scratchpads)
    pub name: String,
    /// Base64-encoded content
    pub content: String,
}

/// Request to update a scratchpad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScratchpadUpdateRequest {
    /// Base64-encoded content
    pub content: String,
}

/// Response after creating a scratchpad
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScratchpadResponse {
    pub address: String,
}

/// Scratchpad data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScratchpadData {
    pub content: String,
}

// ============================================================================
// ARCHIVE - File collections (multipart upload)
// ============================================================================

/// Response after creating an archive
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveResponse {
    /// ONLY field from network - matches AntTP exactly
    pub address: String,
}

/// Archive file for internal use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveFile {
    pub path: String,
    #[serde(with = "base64_bytes")]
    pub content: Vec<u8>,
}

// Helper module for base64 serialization of bytes
mod base64_bytes {
    use base64::{engine::general_purpose, Engine as _};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&general_purpose::STANDARD.encode(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        general_purpose::STANDARD
            .decode(s)
            .map_err(serde::de::Error::custom)
    }
}

// ============================================================================
// GRAPH - Graph data structure
// ============================================================================

/// Request to create a graph entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEntryRequest {
    pub name: String,
    /// Hex-encoded content
    pub content: String,
}

// ============================================================================
// PNR - Pointer Name Registry
// ============================================================================

/// DNS-like record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PnrRecord {
    pub address: String,
    pub record_type: String,
    pub ttl: u32,
}

/// Request to create/update PNR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PnrRequest {
    pub name: String,
    pub records: HashMap<String, PnrRecord>,
}

// ============================================================================
// KEY/VALUE - Object storage
// ============================================================================

/// Request to create key/value pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValueRequest {
    pub bucket: String,
    pub object: String,
    /// Base64-encoded content
    pub content: String,
}

/// Key/value data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValueData {
    pub content: String,
}

// ============================================================================
// ERROR - Error responses
// ============================================================================

/// Standard error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<String>,
}

impl ErrorResponse {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            details: None,
        }
    }

    pub fn with_details(error: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            details: Some(details.into()),
        }
    }
}

// ============================================================================
// STORAGE TYPE - From x-store-type header
// ============================================================================

/// Storage backend type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreType {
    /// RAM storage (fast, temporary)
    Memory,
    /// Disk storage (persistent)
    Disk,
    /// Network storage (permanent, distributed)
    Network,
}

impl Default for StoreType {
    fn default() -> Self {
        Self::Memory
    }
}

impl std::str::FromStr for StoreType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "memory" => Ok(Self::Memory),
            "disk" => Ok(Self::Disk),
            "network" => Ok(Self::Network),
            _ => Err(anyhow::anyhow!("Invalid store type: {}", s)),
        }
    }
}
