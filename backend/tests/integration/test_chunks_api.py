"""Integration tests for chunks API endpoints."""
import pytest
from httpx import AsyncClient
from app.main import app


class TestChunksAPI:
    """Integration tests for /api/v1/chunks endpoints."""
    
    @pytest.mark.asyncio
    async def test_create_chunk_endpoint(self, sample_base64_content):
        """Test POST /api/v1/chunks endpoint."""
        async with AsyncClient(app=app, base_url="http://test") as client:
            response = await client.post(
                "/api/v1/chunks",
                json={
                    "content": sample_base64_content,
                    "store_type": "memory"
                }
            )
            
            # Should return 200 or 503 if AntTP is not available
            assert response.status_code in (200, 503)
            
            if response.status_code == 200:
                data = response.json()
                assert "address" in data
                assert "store_type" in data
    
    @pytest.mark.asyncio
    async def test_create_chunk_invalid_base64(self):
        """Test POST /api/v1/chunks with invalid base64."""
        async with AsyncClient(app=app, base_url="http://test") as client:
            response = await client.post(
                "/api/v1/chunks",
                json={
                    "content": "not-base64!@#",
                    "store_type": "memory"
                }
            )
            
            # Should return 400 Bad Request
            assert response.status_code == 400
    
    @pytest.mark.asyncio
    async def test_get_chunk_endpoint(self, sample_chunk_address):
        """Test GET /api/v1/chunks/{address} endpoint."""
        async with AsyncClient(app=app, base_url="http://test") as client:
            response = await client.get(f"/api/v1/chunks/{sample_chunk_address}")
            
            # Should return 404 or 503 if AntTP is not available
            assert response.status_code in (404, 503)
