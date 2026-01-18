import os
from pydantic_settings import BaseSettings

class Settings(BaseSettings):
    anttp_base_url: str = os.getenv("ANTP_BASE_URL", "http://localhost:8080")
    anttp_api_url: str = os.getenv("ANTP_API_URL", "http://localhost:8080")
    
settings = Settings()
