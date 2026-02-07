<script>
  import { onMount } from 'svelte';
  
  let archiveName = '';
  let files = [{ path: '', content: '' }];
  let metadata = { author: '', version: '', description: '' };
  let archives = [];
  let selectedArchive = null;
  let loading = false;
  let error = '';
  let success = '';

  const BACKEND_URL = 'http://localhost:8080';

  function addFile() {
    files = [...files, { path: '', content: '' }];
  }

  function removeFile(index) {
    files = files.filter((_, i) => i !== index);
  }

  async function createArchive() {
    if (!archiveName.trim()) {
      error = 'Please enter an archive name';
      return;
    }

    const validFiles = files.filter(f => f.path.trim() && f.content.trim());
    if (validFiles.length === 0) {
      error = 'Please add at least one file with path and content';
      return;
    }

    loading = true;
    error = '';
    success = '';

    try {
      const metadataObj = {};
      if (metadata.author) metadataObj.author = metadata.author;
      if (metadata.version) metadataObj.version = metadata.version;
      if (metadata.description) metadataObj.description = metadata.description;

      const response = await fetch(`${BACKEND_URL}/api/archives`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: archiveName,
          files: validFiles,
          metadata: Object.keys(metadataObj).length > 0 ? metadataObj : null
        })
      });

      const data = await response.json();
      
      if (data.success) {
        success = `Archive "${archiveName}" created with ${validFiles.length} files!`;
        archiveName = '';
        files = [{ path: '', content: '' }];
        metadata = { author: '', version: '', description: '' };
        await loadArchives();
      } else {
        error = data.error || 'Failed to create archive';
      }
    } catch (e) {
      error = `Error: ${e.message}`;
    } finally {
      loading = false;
    }
  }

  async function loadArchives() {
    try {
      const response = await fetch(`${BACKEND_URL}/api/archives`);
      const data = await response.json();
      
      if (data.success) {
        archives = data.archives;
      }
    } catch (e) {
      console.error('Error loading archives:', e);
    }
  }

  async function viewArchive(address) {
    try {
      const response = await fetch(`${BACKEND_URL}/api/archives/${address}`);
      const data = await response.json();
      
      if (data.success) {
        selectedArchive = data.archive;
      }
    } catch (e) {
      error = `Error viewing archive: ${e.message}`;
    }
  }

  function loadWebsiteExample() {
    archiveName = 'my-website';
    files = [
      { path: 'index.html', content: '<html>\n  <head><title>My Site</title></head>\n  <body><h1>Hello World!</h1></body>\n</html>' },
      { path: 'style.css', content: 'body {\n  font-family: Arial;\n  color: #333;\n}' },
      { path: 'script.js', content: 'console.log("Website loaded!");' }
    ];
    metadata = { author: 'Me', version: '1.0', description: 'My personal website' };
  }

  function loadPhotoExample() {
    archiveName = 'vacation-photos';
    files = [
      { path: 'beach.jpg', content: '[JPEG image data]' },
      { path: 'sunset.jpg', content: '[JPEG image data]' },
      { path: 'README.txt', content: 'Summer vacation 2024' }
    ];
    metadata = { author: 'Me', version: '1.0', description: 'Vacation photos from summer' };
  }

  function formatSize(files) {
    const total = files.reduce((sum, f) => sum + f.size, 0);
    if (total < 1024) return total + ' B';
    if (total < 1024 * 1024) return (total / 1024).toFixed(2) + ' KB';
    return (total / (1024 * 1024)).toFixed(2) + ' MB';
  }

  onMount(() => {
    loadArchives();
  });
</script>

