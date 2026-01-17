<script lang="ts">
  import { apiClient } from '$lib/api/chunks';
  
  let content = '';
  let storeType = 'memory';
  let loading = false;
  let error = '';
  let result: { address: string; store_type: string } | null = null;

  const storeTypes = [
    { value: 'memory', label: 'Memory (Cache Only)' },
    { value: 'disk', label: 'Disk (Cache Only)' },
    { value: 'network', label: 'Network (Autonomi)' }
  ];

  async function createChunk() {
    if (!content.trim()) return;
    
    loading = true;
    error = '';
    result = null;

    try {
      // Convert content to base64
      const base64Content = btoa(content);
      
      // Create chunk via API
      const response = await apiClient.chunks.createChunk(base64Content, storeType);
      result = response;
    } catch (e: any) {
      error = e.response?.data?.detail || e.message || 'Failed to create chunk';
    } finally {
      loading = false;
    }
  }

  function reset() {
    content = '';
    result = null;
    error = '';
  }
</script>

<div class="chunk-creator">
  <h2>Create Chunk</h2>
  
  <div class="form-group">
    <label for="content">Content:</label>
    <textarea
      id="content"
      bind:value={content}
      placeholder="Enter text content"
      rows="4"
      disabled={loading}
    />
  </div>

  <div class="form-group">
    <label for="storeType">Storage Type:</label>
    <select id="storeType" bind:value={storeType} disabled={loading}>
      {#each storeTypes as type}
        <option value={type.value}>{type.label}</option>
      {/each}
    </select>
  </div>

  <button 
    on:click={createChunk} 
    disabled={!content.trim() || loading}
    class="create-btn"
  >
    {loading ? 'Creating...' : 'Create Chunk'}
  </button>

  {#if result}
    <div class="result success">
      <h3>✓ Chunk Created Successfully</h3>
      <p><strong>Address:</strong> <code>{result.address}</code></p>
      <p><strong>Storage:</strong> {result.store_type}</p>
      <button on:click={reset} class="reset-btn">Create Another</button>
    </div>
  {/if}

  {#if error}
    <div class="result error">
      <h3>✗ Error</h3>
      <p>{error}</p>
    </div>
  {/if}
</div>

<style>
  .chunk-creator {
    max-width: 600px;
    margin: 2rem auto;
    padding: 2rem;
    border: 1px solid #ddd;
    border-radius: 8px;
    background: white;
  }

  h2 {
    margin-top: 0;
    color: #333;
  }

  .form-group {
    margin-bottom: 1.5rem;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 600;
    color: #555;
  }

  textarea,
  select {
    width: 100%;
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 1rem;
    font-family: inherit;
  }

  textarea:focus,
  select:focus {
    outline: none;
    border-color: #4CAF50;
  }

  textarea:disabled,
  select:disabled {
    background-color: #f5f5f5;
    cursor: not-allowed;
  }

  button {
    padding: 0.75rem 1.5rem;
    font-size: 1rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .create-btn {
    background-color: #4CAF50;
    color: white;
    width: 100%;
  }

  .create-btn:hover:not(:disabled) {
    background-color: #45a049;
  }

  .create-btn:disabled {
    background-color: #ccc;
    cursor: not-allowed;
  }

  .reset-btn {
    background-color: #2196F3;
    color: white;
    margin-top: 1rem;
  }

  .reset-btn:hover {
    background-color: #0b7dda;
  }

  .result {
    margin-top: 1.5rem;
    padding: 1rem;
    border-radius: 4px;
  }

  .result.success {
    background-color: #d4edda;
    border: 1px solid #c3e6cb;
    color: #155724;
  }

  .result.error {
    background-color: #f8d7da;
    border: 1px solid #f5c6cb;
    color: #721c24;
  }

  .result h3 {
    margin-top: 0;
  }

  code {
    background-color: rgba(0, 0, 0, 0.05);
    padding: 0.2rem 0.4rem;
    border-radius: 3px;
    font-family: 'Courier New', monospace;
    word-break: break-all;
  }
</style>
