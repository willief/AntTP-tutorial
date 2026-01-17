"""Scratchpad service for interacting with AntTP scratchpads API."""
import httpx
from typing import Optional, Any
from app.config import get_settings


class ScratchpadService:
    """Service for managing scratchpads on Autonomi Network."""
    
    def __init__(self, anttp_client: Optional[httpx.AsyncClient] = None):
        """Initialize scratchpad service."""
        self.settings = get_settings()
        self.client = anttp_client or httpx.AsyncClient(
            base_url=self.settings.anttp_base_url,
            timeout=self.settings.anttp_timeout
        )
    
    async def create_public_scratchpad(
        self,
        content: str,
        store_type: str = "network"
    ) -> dict[str, Any]:
        """Create a new public scratchpad.
        
        Args:
            content: Content to store
            store_type: Storage type (memory, disk, or network)
            
        Returns:
            Dict with address, is_private, and store_type
        """
        headers = {}
        if store_type in ("memory", "disk"):
            headers["x-cache-only"] = "true"
        
        payload = {"content": content}
        
        response = await self.client.post(
            "/api/v1/scratchpads/public",
            json=payload,
            headers=headers
        )
        
        if not response.is_success:
            raise Exception(f"Failed to create public scratchpad: {response.status_code}")
        
        data = response.json()
        return {
            "address": data["address"],
            "is_private": False,
            "store_type": store_type
        }
    
    async def create_private_scratchpad(
        self,
        content: str,
        name: str,
        store_type: str = "network"
    ) -> dict[str, Any]:
        """Create a new private (encrypted) scratchpad.
        
        Args:
            content: Content to store
            name: Name for the private scratchpad
            store_type: Storage type
            
        Returns:
            Dict with address, is_private, and store_type
        """
        headers = {}
        if store_type in ("memory", "disk"):
            headers["x-cache-only"] = "true"
        
        payload = {
            "content": content,
            "name": name
        }
        
        response = await self.client.post(
            "/api/v1/scratchpads/private",
            json=payload,
            headers=headers
        )
        
        if not response.is_success:
            raise Exception(f"Failed to create private scratchpad: {response.status_code}")
        
        data = response.json()
        return {
            "address": data["address"],
            "is_private": True,
            "store_type": store_type
        }
    
    async def update_public_scratchpad(
        self,
        address: str,
        content: str,
        store_type: str = "network"
    ) -> dict[str, Any]:
        """Update a public scratchpad.
        
        Args:
            address: Scratchpad address
            content: New content
            store_type: Storage type
            
        Returns:
            Dict with success status
        """
        headers = {}
        if store_type in ("memory", "disk"):
            headers["x-cache-only"] = "true"
        
        payload = {"content": content}
        
        response = await self.client.put(
            f"/api/v1/scratchpads/public/{address}",
            json=payload,
            headers=headers
        )
        
        if not response.is_success:
            raise Exception(f"Failed to update scratchpad: {response.status_code}")
        
        return {"success": True}
    
    async def get_public_scratchpad(self, address: str) -> dict[str, Any]:
        """Retrieve a public scratchpad.
        
        Args:
            address: Scratchpad address
            
        Returns:
            Dict with address and content
        """
        response = await self.client.get(f"/api/v1/scratchpads/public/{address}")
        
        if not response.is_success:
            raise Exception(f"Failed to get scratchpad: {response.status_code}")
        
        data = response.json()
        return {
            "address": data["address"],
            "content": data.get("content")
        }
    
    async def get_private_scratchpad(
        self,
        address: str,
        name: Optional[str]
    ) -> dict[str, Any]:
        """Retrieve a private scratchpad.
        
        Args:
            address: Scratchpad address
            name: Name used when creating the scratchpad
            
        Returns:
            Dict with address and content
            
        Raises:
            ValueError: If name is not provided
        """
        if not name:
            raise ValueError("name is required for private scratchpads")
        
        response = await self.client.get(
            f"/api/v1/scratchpads/private/{address}",
            params={"name": name}
        )
        
        if not response.is_success:
            raise Exception(f"Failed to get private scratchpad: {response.status_code}")
        
        data = response.json()
        return {
            "address": data["address"],
            "content": data.get("content")
        }
    
    async def close(self):
        """Close the HTTP client."""
        await self.client.aclose()