<div class="page">
  <div class="page-header">
    <h1>📚 Archives</h1>
    <p>Collections of files with metadata</p>
  </div>

  {#if error}
    <div class="alert alert-error">{error}</div>
  {/if}

  {#if success}
    <div class="alert alert-success">{success}</div>
  {/if}

  <section class="card">
    <h2>Create New Archive</h2>
    
    <div class="examples">
      <p><strong>Load examples:</strong></p>
      <button on:click={loadWebsiteExample} class="btn btn-small">Website</button>
      <button on:click={loadPhotoExample} class="btn btn-small">Photo Album</button>
    </div>

    <div class="form-group">
      <label for="archiveName">Archive Name:</label>
      <input
        id="archiveName"
        type="text"
        bind:value={archiveName}
        placeholder="my-website or photo-album"
      />
    </div>

    <h3>Files</h3>
    {#each files as file, index}
      <div class="file-input-group">
        <div class="file-input-row">
          <input
            type="text"
            bind:value={file.path}
            placeholder="path/to/file.txt"
            class="path-input"
          />
          {#if files.length > 1}
            <button 
              on:click={() => removeFile(index)} 
              class="btn btn-small btn-remove"
              type="button"
            >
              ✕
            </button>
          {/if}
        </div>
        <textarea
          bind:value={file.content}
          placeholder="File content..."
          rows="3"
        ></textarea>
      </div>
    {/each}
    
    <button on:click={addFile} class="btn btn-small" type="button">
      + Add File
    </button>

    <h3 style="margin-top: 2rem;">Metadata (Optional)</h3>
    <div class="metadata-grid">
      <div class="form-group">
        <label for="author">Author:</label>
        <input
          id="author"
          type="text"
          bind:value={metadata.author}
          placeholder="Your name"
        />
      </div>
      <div class="form-group">
        <label for="version">Version:</label>
        <input
          id="version"
          type="text"
          bind:value={metadata.version}
          placeholder="1.0"
        />
      </div>
    </div>
    <div class="form-group">
      <label for="description">Description:</label>
      <textarea
        id="description"
        bind:value={metadata.description}
        placeholder="Brief description of the archive"
        rows="2"
      ></textarea>
    </div>

    <button on:click={createArchive} disabled={loading} class="btn btn-primary">
      {loading ? 'Creating...' : 'Create Archive'}
    </button>
  </section>

  <section class="card">
    <h2>Your Archives ({archives.length})</h2>
    {#if archives.length === 0}
      <p class="empty-state">No archives yet. Create your first one above!</p>
    {:else}
      <div class="archives-list">
        {#each archives as archive}
          <div class="archive-item">
            <div class="archive-header">
              <div class="archive-info">
                <span class="archive-name">📚 {archive.name}</span>
                <span class="archive-meta">
                  {archive.files.length} files • {formatSize(archive.files)}
                </span>
              </div>
              <button on:click={() => viewArchive(archive.address)} class="btn btn-small">
                View Details
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </section>

  {#if selectedArchive}
    <section class="card">
      <h2>Archive: {selectedArchive.name}</h2>
      
      <div class="detail-group">
        <strong>Address:</strong>
        <code>{selectedArchive.address}</code>
      </div>

      {#if Object.keys(selectedArchive.metadata).length > 0}
        <div class="detail-group">
          <strong>Metadata:</strong>
          <div class="metadata-display">
            {#each Object.entries(selectedArchive.metadata) as [key, value]}
              <div class="metadata-item">
                <span class="metadata-key">{key}:</span>
                <span class="metadata-value">{value}</span>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <div class="detail-group">
        <strong>Files ({selectedArchive.files.length}):</strong>
        <div class="archive-files">
          {#each selectedArchive.files as file}
            <div class="archive-file-item">
              <div class="file-path">📄 {file.path}</div>
              <div class="file-details">
                <code class="file-datamap">{file.data_map}</code>
                <span class="file-size">{file.size} bytes</span>
              </div>
            </div>
          {/each}
        </div>
      </div>

      <button on:click={() => selectedArchive = null} class="btn">Close</button>
    </section>
  {/if}

  <section class="card info-card">
    <h3>About Archives</h3>
    <p>
      Archives in Autonomi are file containers that work like website hosting on the traditional web:
    </p>
    <ul>
      <li><strong>File Containers:</strong> Bundle multiple files with directory structure</li>
      <li><strong>Public Archives:</strong> Standard archives accessible via their address</li>
      <li><strong>TArchives (Typed Archives):</strong> Archives with type metadata for applications</li>
      <li><strong>Web Hosting:</strong> Perfect for static websites (HTML, CSS, JS, images)</li>
      <li><strong>Path Preservation:</strong> Directory structures are maintained</li>
      <li><strong>Single Address:</strong> Retrieve entire archive with one XOR address</li>
    </ul>
    <p>
      <strong>How AntTP Serves Archives:</strong>
    </p>
    <ul>
      <li>Archives are treated like a file system - you can request specific files by path</li>
      <li>Access via: <code>http://anttp-server/[archive-address]/path/to/file.html</code></li>
      <li>Index files (index.html) are served automatically for directory requests</li>
      <li>Can be combined with Pointers for mutable website addresses</li>
      <li>Combined with PNR for human-readable domain names</li>
    </ul>
    <p>
      <strong>Perfect for:</strong>
    </p>
    <ul>
      <li><strong>Static Websites:</strong> Host entire websites with HTML, CSS, JS, and assets</li>
      <li><strong>Web Applications:</strong> Deploy SPAs and static web apps</li>
      <li><strong>Media Galleries:</strong> Photo albums, video collections</li>
      <li><strong>Documentation Sites:</strong> Technical docs, wikis, knowledge bases</li>
      <li><strong>Software Packages:</strong> Distribute applications with all dependencies</li>
    </ul>
    <div class="info-box">
      <strong>💡 Pro Tip:</strong> Use a Pointer to an Archive for updatable websites. When you update your site, 
      create a new archive and update the pointer - visitors always get the latest version at the same address!
    </div>
  </section>
</div>

<style>
  .file-input-group {
    margin-bottom: 1rem;
    padding: 1rem;
    background: var(--surface);
    border-radius: 4px;
  }

  .file-input-row {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .path-input {
    flex: 1;
    padding: 0.5rem;
    border: 1px solid var(--border);
    border-radius: 4px;
    font-family: monospace;
  }

  .btn-remove {
    background: var(--error);
    color: white;
  }

  .btn-remove:hover {
    background: #dc2626;
  }

  .metadata-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1rem;
  }

  .archives-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .archive-item {
    padding: 1rem;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--surface);
  }

  .archive-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .archive-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .archive-name {
    font-weight: 600;
    font-size: 1.125rem;
    color: var(--text);
  }

  .archive-meta {
    font-size: 0.875rem;
    color: var(--text-light);
  }

  .metadata-display {
    margin-top: 0.5rem;
    padding: 1rem;
    background: var(--surface);
    border-radius: 4px;
  }

  .metadata-item {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .metadata-key {
    font-weight: 600;
    color: var(--text);
  }

  .metadata-value {
    color: var(--text-light);
  }

  .archive-files {
    margin-top: 0.5rem;
  }

  .archive-file-item {
    padding: 0.75rem;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 4px;
    margin-bottom: 0.5rem;
  }

  .file-path {
    font-weight: 600;
    margin-bottom: 0.5rem;
    color: var(--text);
  }

  .file-details {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 0.875rem;
  }

  .file-datamap {
    color: var(--text-light);
    font-size: 0.8rem;
  }

  .file-size {
    color: var(--text-light);
  }

  .info-box {
    margin-top: 1.5rem;
    padding: 1rem;
    background: #dbeafe;
    border-left: 4px solid var(--primary-color);
    border-radius: 4px;
  }

  .info-box strong {
    color: var(--primary-color);
  }

  @media (max-width: 768px) {
    .metadata-grid {
      grid-template-columns: 1fr;
    }

    .file-details {
      flex-direction: column;
      align-items: flex-start;
      gap: 0.25rem;
    }
  }
</style>
