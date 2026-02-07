<script>
  import { onMount } from 'svelte';
  
  let name = '';
  let target = '';
  let recordType = 'chunk';
  let entries = [];
  let searchName = '';
  let searchResult = null;
  let loading = false;
  let error = '';
  let success = '';

  const BACKEND_URL = 'http://localhost:8080';

  const recordTypes = [
    { value: 'chunk', label: 'Chunk' },
    { value: 'file', label: 'File' },
    { value: 'register', label: 'Register' },
    { value: 'pointer', label: 'Pointer' },
    { value: 'archive', label: 'Archive' }
  ];

  const exampleMappings = [
    { name: 'my-website', target: 'archive_abc123def456', recordType: 'archive' },
    { name: 'profile-photo', target: 'file_789xyz012abc', recordType: 'file' },
    { name: 'app-config', target: 'register_config_v1', recordType: 'register' }
  ];

  async function createPnr() {
    if (!name.trim() || !target.trim()) {
      error = 'Please enter both name and target address';
      return;
    }

    loading = true;
    error = '';
    success = '';

    try {
      const response = await fetch(`${BACKEND_URL}/api/pnr`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ 
          name, 
          target,
          record_type: recordType 
        })
      });

      const data = await response.json();
      
      if (data.success) {
        success = `PNR entry created: ${name} → ${target} (${recordType})`;
        name = '';
        target = '';
        recordType = 'chunk';
        await loadEntries();
      } else {
        error = data.error || 'Failed to create PNR entry';
      }
    } catch (e) {
      error = `Error: ${e.message}`;
    } finally {
      loading = false;
    }
  }

  async function loadEntries() {
    try {
      const response = await fetch(`${BACKEND_URL}/api/pnr`);
      const data = await response.json();
      
      if (data.success) {
        entries = data.entries;
      }
    } catch (e) {
      console.error('Error loading PNR entries:', e);
    }
  }

  async function resolveName() {
    if (!searchName.trim()) {
      error = 'Please enter a name to resolve';
      return;
    }

    loading = true;
    error = '';
    searchResult = null;

    try {
      const response = await fetch(`${BACKEND_URL}/api/pnr/${searchName}`);
      const data = await response.json();
      
      if (data.success) {
        searchResult = data.pnr;
        success = `Resolved: ${searchResult.name} → ${searchResult.target}`;
      } else {
        error = data.error || 'Name not found';
      }
    } catch (e) {
      error = `Error: ${e.message}`;
    } finally {
      loading = false;
    }
  }

  function useExample(example) {
    name = example.name;
    target = example.target;
    recordType = example.recordType;
  }

  onMount(() => {
    loadEntries();
  });
</script>

<div class="page">
  <div class="page-header">
    <h1>🏷️ PNR (Personal Name Resolution)</h1>
    <p>Human-readable names for network addresses</p>
  </div>

  {#if error}
    <div class="alert alert-error">{error}</div>
  {/if}

  {#if success}
    <div class="alert alert-success">{success}</div>
  {/if}

  <section class="card">
    <h2>Create Name Mapping</h2>
    
    <div class="examples">
      <p><strong>Example mappings:</strong></p>
      {#each exampleMappings as example}
        <button on:click={() => useExample(example)} class="btn btn-small">
          {example.name}
        </button>
      {/each}
    </div>

    <div class="form-group">
      <label for="name">Human-readable Name:</label>
      <input
        id="name"
        type="text"
        bind:value={name}
        placeholder="AntTP-tutorial"
      />
    </div>

    <div class="form-group">
      <label for="recordType">Resource Type:</label>
      <select id="recordType" bind:value={recordType}>
        {#each recordTypes as type}
          <option value={type.value}>{type.label}</option>
        {/each}
      </select>
    </div>

    <div class="form-group">
      <label for="target">Network Address:</label>
      <input
        id="target"
        type="text"
        bind:value={target}
        placeholder="archive_1890cc1f3718fef8"
      />
    </div>

    <button on:click={createPnr} disabled={loading} class="btn btn-primary">
      {loading ? 'Creating...' : 'Create Mapping'}
    </button>
  </section>

  <section class="card">
    <h2>Resolve Name</h2>
    <div class="search-box">
      <input
        type="text"
        bind:value={searchName}
        placeholder="AnttpTutorial"
      />
      <button on:click={resolveName} disabled={loading} class="btn btn-primary">
        Resolve
      </button>
    </div>

    {#if searchResult}
      <div class="result-box">
        <div class="detail-group">
          <strong>Name:</strong>
          <span>{searchResult.name}</span>
        </div>
        <div class="detail-group">
          <strong>Target:</strong>
          <code>{searchResult.target}</code>
        </div>
        <div class="detail-group">
          <strong>Type:</strong>
          <span class="badge badge-version">{searchResult.record_type}</span>
        </div>
      </div>
    {/if}
  </section>

  <section class="card">
    <h2>Your PNR Entries ({entries.length})</h2>
    {#if entries.length === 0}
      <p class="empty-state">No PNR entries yet. Create your first mapping above!</p>
    {:else}
      <div class="pnr-list">
        {#each entries as entry}
          <div class="pnr-item">
            <span class="pnr-name">{entry.name}</span>
            <span class="pnr-arrow">→</span>
            <span class="pnr-address">{entry.target}</span>
            <span class="badge badge-version">{entry.record_type}</span>
          </div>
        {/each}
      </div>
    {/if}
  </section>

  <section class="card info-card">
    <h3>About PNR</h3>
    <p>
      Personal Name Resolution provides a DNS-like naming system for the Autonomi Network:
    </p>
    <ul>
      <li><strong>User-Friendly:</strong> Use memorable names instead of long addresses</li>
      <li><strong>Flexible:</strong> Map names to any resource type (chunks, files, archives, etc.)</li>
      <li><strong>Decentralized:</strong> No central authority controls the namespace</li>
      <li><strong>Updatable:</strong> Change what a name points to over time</li>
    </ul>
    <p>
      Think of it like bookmarks for the network - you can give friendly names to any type of data.
    </p>
    <div class="info-box">
      <strong>💡 Pro Tip:</strong> Combine PNR with Pointers for the ultimate flexibility!
      Create a PNR entry that points to a Pointer, which in turn points to your content.
      This gives you both a friendly name AND the ability to update what it points to.
    </div>
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
</style>
