from fastapi import FastAPI
app = FastAPI()
@app.get("/health")
def health(): return {"status": "mock"}
@app.get("/api/v1/chunks")
def chunks(): return []
