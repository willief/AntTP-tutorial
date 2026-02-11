<script lang="ts">
	import '../app.css';
	import { page } from '$app/stores';
	
	const features = [
		{ name: 'Chunks', path: '/chunks', icon: '📦', desc: 'Immutable data' },
		{ name: 'Registers', path: '/registers', icon: '📝', desc: 'Mutable with history' },
		{ name: 'Pointers', path: '/pointers', icon: '👉', desc: 'Mutable references' },
		{ name: 'Scratchpads', path: '/scratchpads', icon: '📋', desc: 'Public & private' },
		{ name: 'Archives', path: '/archives', icon: '🗂️', desc: 'File collections' },
		{ name: 'Graph', path: '/graph', icon: '🕸️', desc: 'Graph structures' },
		{ name: 'PNR', path: '/pnr', icon: '🌐', desc: 'DNS-like registry' },
		{ name: 'Key/Value', path: '/keyvalue', icon: '🗄️', desc: 'Object storage' },
	];
</script>

<div class="min-h-screen flex flex-col">
	<!-- Header -->
	<header class="bg-white shadow-sm border-b border-gray-200">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-4">
			<div class="flex items-center justify-between">
				<div class="flex items-center space-x-3">
					<span class="text-3xl">🦀</span>
					<div>
						<h1 class="text-2xl font-bold text-gray-900">AntTP Explorer</h1>
						<p class="text-sm text-gray-600">Autonomi Network Transfer Protocol</p>
					</div>
				</div>
				
				<a href="/" class="text-blue-600 hover:text-blue-700 font-medium">
					← Home
				</a>
			</div>
		</div>
	</header>

	<!-- Navigation -->
	<nav class="bg-gray-100 border-b border-gray-200">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
			<div class="flex space-x-1 overflow-x-auto py-2">
				{#each features as feature}
					<a
						href={feature.path}
						class="flex items-center space-x-2 px-4 py-2 rounded-lg text-sm font-medium whitespace-nowrap hover:bg-white transition-colors"
						class:bg-white={$page.url.pathname === feature.path}
					>
						<span>{feature.icon}</span>
						<span>{feature.name}</span>
					</a>
				{/each}
			</div>
		</div>
	</nav>

	<!-- Main Content -->
	<main class="flex-1">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
			<slot />
		</div>
	</main>

	<!-- Footer -->
	<footer class="bg-white border-t border-gray-200">
		<div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
			<div class="text-center text-sm text-gray-600">
				<p>Built with Rust (Actix-web) + SvelteKit + TypeScript</p>
				<p class="mt-1">
					<span class="font-medium">37+ endpoints</span> • 
					<span class="font-medium">10 feature types</span> • 
					<a href="http://localhost:18888/anttp-0/command" target="_blank" class="text-blue-600 hover:underline">
						API Docs
					</a>
				</p>
			</div>
		</div>
	</footer>
</div>

<style>
	:global(body) {
		margin: 0;
		font-family: system-ui, -apple-system, sans-serif;
	}
</style>
