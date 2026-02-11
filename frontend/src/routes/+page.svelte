<script lang="ts">
	const features = [
		{
			name: 'Chunks',
			icon: '📦',
			description: 'Immutable data storage - once stored, never changes',
			endpoints: 4,
			color: 'blue'
		},
		{
			name: 'Registers',
			icon: '📝',
			description: 'Mutable key-value storage with complete history tracking',
			endpoints: 4,
			color: 'green'
		},
		{
			name: 'Pointers',
			icon: '👉',
			description: 'Mutable address references that can be updated',
			endpoints: 3,
			color: 'purple'
		},
		{
			name: 'Scratchpads',
			icon: '📋',
			description: 'Public and private mutable data storage',
			endpoints: 6,
			color: 'yellow'
		},
		{
			name: 'Archives',
			icon: '🗂️',
			description: 'File collections with multipart upload support',
			endpoints: 4,
			color: 'red'
		},
		{
			name: 'Tarchive',
			icon: '📁',
			description: 'TAR-based archive format for file bundles',
			endpoints: 1,
			color: 'orange'
		},
		{
			name: 'Graph',
			icon: '🕸️',
			description: 'Graph data structures for connected information',
			endpoints: 2,
			color: 'indigo'
		},
		{
			name: 'PNR',
			icon: '🌐',
			description: 'DNS-like pointer name registry system',
			endpoints: 4,
			color: 'cyan'
		},
		{
			name: 'Key/Value',
			icon: '🗄️',
			description: 'Object storage with buckets (like AWS S3)',
			endpoints: 2,
			color: 'pink'
		},
		{
			name: 'Public Data',
			icon: '📤',
			description: 'Simple binary data storage',
			endpoints: 2,
			color: 'teal'
		}
	];

	const stats = {
		totalEndpoints: 37,
		totalFeatures: 10,
		version: '1.0.4'
	};
</script>

<svelte:head>
	<title>AntTP Explorer - Autonomi Network Interface</title>
</svelte:head>

<div class="space-y-8">
	<!-- Hero Section -->
	<div class="text-center space-y-4">
		<h1 class="text-5xl font-bold text-gray-900">
			🦀 Autonomi AntTP Explorer
		</h1>
		<p class="text-xl text-gray-600 max-w-2xl mx-auto">
			Complete interface for exploring the Autonomi Network Transfer Protocol.
			All 10 feature types, 37+ endpoints, production-ready.
		</p>
		
		<!-- Stats -->
		<div class="flex justify-center space-x-8 pt-4">
			<div class="text-center">
				<div class="text-3xl font-bold text-blue-600">{stats.totalEndpoints}+</div>
				<div class="text-sm text-gray-600">Endpoints</div>
			</div>
			<div class="text-center">
				<div class="text-3xl font-bold text-green-600">{stats.totalFeatures}</div>
				<div class="text-sm text-gray-600">Feature Types</div>
			</div>
			<div class="text-center">
				<div class="text-3xl font-bold text-purple-600">v{stats.version}</div>
				<div class="text-sm text-gray-600">Version</div>
			</div>
		</div>
	</div>

	<!-- Quick Start -->
	<div class="card">
		<h2 class="text-2xl font-bold mb-4">🚀 Quick Start</h2>
		<div class="space-y-3">
			<div class="flex items-start space-x-3">
				<span class="text-2xl">1️⃣</span>
				<div>
					<p class="font-medium">Choose a Feature</p>
					<p class="text-sm text-gray-600">Click any feature card below to explore</p>
				</div>
			</div>
			<div class="flex items-start space-x-3">
				<span class="text-2xl">2️⃣</span>
				<div>
					<p class="font-medium">Try It Out</p>
					<p class="text-sm text-gray-600">Use the interactive UI to test API calls</p>
				</div>
			</div>
			<div class="flex items-start space-x-3">
				<span class="text-2xl">3️⃣</span>
				<div>
					<p class="font-medium">See the Results</p>
					<p class="text-sm text-gray-600">Real-time responses from the backend</p>
				</div>
			</div>
		</div>
	</div>

	<!-- Features Grid -->
	<div>
		<h2 class="text-2xl font-bold mb-6">📚 All Features</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
			{#each features as feature}
				<a
					href="/{feature.name.toLowerCase().replace('/', '')}"
					class="card hover:shadow-lg transition-shadow cursor-pointer group"
				>
					<div class="flex items-start justify-between">
						<div class="text-4xl mb-3">{feature.icon}</div>
						<span class="text-xs bg-gray-100 px-2 py-1 rounded">
							{feature.endpoints} endpoint{feature.endpoints > 1 ? 's' : ''}
						</span>
					</div>
					<h3 class="text-xl font-bold mb-2 group-hover:text-blue-600 transition-colors">
						{feature.name}
					</h3>
					<p class="text-gray-600 text-sm">
						{feature.description}
					</p>
				</a>
			{/each}
		</div>
	</div>

	<!-- Technical Details -->
	<div class="card bg-gray-50">
		<h2 class="text-2xl font-bold mb-4">⚙️ Technical Stack</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div>
				<h3 class="font-bold text-lg mb-2">Backend</h3>
				<ul class="space-y-2 text-sm text-gray-600">
					<li>✓ Rust with Actix-web</li>
					<li>✓ Real Autonomi SDK integration</li>
					<li>✓ Memory/Disk/Network storage modes</li>
					<li>✓ Comprehensive error handling</li>
				</ul>
			</div>
			<div>
				<h3 class="font-bold text-lg mb-2">Frontend</h3>
				<ul class="space-y-2 text-sm text-gray-600">
					<li>✓ SvelteKit + TypeScript</li>
					<li>✓ Tailwind CSS</li>
					<li>✓ Type-safe API client</li>
					<li>✓ Responsive design</li>
				</ul>
			</div>
		</div>
	</div>

	<!-- API Reference -->
	<div class="card bg-blue-50 border-2 border-blue-200">
		<div class="flex items-center justify-between">
			<div>
				<h3 class="text-xl font-bold text-blue-900">📖 API Documentation</h3>
				<p class="text-blue-700 mt-1">View complete API specification</p>
			</div>
			<a
				href="http://localhost:18888/anttp-0/command"
				target="_blank"
				class="btn btn-primary"
			>
				View API Docs →
			</a>
		</div>
	</div>
</div>
