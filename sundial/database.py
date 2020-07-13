import os

from sqlalchemy import create_engine
from sqlalchemy.engine import Engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker

path = os.path.dirname(os.path.abspath(__file__))
SQLALCHEMY_DATABASE_URL = "sqlite:///" + path + "/classes.db"
print(SQLALCHEMY_DATABASE_URL)


Base = declarative_base()


def db_connect() -> Engine:
    """Perform database connection using database settings from settings.py.

    Returns
    -------
    Engine
        SQLAlchemy engine instance.
    """
    return create_engine(
        SQLALCHEMY_DATABASE_URL, connect_args={"check_same_thread": False}
    )


def create_table(engine: Engine):
    """Create database tables.

    Parameters
    ----------
    engine :
        SQLAlchemy engine instance.
    """
    Base.metadata.create_all(engine)


engine = db_connect()

SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)


def get_db():
    """Generate a local session."""
    db = SessionLocal()
    try:
        yield db
    finally:
        db.close()
