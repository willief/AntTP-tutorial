/**
 * API client for chunks endpoints
 */
import axios, { type AxiosInstance } from 'axios';

export interface ChunkResponse {
  address: string;
  content?: string;
  store_type: string;
}

export class ChunksAPI {
  private client: AxiosInstance;

  constructor(baseURL: string) {
    this.client = axios.create({
      baseURL,
      timeout: 30000,
      headers: {
        'Content-Type': 'application/json'
      }
    });
  }

  async createChunk(content: string, storeType: string = 'network'): Promise<ChunkResponse> {
    const response = await this.client.post('/chunks', {
      content,
      store_type: storeType
    });
    return response.data;
  }

  async getChunk(address: string): Promise<ChunkResponse> {
    const response = await this.client.get(`/chunks/${address}`);
    return response.data;
  }
}

/**
 * Main API client aggregating all endpoints
 */
export class APIClient {
  public chunks: ChunksAPI;
  
  constructor(baseURL: string = 'http://localhost:8000/api/v1') {
    this.chunks = new ChunksAPI(baseURL);
  }
}

// Export singleton instance
export const apiClient = new APIClient(
  import.meta.env.VITE_API_BASE_URL || 'http://localhost:8000/api/v1'
);
