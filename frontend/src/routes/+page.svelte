<script>
  const primitives = [
    {
      title: 'Chunks',
      icon: '📦',
      description: 'Immutable, content-addressed storage',
      features: [
        'Store data once, never changes',
        'Content-addressed (XOR hash-based)',
        'Automatic self-encryption',
        'Foundation for all other data types'
      ],
      useCase: 'Storing permanent data - the building block of the network',
      link: '/chunks'
    },
    {
      title: 'Files',
      icon: '📁',
      description: 'Large data with automatic chunking',
      features: [
        'Automatic file splitting into chunks',
        'DataMap for reassembly',
        'Self-encryption for security',
        'Efficient storage and retrieval'
      ],
      useCase: 'Uploading large files with metadata (documents, media, binaries)',
      link: '/files'
    },
    {
      title: 'Registers',
      icon: '📊',
      description: 'Versioned mutable key-value storage',
      features: [
        'Key-value pair storage',
        'Automatic versioning',
        'CRDT conflict resolution',
        'Merkle tree for history'
      ],
      useCase: 'User profiles, game state, or application configuration',
      link: '/registers'
    },
    {
      title: 'Pointers',
      icon: '👉',
      description: 'Mutable references to data',
      features: [
        'Point to any data type',
        'Update target dynamically',
        'Version counter tracking',
        'Ownership control'
      ],
      useCase: 'Latest version references, bookmarks, or dynamic redirects',
      link: '/pointers'
    },
    {
      title: 'Archives',
      icon: '📚',
      description: 'File containers for web content',
      features: [
        'Bundle multiple files together',
        'Website hosting support',
        'Directory structure preservation',
        'Public or typed archives (tarchive)'
      ],
      useCase: 'Hosting websites, apps, or collections of related files',
      link: '/archives'
    },
    {
      title: 'PNR',
      icon: '🏷️',
      description: 'Personal Name Resolution',
      features: [
        'Human-readable names',
        'Maps to any address type',
        'Decentralized namespace',
        'DNS-like for Autonomi'
      ],
      useCase: 'Creating memorable names for any network resource',
      link: '/pnr'
    }
  ];
</script>

<div class="home">
  <section class="hero">
    <h1>AntTP Storage Primitives</h1>
    <p class="subtitle">
      Learn the six core storage primitives of the Autonomi Network
    </p>
  </section>

  <section class="primitives-overview">
    <div class="primitives-grid">
      {#each primitives as primitive}
        <div class="primitive-card">
          <div class="primitive-header">
            <span class="primitive-icon">{primitive.icon}</span>
            <h2>{primitive.title}</h2>
          </div>
          
          <p class="primitive-description">{primitive.description}</p>
          
          <div class="primitive-features">
            <h3>Key Features:</h3>
            <ul>
              {#each primitive.features as feature}
                <li>{feature}</li>
              {/each}
            </ul>
          </div>
          
          <div class="primitive-usecase">
            <strong>Use Case:</strong>
            <p>{primitive.useCase}</p>
          </div>
          
          <a href={primitive.link} class="primitive-link">
            Try {primitive.title} →
          </a>
        </div>
      {/each}
    </div>
  </section>

  <section class="api-reference">
    <h2>API Quick Reference</h2>
    <div class="api-grid">
      <div class="api-section">
        <h3>📦 Chunks API</h3>
        <code>POST /api/chunks</code>
        <p>Store immutable data</p>
        <code>GET /api/chunks/:address</code>
        <p>Retrieve by address</p>
      </div>
      
      <div class="api-section">
        <h3>📁 Files API</h3>
        <code>POST /api/files</code>
        <p>Upload with chunking</p>
        <code>GET /api/files/:datamap</code>
        <p>Retrieve by datamap</p>
      </div>
      
      <div class="api-section">
        <h3>📊 Registers API</h3>
        <code>POST /api/registers</code>
        <p>Set key-value pair</p>
        <code>GET /api/registers/:key</code>
        <p>Get versioned value</p>
      </div>
      
      <div class="api-section">
        <h3>👉 Pointers API</h3>
        <code>POST /api/pointers</code>
        <p>Create mutable reference</p>
        <code>PUT /api/pointers/:name</code>
        <p>Update target</p>
      </div>
      
      <div class="api-section">
        <h3>📚 Archives API</h3>
        <code>POST /api/archives</code>
        <p>Create file collection</p>
        <code>GET /api/archives/:address</code>
        <p>Retrieve archive</p>
      </div>
      
      <div class="api-section">
        <h3>🏷️ PNR API</h3>
        <code>POST /api/pnr</code>
        <p>Create name mapping</p>
        <code>GET /api/pnr/:name</code>
        <p>Resolve to address</p>
      </div>
    </div>
  </section>

  <section class="getting-started">
    <h2>Getting Started</h2>
    <div class="steps">
      <div class="step">
        <div class="step-number">1</div>
        <h3>Choose a Primitive</h3>
        <p>Select the storage type that fits your use case from the cards above</p>
      </div>
      <div class="step">
        <div class="step-number">2</div>
        <h3>Explore the Interface</h3>
        <p>Use the interactive forms to create and manage your data</p>
      </div>
      <div class="step">
        <div class="step-number">3</div>
        <h3>Learn the API</h3>
        <p>See how the REST API works behind the scenes</p>
      </div>
    </div>
  </section>

  <section class="comparison">
    <h2>When to Use Each Primitive</h2>
    <table class="comparison-table">
      <thead>
        <tr>
          <th>Primitive</th>
          <th>Mutable?</th>
          <th>Addressing</th>
          <th>Best For</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td><strong>📦 Chunks</strong></td>
          <td>❌ No</td>
          <td>XOR Hash</td>
          <td>Raw data, building block for everything</td>
        </tr>
        <tr>
          <td><strong>📁 Files</strong></td>
          <td>❌ No</td>
          <td>DataMap</td>
          <td>Large files (docs, media, binaries)</td>
        </tr>
        <tr>
          <td><strong>📊 Registers</strong></td>
          <td>✅ Yes</td>
          <td>Key-based</td>
          <td>Versioned data, user profiles, config</td>
        </tr>
        <tr>
          <td><strong>👉 Pointers</strong></td>
          <td>✅ Yes</td>
          <td>Named</td>
          <td>Updatable references, "latest" links</td>
        </tr>
        <tr>
          <td><strong>📚 Archives</strong></td>
          <td>❌ No</td>
          <td>Archive Address</td>
          <td>Websites, file collections, apps</td>
        </tr>
        <tr>
          <td><strong>🏷️ PNR</strong></td>
          <td>✅ Yes</td>
          <td>Human-readable</td>
          <td>Friendly names for any resource</td>
        </tr>
      </tbody>
    </table>
    
    <div class="info-box" style="margin-top: 2rem;">
      <strong>💡 Real-World Example:</strong> Hosting an updatable website with a friendly name:
      <ol style="margin-top: 0.5rem; margin-left: 1.5rem;">
        <li>Create an <strong>Archive</strong> with your website files → gets address like <code>abc123...</code></li>
        <li>Create a <strong>Pointer</strong> pointing to the archive → updatable reference</li>
        <li>Create a <strong>PNR</strong> entry mapping <code>mysite.antp</code> → pointer address</li>
        <li>Access via: <code>http://anttp-server/mysite.antp/</code></li>
        <li>To update: Create new archive, update pointer target ✨</li>
      </ol>
    </div>
  </section>
</div>
