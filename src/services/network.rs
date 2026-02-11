// src/services/network.rs
//! Network service - Memory storage with optional Autonomi integration
//!
//! For 1st Year CS Students:
//! This is where we store data. For now, it uses memory (RAM).
//! Later, we can add real network storage!

use anyhow::Result;
use bytes::Bytes;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Network client wrapper
pub struct NetworkService {
    /// Local cache for memory storage
    memory_cache: Arc<RwLock<HashMap<String, Bytes>>>,
}

impl NetworkService {
    /// Create a new network service
    ///
    /// For Students:
    /// This sets up our storage system (memory-based for now)
    pub async fn new() -> Result<Self> {
        log::info!("🔌 Initializing storage service...");
        
        #[cfg(feature = "network")]
        {
            log::info!("✅ Network feature enabled - Autonomi SDK available");
            // TODO: Initialize real network client when feature is enabled
        }
        
        #[cfg(not(feature = "network"))]
        {
            log::info!("💾 Memory-only mode (compile with --features network for real Autonomi)");
        }

        Ok(Self {
            memory_cache: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Store a chunk on the network
    ///
    /// For Students:
    /// Chunks are immutable - once stored, they never change!
    /// Like writing in permanent marker vs pencil
    pub async fn store_chunk(&self, data: Bytes, use_network: bool) -> Result<String> {
        if use_network {
            log::warn!("⚠️  Network storage requested but not available (compile with --features network)");
            log::info!("💾 Falling back to memory storage");
            // Fall through to memory storage
        }
        
        // Memory storage (for testing and fallback)
        log::info!("💾 Storing {} bytes as chunk in memory", data.len());
        let hash = sha256_hash(&data);
        let hex_address = hex::encode(&hash);

        self.memory_cache
            .write()
            .await
            .insert(hex_address.clone(), data);

        log::info!("💾 Chunk cached in memory: {}", hex_address);
        Ok(hex_address)
    }

    /// Retrieve a chunk from the network
    ///
    /// For Students:
    /// We use the address (like a receipt number) to get our data back
    pub async fn get_chunk(&self, address: &str, use_network: bool) -> Result<Bytes> {
        if use_network {
            log::warn!("⚠️  Network retrieval requested but not available (compile with --features network)");
            log::info!("💾 Falling back to memory retrieval");
            // Fall through to memory retrieval
        }
        
        // Memory retrieval
        log::info!("💾 Fetching chunk from memory: {}", address);
        let cache = self.memory_cache.read().await;
        cache
            .get(address)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Chunk not found in memory: {}", address))
    }

    /// Store an archive (directory of files)
    ///
    /// For Students:
    /// Archives are like ZIP files - multiple files stored together!
    pub async fn store_archive(
        &self,
        files: Vec<(PathBuf, Bytes)>,
        use_network: bool,
    ) -> Result<String> {
        if use_network {
            log::warn!("⚠️  Network storage requested but not available (compile with --features network)");
            log::info!("💾 Falling back to memory storage");
            // Fall through to memory storage
        }
        
        // Memory storage - serialize files
        log::info!("💾 Storing archive with {} files in memory", files.len());
        
        // Convert Bytes to Vec<u8> for serialization
        let files_serializable: Vec<(PathBuf, Vec<u8>)> = files
            .into_iter()
            .map(|(path, bytes)| (path, bytes.to_vec()))
            .collect();
        
        let serialized = serde_json::to_vec(&files_serializable)?;
        let hash = sha256_hash(&serialized);
        let hex_address = hex::encode(&hash);

        self.memory_cache
            .write()
            .await
            .insert(hex_address.clone(), Bytes::from(serialized));

        log::info!("💾 Archive cached in memory: {}", hex_address);
        Ok(hex_address)
    }

    /// Retrieve an archive from the network
    pub async fn get_archive(
        &self,
        address: &str,
        use_network: bool,
    ) -> Result<Vec<(PathBuf, Bytes)>> {
        if use_network {
            log::warn!("⚠️  Network retrieval requested but not available (compile with --features network)");
            log::info!("💾 Falling back to memory retrieval");
            // Fall through to memory retrieval
        }
        
        // Memory retrieval
        log::info!("💾 Fetching archive from memory: {}", address);
        let cache = self.memory_cache.read().await;
        let serialized = cache
            .get(address)
            .ok_or_else(|| anyhow::anyhow!("Archive not found: {}", address))?;

        let files_serializable: Vec<(PathBuf, Vec<u8>)> = serde_json::from_slice(serialized)?;
        
        // Convert Vec<u8> back to Bytes
        let files: Vec<(PathBuf, Bytes)> = files_serializable
            .into_iter()
            .map(|(path, vec)| (path, Bytes::from(vec)))
            .collect();
        
        Ok(files)
    }

    // ========================================================================
    // REGISTERS - Mutable key-value with history
    // ========================================================================

    pub async fn store_register(
        &self,
        name: &str,
        content: &str,
        use_network: bool,
    ) -> Result<String> {
        if use_network {
            log::info!("📝 Storing register '{}' on network", name);
            // TODO: Real network implementation
            // For now, fallback to memory
            let data = format!("{}:{}", name, content);
            self.store_chunk(Bytes::from(data), false).await
        } else {
            let data = serde_json::json!({
                "name": name,
                "content": content,
                "timestamp": chrono::Utc::now().timestamp()
            });
            let serialized = serde_json::to_vec(&data)?;
            let hash = sha256_hash(&serialized);
            let hex_address = hex::encode(&hash);

            let mut cache = self.memory_cache.write().await;
            cache.insert(hex_address.clone(), Bytes::from(serialized));
            cache.insert(
                format!("{}_history", hex_address),
                Bytes::from(serde_json::to_vec(&vec![data])?),
            );

            Ok(hex_address)
        }
    }

    pub async fn update_register(
        &self,
        address: &str,
        name: &str,
        content: &str,
        use_network: bool,
    ) -> Result<()> {
        if !use_network {
            let data = serde_json::json!({
                "name": name,
                "content": content,
                "timestamp": chrono::Utc::now().timestamp()
            });

            let mut cache = self.memory_cache.write().await;

            // Update current value
            cache.insert(address.to_string(), Bytes::from(serde_json::to_vec(&data)?));

            // Update history
            let history_key = format!("{}_history", address);
            let mut history: Vec<serde_json::Value> = if let Some(hist) = cache.get(&history_key) {
                serde_json::from_slice(hist)?
            } else {
                Vec::new()
            };
            history.push(data);
            cache.insert(history_key, Bytes::from(serde_json::to_vec(&history)?));
        }
        Ok(())
    }

    pub async fn get_register(&self, address: &str, use_network: bool) -> Result<String> {
        if !use_network {
            let cache = self.memory_cache.read().await;
            if let Some(data) = cache.get(address) {
                let value: serde_json::Value = serde_json::from_slice(data)?;
                Ok(value["content"]
                    .as_str()
                    .unwrap_or("")
                    .to_string())
            } else {
                Err(anyhow::anyhow!("Register not found"))
            }
        } else {
            Err(anyhow::anyhow!("Network registers not yet implemented"))
        }
    }

    pub async fn get_register_history(
        &self,
        address: &str,
        use_network: bool,
    ) -> Result<Vec<crate::models::RegisterHistoryEntry>> {
        if !use_network {
            let cache = self.memory_cache.read().await;
            let history_key = format!("{}_history", address);
            if let Some(hist) = cache.get(&history_key) {
                let history: Vec<serde_json::Value> = serde_json::from_slice(hist)?;
                let entries = history
                    .into_iter()
                    .map(|v| crate::models::RegisterHistoryEntry {
                        content: v["content"].as_str().unwrap_or("").to_string(),
                        timestamp: v["timestamp"].as_i64().unwrap_or(0),
                    })
                    .collect();
                Ok(entries)
            } else {
                Err(anyhow::anyhow!("Register not found"))
            }
        } else {
            Err(anyhow::anyhow!("Network registers not yet implemented"))
        }
    }

    // ========================================================================
    // POINTERS - Mutable references
    // ========================================================================

    pub async fn store_pointer(
        &self,
        name: &str,
        target: &str,
        use_network: bool,
    ) -> Result<String> {
        let data = serde_json::json!({
            "name": name,
            "target": target
        });
        let serialized = serde_json::to_vec(&data)?;

        if use_network {
            // TODO: Real network implementation
            self.store_chunk(Bytes::from(serialized), false).await
        } else {
            let hash = sha256_hash(&serialized);
            let hex_address = hex::encode(&hash);
            self.memory_cache
                .write()
                .await
                .insert(hex_address.clone(), Bytes::from(serialized));
            Ok(hex_address)
        }
    }

    pub async fn update_pointer(
        &self,
        address: &str,
        name: &str,
        target: &str,
        use_network: bool,
    ) -> Result<()> {
        if !use_network {
            let data = serde_json::json!({
                "name": name,
                "target": target
            });
            self.memory_cache
                .write()
                .await
                .insert(address.to_string(), Bytes::from(serde_json::to_vec(&data)?));
        }
        Ok(())
    }

    pub async fn get_pointer(&self, address: &str, use_network: bool) -> Result<String> {
        if !use_network {
            let cache = self.memory_cache.read().await;
            if let Some(data) = cache.get(address) {
                let value: serde_json::Value = serde_json::from_slice(data)?;
                Ok(value["target"].as_str().unwrap_or("").to_string())
            } else {
                Err(anyhow::anyhow!("Pointer not found"))
            }
        } else {
            Err(anyhow::anyhow!("Network pointers not yet implemented"))
        }
    }

    // ========================================================================
    // SCRATCHPADS - Public and Private mutable data
    // ========================================================================

    pub async fn store_public_scratchpad(
        &self,
        name: &str,
        content: &str,
        use_network: bool,
    ) -> Result<String> {
        let data = serde_json::json!({
            "name": name,
            "content": content,
            "type": "public"
        });
        let serialized = serde_json::to_vec(&data)?;

        if use_network {
            self.store_chunk(Bytes::from(serialized), false).await
        } else {
            let hash = sha256_hash(&serialized);
            let hex_address = hex::encode(&hash);
            self.memory_cache
                .write()
                .await
                .insert(hex_address.clone(), Bytes::from(serialized));
            Ok(hex_address)
        }
    }

    pub async fn update_public_scratchpad(
        &self,
        address: &str,
        name: &str,
        content: &str,
        use_network: bool,
    ) -> Result<()> {
        if !use_network {
            let data = serde_json::json!({
                "name": name,
                "content": content,
                "type": "public"
            });
            self.memory_cache
                .write()
                .await
                .insert(address.to_string(), Bytes::from(serde_json::to_vec(&data)?));
        }
        Ok(())
    }

    pub async fn get_public_scratchpad(
        &self,
        address: &str,
        use_network: bool,
    ) -> Result<String> {
        if !use_network {
            let cache = self.memory_cache.read().await;
            if let Some(data) = cache.get(address) {
                let value: serde_json::Value = serde_json::from_slice(data)?;
                Ok(value["content"].as_str().unwrap_or("").to_string())
            } else {
                Err(anyhow::anyhow!("Scratchpad not found"))
            }
        } else {
            Err(anyhow::anyhow!("Network scratchpads not yet implemented"))
        }
    }

    pub async fn store_private_scratchpad(
        &self,
        name: &str,
        content: &str,
        use_network: bool,
    ) -> Result<String> {
        // Private scratchpads are stored with name as part of the key
        let data = serde_json::json!({
            "name": name,
            "content": content,
            "type": "private"
        });
        let serialized = serde_json::to_vec(&data)?;

        if use_network {
            self.store_chunk(Bytes::from(serialized), false).await
        } else {
            let hash = sha256_hash(&serialized);
            let hex_address = hex::encode(&hash);
            self.memory_cache
                .write()
                .await
                .insert(hex_address.clone(), Bytes::from(serialized));
            Ok(hex_address)
        }
    }

    pub async fn update_private_scratchpad(
        &self,
        address: &str,
        name: &str,
        content: &str,
        use_network: bool,
    ) -> Result<()> {
        if !use_network {
            let data = serde_json::json!({
                "name": name,
                "content": content,
                "type": "private"
            });
            self.memory_cache
                .write()
                .await
                .insert(format!("{}:{}", address, name), Bytes::from(serde_json::to_vec(&data)?));
        }
        Ok(())
    }

    pub async fn get_private_scratchpad(
        &self,
        address: &str,
        name: &str,
        use_network: bool,
    ) -> Result<String> {
        if !use_network {
            let cache = self.memory_cache.read().await;
            let key = format!("{}:{}", address, name);
            if let Some(data) = cache.get(&key) {
                let value: serde_json::Value = serde_json::from_slice(data)?;
                Ok(value["content"].as_str().unwrap_or("").to_string())
            } else {
                Err(anyhow::anyhow!("Private scratchpad not found"))
            }
        } else {
            Err(anyhow::anyhow!("Network scratchpads not yet implemented"))
        }
    }

    // ========================================================================
    // TARCHIVE - Tar-based archives
    // ========================================================================

    pub async fn store_tarchive(
        &self,
        files: Vec<(PathBuf, Bytes)>,
        use_network: bool,
    ) -> Result<String> {
        if use_network {
            log::warn!("⚠️  Network storage requested but not available (compile with --features network)");
            log::info!("💾 Falling back to memory storage");
            // Don't recurse - just fall through to memory storage below
        }
        
        log::info!("💾 Storing tarchive in memory ({} files)", files.len());

        // Serialize files to JSON (in real TAR would be .tar format)
        let archive_data = serde_json::json!({
            "type": "tarchive",
            "files": files.iter().map(|(path, content)| {
                serde_json::json!({
                    "path": path.to_string_lossy(),
                    "content": content.to_vec()
                })
            }).collect::<Vec<_>>()
        });

        let serialized = serde_json::to_vec(&archive_data)?;
        let hash = sha256_hash(&serialized);
        let hex_address = hex::encode(&hash);

        self.memory_cache
            .write()
            .await
            .insert(hex_address.clone(), Bytes::from(serialized));

        Ok(hex_address)
    }

    // ========================================================================
    // GRAPH - Graph data structures
    // ========================================================================

    pub async fn store_graph_entry(
        &self,
        name: &str,
        content: &str,
        use_network: bool,
    ) -> Result<String> {
        let data = serde_json::json!({
            "name": name,
            "content": content,
            "type": "graph_entry"
        });
        let serialized = serde_json::to_vec(&data)?;

        if use_network {
            log::info!("🕸️ Storing graph entry on network");
            // TODO: Real network implementation
            self.store_chunk(Bytes::from(serialized), false).await
        } else {
            let hash = sha256_hash(&serialized);
            let hex_address = hex::encode(&hash);
            self.memory_cache
                .write()
                .await
                .insert(hex_address.clone(), Bytes::from(serialized));
            Ok(hex_address)
        }
    }

    pub async fn get_graph_entry(
        &self,
        address: &str,
        use_network: bool,
    ) -> Result<(String, String)> {
        if !use_network {
            let cache = self.memory_cache.read().await;
            if let Some(data) = cache.get(address) {
                let value: serde_json::Value = serde_json::from_slice(data)?;
                let name = value["name"].as_str().unwrap_or("").to_string();
                let content = value["content"].as_str().unwrap_or("").to_string();
                Ok((name, content))
            } else {
                Err(anyhow::anyhow!("Graph entry not found"))
            }
        } else {
            Err(anyhow::anyhow!("Network graph not yet implemented"))
        }
    }

    // ========================================================================
    // PNR - Pointer Name Registry (DNS-like)
    // ========================================================================

    pub async fn store_pnr(
        &self,
        name: &str,
        records: &std::collections::HashMap<String, crate::models::PnrRecord>,
        use_network: bool,
    ) -> Result<String> {
        let data = serde_json::json!({
            "name": name,
            "records": records,
            "type": "pnr"
        });
        let serialized = serde_json::to_vec(&data)?;

        if use_network {
            log::info!("🌐 Storing PNR on network: {}", name);
            // TODO: Real network implementation
            self.store_chunk(Bytes::from(serialized), false).await
        } else {
            let hash = sha256_hash(&serialized);
            let hex_address = hex::encode(&hash);
            self.memory_cache
                .write()
                .await
                .insert(format!("pnr:{}", name), Bytes::from(serialized));
            Ok(hex_address)
        }
    }

    pub async fn update_pnr(
        &self,
        name: &str,
        records: &std::collections::HashMap<String, crate::models::PnrRecord>,
        use_network: bool,
    ) -> Result<()> {
        if !use_network {
            let data = serde_json::json!({
                "name": name,
                "records": records,
                "type": "pnr"
            });
            self.memory_cache.write().await.insert(
                format!("pnr:{}", name),
                Bytes::from(serde_json::to_vec(&data)?),
            );
        }
        Ok(())
    }

    pub async fn get_pnr(
        &self,
        name: &str,
        use_network: bool,
    ) -> Result<std::collections::HashMap<String, crate::models::PnrRecord>> {
        if !use_network {
            let cache = self.memory_cache.read().await;
            let key = format!("pnr:{}", name);
            if let Some(data) = cache.get(&key) {
                let value: serde_json::Value = serde_json::from_slice(data)?;
                let records = serde_json::from_value(value["records"].clone())?;
                Ok(records)
            } else {
                Err(anyhow::anyhow!("PNR not found"))
            }
        } else {
            Err(anyhow::anyhow!("Network PNR not yet implemented"))
        }
    }

    pub async fn append_pnr(
        &self,
        name: &str,
        new_records: &std::collections::HashMap<String, crate::models::PnrRecord>,
        use_network: bool,
    ) -> Result<()> {
        if !use_network {
            let mut cache = self.memory_cache.write().await;
            let key = format!("pnr:{}", name);

            // Get existing records
            let mut all_records = if let Some(data) = cache.get(&key) {
                let value: serde_json::Value = serde_json::from_slice(data)?;
                serde_json::from_value(value["records"].clone())?
            } else {
                std::collections::HashMap::new()
            };

            // Merge with new records
            all_records.extend(new_records.clone());

            // Store back
            let data = serde_json::json!({
                "name": name,
                "records": all_records,
                "type": "pnr"
            });
            cache.insert(key, Bytes::from(serde_json::to_vec(&data)?));
        }
        Ok(())
    }

    // ========================================================================
    // KEY/VALUE - Object storage with buckets
    // ========================================================================

    pub async fn store_key_value(
        &self,
        bucket: &str,
        object: &str,
        content: &str,
        use_network: bool,
    ) -> Result<String> {
        let data = serde_json::json!({
            "bucket": bucket,
            "object": object,
            "content": content
        });
        let serialized = serde_json::to_vec(&data)?;

        if use_network {
            log::info!("🗄️ Storing key/value on network: {}/{}", bucket, object);
            // TODO: Real network implementation
            self.store_chunk(Bytes::from(serialized), false).await
        } else {
            let hash = sha256_hash(&serialized);
            let hex_address = hex::encode(&hash);
            let key = format!("kv:{}:{}", bucket, object);
            self.memory_cache
                .write()
                .await
                .insert(key, Bytes::from(serialized));
            Ok(hex_address)
        }
    }

    pub async fn get_key_value(
        &self,
        bucket: &str,
        object: &str,
        use_network: bool,
    ) -> Result<String> {
        if !use_network {
            let cache = self.memory_cache.read().await;
            let key = format!("kv:{}:{}", bucket, object);
            if let Some(data) = cache.get(&key) {
                let value: serde_json::Value = serde_json::from_slice(data)?;
                Ok(value["content"].as_str().unwrap_or("").to_string())
            } else {
                Err(anyhow::anyhow!("Key/value not found"))
            }
        } else {
            Err(anyhow::anyhow!("Network key/value not yet implemented"))
        }
    }

    // ========================================================================
    // PUBLIC DATA - Simple binary storage
    // ========================================================================

    pub async fn store_public_data(&self, data: Bytes, use_network: bool) -> Result<String> {
        if use_network {
            log::warn!("⚠️  Network storage requested but not available (compile with --features network)");
            log::info!("💾 Falling back to memory storage");
        }
        
        log::info!("💾 Storing public data in memory ({} bytes)", data.len());
        let hash = sha256_hash(&data);
        let hex_address = hex::encode(&hash);
        self.memory_cache
            .write()
            .await
            .insert(hex_address.clone(), data);
        Ok(hex_address)
    }

    pub async fn get_public_data(&self, address: &str, use_network: bool) -> Result<Bytes> {
        if use_network {
            log::warn!("⚠️  Network retrieval requested but not available (compile with --features network)");
            log::info!("💾 Falling back to memory retrieval");
        }
        
        log::info!("💾 Getting public data from memory: {}", address);
        let cache = self.memory_cache.read().await;
        cache
            .get(address)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Public data not found"))
    }
}

/// Simple SHA256 hash for memory storage
fn sha256_hash(data: &[u8]) -> Vec<u8> {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}
