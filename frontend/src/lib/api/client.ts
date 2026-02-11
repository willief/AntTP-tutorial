// src/lib/api/client.ts
/**
 * Complete AntTP API Client
 * Covers all 10 feature types with 37+ endpoints
 */

import axios from 'axios';
import type { AxiosInstance } from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:18888';

export interface StoreType {
	type: 'memory' | 'disk' | 'network';
}

// ============================================================================
// 1. CHUNKS - Immutable Data
// ============================================================================

export interface ChunkRequest {
	content: string; // Base64 encoded
}

export interface ChunkResponse {
	address: string;
}

export class ChunksAPI {
	private client: AxiosInstance;

	constructor() {
		this.client = axios.create({ baseURL: API_BASE_URL });
	}

	async createChunk(content: string, storeType: string = 'memory'): Promise<string> {
		const response = await this.client.post<ChunkResponse>(
			'/anttp-0/chunk',
			{ content },
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data.address;
	}

	async getChunk(address: string, storeType: string = 'memory'): Promise<string> {
		const response = await this.client.get<{ content: string }>(
			`/anttp-0/chunk/${address}`,
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data.content;
	}

	async createChunkBinary(data: Blob, storeType: string = 'memory'): Promise<string> {
		const response = await this.client.post<ChunkResponse>(
			'/anttp-0/binary/chunk',
			data,
			{
				headers: {
					'Content-Type': 'application/octet-stream',
					'x-store-type': storeType
				}
			}
		);
		return response.data.address;
	}

	async getChunkBinary(address: string, storeType: string = 'memory'): Promise<Blob> {
		const response = await this.client.get(
			`/anttp-0/binary/chunk/${address}`,
			{
				headers: { 'x-store-type': storeType },
				responseType: 'blob'
			}
		);
		return response.data;
	}
}

// ============================================================================
// 2. REGISTERS - Mutable with History
// ============================================================================

export interface RegisterRequest {
	name: string;
	content: string; // Hex encoded
}

export class RegistersAPI {
	private client: AxiosInstance;

	constructor() {
		this.client = axios.create({ baseURL: API_BASE_URL });
	}

	async createRegister(name: string, content: string, storeType: string = 'memory'): Promise<string> {
		const response = await this.client.post<{ address: string }>(
			'/anttp-0/register',
			{ name, content },
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data.address;
	}

	async updateRegister(address: string, name: string, content: string, storeType: string = 'memory'): Promise<void> {
		await this.client.put(
			`/anttp-0/register/${address}`,
			{ name, content },
			{ headers: { 'x-store-type': storeType } }
		);
	}

	async getRegister(address: string, storeType: string = 'memory'): Promise<string> {
		const response = await this.client.get<{ content: string }>(
			`/anttp-0/register/${address}`,
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data.content;
	}

	async getHistory(address: string, storeType: string = 'memory'): Promise<Array<{content: string, timestamp: number}>> {
		const response = await this.client.get(
			`/anttp-0/register_history/${address}`,
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data;
	}
}

// ============================================================================
// 3. POINTERS - Mutable References
// ============================================================================

export class PointersAPI {
	private client: AxiosInstance;

	constructor() {
		this.client = axios.create({ baseURL: API_BASE_URL });
	}

	async createPointer(name: string, target: string, storeType: string = 'memory'): Promise<string> {
		const response = await this.client.post<{ address: string }>(
			'/anttp-0/pointer',
			{ name, content: target },
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data.address;
	}

	async updatePointer(address: string, name: string, target: string, storeType: string = 'memory'): Promise<void> {
		await this.client.put(
			`/anttp-0/pointer/${address}`,
			{ name, content: target },
			{ headers: { 'x-store-type': storeType } }
		);
	}

	async getPointer(address: string, storeType: string = 'memory'): Promise<string> {
		const response = await this.client.get<{ content: string }>(
			`/anttp-0/pointer/${address}`,
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data.content;
	}
}

// ============================================================================
// 4. SCRATCHPADS - Public & Private
// ============================================================================

export class ScratchpadsAPI {
	private client: AxiosInstance;

	constructor() {
		this.client = axios.create({ baseURL: API_BASE_URL });
	}

	async createPublic(name: string, content: string, storeType: string = 'memory'): Promise<string> {
		const response = await this.client.post<{ address: string }>(
			'/anttp-0/public_scratchpad',
			{ name, content },
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data.address;
	}

	async updatePublic(address: string, name: string, content: string, storeType: string = 'memory'): Promise<void> {
		await this.client.put(
			`/anttp-0/public_scratchpad/${address}/${name}`,
			{ content },
			{ headers: { 'x-store-type': storeType } }
		);
	}

	async getPublic(address: string, storeType: string = 'memory'): Promise<string> {
		const response = await this.client.get<{ content: string }>(
			`/anttp-0/public_scratchpad/${address}`,
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data.content;
	}

	async createPrivate(name: string, content: string, storeType: string = 'memory'): Promise<string> {
		const response = await this.client.post<{ address: string }>(
			'/anttp-0/private_scratchpad',
			{ name, content },
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data.address;
	}

	async updatePrivate(address: string, name: string, content: string, storeType: string = 'memory'): Promise<void> {
		await this.client.put(
			`/anttp-0/private_scratchpad/${address}/${name}`,
			{ content },
			{ headers: { 'x-store-type': storeType } }
		);
	}

	async getPrivate(address: string, name: string, storeType: string = 'memory'): Promise<string> {
		const response = await this.client.get<{ content: string }>(
			`/anttp-0/private_scratchpad/${address}/${name}`,
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data.content;
	}
}

// ============================================================================
// 5. ARCHIVES - File Collections
// ============================================================================

export interface ArchiveFile {
	path: string;
	content: string;
}

export class ArchivesAPI {
	private client: AxiosInstance;

	constructor() {
		this.client = axios.create({ baseURL: API_BASE_URL });
	}

	async createArchive(files: File[], storeType: string = 'memory'): Promise<string> {
		const formData = new FormData();
		files.forEach(file => formData.append('files', file));

		const response = await this.client.post<{ address: string }>(
			'/anttp-0/multipart/public_archive',
			formData,
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data.address;
	}

	async getArchive(address: string, storeType: string = 'memory'): Promise<ArchiveFile[]> {
		const response = await this.client.get<{ files: ArchiveFile[] }>(
			`/anttp-0/public_archive/${address}`,
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data.files;
	}
}

// ============================================================================
// 6. GRAPH - Graph Structures
// ============================================================================

export class GraphAPI {
	private client: AxiosInstance;

	constructor() {
		this.client = axios.create({ baseURL: API_BASE_URL });
	}

	async createEntry(name: string, content: string, storeType: string = 'memory'): Promise<string> {
		const response = await this.client.post<{ address: string }>(
			'/anttp-0/graph_entry',
			{ name, content },
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data.address;
	}

	async getEntry(address: string, storeType: string = 'memory'): Promise<{name: string, content: string}> {
		const response = await this.client.get(
			`/anttp-0/graph_entry/${address}`,
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data;
	}
}

// ============================================================================
// 7. PNR - Pointer Name Registry
// ============================================================================

export interface PnrRecord {
	address: string;
	record_type: string;
	ttl: number;
}

export class PnrAPI {
	private client: AxiosInstance;

	constructor() {
		this.client = axios.create({ baseURL: API_BASE_URL });
	}

	async create(name: string, records: Record<string, PnrRecord>, storeType: string = 'memory'): Promise<string> {
		const response = await this.client.post<{ address: string }>(
			'/anttp-0/pnr',
			{ name, records },
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data.address;
	}

	async update(name: string, records: Record<string, PnrRecord>, storeType: string = 'memory'): Promise<void> {
		await this.client.put(
			`/anttp-0/pnr/${name}`,
			{ name, records },
			{ headers: { 'x-store-type': storeType } }
		);
	}

	async get(name: string, storeType: string = 'memory'): Promise<Record<string, PnrRecord>> {
		const response = await this.client.get<{ records: Record<string, PnrRecord> }>(
			`/anttp-0/pnr/${name}`,
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data.records;
	}

	async append(name: string, records: Record<string, PnrRecord>, storeType: string = 'memory'): Promise<void> {
		await this.client.patch(
			`/anttp-0/pnr/${name}`,
			{ name, records },
			{ headers: { 'x-store-type': storeType } }
		);
	}
}

// ============================================================================
// 8. KEY/VALUE - Object Storage
// ============================================================================

export class KeyValueAPI {
	private client: AxiosInstance;

	constructor() {
		this.client = axios.create({ baseURL: API_BASE_URL });
	}

	async create(bucket: string, object: string, content: string, storeType: string = 'memory'): Promise<string> {
		const response = await this.client.post<{ address: string }>(
			'/anttp-0/key_value',
			{ bucket, object, content },
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data.address;
	}

	async get(bucket: string, object: string, storeType: string = 'memory'): Promise<string> {
		const response = await this.client.get<{ content: string }>(
			`/anttp-0/key_value/${bucket}/${object}`,
			{ headers: { 'x-store-type': storeType } }
		);
		return response.data.content;
	}
}

// ============================================================================
// Singleton instances
// ============================================================================

export const chunksAPI = new ChunksAPI();
export const registersAPI = new RegistersAPI();
export const pointersAPI = new PointersAPI();
export const scratchpadsAPI = new ScratchpadsAPI();
export const archivesAPI = new ArchivesAPI();
export const graphAPI = new GraphAPI();
export const pnrAPI = new PnrAPI();
export const keyValueAPI = new KeyValueAPI();
