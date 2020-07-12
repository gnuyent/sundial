from typing import List

from sqlalchemy.orm import Session

from sundial import models


def get_course_all(db: Session):
    """Retrieve all courses.

    Parameters
    ----------
    db : Session
        SQLAlchemy connection to a database.
    """
    return db.query(models.Course).all()


def get_course_unassigned(db: Session):
    """Retrieve all unassigned (no schedule number) courses.

    Parameters
    ----------
    db : Session
        SQLAlchemy connection to a database.
    """
    return db.query(models.Course).filter(models.Course.schedule_num == 0).all()


def get_course(db: Session, schedule_number: int):
    """Retrieve a specified course by schedule number.

    Parameters
    ----------
    db : Session
        SQLAlchemy connection to a database.
    schedule_number : int
        Schedule number to search for.
    """
    # TODO: Ensure that schedule number '0' (unassigned) is handled.
    return (
        db.query(models.Course)
        .filter(models.Course.schedule_num == schedule_number)
        .first()
    )


def get_course_multiple(db: Session, multiple: List[int]):
    """Retrieve multiple courses by schedule number.

    Requires one or more entries within the list.

    Parameters
    ----------
    db : Session
        SQLAlchemy connection to a database.
    multiple : List[int]
        List of schedule numbers to search for.
    """
    # TODO: Add error handling here?
    return (
        db.query(models.Course).filter(models.Course.schedule_num.in_(multiple)).all()
    )


def get_subject(db: Session, subject: str):
    """Retrieve all courses by subject.

    Parameters
    ----------
    db : Session
        SQLAlchemy connection to a database.
    subject : str
        Subject to search for.
    """
    search = f"{subject}-%"
    return db.query(models.Course).filter(models.Course.course.like(search)).all()
