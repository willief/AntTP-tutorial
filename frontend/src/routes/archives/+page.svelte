<script lang="ts">
	let files: FileList | null = null;
	let address = '';
	let loading = false;
	let error = '';
	let success = '';

	async function uploadArchive() {
		if (!files || files.length === 0) return;
		
		loading = true;
		error = '';
		success = '';
		
		try {
			const formData = new FormData();
			Array.from(files).forEach(file => formData.append('files', file));
			
			const response = await fetch('http://localhost:18888/anttp-0/multipart/public_archive', {
				method: 'POST',
				headers: { 'x-store-type': 'memory' },
				body: formData
			});
			
			const data = await response.json();
			address = data.address;
			success = `Archive uploaded! ${files.length} file(s)`;
		} catch (e: any) {
			error = e.message || 'Failed to upload';
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head><title>Archives - AntTP Explorer</title></svelte:head>

<div class="space-y-8">
	<div>
		<h1 class="text-3xl font-bold">🗂️ Archives</h1>
		<p class="text-gray-600 mt-2">Upload file collections</p>
	</div>

	<div class="card bg-red-50 border-2 border-red-200">
		<h3 class="font-bold text-red-900 mb-2">What are Archives?</h3>
		<p class="text-red-700 text-sm">
			Archives store multiple files together as a collection. Like a ZIP file on the network!
		</p>
	</div>

	<div class="card">
		<h2 class="text-xl font-bold mb-4">Upload Files</h2>
		<div class="space-y-4">
			<div>
				<label class="label">Select Files</label>
				<input type="file" multiple bind:files class="input" />
			</div>
			
			<button on:click={uploadArchive} disabled={loading || !files} class="btn btn-primary w-full">
				{loading ? 'Uploading...' : 'Upload Archive'}
			</button>
			
			{#if address}
				<div class="bg-green-50 border border-green-200 p-3 rounded">
					<p class="font-medium text-sm text-green-900">Archive Address:</p>
					<code class="text-xs break-all text-green-700">{address}</code>
				</div>
			{/if}
		</div>
	</div>

	{#if error}
		<div class="bg-red-50 border-2 border-red-200 p-4 rounded">
			<p class="text-red-700">❌ {error}</p>
		</div>
	{/if}
	
	{#if success}
		<div class="bg-green-50 border-2 border-green-200 p-4 rounded">
			<p class="text-green-700">✅ {success}</p>
		</div>
	{/if}
</div>
