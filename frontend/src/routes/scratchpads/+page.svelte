<script lang="ts">
	import { scratchpadsAPI } from '$lib/api/client';
	
	let name = '';
	let content = '';
	let address = '';
	let retrievedContent = '';
	let isPrivate = false;
	let loading = false;
	let error = '';
	let success = '';

	async function createScratchpad() {
		loading = true;
		error = '';
		success = '';
		
		try {
			const base64Content = btoa(content);
			if (isPrivate) {
				address = await scratchpadsAPI.createPrivate(name, base64Content);
			} else {
				address = await scratchpadsAPI.createPublic(name, base64Content);
			}
			success = `${isPrivate ? 'Private' : 'Public'} scratchpad created!`;
			content = '';
		} catch (e: any) {
			error = e.response?.data?.error || e.message || 'Failed to create scratchpad';
		} finally {
			loading = false;
		}
	}

	async function getScratchpad() {
		loading = true;
		error = '';
		success = '';
		
		try {
			let base64Content;
			if (isPrivate) {
				base64Content = await scratchpadsAPI.getPrivate(address, name);
			} else {
				base64Content = await scratchpadsAPI.getPublic(address);
			}
			retrievedContent = atob(base64Content);
			success = 'Scratchpad retrieved!';
		} catch (e: any) {
			error = e.response?.data?.error || e.message || 'Failed to retrieve';
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Scratchpads - AntTP Explorer</title>
</svelte:head>

<div class="space-y-8">
	<div>
		<h1 class="text-3xl font-bold">📋 Scratchpads</h1>
		<p class="text-gray-600 mt-2">Public and private mutable data storage</p>
	</div>

	<div class="card bg-yellow-50 border-2 border-yellow-200">
		<h3 class="font-bold text-yellow-900 mb-2">Public vs Private</h3>
		<p class="text-yellow-700 text-sm">
			<strong>Public:</strong> Anyone can read • <strong>Private:</strong> Requires name to access (encrypted)
		</p>
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
		<div class="card">
			<h2 class="text-xl font-bold mb-4">Create Scratchpad</h2>
			
			<div class="space-y-4">
				<div class="flex items-center space-x-4">
					<label class="flex items-center space-x-2 cursor-pointer">
						<input type="radio" bind:group={isPrivate} value={false} />
						<span>Public</span>
					</label>
					<label class="flex items-center space-x-2 cursor-pointer">
						<input type="radio" bind:group={isPrivate} value={true} />
						<span>Private</span>
					</label>
				</div>

				<div>
					<label class="label">Name</label>
					<input type="text" bind:value={name} class="input" />
				</div>

				<div>
					<label class="label">Content</label>
					<textarea bind:value={content} rows="5" class="textarea"></textarea>
				</div>

				<button on:click={createScratchpad} disabled={loading || !name || !content} class="btn btn-primary w-full">
					Create {isPrivate ? 'Private' : 'Public'} Scratchpad
				</button>

				{#if address}
					<div class="bg-green-50 border border-green-200 rounded p-3">
						<p class="text-sm font-medium">Address:</p>
						<code class="text-xs break-all">{address}</code>
					</div>
				{/if}
			</div>
		</div>

		<div class="card">
			<h2 class="text-xl font-bold mb-4">Retrieve Scratchpad</h2>
			
			<div class="space-y-4">
				<div>
					<label class="label">Address</label>
					<input type="text" bind:value={address} class="input" />
				</div>

				{#if isPrivate}
					<div>
						<label class="label">Name (required for private)</label>
						<input type="text" bind:value={name} class="input" />
					</div>
				{/if}

				<button on:click={getScratchpad} disabled={loading || !address} class="btn btn-primary w-full">
					Get Scratchpad
				</button>

				{#if retrievedContent}
					<div class="bg-gray-50 border rounded p-3">
						<p class="text-sm font-medium mb-2">Content:</p>
						<div class="bg-white border rounded p-2 text-sm whitespace-pre-wrap">{retrievedContent}</div>
					</div>
				{/if}
			</div>
		</div>
	</div>

	{#if error}
		<div class="bg-red-50 border-2 border-red-200 rounded p-4">
			<p class="text-red-700">❌ {error}</p>
		</div>
	{/if}

	{#if success}
		<div class="bg-green-50 border-2 border-green-200 rounded p-4">
			<p class="text-green-700">✅ {success}</p>
		</div>
	{/if}
</div>
