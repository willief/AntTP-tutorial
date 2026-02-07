<script>
  import '../app.css';
  import { page } from '$app/stores';
  
  let sidebarOpen = false;
  
  const navItems = [
    { path: '/', label: 'Home', icon: '🏠' },
    { path: '/chunks', label: 'Chunks', icon: '📦' },
    { path: '/files', label: 'Files', icon: '📁' },
    { path: '/registers', label: 'Registers', icon: '📊' },
    { path: '/pointers', label: 'Pointers', icon: '👉' },
    { path: '/archives', label: 'Archives', icon: '📚' },
    { path: '/pnr', label: 'PNR', icon: '🏷️' }
  ];
  
  function toggleSidebar() {
    sidebarOpen = !sidebarOpen;
  }
  
  $: currentPath = $page.url.pathname;
</script>

<div class="app">
  <header>
    <nav class="top-nav">
      <button class="menu-toggle" on:click={toggleSidebar} aria-label="Toggle menu">
        ☰
      </button>
      <a href="/" class="logo">AntTP Tutorial</a>
      <div class="nav-links desktop-only">
        <a href="/chunks">Chunks</a>
        <a href="/files">Files</a>
        <a href="/registers">Registers</a>
        <a href="/pointers">Pointers</a>
        <a href="/archives">Archives</a>
        <a href="/pnr">PNR</a>
      </div>
    </nav>
  </header>

  <div class="app-container">
    <!-- Sidebar -->
    <aside class="sidebar {sidebarOpen ? 'open' : ''}">
      <div class="sidebar-header">
        <h3>Navigation</h3>
        <button class="close-sidebar" on:click={toggleSidebar} aria-label="Close menu">
          ✕
        </button>
      </div>
      
      <nav class="sidebar-nav">
        {#each navItems as item}
          <a 
            href={item.path} 
            class="sidebar-link {currentPath === item.path ? 'active' : ''}"
            on:click={() => sidebarOpen = false}
          >
            <span class="sidebar-icon">{item.icon}</span>
            <span class="sidebar-label">{item.label}</span>
          </a>
        {/each}
      </nav>
      
      <div class="sidebar-footer">
        <div class="sidebar-section">
          <h4>Quick Links</h4>
          <a href="http://localhost:8080/health" target="_blank" class="sidebar-link small">
            <span>🔍</span>
            <span>Backend Health</span>
          </a>
        </div>
      </div>
    </aside>

    <!-- Overlay for mobile -->
    {#if sidebarOpen}
      <div class="sidebar-overlay" on:click={toggleSidebar}></div>
    {/if}

    <!-- Main content -->
    <main class="main-content">
      <slot />
    </main>
  </div>

  <footer>
    <p>AntTP Tutorial - Learn Autonomi Storage Primitives</p>
  </footer>
</div>
