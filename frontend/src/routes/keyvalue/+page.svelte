<svelte:head><title>Key/Value - AntTP Explorer</title></svelte:head>

<div class="space-y-8">
	<div>
		<h1 class="text-3xl font-bold">🗄️ Key/Value</h1>
		<p class="text-gray-600 mt-2">Object storage with buckets</p>
	</div>

	<div class="card bg-pink-50 border-2 border-pink-200">
		<h3 class="font-bold text-pink-900 mb-2">Bucket Storage</h3>
		<p class="text-pink-700 text-sm">
			Store objects in named buckets. Like AWS S3 or Google Cloud Storage!
		</p>
	</div>

	<div class="card">
		<h2 class="text-xl font-bold mb-4">Coming Soon</h2>
		<p class="text-gray-600">
			Key/Value UI is under development. API endpoints:
		</p>
		<div class="mt-4 space-y-2 text-sm font-mono bg-gray-50 p-4 rounded">
			<div class="flex items-center space-x-2">
				<span class="bg-green-100 text-green-700 px-2 py-1 rounded">POST</span>
				<span>/anttp-0/key_value</span>
			</div>
			<div class="flex items-center space-x-2">
				<span class="bg-blue-100 text-blue-700 px-2 py-1 rounded">GET</span>
				<span>/anttp-0/key_value/{"{bucket}"}/{"{object}"}</span>
			</div>
		</div>
		<p class="text-sm text-gray-500 mt-4">
			Example: bucket="photos", object="vacation.jpg"
		</p>
	</div>
</div>
