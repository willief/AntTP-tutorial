<script lang="ts">
  import ChunkCreator from '$lib/components/ChunkCreator.svelte';
</script>

<svelte:head>
  <title>Chunks - Autonomi AntTP Explorer</title>
</svelte:head>

<div class="chunks-page">
  <h1>Chunks: Immutable Data Storage</h1>
  
  <section class="info-card card">
    <h2>What are Chunks?</h2>
    <p>
      Chunks are the fundamental unit of immutable data storage on the Autonomi Network. 
      Each chunk is content-addressed, meaning its address (XOR) is derived from its content.
    </p>
    <h3>Key Features:</h3>
    <ul>
      <li>Content-addressed storage (address = hash of content)</li>
      <li>Immutable - cannot be changed once created</li>
      <li>Deduplication - identical content shares the same address</li>
      <li>Permanent storage on the Autonomi Network</li>
    </ul>
    <h3>Storage Types:</h3>
    <ul>
      <li><strong>Memory:</strong> Cached in RAM only (volatile, for testing)</li>
      <li><strong>Disk:</strong> Cached on disk (volatile, local storage)</li>
      <li><strong>Network:</strong> Uploaded to Autonomi Network (permanent)</li>
    </ul>
  </section>

  <ChunkCreator />

  <section class="tips-card card">
    <h2>💡 Tips</h2>
    <ul>
      <li>Content is automatically base64-encoded before upload</li>
      <li>Use "Memory" or "Disk" storage for testing without network costs</li>
      <li>The chunk address can be used to retrieve the data later</li>
      <li>Share the address with others to let them access your chunk</li>
    </ul>
  </section>
</div>

<style>
  .chunks-page {
    max-width: 900px;
    margin: 0 auto;
  }

  h1 {
    color: #2c3e50;
    margin-bottom: 2rem;
  }

  .card {
    background: white;
    padding: 2rem;
    border-radius: 8px;
    margin-bottom: 2rem;
    box-shadow: 0 2px 4px rgba(0,0,0,0.05);
  }

  .info-card h2,
  .tips-card h2 {
    margin-top: 0;
    color: #4CAF50;
  }

  .info-card h3 {
    margin-top: 1.5rem;
    color: #2c3e50;
  }

  ul {
    line-height: 1.8;
    color: #555;
  }

  li {
    margin-bottom: 0.5rem;
  }

  strong {
    color: #2c3e50;
  }
</style>
