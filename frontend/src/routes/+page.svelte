<script lang="ts">
  const features = [
    {
      title: 'Chunks',
      description: 'Immutable data storage with content addressing',
      link: '/chunks',
      icon: '📦'
    },
    {
      title: 'Scratchpads',
      description: 'Volatile mutable data (public and private)',
      link: '/scratchpads',
      icon: '📝'
    },
    {
      title: 'Registers',
      description: 'Signed mutable data with version history',
      link: '/registers',
      icon: '📋'
    },
    {
      title: 'Archives',
      description: 'File collections and directory management',
      link: '/archives',
      icon: '🗄️'
    },
    {
      title: 'Pointers & PNR',
      description: 'DNS-like name resolution and pointers',
      link: '/pointers',
      icon: '🔗'
    },
    {
      title: 'Graph',
      description: 'Structured graph data storage',
      link: '/graph',
      icon: '🕸️'
    }
  ];
</script>

<svelte:head>
  <title>Autonomi AntTP Explorer - Home</title>
</svelte:head>

<div class="home">
  <section class="hero">
    <h1>Welcome to Autonomi AntTP Explorer</h1>
    <p class="subtitle">
      Explore all features of AntTP - the HTTP server for the Autonomi Network
    </p>
    <p class="description">
      This application demonstrates Test-Driven Development (TDD) with Python (FastAPI), 
      SvelteKit, and Docker to interact with the Autonomi decentralized network.
    </p>
  </section>

  <section class="features">
    <h2>Explore AntTP Features</h2>
    <div class="feature-grid">
      {#each features as feature}
        <a href={feature.link} class="feature-card">
          <div class="feature-icon">{feature.icon}</div>
          <h3>{feature.title}</h3>
          <p>{feature.description}</p>
        </a>
      {/each}
    </div>
  </section>

  <section class="about-stack">
    <h2>Technology Stack</h2>
    <div class="stack-cards">
      <div class="stack-card">
        <h3>🐍 Backend</h3>
        <ul>
          <li>FastAPI (Python)</li>
          <li>pytest for TDD</li>
          <li>httpx async client</li>
        </ul>
      </div>
      <div class="stack-card">
        <h3>🎨 Frontend</h3>
        <ul>
          <li>SvelteKit</li>
          <li>Vitest for TDD</li>
          <li>TypeScript</li>
        </ul>
      </div>
      <div class="stack-card">
        <h3>🐳 Infrastructure</h3>
        <ul>
          <li>Docker Compose</li>
          <li>AntTP (Rust)</li>
          <li>Autonomi Network</li>
        </ul>
      </div>
    </div>
  </section>
</div>

<style>
  .home {
    max-width: 1000px;
    margin: 0 auto;
  }

  .hero {
    text-align: center;
    padding: 3rem 0;
    background: white;
    border-radius: 8px;
    margin-bottom: 3rem;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  }

  .hero h1 {
    font-size: 2.5rem;
    color: #2c3e50;
    margin-bottom: 1rem;
  }

  .subtitle {
    font-size: 1.3rem;
    color: #4CAF50;
    margin-bottom: 1rem;
  }

  .description {
    color: #666;
    max-width: 700px;
    margin: 0 auto;
    line-height: 1.6;
  }

  .features {
    margin-bottom: 3rem;
  }

  .features h2 {
    text-align: center;
    margin-bottom: 2rem;
    color: #2c3e50;
  }

  .feature-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 1.5rem;
  }

  .feature-card {
    background: white;
    padding: 2rem;
    border-radius: 8px;
    border: 2px solid #eee;
    text-decoration: none;
    color: inherit;
    transition: all 0.3s;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
  }

  .feature-card:hover {
    border-color: #4CAF50;
    transform: translateY(-4px);
    box-shadow: 0 4px 12px rgba(76, 175, 80, 0.2);
  }

  .feature-icon {
    font-size: 3rem;
    margin-bottom: 1rem;
  }

  .feature-card h3 {
    color: #2c3e50;
    margin-bottom: 0.5rem;
  }

  .feature-card p {
    color: #666;
    font-size: 0.95rem;
  }

  .about-stack {
    background: white;
    padding: 2rem;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
  }

  .about-stack h2 {
    text-align: center;
    margin-bottom: 2rem;
    color: #2c3e50;
  }

  .stack-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5rem;
  }

  .stack-card {
    padding: 1.5rem;
    border: 1px solid #ddd;
    border-radius: 8px;
  }

  .stack-card h3 {
    margin-top: 0;
    color: #2c3e50;
  }

  .stack-card ul {
    list-style: none;
    padding: 0;
  }

  .stack-card li {
    padding: 0.5rem 0;
    color: #666;
  }

  .stack-card li:before {
    content: "✓ ";
    color: #4CAF50;
    font-weight: bold;
    margin-right: 0.5rem;
  }
</style>
