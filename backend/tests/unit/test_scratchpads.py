"""Unit tests for scratchpad service - TDD approach."""
import pytest
from unittest.mock import AsyncMock


class TestScratchpadService:
    """Test suite for ScratchpadService following TDD."""
    
    @pytest.mark.asyncio
    async def test_create_public_scratchpad_success(
        self,
        mock_anttp_client,
        sample_scratchpad_address,
        mock_anttp_response
    ):
        """Test creating a public scratchpad successfully."""
        # Arrange
        from app.services.scratchpads import ScratchpadService
        
        mock_response = mock_anttp_response(
            status_code=200,
            json_data={
                "address": sample_scratchpad_address,
                "store_type": "memory"
            }
        )
        mock_anttp_client.post.return_value = mock_response
        
        service = ScratchpadService(anttp_client=mock_anttp_client)
        
        # Act
        result = await service.create_public_scratchpad(
            content="test data",
            store_type="memory"
        )
        
        # Assert
        assert result["address"] == sample_scratchpad_address
        assert result["is_private"] is False
        
    @pytest.mark.asyncio
    async def test_create_private_scratchpad_success(
        self,
        mock_anttp_client,
        sample_scratchpad_address,
        mock_anttp_response
    ):
        """Test creating a private scratchpad successfully."""
        # Arrange
        from app.services.scratchpads import ScratchpadService
        
        mock_response = mock_anttp_response(
            status_code=200,
            json_data={
                "address": sample_scratchpad_address,
                "store_type": "memory"
            }
        )
        mock_anttp_client.post.return_value = mock_response
        
        service = ScratchpadService(anttp_client=mock_anttp_client)
        
        # Act
        result = await service.create_private_scratchpad(
            content="secret data",
            name="my-secret",
            store_type="memory"
        )
        
        # Assert
        assert result["address"] == sample_scratchpad_address
        assert result["is_private"] is True
        
    @pytest.mark.asyncio
    async def test_update_public_scratchpad_success(
        self,
        mock_anttp_client,
        sample_scratchpad_address,
        mock_anttp_response
    ):
        """Test updating a public scratchpad successfully."""
        # Arrange
        from app.services.scratchpads import ScratchpadService
        
        mock_response = mock_anttp_response(status_code=200)
        mock_anttp_client.put.return_value = mock_response
        
        service = ScratchpadService(anttp_client=mock_anttp_client)
        
        # Act
        result = await service.update_public_scratchpad(
            address=sample_scratchpad_address,
            content="updated data"
        )
        
        # Assert
        assert result["success"] is True
        mock_anttp_client.put.assert_called_once()
        
    @pytest.mark.asyncio
    async def test_get_public_scratchpad_success(
        self,
        mock_anttp_client,
        sample_scratchpad_address,
        mock_anttp_response
    ):
        """Test retrieving a public scratchpad successfully."""
        # Arrange
        from app.services.scratchpads import ScratchpadService
        
        mock_response = mock_anttp_response(
            status_code=200,
            json_data={
                "address": sample_scratchpad_address,
                "content": "test data"
            }
        )
        mock_anttp_client.get.return_value = mock_response
        
        service = ScratchpadService(anttp_client=mock_anttp_client)
        
        # Act
        result = await service.get_public_scratchpad(sample_scratchpad_address)
        
        # Assert
        assert result["address"] == sample_scratchpad_address
        assert result["content"] == "test data"
        
    @pytest.mark.asyncio
    async def test_get_private_scratchpad_requires_name(
        self,
        mock_anttp_client,
        sample_scratchpad_address
    ):
        """Test that getting private scratchpad requires name."""
        # Arrange
        from app.services.scratchpads import ScratchpadService
        service = ScratchpadService(anttp_client=mock_anttp_client)
        
        # Act & Assert
        with pytest.raises(ValueError, match="name is required"):
            await service.get_private_scratchpad(
                address=sample_scratchpad_address,
                name=None
            )
