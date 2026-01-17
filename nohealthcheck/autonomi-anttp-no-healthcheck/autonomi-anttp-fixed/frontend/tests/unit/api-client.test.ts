/**
 * Unit tests for API client - TDD approach
 */
import { describe, it, expect, vi, beforeEach } from 'vitest';
import { ChunksAPI } from '$lib/api/chunks';
import axios from 'axios';

// Mock axios
vi.mock('axios');

describe('ChunksAPI', () => {
  let chunksAPI: ChunksAPI;
  const mockBaseURL = 'http://localhost:8000/api/v1';

  beforeEach(() => {
    vi.clearAllMocks();
    chunksAPI = new ChunksAPI(mockBaseURL);
  });

  it('should create a chunk successfully', async () => {
    // Arrange
    const mockResponse = {
      data: {
        address: '71b9fcd6d0fff9da53d2833ebc8d795527d28dfbcb90cee118be25ca57a63873',
        store_type: 'memory'
      }
    };
    vi.mocked(axios.post).mockResolvedValueOnce(mockResponse);

    // Act
    const result = await chunksAPI.createChunk('dGVzdCBjb250ZW50', 'memory');

    // Assert
    expect(result.address).toBe(mockResponse.data.address);
    expect(result.store_type).toBe('memory');
    expect(axios.post).toHaveBeenCalledWith(
      `${mockBaseURL}/chunks`,
      {
        content: 'dGVzdCBjb250ZW50',
        store_type: 'memory'
      }
    );
  });

  it('should get a chunk successfully', async () => {
    // Arrange
    const address = '71b9fcd6d0fff9da53d2833ebc8d795527d28dfbcb90cee118be25ca57a63873';
    const mockResponse = {
      data: {
        address,
        content: 'dGVzdCBjb250ZW50',
        store_type: 'network'
      }
    };
    vi.mocked(axios.get).mockResolvedValueOnce(mockResponse);

    // Act
    const result = await chunksAPI.getChunk(address);

    // Assert
    expect(result.address).toBe(address);
    expect(result.content).toBe('dGVzdCBjb250ZW50');
    expect(axios.get).toHaveBeenCalledWith(`${mockBaseURL}/chunks/${address}`);
  });

  it('should throw error when chunk not found', async () => {
    // Arrange
    const address = 'nonexistent';
    vi.mocked(axios.get).mockRejectedValueOnce({
      response: { status: 404, data: { detail: 'Chunk not found' } }
    });

    // Act & Assert
    await expect(chunksAPI.getChunk(address)).rejects.toThrow();
  });

  it('should handle network errors', async () => {
    // Arrange
    vi.mocked(axios.post).mockRejectedValueOnce(new Error('Network error'));

    // Act & Assert
    await expect(chunksAPI.createChunk('test', 'memory')).rejects.toThrow('Network error');
  });
});
