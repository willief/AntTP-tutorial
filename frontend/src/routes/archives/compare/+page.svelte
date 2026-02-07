<script>
  import { goto } from '$app/navigation';
</script>

<div class="page">
  <div class="page-header">
    <h1>⚖️ Archives Comparison</h1>
    <p>Choosing between Public Archives and TArchives</p>
  </div>

  <!-- Quick Decision -->
  <section class="card highlight-card">
    <h2>🎯 Quick Decision Guide</h2>
    <div class="decision-grid">
      <div class="decision-box public">
        <h3>Choose <strong>Public Archives</strong> if:</h3>
        <ul>
          <li>✓ You have files of varying sizes</li>
          <li>✓ Some files are larger than 4MB</li>
          <li>✓ You want the simplest setup</li>
          <li>✓ You have fewer than 100 files</li>
          <li>✓ Upload speed isn't critical</li>
        </ul>
        <button on:click={() => goto('/archives')} class="btn btn-primary">
          Go to Public Archives →
        </button>
      </div>

      <div class="decision-box tarchive">
        <h3>Choose <strong>TArchives</strong> if:</h3>
        <ul>
          <li>✓ Most files are under 4MB</li>
          <li>✓ You have 100+ small files</li>
          <li>✓ Fast lookup is important</li>
          <li>✓ You want to minimize costs</li>
          <li>✓ Chronological order matters</li>
        </ul>
        <button on:click={() => goto('/tarchives')} class="btn btn-primary">
          Go to TArchives →
        </button>
      </div>
    </div>
  </section>

  <!-- Detailed Comparison Table -->
  <section class="card">
    <h2>📊 Detailed Comparison</h2>
    <div class="comparison-table-wrapper">
      <table class="comparison-table">
        <thead>
          <tr>
            <th>Feature</th>
            <th class="public-col">📦 Public Archives</th>
            <th class="tarchive-col">⚡ TArchives</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td><strong>File Size Limit</strong></td>
            <td class="public-col">✅ No limit</td>
            <td class="tarchive-col">⚠️ Best under 4MB each</td>
          </tr>
          <tr>
            <td><strong>Number of Files</strong></td>
            <td class="public-col">⚠️ Slower with 100+</td>
            <td class="tarchive-col">✅ Optimized for 100+</td>
          </tr>
          <tr>
            <td><strong>Lookup Speed</strong></td>
            <td class="public-col">⚠️ Dynamic scan</td>
            <td class="tarchive-col">✅ O(1) instant</td>
          </tr>
          <tr>
            <td><strong>Storage Cost</strong></td>
            <td class="public-col">⚠️ Higher for many files</td>
            <td class="tarchive-col">✅ 50-80% savings</td>
          </tr>
          <tr>
            <td><strong>Upload Complexity</strong></td>
            <td class="public-col">✅ Simple</td>
            <td class="tarchive-col">⚠️ Requires tarindexer</td>
          </tr>
          <tr>
            <td><strong>Index Structure</strong></td>
            <td class="public-col">⚠️ Generated on request</td>
            <td class="tarchive-col">✅ Pre-computed</td>
          </tr>
          <tr>
            <td><strong>File Ordering</strong></td>
            <td class="public-col">⚠️ No guarantee</td>
            <td class="tarchive-col">✅ Chronological</td>
          </tr>
          <tr>
            <td><strong>Deduplication</strong></td>
            <td class="public-col">⚠️ Standard</td>
            <td class="tarchive-col">✅ More efficient</td>
          </tr>
          <tr>
            <td><strong>Update Method</strong></td>
            <td class="public-col">✅ Add/replace files</td>
            <td class="tarchive-col">✅ Append to tar</td>
          </tr>
          <tr>
            <td><strong>Retrieval Method</strong></td>
            <td class="public-col">Direct file request</td>
            <td class="tarchive-col">Index lookup + extract</td>
          </tr>
        </tbody>
      </table>
    </div>
  </section>

  <!-- Use Cases -->
  <section class="card">
    <h2>💡 Common Use Cases</h2>
    <div class="use-cases-grid">
      <div class="use-case-card">
        <h3>📦 Public Archives Examples</h3>
        <div class="use-case-item">
          <h4>🌐 Portfolio Website</h4>
          <p><strong>Files:</strong> 5 HTML, 3 CSS, 10 images (2-5MB each), 2 videos (20MB)</p>
          <p><strong>Why Public:</strong> Large video files, mixed sizes, simple structure</p>
        </div>
        <div class="use-case-item">
          <h4>📦 Software Release Package</h4>
          <p><strong>Files:</strong> Binary executable (50MB), libraries (5-10MB), docs (1MB)</p>
          <p><strong>Why Public:</strong> Large binaries, fewer files, standard format</p>
        </div>
        <div class="use-case-item">
          <h4>🖼️ Photo Gallery</h4>
          <p><strong>Files:</strong> 50 high-res photos (8-15MB each)</p>
          <p><strong>Why Public:</strong> Large individual files, straightforward access</p>
        </div>
      </div>

      <div class="use-case-card">
        <h3>⚡ TArchive Examples</h3>
        <div class="use-case-item">
          <h4>📝 Blog Platform</h4>
          <p><strong>Files:</strong> 500 markdown posts (5-20KB each), 200 thumbnails (50KB)</p>
          <p><strong>Why TArchive:</strong> Many small files, fast lookup needed, chronological order</p>
        </div>
        <div class="use-case-item">
          <h4>📚 Documentation Site</h4>
          <p><strong>Files:</strong> 300 HTML pages (10-50KB), 150 images (100KB), CSS/JS</p>
          <p><strong>Why TArchive:</strong> 450+ files, instant navigation, cost savings</p>
        </div>
        <div class="use-case-item">
          <h4>🗂️ Log Archive</h4>
          <p><strong>Files:</strong> 1000 daily logs (1-3MB each), index files</p>
          <p><strong>Why TArchive:</strong> Many files, chronological order critical, efficient lookup</p>
        </div>
      </div>
    </div>
  </section>

  <!-- Performance Comparison -->
  <section class="card">
    <h2>⚡ Performance Characteristics</h2>
    <div class="perf-grid">
      <div class="perf-card">
        <h3>Upload Time</h3>
        <div class="perf-bar-container">
          <div class="perf-label">Public (100 files):</div>
          <div class="perf-bar public" style="width: 80%;">~120s</div>
        </div>
        <div class="perf-bar-container">
          <div class="perf-label">TArchive (100 files):</div>
          <div class="perf-bar tarchive" style="width: 50%;">~60s</div>
        </div>
      </div>

      <div class="perf-card">
        <h3>File Lookup</h3>
        <div class="perf-bar-container">
          <div class="perf-label">Public:</div>
          <div class="perf-bar public" style="width: 70%;">~50ms</div>
        </div>
        <div class="perf-bar-container">
          <div class="perf-label">TArchive:</div>
          <div class="perf-bar tarchive" style="width: 10%;">~5ms</div>
        </div>
      </div>

      <div class="perf-card">
        <h3>Storage Cost (500 files)</h3>
        <div class="perf-bar-container">
          <div class="perf-label">Public:</div>
          <div class="perf-bar public" style="width: 100%;">$5.00</div>
        </div>
        <div class="perf-bar-container">
          <div class="perf-label">TArchive:</div>
          <div class="perf-bar tarchive" style="width: 30%;">$1.50</div>
        </div>
      </div>
    </div>
    <p class="note">* Estimated values for educational purposes. Actual performance depends on file sizes and network conditions.</p>
  </section>

  <!-- Technical Details -->
  <section class="card info-card">
    <h2>🔧 Technical Implementation</h2>
    
    <div class="tech-section">
      <h3>📦 Public Archives</h3>
      <pre><code># Upload via API
