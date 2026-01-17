"""API router for chunks endpoints."""
from fastapi import APIRouter, HTTPException, Header
from typing import Optional
from app.models import ChunkCreateRequest, ChunkResponse
from app.services.chunks import ChunkService


router = APIRouter(prefix="/chunks", tags=["chunks"])


@router.post("", response_model=ChunkResponse, status_code=200)
async def create_chunk(
    request: ChunkCreateRequest,
    x_cache_only: Optional[str] = Header(None)
):
    """Create a new chunk with base64-encoded content.
    
    Args:
        request: Chunk creation request
        x_cache_only: Optional header to cache only (not upload to network)
        
    Returns:
        ChunkResponse with address and store type
        
    Raises:
        HTTPException: If creation fails
    """
    service = ChunkService()
    
    try:
        # Adjust store_type based on header
        store_type = request.store_type
        if x_cache_only:
            store_type = "memory" if store_type == "network" else store_type
        
        result = await service.create_chunk(
            content=request.content,
            store_type=store_type
        )
        
        return ChunkResponse(**result)
        
    except ValueError as e:
        raise HTTPException(status_code=400, detail=str(e))
    except Exception as e:
        raise HTTPException(status_code=503, detail=f"AntTP error: {str(e)}")
    finally:
        await service.close()


@router.get("/{address}", response_model=ChunkResponse)
async def get_chunk(address: str):
    """Retrieve a chunk by its address.
    
    Args:
        address: Chunk XOR address
        
    Returns:
        ChunkResponse with address and content
        
    Raises:
        HTTPException: If chunk not found or retrieval fails
    """
    service = ChunkService()
    
    try:
        result = await service.get_chunk(address)
        return ChunkResponse(**result)
        
    except Exception as e:
        if "404" in str(e):
            raise HTTPException(status_code=404, detail="Chunk not found")
        raise HTTPException(status_code=503, detail=f"AntTP error: {str(e)}")
    finally:
        await service.close()
