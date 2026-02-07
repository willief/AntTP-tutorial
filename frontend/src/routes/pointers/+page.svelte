<script>
  import { onMount } from 'svelte';
  
  let name = '';
  let target = '';
  let pointers = [];
  let selectedPointer = null;
  let loading = false;
  let error = '';
  let success = '';
  let updateMode = false;
  let updateTarget = '';

  const BACKEND_URL = 'http://localhost:8080';

  const examplePointers = [
    { name: 'latest-version', target: 'chunk_abc123' },
    { name: 'current-config', target: 'register_settings' },
    { name: 'homepage', target: 'archive_website_v1' }
  ];

  async function createPointer() {
    if (!name.trim() || !target.trim()) {
      error = 'Please enter both name and target';
      return;
    }

    loading = true;
    error = '';
    success = '';

    try {
      const response = await fetch(`${BACKEND_URL}/api/pointers`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, target })
      });

      const data = await response.json();
      
      if (data.success) {
        success = `Pointer "${name}" created → ${target}`;
        name = '';
        target = '';
        await loadPointers();
      } else {
        error = data.error || 'Failed to create pointer';
      }
    } catch (e) {
      error = `Error: ${e.message}`;
    } finally {
      loading = false;
    }
  }

  async function updatePointer() {
    if (!selectedPointer || !updateTarget.trim()) {
      error = 'Please enter a new target address';
      return;
    }

    loading = true;
    error = '';
    success = '';

    try {
      const response = await fetch(`${BACKEND_URL}/api/pointers/${selectedPointer.name}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ 
          name: selectedPointer.name,
          target: updateTarget 
        })
      });

      const data = await response.json();
      
      if (data.success) {
        success = `Pointer updated! Counter: ${data.pointer.counter}`;
        updateMode = false;
        updateTarget = '';
        selectedPointer = data.pointer;
        await loadPointers();
      } else {
        error = data.error || 'Failed to update pointer';
      }
    } catch (e) {
      error = `Error: ${e.message}`;
    } finally {
      loading = false;
    }
  }

  async function loadPointers() {
    try {
      const response = await fetch(`${BACKEND_URL}/api/pointers`);
      const data = await response.json();
      
      if (data.success) {
        pointers = data.pointers;
      }
    } catch (e) {
      console.error('Error loading pointers:', e);
    }
  }

  async function viewPointer(pointerName) {
    try {
      const response = await fetch(`${BACKEND_URL}/api/pointers/${pointerName}`);
      const data = await response.json();
      
      if (data.success) {
        selectedPointer = data.pointer;
        updateMode = false;
        updateTarget = data.pointer.target;
      }
    } catch (e) {
      error = `Error viewing pointer: ${e.message}`;
    }
  }

  function useExample(example) {
    name = example.name;
    target = example.target;
  }

  function startUpdate() {
    updateMode = true;
    updateTarget = selectedPointer.target;
  }

  onMount(() => {
    loadPointers();
  });
</script>

<div class="page">
  <div class="page-header">
    <h1>👉 Pointers</h1>
    <p>Mutable references to other data</p>
  </div>

  {#if error}
    <div class="alert alert-error">{error}</div>
  {/if}

  {#if success}
    <div class="alert alert-success">{success}</div>
  {/if}

  <section class="card">
    <h2>Create New Pointer</h2>
    <div class="examples">
      <p><strong>Try these examples:</strong></p>
      {#each examplePointers as example}
        <button on:click={() => useExample(example)} class="btn btn-small">
          {example.name}
        </button>
      {/each}
    </div>
    <div class="form-group">
      <label for="name">Pointer Name:</label>
      <input
        id="name"
        type="text"
        bind:value={name}
        placeholder="latest-version"
      />
    </div>
    <div class="form-group">
      <label for="target">Target Address:</label>
      <input
        id="target"
        type="text"
        bind:value={target}
        placeholder="chunk_abc123 or register_settings"
      />
    </div>
    <button on:click={createPointer} disabled={loading} class="btn btn-primary">
      {loading ? 'Creating...' : 'Create Pointer'}
    </button>
  </section>

  <section class="card">
    <h2>Your Pointers ({pointers.length})</h2>
    {#if pointers.length === 0}
      <p class="empty-state">No pointers yet. Create your first one above!</p>
    {:else}
      <div class="pointers-list">
        {#each pointers as pointer}
          <div class="pointer-item">
            <div class="pointer-header">
              <div class="pointer-info">
                <span class="pointer-name">👉 {pointer.name}</span>
                <span class="pointer-meta">
                  Counter: {pointer.counter} • {pointer.target}
                </span>
              </div>
              <button on:click={() => viewPointer(pointer.name)} class="btn btn-small">
                View/Update
              </button>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </section>

  {#if selectedPointer && !updateMode}
    <section class="card">
      <h2>Pointer Details</h2>
      <div class="detail-group">
        <strong>Name:</strong>
        <code>{selectedPointer.name}</code>
      </div>
      <div class="detail-group">
        <strong>Target:</strong>
        <code>{selectedPointer.target}</code>
      </div>
      <div class="detail-group">
        <strong>Version Counter:</strong>
        <span class="badge badge-version">v{selectedPointer.counter}</span>
      </div>
      <div class="detail-group">
        <strong>Owner:</strong>
        <code>{selectedPointer.owner}</code>
      </div>
      <div class="button-group">
        <button on:click={startUpdate} class="btn btn-primary">
          Update Target
        </button>
        <button on:click={() => selectedPointer = null} class="btn">Close</button>
      </div>
    </section>
  {/if}

  {#if selectedPointer && updateMode}
    <section class="card">
      <h2>Update Pointer: {selectedPointer.name}</h2>
      <div class="form-group">
        <label>Current Target:</label>
        <code class="current-target">{selectedPointer.target}</code>
      </div>
      <div class="form-group">
        <label for="updateTarget">New Target:</label>
        <input
          id="updateTarget"
          type="text"
          bind:value={updateTarget}
          placeholder="chunk_xyz789"
        />
      </div>
      <div class="button-group">
        <button on:click={updatePointer} disabled={loading} class="btn btn-primary">
          {loading ? 'Updating...' : 'Update Pointer'}
        </button>
        <button on:click={() => updateMode = false} class="btn">Cancel</button>
      </div>
    </section>
  {/if}

  <section class="card info-card">
    <h3>About Pointers</h3>
    <p>
      Pointers provide mutable references to other data in the network:
    </p>
    <ul>
      <li><strong>Mutable:</strong> Can be updated to point to different targets</li>
      <li><strong>Version Counter:</strong> Increments with each update</li>
      <li><strong>Ownership:</strong> Only the owner can update the pointer</li>
      <li><strong>Flexible:</strong> Can point to any type of data (chunks, files, registers, etc.)</li>
    </ul>
    <p>
      <strong>Use Cases:</strong>
    </p>
    <ul>
      <li>Maintain "latest version" references</li>
      <li>Create bookmarks that can be updated</li>
      <li>Build dynamic redirects or aliases</li>
      <li>Implement version control systems</li>
    </ul>
  </section>
</div>

<style>
  .pointers-list {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .pointer-item {
    padding: 1rem;
    border: 1px solid var(--border);
    border-radius: 4px;
    background: var(--surface);
  }

  .pointer-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .pointer-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    flex: 1;
  }

  .pointer-name {
    font-weight: 600;
    font-size: 1.125rem;
    color: var(--text);
  }

  .pointer-meta {
    font-size: 0.875rem;
    color: var(--text-light);
    font-family: monospace;
  }

  .current-target {
    display: block;
    padding: 0.75rem;
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 4px;
    color: var(--text-light);
    font-size: 0.95rem;
  }
</style>
