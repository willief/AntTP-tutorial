<script>
  import { onMount } from 'svelte';
  
  let name = '';
  let content = '';
  let isPublic = false;
  let scratchpads = [];
  let selectedPad = null;
  let loading = false;
  let error = '';
  let success = '';

  const BACKEND_URL = 'http://localhost:8080';

  async function createScratchpad() {
    if (!name.trim() || !content.trim()) {
      error = 'Please enter both name and content';
      return;
    }

    loading = true;
    error = '';
    success = '';

    try {
      const response = await fetch(`${BACKEND_URL}/api/scratchpads`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name, content, is_public: isPublic })
      });

      const data = await response.json();
      
      if (data.success) {
        success = `Scratchpad "${name}" created successfully`;
        name = '';
        content = '';
        isPublic = false;
        await loadScratchpads();
      } else {
        error = data.error || 'Failed to create scratchpad';
      }
    } catch (e) {
      error = `Error: ${e.message}`;
    } finally {
      loading = false;
    }
  }

  async function loadScratchpads() {
    try {
      const response = await fetch(`${BACKEND_URL}/api/scratchpads`);
      const data = await response.json();
      
      if (data.success) {
        scratchpads = data.scratchpads;
      }
    } catch (e) {
      console.error('Error loading scratchpads:', e);
    }
  }

  async function viewScratchpad(padName) {
    try {
      const response = await fetch(`${BACKEND_URL}/api/scratchpads/${padName}`);
      const data = await response.json();
      
      if (data.success) {
        selectedPad = data.scratchpad;
      }
    } catch (e) {
      error = `Error viewing scratchpad: ${e.message}`;
    }
  }

  async function updateScratchpad() {
    if (!selectedPad) return;

    loading = true;
    error = '';

    try {
      const response = await fetch(`${BACKEND_URL}/api/scratchpads/${selectedPad.name}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: selectedPad.name,
          content: selectedPad.content,
          is_public: selectedPad.is_public
        })
      });

      const data = await response.json();
      
      if (data.success) {
        success = 'Scratchpad updated successfully';
        await loadScratchpads();
      } else {
        error = data.error || 'Failed to update scratchpad';
      }
    } catch (e) {
      error = `Error: ${e.message}`;
    } finally {
      loading = false;
    }
  }

  onMount(() => {
    loadScratchpads();
  });
</script>

<div class="page">
  <div class="page-header">
    <h1>📝 Scratchpads</h1>
    <p>Mutable data with public/private modes</p>
  </div>

  {#if error}
    <div class="alert alert-error">{error}</div>
  {/if}

  {#if success}
    <div class="alert alert-success">{success}</div>
  {/if}

  <section class="card">
    <h2>Create New Scratchpad</h2>
    <div class="form-group">
      <label for="name">Name:</label>
      <input
        id="name"
        type="text"
        bind:value={name}
        placeholder="my-scratchpad"
      />
    </div>
    <div class="form-group">
      <label for="content">Content:</label>
      <textarea
        id="content"
        bind:value={content}
        placeholder="Enter mutable data..."
        rows="6"
      ></textarea>
    </div>
    <div class="form-group checkbox-group">
      <label>
        <input type="checkbox" bind:checked={isPublic} />
        Public (anyone can read)
      </label>
    </div>
    <button on:click={createScratchpad} disabled={loading} class="btn btn-primary">
      {loading ? 'Creating...' : 'Create Scratchpad'}
    </button>
  </section>

  <section class="card">
    <h2>Your Scratchpads ({scratchpads.length})</h2>
    {#if scratchpads.length === 0}
      <p class="empty-state">No scratchpads yet. Create your first one above!</p>
    {:else}
      <div class="scratchpads-list">
        {#each scratchpads as pad}
          <div class="scratchpad-item">
            <div class="scratchpad-header">
              <div>
                <span class="scratchpad-name">{pad.name}</span>
                <span class="badge {pad.is_public ? 'badge-public' : 'badge-private'}">
                  {pad.is_public ? 'Public' : 'Private'}
                </span>
              </div>
              <button on:click={() => viewScratchpad(pad.name)} class="btn btn-small">
                View/Edit
              </button>
            </div>
            <div class="scratchpad-preview">{pad.content.substring(0, 80)}...</div>
          </div>
        {/each}
      </div>
    {/if}
  </section>

  {#if selectedPad}
    <section class="card">
      <h2>Edit Scratchpad: {selectedPad.name}</h2>
      <div class="form-group">
        <label>Content:</label>
        <textarea
          bind:value={selectedPad.content}
          rows="8"
        ></textarea>
      </div>
      <div class="form-group checkbox-group">
        <label>
          <input type="checkbox" bind:checked={selectedPad.is_public} />
          Public
        </label>
      </div>
      <div class="button-group">
        <button on:click={updateScratchpad} disabled={loading} class="btn btn-primary">
          {loading ? 'Saving...' : 'Save Changes'}
        </button>
        <button on:click={() => selectedPad = null} class="btn">Close</button>
      </div>
    </section>
  {/if}

  <section class="card info-card">
    <h3>About Scratchpads</h3>
    <p>
      Scratchpads are mutable storage units that can be updated over time:
    </p>
    <ul>
      <li><strong>Mutable:</strong> Content can be updated multiple times</li>
      <li><strong>Access Control:</strong> Choose between public (readable by anyone) or private</li>
      <li><strong>State Management:</strong> Perfect for application state or user preferences</li>
      <li><strong>Version Tracking:</strong> Each update is recorded</li>
    </ul>
    <p>
      Use scratchpads for configuration, user settings, or any data that needs to change.
    </p>
  </section>
</div>
