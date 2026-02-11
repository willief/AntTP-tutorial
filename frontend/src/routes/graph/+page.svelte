<svelte:head><title>Graph - AntTP Explorer</title></svelte:head>

<div class="space-y-8">
	<div>
		<h1 class="text-3xl font-bold">🕸️ Graph</h1>
		<p class="text-gray-600 mt-2">Graph data structures</p>
	</div>

	<div class="card bg-indigo-50 border-2 border-indigo-200">
		<h3 class="font-bold text-indigo-900 mb-2">Graph Entries</h3>
		<p class="text-indigo-700 text-sm">
			Store graph nodes and connections. Perfect for social networks, knowledge graphs, and connected data.
		</p>
	</div>

	<div class="card">
		<h2 class="text-xl font-bold mb-4">Coming Soon</h2>
		<p class="text-gray-600">
			Graph UI is under development. Use the API directly for now:
		</p>
		<div class="mt-4 space-y-2 text-sm font-mono bg-gray-50 p-4 rounded">
			<div class="flex items-center space-x-2">
				<span class="bg-green-100 text-green-700 px-2 py-1 rounded">POST</span>
				<span>/anttp-0/graph_entry</span>
			</div>
			<div class="flex items-center space-x-2">
				<span class="bg-blue-100 text-blue-700 px-2 py-1 rounded">GET</span>
				<span>/anttp-0/graph_entry/{"{address}"}</span>
			</div>
		</div>
	</div>
</div>
