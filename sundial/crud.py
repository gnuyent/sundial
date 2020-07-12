from sqlalchemy.orm import Session
from sundial import models
from typing import List


def get_course_all(db: Session):
    return db.query(models.Course).all()


def get_course_unassigned(db: Session):
    return db.query(models.Course).filter(models.Course.schedule_num == 0).all()


def get_course(db: Session, schedule_number: int):
    # TODO: Ensure that schedule number '0' (unassigned) is handled.
    return (
        db.query(models.Course)
        .filter(models.Course.schedule_num == schedule_number)
        .first()
    )


def get_course_multiple(db: Session, multiple: List[int]):
    return (
        db.query(models.Course).filter(models.Course.schedule_num.in_(multiple)).all()
    )


def get_subject(db: Session, subject: str):
    search = f"{subject}-%"
    return db.query(models.Course).filter(models.Course.course.like(search)).all()
