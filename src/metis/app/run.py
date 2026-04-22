import uvicorn

from metis.rest.app import app


def main():
    uvicorn.run(app, host="0.0.0.0", port=8100, reload=True)
