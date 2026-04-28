import logging
from dataclasses import dataclass

import uvicorn
from sqlalchemy import Engine
from sqlmodel import Session

from metis.app.db import engine, init_db
from metis.rest.app import app

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


@dataclass(frozen=True)
class RuntimeOptions:
    dev: bool = False


def init(db_engine: Engine) -> None:
    try:
        with Session(db_engine) as session:
            init_db(session)
    except Exception as exc:
        logger.error(exc)
        raise exc


def main(options: RuntimeOptions):
    if options.dev:
        logger.info("Starting Metis in dev mode")
        app_path = "metis.rest.app:app"
    else:
        logger.info("Starting Metis")
        app_path = app

    init(engine)
    uvicorn.run(app_path, host="0.0.0.0", port=8100, reload=options.dev)
