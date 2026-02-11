<script lang="ts">
	import { pointersAPI } from '$lib/api/client';
	
	let name = '';
	let target = '';
	let address = '';
	let retrievedTarget = '';
	let loading = false;
	let error = '';
	let success = '';

	async function createPointer() {
		loading = true;
		error = '';
		success = '';
		
		try {
			address = await pointersAPI.createPointer(name, target);
			success = 'Pointer created successfully!';
			target = '';
		} catch (e: any) {
			error = e.response?.data?.error || e.message || 'Failed to create pointer';
		} finally {
			loading = false;
		}
	}

	async function updatePointer() {
		loading = true;
		error = '';
		success = '';
		
		try {
			await pointersAPI.updatePointer(address, name, target);
			success = 'Pointer updated successfully!';
		} catch (e: any) {
			error = e.response?.data?.error || e.message || 'Failed to update pointer';
		} finally {
			loading = false;
		}
	}

	async function getPointer() {
		loading = true;
		error = '';
		success = '';
		
		try {
			retrievedTarget = await pointersAPI.getPointer(address);
			success = 'Pointer retrieved successfully!';
		} catch (e: any) {
			error = e.response?.data?.error || e.message || 'Failed to retrieve pointer';
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Pointers - AntTP Explorer</title>
</svelte:head>

<div class="space-y-8">
	<div>
		<h1 class="text-3xl font-bold text-gray-900">👉 Pointers</h1>
		<p class="text-gray-600 mt-2">Mutable address references - point to changing targets</p>
	</div>

	<div class="card bg-purple-50 border-2 border-purple-200">
		<h3 class="font-bold text-purple-900 mb-2">What are Pointers?</h3>
		<p class="text-purple-700 text-sm">
			Pointers are mutable references to other addresses. Update the pointer to redirect to a new target!
			Like a shortcut or symbolic link that can be updated.
		</p>
		<div class="mt-3 text-sm text-purple-600">
			<strong>Use cases:</strong> Latest version pointers, content redirects, mutable links
		</div>
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
		<!-- Create/Update -->
		<div class="card">
			<h2 class="text-xl font-bold mb-4">Create / Update Pointer</h2>
			
			<div class="space-y-4">
				<div>
					<label class="label">Pointer Name</label>
					<input
						type="text"
						bind:value={name}
						placeholder="my-pointer"
						class="input"
					/>
				</div>

				<div>
					<label class="label">Target Address</label>
					<input
						type="text"
						bind:value={target}
						placeholder="Address to point to..."
						class="input"
					/>
					<p class="text-xs text-gray-500 mt-1">Can be any chunk, register, or other address</p>
				</div>

				<div>
					<label class="label">Pointer Address (for updates)</label>
					<input
						type="text"
						bind:value={address}
						placeholder="Leave empty to create new"
						class="input"
					/>
				</div>

				<div class="flex space-x-2">
					<button
						on:click={createPointer}
						disabled={loading || !name || !target}
						class="btn btn-primary flex-1"
					>
						Create New
					</button>
					<button
						on:click={updatePointer}
						disabled={loading || !address || !name || !target}
						class="btn btn-secondary flex-1"
					>
						Update Target
					</button>
				</div>

				{#if address && success.includes('created')}
					<div class="bg-purple-50 border border-purple-200 rounded-lg p-4">
						<p class="text-sm font-medium text-purple-900">Pointer Address:</p>
						<code class="text-xs text-purple-700 break-all">{address}</code>
					</div>
				{/if}
			</div>
		</div>

		<!-- Get -->
		<div class="card">
			<h2 class="text-xl font-bold mb-4">Retrieve Pointer</h2>
			
			<div class="space-y-4">
				<div>
					<label class="label">Pointer Address</label>
					<input
						type="text"
						bind:value={address}
						placeholder="Enter pointer address..."
						class="input"
					/>
				</div>

				<button
					on:click={getPointer}
					disabled={loading || !address}
					class="btn btn-primary w-full"
				>
					Get Target
				</button>

				{#if retrievedTarget}
					<div class="bg-gray-50 border border-gray-200 rounded-lg p-4">
						<p class="text-sm font-medium text-gray-900 mb-2">Points to:</p>
						<code class="text-xs text-gray-700 break-all block bg-white border rounded p-3">
							{retrievedTarget}
						</code>
						<button
							on:click={() => navigator.clipboard.writeText(retrievedTarget)}
							class="text-xs text-blue-600 hover:underline mt-2"
						>
							Copy target address
						</button>
					</div>
				{/if}
			</div>
		</div>
	</div>

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

	<div class="card bg-gray-50">
		<h3 class="font-bold text-lg mb-3">API Endpoints</h3>
		<div class="space-y-2 text-sm font-mono">
			<div class="flex items-center space-x-2">
				<span class="bg-green-100 text-green-700 px-2 py-1 rounded">POST</span>
				<span>/anttp-0/pointer</span>
			</div>
			<div class="flex items-center space-x-2">
				<span class="bg-yellow-100 text-yellow-700 px-2 py-1 rounded">PUT</span>
				<span>/anttp-0/pointer/{"{address}"}</span>
			</div>
			<div class="flex items-center space-x-2">
				<span class="bg-blue-100 text-blue-700 px-2 py-1 rounded">GET</span>
				<span>/anttp-0/pointer/{"{address}"}</span>
			</div>
		</div>
	</div>
</div>
