from sqlalchemy import Column, Float, ForeignKey, Integer, String, Text
from sqlalchemy.orm import relationship

from sundial.database import Base


class Course(Base):
    """SQLAlchemy model for the course table."""

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
    """SQLAlchemy model for the meeting table."""

    __tablename__ = "meeting"

    course_id = Column("course_id", String(20), ForeignKey("course.id"))
    meeting_id = Column("meeting_id", String(20), primary_key=True)
    meeting_type = Column("meeting_type", Text)
    hours = Column("hours", Text)
    days = Column("days", Text)
    location = Column("location", Text)
    instructor = Column("instructor", Text)


class Footnote(Base):
    """SQLAlchemy model for the footnote table."""

    __tablename__ = "footnote"

    course_id = Column("course_id", String(20), ForeignKey("course.id"))
    footnote_id = Column("footnote_id", String(20), primary_key=True)
    code = Column("code", Text)
    text = Column("text", Text)
