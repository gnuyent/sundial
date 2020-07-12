from typing import Any

from scrapy.utils.project import get_project_settings
from sqlalchemy import Column, Float, ForeignKey, Integer, String, Text, create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import relationship

# TODO: Refactor this code into PROJECT_ROOT/sundial/models.py
Base: Any = declarative_base()


def db_connect():
    """
    Performs database connection using database settings from settings.py.
    Returns sqlalchemy engine instance
    """
    return create_engine(get_project_settings().get("CONNECTION_STRING"))


def create_table(engine):
    Base.metadata.create_all(engine)


class Course(Base):
    __tablename__ = "course"

    id = Column("id", String(20), primary_key=True, sqlite_on_conflict_unique="REPLACE")
    url = Column("url", Text)
    period = Column("period", Integer)
    course = Column("course", Text)
    course_title = Column("course_title", Text)
    section = Column("section", String)
    schedule_num = Column("schedule_num", Integer)
    units = Column("units", Float)
    session = Column("session", Text)
    seats_available = Column("seats_available", Integer)
    seats_total = Column("seats_total", Integer)
    full_title = Column("full_title", Text)
    description = Column("description", Text)
    prerequisite = Column("prerequisite", Text)
    course_hours = Column("course_hours", Text)
    statement = Column("statement", Text)
    general_text = Column("general_text", Text)
    meetings = relationship("Meeting", backref="course")
    footnotes = relationship("Footnote", backref="course")


class Meeting(Base):
    __tablename__ = "meeting"

    course_id = Column("course_id", String(20), ForeignKey("course.id"))
    meeting_id = Column("meeting_id", String(20), primary_key=True)
    meeting_type = Column("meeting_type", Text)
    hours = Column("hours", Text)
    days = Column("days", Text)
    location = Column("location", Text)
    instructor = Column("instructor", Text)


class Footnote(Base):
    __tablename__ = "footnote"

    course_id = Column("course_id", String(20), ForeignKey("course.id"))
    footnote_id = Column("footnote_id", String(20), primary_key=True)
    code = Column("code", Text)
    text = Column("text", Text)
