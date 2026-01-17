"""Pytest configuration and fixtures."""
import pytest
from unittest.mock import Mock, AsyncMock
from httpx import AsyncClient
import base64


@pytest.fixture
def mock_anttp_client():
    """Mock AntTP HTTP client."""
    client = Mock()
    client.post = AsyncMock()
    client.get = AsyncMock()
    client.put = AsyncMock()
    client.delete = AsyncMock()
    return client


@pytest.fixture
def sample_chunk_address():
    """Sample chunk address."""
    return "71b9fcd6d0fff9da53d2833ebc8d795527d28dfbcb90cee118be25ca57a63873"


@pytest.fixture
def sample_base64_content():
    """Sample base64-encoded content."""
    return base64.b64encode(b"test content").decode()


@pytest.fixture
def sample_register_address():
    """Sample register address."""
    return "b9a9fec366c03af7bd47ab5c63133dd5728652c2cdffe6c0b67ead0677cf8907"


@pytest.fixture
def sample_pointer_address():
    """Sample pointer address."""
    return "80fad1f709a2b5d9a614d42c28d2c81ba81988fc83ae9176eb701bedf8c654fb"


@pytest.fixture
def sample_scratchpad_address():
    """Sample scratchpad address."""
    return "a33082163be512fb471a1cca385332b32c19917deec3989a97e100d827f97baf"


@pytest.fixture
def mock_anttp_response():
    """Factory for mock AntTP responses."""
    def _create_response(status_code=200, json_data=None):
        response = Mock()
        response.status_code = status_code
        response.json = Mock(return_value=json_data or {})
        response.text = ""
        response.is_success = status_code < 400
        return response
    return _create_response
