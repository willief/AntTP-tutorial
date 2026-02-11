<script lang="ts">
	import { registersAPI } from '$lib/api/client';
	
	let name = '';
	let hexContent = '';
	let address = '';
	let retrievedContent = '';
	let history: Array<{content: string, timestamp: number}> = [];
	let loading = false;
	let error = '';
	let success = '';

	function stringToHex(str: string): string {
		return Array.from(str)
			.map(c => c.charCodeAt(0).toString(16).padStart(2, '0'))
			.join('');
	}

	function hexToString(hex: string): string {
		const bytes = hex.match(/.{1,2}/g) || [];
		return bytes.map(byte => String.fromCharCode(parseInt(byte, 16))).join('');
	}

	async function createRegister() {
		loading = true;
		error = '';
		success = '';
		
		try {
			const hex = stringToHex(hexContent);
			address = await registersAPI.createRegister(name, hex);
			success = 'Register created successfully!';
			hexContent = '';
		} catch (e: any) {
			error = e.response?.data?.error || e.message || 'Failed to create register';
		} finally {
			loading = false;
		}
	}

	async function updateRegister() {
		loading = true;
		error = '';
		success = '';
		
		try {
			const hex = stringToHex(hexContent);
			await registersAPI.updateRegister(address, name, hex);
			success = 'Register updated successfully!';
			hexContent = '';
		} catch (e: any) {
			error = e.response?.data?.error || e.message || 'Failed to update register';
		} finally {
			loading = false;
		}
	}

	async function getRegister() {
		loading = true;
		error = '';
		success = '';
		
		try {
			const hex = await registersAPI.getRegister(address);
			retrievedContent = hexToString(hex);
			success = 'Register retrieved successfully!';
		} catch (e: any) {
			error = e.response?.data?.error || e.message || 'Failed to retrieve register';
		} finally {
			loading = false;
		}
	}

	async function getRegisterHistory() {
		loading = true;
		error = '';
		success = '';
		
		try {
			history = await registersAPI.getHistory(address);
			history = history.map(h => ({
				...h,
				content: hexToString(h.content)
			}));
			success = 'History retrieved successfully!';
		} catch (e: any) {
			error = e.response?.data?.error || e.message || 'Failed to get history';
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Registers - AntTP Explorer</title>
</svelte:head>

<div class="space-y-8">
	<div>
		<h1 class="text-3xl font-bold text-gray-900">📝 Registers</h1>
		<p class="text-gray-600 mt-2">Mutable key-value storage with complete history tracking</p>
	</div>

	<div class="card bg-green-50 border-2 border-green-200">
		<h3 class="font-bold text-green-900 mb-2">What are Registers?</h3>
		<p class="text-green-700 text-sm">
			Registers are mutable storage that keeps a complete history of all changes.
			Like a ledger - you can see every update that was ever made!
		</p>
		<div class="mt-3 text-sm text-green-600">
			<strong>Use cases:</strong> Counters, status tracking, version control, audit logs
		</div>
	</div>

	<div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
		<!-- Create/Update -->
		<div class="card">
			<h2 class="text-xl font-bold mb-4">Create / Update Register</h2>
			
			<div class="space-y-4">
				<div>
					<label class="label">Name</label>
					<input
						type="text"
						bind:value={name}
						placeholder="register-name"
						class="input"
					/>
				</div>

				<div>
					<label class="label">Content</label>
					<textarea
						bind:value={hexContent}
						placeholder="Your content here..."
						rows="4"
						class="textarea"
					></textarea>
					<p class="text-xs text-gray-500 mt-1">Will be converted to hex encoding</p>
				</div>

				<div>
					<label class="label">Address (for updates)</label>
					<input
						type="text"
						bind:value={address}
						placeholder="Leave empty to create new"
						class="input"
					/>
				</div>

				<div class="flex space-x-2">
					<button
						on:click={createRegister}
						disabled={loading || !name || !hexContent}
						class="btn btn-primary flex-1"
					>
						Create New
					</button>
					<button
						on:click={updateRegister}
						disabled={loading || !address || !name || !hexContent}
						class="btn btn-secondary flex-1"
					>
						Update Existing
					</button>
				</div>

				{#if address && success.includes('created')}
					<div class="bg-green-50 border border-green-200 rounded-lg p-4">
						<p class="text-sm font-medium text-green-900">Address:</p>
						<code class="text-xs text-green-700 break-all">{address}</code>
					</div>
				{/if}
			</div>
		</div>

		<!-- Get & History -->
		<div class="card">
			<h2 class="text-xl font-bold mb-4">Retrieve & History</h2>
			
			<div class="space-y-4">
				<div>
					<label class="label">Register Address</label>
					<input
						type="text"
						bind:value={address}
						placeholder="Enter address..."
						class="input"
					/>
				</div>

				<div class="flex space-x-2">
					<button
						on:click={getRegister}
						disabled={loading || !address}
						class="btn btn-primary flex-1"
					>
						Get Current
					</button>
					<button
						on:click={getRegisterHistory}
						disabled={loading || !address}
						class="btn btn-secondary flex-1"
					>
						Get History
					</button>
				</div>

				{#if retrievedContent}
					<div class="bg-gray-50 border border-gray-200 rounded-lg p-4">
						<p class="text-sm font-medium text-gray-900 mb-2">Current Value:</p>
						<div class="bg-white border rounded p-3 text-sm">{retrievedContent}</div>
					</div>
				{/if}

				{#if history.length > 0}
					<div class="bg-gray-50 border border-gray-200 rounded-lg p-4">
						<p class="text-sm font-medium text-gray-900 mb-2">History ({history.length} entries):</p>
						<div class="space-y-2 max-h-64 overflow-y-auto">
							{#each history as entry, i}
								<div class="bg-white border rounded p-2 text-xs">
									<div class="font-medium text-gray-600">Entry {i + 1}</div>
									<div class="text-gray-800">{entry.content}</div>
								</div>
							{/each}
						</div>
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
				<span>/anttp-0/register</span>
			</div>
			<div class="flex items-center space-x-2">
				<span class="bg-yellow-100 text-yellow-700 px-2 py-1 rounded">PUT</span>
				<span>/anttp-0/register/{"{address}"}</span>
			</div>
			<div class="flex items-center space-x-2">
				<span class="bg-blue-100 text-blue-700 px-2 py-1 rounded">GET</span>
				<span>/anttp-0/register/{"{address}"}</span>
			</div>
			<div class="flex items-center space-x-2">
				<span class="bg-blue-100 text-blue-700 px-2 py-1 rounded">GET</span>
				<span>/anttp-0/register_history/{"{address}"}</span>
			</div>
		</div>
	</div>
</div>
