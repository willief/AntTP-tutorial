"""Pydantic models for API request/response schemas."""
from pydantic import BaseModel, Field
from typing import Optional, Any
from datetime import datetime


# Base Models
class ResponseBase(BaseModel):
    """Base response model."""
    success: bool = True
    message: Optional[str] = None


# Chunk Models
class ChunkCreateRequest(BaseModel):
    """Request model for creating a chunk."""
    content: str = Field(..., description="Base64-encoded content")
    store_type: str = Field(default="network", description="Storage type: memory, disk, or network")


class ChunkResponse(BaseModel):
    """Response model for chunk operations."""
    address: str = Field(..., description="Chunk address (XOR)")
    content: Optional[str] = Field(None, description="Base64-encoded content")
    store_type: str


# Scratchpad Models
class ScratchpadCreateRequest(BaseModel):
    """Request model for creating a scratchpad."""
    content: str = Field(..., description="Content to store")
    is_private: bool = Field(default=False, description="Whether scratchpad is encrypted")
    name: Optional[str] = Field(None, description="Name for private scratchpad")
    store_type: str = Field(default="network", description="Storage type")


class ScratchpadUpdateRequest(BaseModel):
    """Request model for updating a scratchpad."""
    content: str = Field(..., description="New content")
    store_type: str = Field(default="network", description="Storage type")


class ScratchpadResponse(BaseModel):
    """Response model for scratchpad operations."""
    address: str
    content: Optional[str] = None
    is_private: bool
    store_type: str


# Register Models
class RegisterCreateRequest(BaseModel):
    """Request model for creating a register."""
    content: str = Field(..., description="Hex-encoded content")
    name: str = Field(..., description="Register name")
    store_type: str = Field(default="network", description="Storage type")


class RegisterUpdateRequest(BaseModel):
    """Request model for updating a register."""
    content: str = Field(..., description="New hex-encoded content")
    store_type: str = Field(default="network", description="Storage type")


class RegisterEntry(BaseModel):
    """Single register history entry."""
    content: str
    timestamp: Optional[datetime] = None


class RegisterResponse(BaseModel):
    """Response model for register operations."""
    address: str
    content: Optional[str] = None
    store_type: str


class RegisterHistoryResponse(BaseModel):
    """Response model for register history."""
    address: str
    history: list[RegisterEntry]


# Archive Models
class FileItem(BaseModel):
    """File item for archive."""
    name: str = Field(..., description="File name")
    content: str = Field(..., description="Base64-encoded file content")


class ArchiveCreateRequest(BaseModel):
    """Request model for creating an archive."""
    files: list[FileItem] = Field(..., description="Files to include in archive")
    store_type: str = Field(default="network", description="Storage type")


class ArchiveResponse(BaseModel):
    """Response model for archive operations."""
    address: str
    files: Optional[list[str]] = None
    store_type: str


# Public Data Models
class PublicDataCreateRequest(BaseModel):
    """Request model for creating public data."""
    content: str = Field(..., description="Base64-encoded content")
    mime_type: Optional[str] = Field(None, description="MIME type")
    store_type: str = Field(default="network", description="Storage type")


class PublicDataResponse(BaseModel):
    """Response model for public data operations."""
    address: str
    content: Optional[str] = None
    mime_type: Optional[str] = None
    store_type: str


# Graph Models
class GraphEntryCreateRequest(BaseModel):
    """Request model for creating a graph entry."""
    content: dict = Field(..., description="Graph entry data")
    store_type: str = Field(default="network", description="Storage type")


class GraphEntryResponse(BaseModel):
    """Response model for graph entry operations."""
    address: str
    content: Optional[dict] = None
    store_type: str


# Pointer Models
class PointerCreateRequest(BaseModel):
    """Request model for creating a pointer."""
    target_address: str = Field(..., description="Target network address")
    name: Optional[str] = Field(None, description="Pointer name")
    store_type: str = Field(default="network", description="Storage type")


class PointerUpdateRequest(BaseModel):
    """Request model for updating a pointer."""
    target_address: str = Field(..., description="New target address")
    store_type: str = Field(default="network", description="Storage type")


class PointerResponse(BaseModel):
    """Response model for pointer operations."""
    address: str
    target_address: Optional[str] = None
    store_type: str


# PNR Models
class PNRRecordCreate(BaseModel):
    """PNR record for zone creation."""
    sub_name: str = Field(default="", description="Subdomain name")
    address: str = Field(..., description="Target address")
    record_type: str = Field(default="X", description="Record type")
    ttl: int = Field(default=60, description="Time to live in seconds")


class PNRZoneCreateRequest(BaseModel):
    """Request model for creating a PNR zone."""
    name: str = Field(..., description="Zone name")
    default_record: PNRRecordCreate = Field(..., description="Default zone record")
    store_type: str = Field(default="network", description="Storage type")


class PNRZoneResponse(BaseModel):
    """Response model for PNR zone operations."""
    name: str
    personal_address: str
    resolver_address: str
    records: list[dict]


# Command Models
class CommandInfo(BaseModel):
    """Command information."""
    id: int
    command_name: str
    state: str
    waiting_at: Optional[int] = None
    running_at: Optional[int] = None
    terminated_at: Optional[int] = None


class CommandListResponse(BaseModel):
    """Response model for command list."""
    commands: list[CommandInfo]
    count: int
