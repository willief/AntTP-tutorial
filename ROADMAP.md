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

### 2.6: MCP (Model Context Protocol) Integration (2-3 weeks)

**Objective**: Enable LLM agents to interact with Autonomi via standardized protocol

**Background:**
The AntTP project now includes MCP support, allowing AI agents (Claude, ChatGPT, etc.) to interface with the Autonomi Network through a standardized protocol. This opens powerful agentic workflows where LLMs can create, retrieve, and manage data on the network.

**What is MCP?**
- Open standard by Anthropic (Nov 2024)
- "USB-C for AI applications"
- Standardizes LLM ↔ External Systems communication
- Enables tool use, context retrieval, and dynamic data access
- Supported by Claude, ChatGPT, IDEs (Cursor, Zed, Replit)

**Implementation Tasks:**
- [ ] Study AntTP's MCP implementation (reference implementation)
- [ ] Create MCP server exposing all 6 primitives as tools
- [ ] Define tool schemas for each primitive
- [ ] Implement prompt templates for common operations
- [ ] Add resource providers for listing stored data
- [ ] Support both STDIO and HTTP+SSE transports
- [ ] Add authentication/authorization layer
- [ ] Test with Claude Desktop app
- [ ] Test with other MCP clients

**MCP Tools to Expose:**

```typescript
// Chunks
- store_chunk(content: string) → ChunkAddress
- retrieve_chunk(address: string) → Content
- list_chunks() → ChunkAddress[]

// Files  
- upload_file(name: string, content: string, type: string) → DataMapAddress
- download_file(address: string) → FileContent
- list_files() → FileMetadata[]

// Registers
- create_register(key: string, value: string) → RegisterAddress
- update_register(key: string, value: string) → Version
- get_register(key: string) → RegisterValue
- list_registers() → RegisterInfo[]

// Pointers
- create_pointer(name: string, target: string) → PointerAddress
- update_pointer(name: string, target: string) → Counter
- resolve_pointer(name: string) → TargetAddress
- list_pointers() → PointerInfo[]

// Archives
- create_archive(name: string, files: File[]) → ArchiveAddress
- list_archive_files(address: string) → FileList
- download_archive_file(address: string, path: string) → Content

// PNR
- create_name(name: string, target: string, type: string) → PnrAddress
- resolve_name(name: string) → TargetAddress
- list_names() → NameMapping[]
```

**Example MCP Workflows:**

1. **AI Agent Website Deployment:**
```
User: "Create a website with home page and about page, deploy to Autonomi"
Agent: [calls create_archive with HTML files]
Agent: [calls create_pointer to archive]
Agent: [calls create_name "mysite" → pointer]
Response: "Site deployed at mysite.antp (via PNR)"
```

2. **Data Management:**
```
User: "Store my notes for later retrieval"
Agent: [calls upload_file with notes]
Agent: [calls create_name "my-notes" → file address]
Response: "Notes saved and accessible via 'my-notes'"
```

3. **Version Control:**
```
User: "Update my website with new content"
Agent: [calls create_archive with new version]
Agent: [calls update_pointer to new archive]
Response: "Website updated, version counter incremented"
```

**Benefits:**
- ✅ LLMs can create/manage Autonomi data
- ✅ Natural language interface to all primitives
- ✅ Agentic workflows (multi-step operations)
- ✅ Integration with AI coding assistants
- ✅ Automated data organization
- ✅ Voice-controlled network operations

**Deliverables:**
- MCP server implementation
- Tool definitions and schemas
- Prompt templates library
- Claude Desktop integration guide
- Example agentic workflows
- Security best practices guide

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
| 2.6 MCP Integration | 2-3 weeks | 40-60 hours |
| **Total Phase 2** | **12-17 weeks** | **240-320 hours** |

## Success Criteria

**Phase 2 Complete When:**
- ✅ All 6 primitives store data on real network
- ✅ Data retrievable with `ant` CLI tools
- ✅ Payment integration working
- ✅ MCP server exposing all primitives as tools
- ✅ Tested with Claude Desktop and other MCP clients
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
