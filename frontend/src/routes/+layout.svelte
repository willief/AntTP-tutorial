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
    { 
      label: 'Archives', 
      icon: '📚',
      section: true,
      items: [
        { path: '/archives', label: 'Public Archives', icon: '📦' },
        { path: '/tarchives', label: 'TArchives', icon: '⚡' },
        { path: '/archives/compare', label: 'Compare Types', icon: '⚖️' }
      ]
    },
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
        <div class="nav-dropdown">
          <button class="nav-dropdown-toggle">Archives ▼</button>
          <div class="nav-dropdown-menu">
            <a href="/archives">Public Archives</a>
            <a href="/tarchives">TArchives</a>
            <a href="/archives/compare">Compare</a>
          </div>
        </div>
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
          {#if item.section}
            <!-- Section with sub-items -->
            <div class="sidebar-section-header">
              <span class="sidebar-icon">{item.icon}</span>
              <span class="sidebar-label">{item.label}</span>
            </div>
            {#each item.items as subItem}
              <a 
                href={subItem.path} 
                class="sidebar-link sub-item {currentPath === subItem.path ? 'active' : ''}"
                on:click={() => sidebarOpen = false}
              >
                <span class="sidebar-icon">{subItem.icon}</span>
                <span class="sidebar-label">{subItem.label}</span>
              </a>
            {/each}
          {:else}
            <!-- Regular item -->
            <a 
              href={item.path} 
              class="sidebar-link {currentPath === item.path ? 'active' : ''}"
              on:click={() => sidebarOpen = false}
            >
              <span class="sidebar-icon">{item.icon}</span>
              <span class="sidebar-label">{item.label}</span>
            </a>
          {/if}
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

<style>
  /* Dropdown navigation for desktop */
  .nav-dropdown {
    position: relative;
    display: inline-block;
  }

  .nav-dropdown-toggle {
    background: none;
    border: none;
    color: var(--text);
    font-size: 0.95rem;
    font-weight: 500;
    padding: 0.5rem 1rem;
    cursor: pointer;
    transition: color 0.2s;
  }

  .nav-dropdown-toggle:hover {
    color: var(--primary-color);
  }

  .nav-dropdown-menu {
    display: none;
    position: absolute;
    top: 100%;
    left: 0;
    background: white;
    border: 1px solid var(--border);
    border-radius: 4px;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    min-width: 180px;
    z-index: 1000;
  }

  .nav-dropdown:hover .nav-dropdown-menu {
    display: block;
  }

  .nav-dropdown-menu a {
    display: block;
    padding: 0.75rem 1rem;
    color: var(--text);
    text-decoration: none;
    transition: background 0.2s;
    border-bottom: 1px solid var(--border);
  }

  .nav-dropdown-menu a:last-child {
    border-bottom: none;
  }

  .nav-dropdown-menu a:hover {
    background: var(--surface);
    color: var(--primary-color);
  }

  /* Sidebar section headers */
  .sidebar-section-header {
    display: flex;
    align-items: center;
    padding: 0.75rem 1rem;
    font-weight: 600;
    color: var(--text);
    background: var(--surface);
    margin-top: 0.5rem;
    border-radius: 4px;
  }

  .sidebar-section-header .sidebar-icon {
    margin-right: 0.75rem;
    font-size: 1.2rem;
  }

  /* Sidebar sub-items */
  .sidebar-link.sub-item {
    padding-left: 2.5rem;
    font-size: 0.9rem;
  }

  .sidebar-link.sub-item .sidebar-icon {
    font-size: 1rem;
  }
</style>