POST /api/archives
{
  "name": "my-site",
  "type": "public",
  "files": [
    {"path": "index.html", "content": "..."}
  ]
}

# Access files
http://anttp-server/{archive_address}/index.html</code></pre>
      <p><strong>How it works:</strong> Each file uploaded individually, stored with its path. Index generated dynamically when browsing.</p>
    </div>

    <div class="tech-section">
      <h3>⚡ TArchives</h3>
      <pre><code># Create tar archive
tar -cf archive.tar files/

# Generate index
tarindexer.py -i archive.tar archive.tar.idx

# Append index
tar -rf archive.tar archive.tar.idx

# Upload
POST /api/archives
{
  "name": "blog",
  "type": "tarchive",
  "files": [...]
}</code></pre>
      <p><strong>How it works:</strong> All files packed into tar, index appended to end. O(1) lookup via pre-computed index.</p>
    </div>
  </section>

  <!-- Migration Guide -->
  <section class="card">
    <h2>🔄 Can I Switch Between Types?</h2>
    <div class="migration-info">
      <p><strong>Yes!</strong> You can convert between types, but it requires creating a new archive.</p>
      
      <h3>Public → TArchive</h3>
      <ol>
        <li>Download all files from Public Archive</li>
        <li>Create tar file with all files</li>
        <li>Generate index with tarindexer</li>
        <li>Upload as new TArchive</li>
        <li>Update any pointers or PNR entries</li>
      </ol>

      <h3>TArchive → Public</h3>
      <ol>
        <li>Extract files from TArchive</li>
        <li>Upload individually as Public Archive</li>
        <li>Update any pointers or PNR entries</li>
      </ol>

      <p class="note"><strong>💡 Tip:</strong> Use Pointers to make switching transparent! Point to new archive address without changing the public-facing address.</p>
    </div>
  </section>

  <!-- Summary -->
  <section class="card highlight-card">
    <h2>📌 Summary</h2>
    <div class="summary-boxes">
      <div class="summary-box">
        <h3>📦 Public Archives</h3>
        <p><strong>Best for:</strong> Simplicity, mixed file sizes, fewer files</p>
        <p><strong>Trade-off:</strong> Higher costs with many files, slower lookups</p>
        <a href="/archives" class="btn">Use Public Archives →</a>
      </div>
      <div class="summary-box">
        <h3>⚡ TArchives</h3>
        <p><strong>Best for:</strong> Many small files, performance, cost savings</p>
        <p><strong>Trade-off:</strong> Requires tarindexer, best for files < 4MB</p>
        <a href="/tarchives" class="btn">Use TArchives →</a>
      </div>
    </div>
  </section>
