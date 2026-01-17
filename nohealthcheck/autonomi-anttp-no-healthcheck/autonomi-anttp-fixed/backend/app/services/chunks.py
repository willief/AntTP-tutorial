"""Chunk service for interacting with AntTP chunks API."""
import httpx
import base64
from typing import Optional, Any
from app.config import get_settings


class ChunkService:
    """Service for managing chunks on Autonomi Network."""
    
    def __init__(self, anttp_client: Optional[httpx.AsyncClient] = None):
        """Initialize chunk service.
        
        Args:
            anttp_client: Optional HTTP client for testing
        """
        self.settings = get_settings()
        self.client = anttp_client or httpx.AsyncClient(
            base_url=self.settings.anttp_base_url,
            timeout=self.settings.anttp_timeout
        )
    
    def _validate_base64(self, content: str) -> None:
        """Validate that content is valid base64.
        
        Args:
            content: Base64-encoded string
            
        Raises:
            ValueError: If content is not valid base64
        """
        try:
            base64.b64decode(content, validate=True)
        except Exception:
            raise ValueError("Invalid base64 content")
    
    async def create_chunk(
        self, 
        content: str, 
        store_type: str = "network"
    ) -> dict[str, Any]:
        """Create a new chunk with the given content.
        
        Args:
            content: Base64-encoded content
            store_type: Storage type (memory, disk, or network)
            
        Returns:
            Dict with address and store_type
            
        Raises:
            ValueError: If content is invalid
            Exception: If creation fails
        """
        # Validate input
        self._validate_base64(content)
        
        # Prepare request
        headers = {}
        if store_type in ("memory", "disk"):
            headers["x-cache-only"] = "true"
        
        payload = {
            "content": content
        }
        
        # Make request
        response = await self.client.post(
            "/api/v1/chunks",
            json=payload,
            headers=headers
        )
        
        if not response.is_success:
            raise Exception(f"Failed to create chunk: {response.status_code}")
        
        data = response.json()
        return {
            "address": data["address"],
            "store_type": store_type
        }
    
    async def get_chunk(self, address: str) -> dict[str, Any]:
        """Retrieve a chunk by its address.
        
        Args:
            address: Chunk address (XOR)
            
        Returns:
            Dict with address, content, and store_type
            
        Raises:
            Exception: If chunk not found or retrieval fails
        """
        response = await self.client.get(f"/api/v1/chunks/{address}")
        
        if response.status_code == 404:
            raise Exception(f"Chunk not found: 404")
        
        if not response.is_success:
            raise Exception(f"Failed to get chunk: {response.status_code}")
        
        data = response.json()
        return {
            "address": data["address"],
            "content": data.get("content"),
            "store_type": data.get("store_type", "unknown")
        }
    
    async def close(self):
        """Close the HTTP client."""
        await self.client.aclose()
