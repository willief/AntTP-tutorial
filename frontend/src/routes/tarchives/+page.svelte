<script>
  import { onMount } from 'svelte';
  
  let archiveType = 'tarchive'; // Default to tarchive for this page
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
          type: archiveType,
          files: validFiles,
          metadata: Object.keys(metadataObj).length > 0 ? metadataObj : null
        })
      });

      const data = await response.json();
      
      if (data.success) {
        const typeLabel = archiveType === 'public' ? 'Public Archive' : 'TArchive';
        success = `${typeLabel} "${archiveName}" created with ${validFiles.length} files!`;
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
    archiveType = 'public';
    files = [
      { path: 'index.html', content: '<html>\n  <head><title>My Site</title></head>\n  <body><h1>Hello World!</h1></body>\n</html>' },
      { path: 'style.css', content: 'body {\n  font-family: Arial;\n  color: #333;\n}' },
      { path: 'script.js', content: 'console.log("Website loaded!");' }
    ];
    metadata = { author: 'Me', version: '1.0', description: 'My personal website' };
  }

  function loadTarchiveExample() {
    archiveName = 'blog-posts';
    archiveType = 'tarchive';
    files = [
      { path: 'posts/2024-01-post1.md', content: '# First Post\n\nContent here...' },
      { path: 'posts/2024-02-post2.md', content: '# Second Post\n\nMore content...' },
      { path: 'posts/2024-03-post3.md', content: '# Third Post\n\nEven more...' },
      { path: 'index.json', content: '{"posts": ["post1", "post2", "post3"]}' }
    ];
    metadata = { author: 'Me', version: '1.0', description: 'Blog posts collection' };
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
    <h1>⚡ TArchives</h1>
    <p>Tar-based archives optimized for many small files</p>
    <a href="/archives/compare" class="compare-link">Compare with Public Archives →</a>
  </div>

  {#if error}
    <div class="alert alert-error">{error}</div>
  {/if}

  {#if success}
    <div class="alert alert-success">{success}</div>
  {/if}

  <!-- Archive Type Comparison -->
  <section class="card">
    <h2>Two Archive Types</h2>
    <div class="comparison-grid">
      <div class="type-card" class:selected={archiveType === 'public'}>
        <h3>📦 Public Archives</h3>
        <p class="type-desc">Standard archive format - versatile and simple</p>
        <div class="pros-cons">
          <div class="pros">
            <strong>✅ Pros:</strong>
            <ul>
              <li>Works with any file sizes</li>
              <li>Simple upload process</li>
              <li>No special tools needed</li>
              <li>Standard format</li>
              <li>Great for mixed content</li>
            </ul>
          </div>
          <div class="cons">
            <strong>⚠️ Cons:</strong>
            <ul>
              <li>Slower with many files (100+)</li>
              <li>No index for quick lookup</li>
              <li>Higher cost for many small files</li>
            </ul>
          </div>
        </div>
        <button 
          on:click={() => archiveType = 'public'} 
          class="btn btn-primary"
          class:active={archiveType === 'public'}
        >
          Use Public Archive
        </button>
      </div>

      <div class="type-card" class:selected={archiveType === 'tarchive'}>
        <h3>⚡ TArchives (Tar Archives)</h3>
        <p class="type-desc">Optimized for many small files with fast indexing</p>
        <div class="pros-cons">
          <div class="pros">
            <strong>✅ Pros:</strong>
            <ul>
              <li>Extremely fast with many files</li>
              <li>Built-in index for instant lookup</li>
              <li>Lower costs for many small files</li>
              <li>Preserves chronological order</li>
              <li>Efficient deduplication</li>
            </ul>
          </div>
          <div class="cons">
            <strong>⚠️ Cons:</strong>
            <ul>
              <li>Best for files &lt; 4MB each</li>
              <li>Requires tarindexer tool</li>
              <li>More complex creation process</li>
            </ul>
          </div>
        </div>
        <button 
          on:click={() => archiveType = 'tarchive'} 
          class="btn btn-primary"
          class:active={archiveType === 'tarchive'}
        >
          Use TArchive
        </button>
      </div>
    </div>
  </section>

  <!-- Use Case Guide -->
  <section class="card info-card">
    <h3>📋 Which Type Should I Use?</h3>
    <div class="use-case-grid">
      <div class="use-case">
        <h4>Use <strong>Public Archives</strong> for:</h4>
        <ul>
          <li>🌐 Standard websites (any size files)</li>
          <li>📹 Media galleries (large files)</li>
          <li>📦 Software packages with binaries</li>
          <li>🖼️ Mixed content (images + docs)</li>
          <li>🎯 Simple, straightforward deployments</li>
        </ul>
      </div>
      <div class="use-case">
        <h4>Use <strong>TArchives</strong> for:</h4>
        <ul>
          <li>📝 Blogs with many posts</li>
          <li>📚 Documentation sites (100+ pages)</li>
          <li>🗂️ File collections (&lt; 4MB each)</li>
          <li>⚡ Performance-critical lookups</li>
          <li>💰 Cost optimization (many small files)</li>
        </ul>
      </div>
    </div>
  </section>

  <!-- Creation Form -->
  <section class="card">
    <h2>Create {archiveType === 'public' ? 'Public Archive' : 'TArchive'}</h2>
    
    <div class="type-indicator">
      <span class="badge" class:badge-public={archiveType === 'public'} class:badge-tarchive={archiveType === 'tarchive'}>
        {archiveType === 'public' ? '📦 Public Archive' : '⚡ TArchive'}
      </span>
    </div>

    <div class="examples">
      <p><strong>Load examples:</strong></p>
      <button on:click={loadWebsiteExample} class="btn btn-small">Website (Public)</button>
      <button on:click={loadTarchiveExample} class="btn btn-small">Blog Posts (TArchive)</button>
    </div>

    <div class="form-group">
      <label for="archiveName">Archive Name:</label>
      <input
        id="archiveName"
        type="text"
        bind:value={archiveName}
        placeholder="my-website or blog-posts"
      />
    </div>

    <h3>Files</h3>
    <p class="help-text">
      {#if archiveType === 'tarchive'}
        💡 For TArchives: Keep files under 4MB each for optimal performance
      {:else}
        💡 For Public Archives: Any file size is supported
      {/if}
    </p>

    {#each files as file, index}
      <div class="file-input-group">
        <div class="file-input-row">
          <input
            type="text"
            bind:value={file.path}
            placeholder={archiveType === 'tarchive' ? 'posts/my-post.md' : 'path/to/file.txt'}
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
      {loading ? 'Creating...' : `Create ${archiveType === 'public' ? 'Public Archive' : 'TArchive'}`}
    </button>
  </section>

  <!-- Archives List -->
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
                <div class="archive-title">
                  <span class="badge badge-type" class:badge-public={archive.type === 'public'} class:badge-tarchive={archive.type === 'tarchive'}>
                    {archive.type === 'public' ? '📦' : '⚡'}
                  </span>
                  <span class="archive-name">{archive.name}</span>
                </div>
                <span class="archive-meta">
                  {archive.files.length} files • {formatSize(archive.files)} • {archive.type}
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

  <!-- Archive Details -->
  {#if selectedArchive}
    <section class="card">
      <h2>
        Archive: {selectedArchive.name}
        <span class="badge" class:badge-public={selectedArchive.type === 'public'} class:badge-tarchive={selectedArchive.type === 'tarchive'}>
          {selectedArchive.type === 'public' ? '📦 Public' : '⚡ TArchive'}
        </span>
      </h2>
      
      <div class="detail-group">
        <strong>Address:</strong>
        <code>{selectedArchive.address}</code>
      </div>

      <div class="detail-group">
        <strong>Type:</strong>
        <span>{selectedArchive.type === 'public' ? 'Public Archive (Standard)' : 'TArchive (Tar-indexed)'}</span>
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

  <!-- Technical Details -->
  <section class="card info-card">
    <h3>🔧 Technical Details</h3>
    
    <h4>Public Archives</h4>
    <p>
      Standard archive format where each file is uploaded individually to the network. 
      Files are chunked, encrypted, and stored with their paths preserved.
    </p>
    <ul>
      <li><strong>Creation:</strong> Upload files via REST API or AntTP interface</li>
      <li><strong>Access:</strong> <code>http://anttp-server/{"{archive_address}"}/path/to/file</code></li>
      <li><strong>Index:</strong> Generated dynamically on request</li>
      <li><strong>Best for:</strong> General purpose, any file sizes</li>
    </ul>

    <h4>TArchives (Tar Archives)</h4>
    <p>
      Tar-based format with a built-in index appended to the tar file. Optimized for many small files 
      with instant lookup capabilities.
    </p>
    <ul>
      <li><strong>Creation:</strong> Create tar, generate index with tarindexer, upload</li>
      <li><strong>Structure:</strong> <code>archive.tar</code> + <code>archive.tar.idx</code> appended</li>
      <li><strong>Index:</strong> Pre-computed, enables O(1) file lookup</li>
      <li><strong>Order:</strong> Maintains chronological sequence (important for some apps)</li>
      <li><strong>Best for:</strong> 100+ small files (&lt; 4MB each)</li>
    </ul>

    <div class="info-box">
      <strong>💡 Real Network Note:</strong> In production with actual Autonomi Network:
      <ul>
        <li>TArchives can reduce costs by 50-80% for many small files</li>
        <li>File lookups are instant (no network round-trips to check each file)</li>
        <li>Deduplication works better with tar format</li>
        <li>Updates can append new files while preserving chronology</li>
      </ul>
    </div>
  </section>
</div>

<style>
  .comparison-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.5rem;
    margin: 1rem 0;
  }

  .type-card {
    padding: 1.5rem;
    border: 2px solid var(--border);
    border-radius: 8px;
    background: var(--surface);
    transition: all 0.3s ease;
  }

  .type-card.selected {
    border-color: var(--primary-color);
    background: #dbeafe;
  }

  .type-card h3 {
    margin: 0 0 0.5rem 0;
    color: var(--text);
  }

  .type-desc {
    color: var(--text-light);
    margin: 0 0 1rem 0;
    font-size: 0.95rem;
  }

  .pros-cons {
    margin: 1rem 0;
  }

  .pros, .cons {
    margin-bottom: 1rem;
  }

  .pros strong {
    color: #059669;
  }

  .cons strong {
    color: #dc2626;
  }

  .pros ul, .cons ul {
    margin: 0.5rem 0 0 0;
    padding-left: 1.5rem;
  }

  .pros li, .cons li {
    font-size: 0.9rem;
    margin: 0.25rem 0;
  }

  .use-case-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 1.5rem;
    margin-top: 1rem;
  }

  .use-case {
    padding: 1rem;
    background: var(--surface);
    border-radius: 4px;
  }

  .use-case h4 {
    margin: 0 0 0.75rem 0;
  }

  .use-case ul {
    margin: 0;
    padding-left: 1.5rem;
  }

  .use-case li {
    margin: 0.5rem 0;
  }

  .type-indicator {
    margin-bottom: 1rem;
  }

  .badge-type {
    font-size: 1.2rem;
    margin-right: 0.5rem;
  }

  .badge-public {
    background: #dbeafe;
    color: #1e40af;
    border: 1px solid #3b82f6;
  }

  .badge-tarchive {
    background: #fef3c7;
    color: #92400e;
    border: 1px solid #f59e0b;
  }

  .help-text {
    font-size: 0.9rem;
    color: var(--text-light);
    margin: -0.5rem 0 1rem 0;
    padding: 0.5rem;
    background: #eff6ff;
    border-left: 3px solid var(--primary-color);
    border-radius: 4px;
  }

  .archive-title {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .btn.active {
    background: var(--primary-color);
    color: white;
  }

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

  @media (max-width: 1024px) {
    .comparison-grid,
    .use-case-grid {
      grid-template-columns: 1fr;
    }

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

<style>
  .compare-link {
    display: inline-block;
    margin-top: 0.5rem;
    padding: 0.5rem 1rem;
    background: var(--primary-color);
    color: white;
    text-decoration: none;
    border-radius: 4px;
    font-size: 0.9rem;
    transition: background 0.2s;
  }

  .compare-link:hover {
    background: #2563eb;
  }
</style>
