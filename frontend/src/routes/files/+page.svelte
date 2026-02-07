<script>
  import { onMount } from 'svelte';
  
  let name = '';
  let content = '';
  let contentType = 'text/plain';
  let files = [];
  let selectedFile = null;
  let loading = false;
  let error = '';
  let success = '';

  const BACKEND_URL = 'http://localhost:8080';

  const contentTypes = [
    { value: 'text/plain', label: 'Text' },
    { value: 'text/html', label: 'HTML' },
    { value: 'application/json', label: 'JSON' },
    { value: 'application/pdf', label: 'PDF' },
    { value: 'image/jpeg', label: 'JPEG Image' },
    { value: 'image/png', label: 'PNG Image' },
    { value: 'application/octet-stream', label: 'Binary' }
  ];

  async function uploadFile() {
    if (!name.trim() || !content.trim()) {
      error = 'Please enter both filename and content';
      return;
    }

    loading = true;
    error = '';
    success = '';

    try {
      const response = await fetch(`${BACKEND_URL}/api/files`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ 
          name, 
          content,
          content_type: contentType 
        })
      });

      const data = await response.json();
      
      if (data.success) {
        success = `File uploaded! DataMap: ${data.data_map} (${data.size} bytes)`;
        name = '';
        content = '';
        await loadFiles();
      } else {
        error = data.error || 'Failed to upload file';
      }
    } catch (e) {
      error = `Error: ${e.message}`;
    } finally {
      loading = false;
    }
  }

  async function loadFiles() {
    try {
      const response = await fetch(`${BACKEND_URL}/api/files`);
      const data = await response.json();
      
      if (data.success) {
        files = data.files;
      }
    } catch (e) {
      console.error('Error loading files:', e);
    }
  }

  async function viewFile(dataMap) {
    try {
      const response = await fetch(`${BACKEND_URL}/api/files/${dataMap}`);
      const data = await response.json();
      
      if (data.success) {
        selectedFile = data.file;
      }
    } catch (e) {
      error = `Error viewing file: ${e.message}`;
    }
  }

  function formatDate(timestamp) {
    const date = new Date(parseInt(timestamp) * 1000);
    return date.toLocaleString();
  }

  function formatSize(bytes) {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + ' KB';
    return (bytes / (1024 * 1024)).toFixed(2) + ' MB';
  }

  onMount(() => {
    loadFiles();
  });
</script>

<div class="page">
  <div class="page-header">
    <h1>📁 Files</h1>
    <p>Large data with automatic chunking and metadata</p>
  </div>

  {#if error}
    <div class="alert alert-error">{error}</div>
  {/if}

  {#if success}
    <div class="alert alert-success">{success}</div>
  {/if}

  <section class="card">
    <h2>Upload New File</h2>
    <div class="form-group">
      <label for="name">Filename:</label>
      <input
        id="name"
        type="text"
        bind:value={name}
        placeholder="document.txt"
      />
    </div>
    <div class="form-group">
      <label for="contentType">Content Type:</label>
      <select id="contentType" bind:value={contentType}>
        {#each contentTypes as type}
          <option value={type.value}>{type.label}</option>
        {/each}
      </select>
    </div>
    <div class="form-group">
      <label for="content">Content:</label>
      <textarea
        id="content"
        bind:value={content}
        placeholder="Enter file content..."
        rows="8"
      ></textarea>
    </div>
    <button on:click={uploadFile} disabled={loading} class="btn btn-primary">
      {loading ? 'Uploading...' : 'Upload File'}
    </button>
  </section>

  <section class="card">
    <h2>Uploaded Files ({files.length})</h2>
    {#if files.length === 0}
      <p class="empty-state">No files uploaded yet. Upload your first file above!</p>
    {:else}
      <div class="files-list">
        {#each files as file}
          <div class="file-item">
            <div class="file-header">
              <div class="file-info">
                <span class="file-name">📄 {file.name}</span>
                <span class="file-meta">
                  {formatSize(file.size)} • {file.content_type}
                </span>
              </div>
              <button on:click={() => viewFile(file.data_map)} class="btn btn-small">
                View Details
              </button>
            </div>
            <div class="file-datamap">
              <strong>DataMap:</strong> <code>{file.data_map}</code>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </section>

  {#if selectedFile}
    <section class="card">
      <h2>File Details</h2>
      <div class="detail-group">
        <strong>Filename:</strong>
        <span>{selectedFile.name}</span>
      </div>
      <div class="detail-group">
        <strong>DataMap:</strong>
        <code>{selectedFile.data_map}</code>
      </div>
      <div class="detail-group">
        <strong>Size:</strong>
        <span>{formatSize(selectedFile.size)}</span>
      </div>
      <div class="detail-group">
        <strong>Content Type:</strong>
        <span>{selectedFile.content_type}</span>
      </div>
      <div class="detail-group">
        <strong>Created:</strong>
        <span>{formatDate(selectedFile.created_at)}</span>
      </div>
      <button on:click={() => selectedFile = null} class="btn">Close</button>
    </section>
  {/if}

  <section class="card info-card">
    <h3>About Files</h3>
    <p>
      Files in Autonomi are automatically split into chunks for efficient storage and retrieval:
    </p>
    <ul>
      <li><strong>Automatic Chunking:</strong> Large files are split into optimal chunk sizes</li>
      <li><strong>DataMap:</strong> A special structure that maps all chunks back together</li>
      <li><strong>Encryption:</strong> Each chunk is encrypted before storage</li>
      <li><strong>Deduplication:</strong> Identical chunks are stored only once</li>
      <li><strong>Metadata:</strong> File information like name, type, and size are preserved</li>
    </ul>
    <p>
      Use Files for storing documents, images, videos, or any large data that needs to be retrieved as a whole.
    </p>
  </section>
</div>

<style>
  select {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid var(--border);
    border-radius: 4px;
    font-size: 1rem;
    font-family: inherit;
    background: white;
    cursor: pointer;
  }

  select:focus {
    outline: none;
    border-color: var(--primary-color);
  }

  .files-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .file-item {
    padding: 1rem;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--surface);
  }

  .file-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 0.75rem;
  }

  .file-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .file-name {
    font-weight: 600;
    font-size: 1.125rem;
    color: var(--text);
  }

  .file-meta {
    font-size: 0.875rem;
    color: var(--text-light);
  }

  .file-datamap {
    font-size: 0.875rem;
    color: var(--text-light);
    word-break: break-all;
  }

  .file-datamap code {
    background: white;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.8rem;
  }
</style>
