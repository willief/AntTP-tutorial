"""Application configuration."""
from pydantic_settings import BaseSettings
from functools import lru_cache


class Settings(BaseSettings):
    """Application settings."""
    
    # API Settings
    app_name: str = "Autonomi AntTP Explorer API"
    app_version: str = "1.0.0"
    api_prefix: str = "/api/v1"
    
    # AntTP Configuration
    anttp_base_url: str = "http://anttp:18888"
    anttp_timeout: int = 30
    
    # CORS
    cors_origins: list[str] = [
        "http://localhost:5173",
        "http://localhost:3000",
        "http://frontend:5173"
    ]
    
    # Logging
    log_level: str = "INFO"
    
    class Config:
        env_file = ".env"
        case_sensitive = False


@lru_cache()
def get_settings() -> Settings:
    """Get cached settings instance."""
    return Settings()
