<script>
  import { onMount } from 'svelte';
  
  let key = '';
  let value = '';
  let registers = [];
  let selectedRegister = null;
  let loading = false;
  let error = '';
  let success = '';

  const BACKEND_URL = 'http://localhost:8080';

  const examplePlaceholder = '{"theme": "dark", "language": "en"}';

  const exampleData = {
    userSettings: {
      theme: 'dark',
      language: 'en',
      notifications: true
    },
    gameState: {
      level: 5,
      score: 12450,
      lives: 3
    },
    counter: 42
  };

  async function setRegister() {
    if (!key.trim() || !value.trim()) {
      error = 'Please enter both key and value';
      return;
    }

    loading = true;
    error = '';
    success = '';

    try {
      const response = await fetch(`${BACKEND_URL}/api/registers`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ key, value })
      });

      const data = await response.json();
      
      if (data.success) {
        success = `Register "${key}" ${data.register.version > 1 ? 'updated' : 'created'} (v${data.register.version})`;
        key = '';
        value = '';
        await loadRegisters();
      } else {
        error = data.error || 'Failed to set register';
      }
    } catch (e) {
      error = `Error: ${e.message}`;
    } finally {
      loading = false;
    }
  }

  async function loadRegisters() {
    try {
      const response = await fetch(`${BACKEND_URL}/api/registers`);
      const data = await response.json();
      
      if (data.success) {
        registers = data.registers;
      }
    } catch (e) {
      console.error('Error loading registers:', e);
    }
  }

  async function viewRegister(regKey) {
    try {
      const response = await fetch(`${BACKEND_URL}/api/registers/${regKey}`);
      const data = await response.json();
      
      if (data.success) {
        selectedRegister = data.register;
      }
    } catch (e) {
      error = `Error viewing register: ${e.message}`;
    }
  }

  function useExample(exampleKey) {
    key = exampleKey;
    value = JSON.stringify(exampleData[exampleKey], null, 2);
  }

  onMount(() => {
    loadRegisters();
  });
</script>

<div class="page">
  <div class="page-header">
    <h1>📊 Registers</h1>
    <p>Distributed key-value storage with conflict resolution</p>
  </div>

  {#if error}
    <div class="alert alert-error">{error}</div>
  {/if}

  {#if success}
    <div class="alert alert-success">{success}</div>
  {/if}

  <section class="card">
    <h2>Set Register Value</h2>
    <div class="examples">
      <p><strong>Try these examples:</strong></p>
      <button on:click={() => useExample('userSettings')} class="btn btn-small">User Settings</button>
      <button on:click={() => useExample('gameState')} class="btn btn-small">Game State</button>
      <button on:click={() => useExample('counter')} class="btn btn-small">Simple Counter</button>
    </div>
    <div class="form-group">
      <label for="key">Key:</label>
      <input
        id="key"
        type="text"
        bind:value={key}
        placeholder="user:123:settings"
      />
    </div>
    <div class="form-group">
      <label for="value">Value (JSON):</label>
      <textarea
        id="value"
        bind:value={value}
        placeholder="{examplePlaceholder}"
        rows="6"
      ></textarea>
    </div>
    <button on:click={setRegister} disabled={loading} class="btn btn-primary">
      {loading ? 'Setting...' : 'Set Register'}
    </button>
  </section>

  <section class="card">
    <h2>Stored Registers ({registers.length})</h2>
    {#if registers.length === 0}
      <p class="empty-state">No registers yet. Create your first one above!</p>
    {:else}
      <div class="registers-list">
        {#each registers as register}
          <div class="register-item">
            <div class="register-header">
              <div>
                <span class="register-key">{register.key}</span>
                <span class="badge badge-version">v{register.version}</span>
              </div>
              <button on:click={() => viewRegister(register.key)} class="btn btn-small">
                View
              </button>
            </div>
            <div class="register-preview">{register.value.substring(0, 80)}...</div>
          </div>
        {/each}
      </div>
    {/if}
  </section>

  {#if selectedRegister}
    <section class="card">
      <h2>Register Details</h2>
      <div class="detail-group">
        <strong>Key:</strong>
        <code>{selectedRegister.key}</code>
      </div>
      <div class="detail-group">
        <strong>Version:</strong>
        <span class="badge badge-version">v{selectedRegister.version}</span>
      </div>
      <div class="detail-group">
        <strong>Value:</strong>
        <pre>{selectedRegister.value}</pre>
      </div>
      <button on:click={() => selectedRegister = null} class="btn">Close</button>
    </section>
  {/if}

  <section class="card info-card">
    <h3>About Registers</h3>
    <p>
      Registers provide distributed key-value storage with built-in versioning:
    </p>
    <ul>
      <li><strong>Key-Value Store:</strong> Store and retrieve data by unique keys</li>
      <li><strong>Versioning:</strong> Each update increments the version number</li>
      <li><strong>Conflict Resolution:</strong> Automatic handling of concurrent updates</li>
      <li><strong>Distributed:</strong> Data is replicated across the network</li>
    </ul>
    <p>
      Perfect for application state, user profiles, or any structured data that needs versioning.
    </p>
  </section>
</div>