</div>

<style>
  .decision-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
    margin-top: 1rem;
  }

  .decision-box {
    padding: 1.5rem;
    border-radius: 8px;
    border: 2px solid;
  }

  .decision-box.public {
    background: #dbeafe;
    border-color: #3b82f6;
  }

  .decision-box.tarchive {
    background: #fef3c7;
    border-color: #f59e0b;
  }

  .decision-box h3 {
    margin: 0 0 1rem 0;
  }

  .decision-box ul {
    list-style: none;
    padding: 0;
    margin: 0 0 1.5rem 0;
  }

  .decision-box li {
    padding: 0.5rem 0;
    font-size: 0.95rem;
  }

  .comparison-table-wrapper {
    overflow-x: auto;
  }

  .comparison-table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 1rem;
  }

  .comparison-table th,
  .comparison-table td {
    padding: 1rem;
    border: 1px solid var(--border);
    text-align: left;
  }

  .comparison-table thead th {
    background: var(--surface);
    font-weight: 600;
  }

  .comparison-table .public-col {
    background: #eff6ff;
  }

  .comparison-table .tarchive-col {
    background: #fffbeb;
  }

  .use-cases-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
    margin-top: 1rem;
  }

  .use-case-card {
    padding: 1.5rem;
    background: var(--surface);
    border-radius: 8px;
  }

  .use-case-card h3 {
    margin: 0 0 1rem 0;
  }

  .use-case-item {
    margin-bottom: 1.5rem;
    padding-bottom: 1.5rem;
    border-bottom: 1px solid var(--border);
  }

  .use-case-item:last-child {
    border-bottom: none;
    margin-bottom: 0;
    padding-bottom: 0;
  }

  .use-case-item h4 {
    margin: 0 0 0.5rem 0;
    color: var(--primary-color);
  }

  .use-case-item p {
    margin: 0.25rem 0;
    font-size: 0.9rem;
  }

  .perf-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 1.5rem;
    margin-top: 1rem;
  }

  .perf-card {
    padding: 1rem;
    background: var(--surface);
    border-radius: 8px;
  }

  .perf-card h3 {
    margin: 0 0 1rem 0;
    font-size: 1rem;
  }

  .perf-bar-container {
    margin-bottom: 1rem;
  }

  .perf-label {
    font-size: 0.85rem;
    margin-bottom: 0.25rem;
    color: var(--text-light);
  }

  .perf-bar {
    padding: 0.5rem;
    border-radius: 4px;
    font-size: 0.85rem;
    font-weight: 600;
    text-align: right;
  }

  .perf-bar.public {
    background: #3b82f6;
    color: white;
  }

  .perf-bar.tarchive {
    background: #f59e0b;
    color: white;
  }

  .tech-section {
    margin: 1.5rem 0;
    padding: 1.5rem;
    background: var(--surface);
    border-radius: 8px;
  }

  .tech-section h3 {
    margin: 0 0 1rem 0;
  }

  .tech-section pre {
    background: #1e293b;
    color: #e2e8f0;
    padding: 1rem;
    border-radius: 4px;
    overflow-x: auto;
    font-size: 0.85rem;
  }

  .tech-section code {
    font-family: 'Courier New', monospace;
  }

  .migration-info {
    padding: 1rem;
    background: var(--surface);
    border-radius: 8px;
  }

  .migration-info h3 {
    margin: 1.5rem 0 0.5rem 0;
  }

  .migration-info ol {
    margin: 0.5rem 0;
    padding-left: 1.5rem;
  }

  .migration-info li {
    margin: 0.5rem 0;
  }

  .summary-boxes {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
    margin-top: 1rem;
  }

  .summary-box {
    padding: 1.5rem;
    background: var(--surface);
    border-radius: 8px;
    text-align: center;
  }

  .summary-box h3 {
    margin: 0 0 1rem 0;
  }

  .summary-box p {
    margin: 0.5rem 0;
  }

  .summary-box .btn {
    margin-top: 1rem;
  }

  @media (max-width: 1024px) {
    .decision-grid,
    .use-cases-grid,
    .summary-boxes {
      grid-template-columns: 1fr;
    }

    .perf-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
