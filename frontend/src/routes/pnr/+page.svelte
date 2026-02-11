<svelte:head><title>PNR - AntTP Explorer</title></svelte:head>

<div class="space-y-8">
	<div>
		<h1 class="text-3xl font-bold">🌐 PNR</h1>
		<p class="text-gray-600 mt-2">Pointer Name Registry - DNS for Autonomi</p>
	</div>

	<div class="card bg-cyan-50 border-2 border-cyan-200">
		<h3 class="font-bold text-cyan-900 mb-2">DNS-Like Registry</h3>
		<p class="text-cyan-700 text-sm">
			Map human-readable names to addresses. Like DNS but decentralized!
		</p>
	</div>

	<div class="card">
		<h2 class="text-xl font-bold mb-4">Coming Soon</h2>
		<p class="text-gray-600">
			PNR UI is under development. API endpoints available:
		</p>
		<div class="mt-4 space-y-2 text-sm font-mono bg-gray-50 p-4 rounded">
			<div class="flex items-center space-x-2">
				<span class="bg-green-100 text-green-700 px-2 py-1 rounded">POST</span>
				<span>/anttp-0/pnr</span>
			</div>
			<div class="flex items-center space-x-2">
				<span class="bg-yellow-100 text-yellow-700 px-2 py-1 rounded">PUT</span>
				<span>/anttp-0/pnr/{"{name}"}</span>
			</div>
			<div class="flex items-center space-x-2">
				<span class="bg-blue-100 text-blue-700 px-2 py-1 rounded">GET</span>
				<span>/anttp-0/pnr/{"{name}"}</span>
			</div>
			<div class="flex items-center space-x-2">
				<span class="bg-purple-100 text-purple-700 px-2 py-1 rounded">PATCH</span>
				<span>/anttp-0/pnr/{"{name}"}</span>
			</div>
		</div>
	</div>
</div>
