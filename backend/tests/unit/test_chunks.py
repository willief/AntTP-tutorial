"""Unit tests for chunk service - TDD approach."""
import pytest
from unittest.mock import AsyncMock, patch
import base64


class TestChunkService:
    """Test suite for ChunkService following TDD."""
    
    @pytest.mark.asyncio
    async def test_create_chunk_success(
        self, 
        mock_anttp_client, 
        sample_base64_content,
        sample_chunk_address,
        mock_anttp_response
    ):
        """Test creating a chunk successfully."""
        # Arrange
        from app.services.chunks import ChunkService
        
        mock_response = mock_anttp_response(
            status_code=200,
            json_data={
                "address": sample_chunk_address,
                "store_type": "memory"
            }
        )
        mock_anttp_client.post.return_value = mock_response
        
        service = ChunkService(anttp_client=mock_anttp_client)
        
        # Act
        result = await service.create_chunk(
            content=sample_base64_content,
            store_type="memory"
        )
        
        # Assert
        assert result["address"] == sample_chunk_address
        assert result["store_type"] == "memory"
        mock_anttp_client.post.assert_called_once()
        
    @pytest.mark.asyncio
    async def test_create_chunk_validates_base64(self, mock_anttp_client):
        """Test that invalid base64 content raises error."""
        # Arrange
        from app.services.chunks import ChunkService
        service = ChunkService(anttp_client=mock_anttp_client)
        
        # Act & Assert
        with pytest.raises(ValueError, match="Invalid base64"):
            await service.create_chunk(
                content="not-valid-base64!@#",
                store_type="memory"
            )
    
    @pytest.mark.asyncio
    async def test_get_chunk_success(
        self,
        mock_anttp_client,
        sample_chunk_address,
        sample_base64_content,
        mock_anttp_response
    ):
        """Test retrieving a chunk successfully."""
        # Arrange
        from app.services.chunks import ChunkService
        
        mock_response = mock_anttp_response(
            status_code=200,
            json_data={
                "address": sample_chunk_address,
                "content": sample_base64_content,
                "store_type": "memory"
            }
        )
        mock_anttp_client.get.return_value = mock_response
        
        service = ChunkService(anttp_client=mock_anttp_client)
        
        # Act
        result = await service.get_chunk(sample_chunk_address)
        
        # Assert
        assert result["address"] == sample_chunk_address
        assert result["content"] == sample_base64_content
        assert "store_type" in result
        
    @pytest.mark.asyncio
    async def test_get_chunk_not_found(
        self,
        mock_anttp_client,
        sample_chunk_address,
        mock_anttp_response
    ):
        """Test retrieving non-existent chunk."""
        # Arrange
        from app.services.chunks import ChunkService
        
        mock_response = mock_anttp_response(status_code=404)
        mock_anttp_client.get.return_value = mock_response
        
        service = ChunkService(anttp_client=mock_anttp_client)
        
        # Act & Assert
        with pytest.raises(Exception, match="404"):
            await service.get_chunk(sample_chunk_address)
    
    @pytest.mark.asyncio
    async def test_create_chunk_with_network_storage(
        self,
        mock_anttp_client,
        sample_base64_content,
        sample_chunk_address,
        mock_anttp_response
    ):
        """Test creating chunk with network storage."""
        # Arrange
        from app.services.chunks import ChunkService
        
        mock_response = mock_anttp_response(
            status_code=200,
            json_data={
                "address": sample_chunk_address,
                "store_type": "network"
            }
        )
        mock_anttp_client.post.return_value = mock_response
        
        service = ChunkService(anttp_client=mock_anttp_client)
        
        # Act
        result = await service.create_chunk(
            content=sample_base64_content,
            store_type="network"
        )
        
        # Assert
        assert result["store_type"] == "network"
        call_args = mock_anttp_client.post.call_args
        assert "x-cache-only" not in call_args[1].get("headers", {})
