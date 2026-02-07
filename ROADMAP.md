# Roadmap: Real Network Integration

This document outlines the plan for integrating the AntTP Tutorial with the real Autonomi Network.

## Current Status: Phase 1 ✅

**Educational Mock Implementation** (Completed)

- ✅ Full REST API for all 6 primitives
- ✅ Interactive web interface
- ✅ Comprehensive documentation
- ✅ API patterns and examples
- ✅ In-memory storage (mock)
- ✅ Docker containerization
- ✅ Test suite

## Phase 2: Network Integration 🚀

**Goal**: Connect to real Autonomi Network and enable actual data storage/retrieval.

### 2.1: SDK Integration (4-6 weeks)

**Objective**: Replace mock storage with Autonomi SDK calls

**Tasks:**
- [ ] Add `autonomi` crate dependency (latest version)
- [ ] Initialize `Client` on backend startup
- [ ] Configure EVM network (testnet initially)
- [ ] Add wallet management for payments
- [ ] Replace in-memory storage with network calls

**Technical Changes:**

```toml
# Cargo.toml additions
[dependencies]
autonomi = "0.x.x"  # Latest version
tokio = { version = "1.35", features = ["full"] }
```

```rust
// Replace Vec<ChunkData> with real network calls
let client = Client::init().await?;
let wallet = Wallet::new_from_private_key(network, key)?;

// Store chunk on network
let address = client.data_put_public(bytes, &wallet).await?;

// Retrieve from network  
let data = client.data_get_public(&address).await?;
```

**Deliverables:**
- Working connection to Autonomi testnet
- Real data storage and retrieval
- Payment integration via EVM wallet

### 2.2: Data Types Implementation (2-3 weeks)

**Objective**: Implement all 6 primitives using real SDK

#### Chunks
```rust
// Real implementation
async fn store_chunk(client: &Client, wallet: &Wallet, content: Bytes) 
    -> Result<XorName> 
{
    client.data_put_public(content, wallet).await
}
```

#### Files
```rust
// Use real DataMaps
async fn upload_file(client: &Client, wallet: &Wallet, path: PathBuf) 
    -> Result<DataMapAddress>
{
    client.dir_upload_public(path, wallet).await
}
```

#### Registers
```rust
// Real Register API
async fn create_register(client: &Client, key: String, value: Bytes) 
    -> Result<RegisterAddress>
{
    let register = client.register_create(value, &key, wallet).await?;
    Ok(register.address())
}
```

#### Pointers
```rust
// Real Pointer implementation
async fn create_pointer(client: &Client, name: String, target: XorName)
    -> Result<PointerAddress>
{
    client.pointer_create(name, target, wallet).await
}
```

#### Archives
```rust
// Real Archives with self-encryption
async fn create_archive(client: &Client, files: Vec<FileData>)
    -> Result<ArchiveAddress>
{
    client.archive_create(files, wallet).await
}
```

#### PNR
```rust
// Real name resolution
async fn create_pnr(client: &Client, name: String, target: Address)
    -> Result<PnrAddress>
{
    client.pnr_create(name, target, wallet).await
}
```

**Deliverables:**
- All 6 primitives working with real network
- Proper XOR addressing
- Self-encryption where applicable

### 2.3: Testing & Validation (1-2 weeks)

**Objective**: Ensure data is actually retrievable with CLI tools

**Test Plan:**

```bash
# Upload via tutorial API
curl -X POST http://localhost:8080/api/chunks \
  -d '{"content":"Hello"}'
# Response: {"address":"<XOR_ADDRESS>"}

# Retrieve with CLI
ant files download <XOR_ADDRESS> output.txt

# Verify content matches
cat output.txt
# Should show: "Hello"
```

**Tasks:**
- [ ] CLI integration tests
- [ ] Cross-validation with ant CLI
- [ ] Performance benchmarking
- [ ] Error handling for network issues
- [ ] Retry logic for failed operations

### 2.4: Payment & Wallet Management (2 weeks)

**Objective**: Proper EVM payment handling

**Features:**
- [ ] Wallet configuration UI
- [ ] Balance checking
- [ ] Cost estimation before upload
- [ ] Payment confirmation
- [ ] Transaction history
- [ ] Testnet/mainnet switching

**Frontend Additions:**
```svelte
<script>
  let walletAddress = '';
  let balance = 0;
  let estimatedCost = 0;

  async function checkCost() {
    const estimate = await fetch('/api/estimate', {
      method: 'POST',
      body: JSON.stringify({ size: fileSize })
    });
    estimatedCost = estimate.tokens;
  }
</script>
```

### 2.5: Documentation Updates (1 week)

**Objective**: Update all docs for network integration

**Tasks:**
- [ ] Update README with network setup
- [ ] Add wallet configuration guide
- [ ] Document EVM network options
- [ ] Update API examples
- [ ] Add network troubleshooting guide
- [ ] Create video tutorials

## Phase 3: Advanced Features (Future)

### 3.1: Multi-Network Support
- [ ] Local testnet
- [ ] Arbitrum testnet
- [ ] Mainnet
- [ ] Custom networks

### 3.2: Enhanced UI Features
- [ ] File browser for archives
- [ ] Register history visualization
- [ ] Pointer graph visualization
- [ ] Network statistics dashboard
- [ ] Cost calculator

### 3.3: Additional Primitives
- [ ] Scratchpads (if added to protocol)
- [ ] Graph entries
- [ ] Any new Autonomi primitives

### 3.4: Developer Tools
- [ ] SDK code generator
- [ ] Postman collection
- [ ] GraphQL API layer
- [ ] WebSocket events
- [ ] Prometheus metrics

### 3.5: Production Features
- [ ] Rate limiting
- [ ] Caching layer
- [ ] Load balancing
- [ ] Monitoring & alerting
- [ ] Backup & recovery

## Technical Dependencies

### Required for Phase 2

**External:**
- Autonomi testnet access
- EVM wallet with test tokens
- Foundry (for local EVM node)

**Internal:**
- Rust 1.70+
- Updated Docker images
- Additional 1-2GB RAM
- Persistent storage

### Breaking Changes

**Phase 2 will introduce:**
- Different response formats (real XOR addresses)
- Async delays (network latency)
- Payment requirements
- Potential failures (network issues)

**Backwards Compatibility:**
- Keep mock mode as fallback
- Environment variable to switch modes
- Clear migration guide

## Timeline Estimate

| Phase | Duration | Effort |
|-------|----------|--------|
| 2.1 SDK Integration | 4-6 weeks | 80-100 hours |
| 2.2 Primitives Implementation | 2-3 weeks | 40-60 hours |
| 2.3 Testing | 1-2 weeks | 20-40 hours |
| 2.4 Payment Integration | 2 weeks | 40 hours |
| 2.5 Documentation | 1 week | 20 hours |
| **Total Phase 2** | **10-14 weeks** | **200-260 hours** |

## Success Criteria

**Phase 2 Complete When:**
- ✅ All 6 primitives store data on real network
- ✅ Data retrievable with `ant` CLI tools
- ✅ Payment integration working
- ✅ Comprehensive tests passing
- ✅ Documentation updated
- ✅ Demo video created

## Contributing to Network Integration

Interested in helping with Phase 2? See [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Development environment setup
- Coding standards
- Pull request process
- Testing requirements

**Priority Issues:**
- Label: `phase-2`
- Label: `network-integration`
- Label: `help-wanted`

## Questions?

- **Discussion**: GitHub Discussions
- **Issues**: GitHub Issues
- **Community**: Autonomi Forums

---

**Last Updated**: February 2026
**Status**: Phase 1 Complete, Phase 2 Planning
