<script lang="ts">
  import '../app.css';
</script>

<div class="app">
  <header>
    <nav>
      <h1>🌐 Autonomi AntTP Explorer</h1>
      <div class="nav-links">
        <a href="/">Home</a>
        <a href="/chunks">Chunks</a>
        <a href="/scratchpads">Scratchpads</a>
        <a href="/registers">Registers</a>
        <a href="/archives">Archives</a>
        <a href="/pointers">Pointers</a>
        <a href="/about">About</a>
      </div>
    </nav>
  </header>

  <main>
    <slot />
  </main>

  <footer>
    <p>
      Built with <a href="https://kit.svelte.dev" target="_blank">SvelteKit</a> + 
      <a href="https://fastapi.tiangolo.com" target="_blank">FastAPI</a> + 
      <a href="https://github.com/traktion/AntTP" target="_blank">AntTP</a>
    </p>
  </footer>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
    background-color: #f5f5f5;
  }

  .app {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
  }

  header {
    background-color: #2c3e50;
    color: white;
    padding: 1rem 2rem;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }

  nav {
    max-width: 1200px;
    margin: 0 auto;
  }

  h1 {
    margin: 0 0 1rem 0;
    font-size: 1.8rem;
  }

  .nav-links {
    display: flex;
    gap: 1.5rem;
    flex-wrap: wrap;
  }

  .nav-links a {
    color: white;
    text-decoration: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    transition: background-color 0.2s;
  }

  .nav-links a:hover {
    background-color: rgba(255, 255, 255, 0.1);
  }

  main {
    flex: 1;
    max-width: 1200px;
    width: 100%;
    margin: 0 auto;
    padding: 2rem;
  }

  footer {
    background-color: #2c3e50;
    color: white;
    text-align: center;
    padding: 1rem;
    margin-top: 2rem;
  }

  footer a {
    color: #4CAF50;
    text-decoration: none;
  }

  footer a:hover {
    text-decoration: underline;
  }
</style>
