<script lang="ts">
	import { chunksAPI } from '$lib/api/client';
	
	let content = '';
	let address = '';
	let retrievedContent = '';
	let loading = false;
	let error = '';
	let success = '';

	async function createChunk() {
		loading = true;
		error = '';
		success = '';
		
		try {
			// Encode content to Base64
			const base64Content = btoa(content);
			address = await chunksAPI.createChunk(base64Content);
			success = `Chunk created successfully!`;
			content = '';
		} catch (e: any) {
			error = e.response?.data?.error || e.message || 'Failed to create chunk';
		} finally {
			loading = false;
		}
	}

	async function getChunk() {
		loading = true;
		error = '';
		success = '';
		
		try {
			const base64Content = await chunksAPI.getChunk(address);
			retrievedContent = atob(base64Content);
			success = 'Chunk retrieved successfully!';
		} catch (e: any) {
			error = e.response?.data?.error || e.message || 'Failed to retrieve chunk';
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Chunks - AntTP Explorer</title>
</svelte:head>

<div class="space-y-8">
	<!-- Header -->
	<div>
		<h1 class="text-3xl font-bold text-gray-900">📦 Chunks</h1>
		<p class="text-gray-600 mt-2">Immutable data storage - once stored, never changes!</p>
	</div>

	<!-- Info -->
	<div class="card bg-blue-50 border-2 border-blue-200">
		<h3 class="font-bold text-blue-900 mb-2">What are Chunks?</h3>
		<p class="text-blue-700 text-sm">
			Chunks are immutable pieces of data. Once created, they can never be modified.
			Think of them like writing in permanent marker - what's written stays written!
		</p>
		<div class="mt-3 text-sm text-blue-600">
			<strong>Use cases:</strong> Documents, images, permanent records, blockchain data
		</div>
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
		<!-- Create Chunk -->
		<div class="card">
			<h2 class="text-xl font-bold mb-4">Create Chunk</h2>
			
			<div class="space-y-4">
				<div>
					<label class="label" for="content">
						Content
					</label>
					<textarea
						id="content"
						bind:value={content}
						placeholder="Enter your content here..."
						rows="6"
						class="textarea"
					></textarea>
					<p class="text-xs text-gray-500 mt-1">
						Will be stored as Base64-encoded immutable data
					</p>
				</div>

				<button
					on:click={createChunk}
					disabled={loading || !content}
					class="btn btn-primary w-full disabled:opacity-50 disabled:cursor-not-allowed"
				>
					{loading ? 'Creating...' : 'Create Chunk'}
				</button>

				{#if address}
					<div class="bg-green-50 border border-green-200 rounded-lg p-4">
						<p class="text-sm font-medium text-green-900">Address:</p>
						<code class="text-xs text-green-700 break-all block mt-1">
							{address}
						</code>
						<button
							on:click={() => navigator.clipboard.writeText(address)}
							class="text-xs text-green-600 hover:underline mt-2"
						>
							Copy address
						</button>
					</div>
				{/if}
			</div>
		</div>

		<!-- Get Chunk -->
		<div class="card">
			<h2 class="text-xl font-bold mb-4">Retrieve Chunk</h2>
			
			<div class="space-y-4">
				<div>
					<label class="label" for="address">
						Chunk Address
					</label>
					<input
						id="address"
						type="text"
						bind:value={address}
						placeholder="Enter chunk address..."
						class="input"
					/>
				</div>

				<button
					on:click={getChunk}
					disabled={loading || !address}
					class="btn btn-primary w-full disabled:opacity-50 disabled:cursor-not-allowed"
				>
					{loading ? 'Retrieving...' : 'Get Chunk'}
				</button>

				{#if retrievedContent}
					<div class="bg-gray-50 border border-gray-200 rounded-lg p-4">
						<p class="text-sm font-medium text-gray-900 mb-2">Retrieved Content:</p>
						<div class="bg-white border rounded p-3 text-sm text-gray-700 whitespace-pre-wrap break-words">
							{retrievedContent}
						</div>
					</div>
				{/if}
			</div>
		</div>
	</div>

	<!-- Messages -->
	{#if error}
		<div class="bg-red-50 border-2 border-red-200 rounded-lg p-4">
			<p class="text-red-700 font-medium">❌ {error}</p>
		</div>
	{/if}

	{#if success}
		<div class="bg-green-50 border-2 border-green-200 rounded-lg p-4">
			<p class="text-green-700 font-medium">✅ {success}</p>
		</div>
	{/if}

	<!-- API Reference -->
	<div class="card bg-gray-50">
		<h3 class="font-bold text-lg mb-3">API Endpoints</h3>
		<div class="space-y-2 text-sm font-mono">
			<div class="flex items-center space-x-2">
				<span class="bg-green-100 text-green-700 px-2 py-1 rounded">POST</span>
				<span>/anttp-0/chunk</span>
			</div>
			<div class="flex items-center space-x-2">
				<span class="bg-blue-100 text-blue-700 px-2 py-1 rounded">GET</span>
				<span>/anttp-0/chunk/{"{address}"}</span>
			</div>
			<div class="flex items-center space-x-2">
				<span class="bg-green-100 text-green-700 px-2 py-1 rounded">POST</span>
				<span>/anttp-0/binary/chunk</span>
			</div>
			<div class="flex items-center space-x-2">
				<span class="bg-blue-100 text-blue-700 px-2 py-1 rounded">GET</span>
				<span>/anttp-0/binary/chunk/{"{address}"}</span>
			</div>
		</div>
	</div>
</div>
