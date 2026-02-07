<script>
  import { onMount } from 'svelte';
  
  let content = '';
  let chunks = [];
  let selectedChunk = null;
  let loading = false;
  let error = '';
  let success = '';

  const BACKEND_URL = 'http://localhost:8080';

  async function storeChunk() {
    if (!content.trim()) {
      error = 'Please enter some content';
      return;
    }

    loading = true;
    error = '';
    success = '';

    try {
      const response = await fetch(`${BACKEND_URL}/api/chunks`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ content })
      });

      const data = await response.json();
      
      if (data.success) {
        success = `Chunk stored with address: ${data.address}`;
        content = '';
        await loadChunks();
      } else {
        error = data.error || 'Failed to store chunk';
      }
    } catch (e) {
      error = `Error: ${e.message}`;
    } finally {
      loading = false;
    }
  }

  async function loadChunks() {
    try {
      const response = await fetch(`${BACKEND_URL}/api/chunks`);
      const data = await response.json();
      
      if (data.success) {
        chunks = data.chunks;
      }
    } catch (e) {
      console.error('Error loading chunks:', e);
    }
  }

  async function viewChunk(address) {
    try {
      const response = await fetch(`${BACKEND_URL}/api/chunks/${address}`);
      const data = await response.json();
      
      if (data.success) {
        selectedChunk = data.chunk;
      }
    } catch (e) {
      error = `Error viewing chunk: ${e.message}`;
    }
  }

  onMount(() => {
    loadChunks();
  });
</script>

<div class="page">
  <div class="page-header">
    <h1>📦 Chunks</h1>
    <p>Immutable data storage - once written, never changes</p>
  </div>

  {#if error}
    <div class="alert alert-error">{error}</div>
  {/if}

  {#if success}
    <div class="alert alert-success">{success}</div>
  {/if}

  <section class="card">
    <h2>Store New Chunk</h2>
    <div class="form-group">
      <label for="content">Content:</label>
      <textarea
        id="content"
        bind:value={content}
        placeholder="Enter data to store as an immutable chunk..."
        rows="6"
      ></textarea>
    </div>
    <button on:click={storeChunk} disabled={loading} class="btn btn-primary">
      {loading ? 'Storing...' : 'Store Chunk'}
    </button>
  </section>

  <section class="card">
    <h2>Stored Chunks ({chunks.length})</h2>
    {#if chunks.length === 0}
      <p class="empty-state">No chunks stored yet. Store your first chunk above!</p>
    {:else}
      <div class="chunks-list">
        {#each chunks as chunk}
          <div class="chunk-item">
            <div class="chunk-header">
              <span class="chunk-address">{chunk.address}</span>
              <button on:click={() => viewChunk(chunk.address)} class="btn btn-small">
                View
              </button>
            </div>
            <div class="chunk-preview">{chunk.content.substring(0, 100)}...</div>
          </div>
        {/each}
      </div>
    {/if}
  </section>

  {#if selectedChunk}
    <section class="card">
      <h2>Chunk Details</h2>
      <div class="detail-group">
        <strong>Address:</strong>
        <code>{selectedChunk.address}</code>
      </div>
      <div class="detail-group">
        <strong>Content:</strong>
        <pre>{selectedChunk.content}</pre>
      </div>
      <button on:click={() => selectedChunk = null} class="btn">Close</button>
    </section>
  {/if}

  <section class="card info-card">
    <h3>About Chunks</h3>
    <p>
      Chunks are the most basic storage primitive in Autonomi. They are:
    </p>
    <ul>
      <li><strong>Immutable:</strong> Once stored, the data never changes</li>
      <li><strong>Content-addressed:</strong> The address is derived from the content</li>
      <li><strong>Deduplicated:</strong> Identical content is stored only once</li>
      <li><strong>Permanent:</strong> Data persists as long as the network exists</li>
    </ul>
    <p>
      Use chunks for storing files, images, or any data that should never change.
    </p>
  </section>
</div>
