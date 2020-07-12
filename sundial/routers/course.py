from typing import List

from fastapi import APIRouter, Depends, HTTPException
from sqlalchemy.orm import Session

from sundial import crud, schemas
from sundial.database import get_db

router = APIRouter()


@router.get("", response_model=List[schemas.Course])
def read_course_all(db: Session = Depends(get_db)):
    db_courses = crud.get_course_all(db)
    if db_courses is None:
        raise HTTPException(status_code=404, detail="No courses found.")
    return db_courses


@router.get("/unassigned", response_model=List[schemas.Course])
def read_course_unassigned(db: Session = Depends(get_db)):
    db_courses = crud.get_course_unassigned(db)
    if db_courses is None:
        raise HTTPException(status_code=404, detail="Unassigned courses not found.")
    return db_courses


@router.get("/{schedule_number}", response_model=schemas.Course)
def read_course(schedule_number: int, db: Session = Depends(get_db)):
    if schedule_number == 0:
        raise HTTPException(
            status_code=403, detail="Use /api/course/unassigned instead"
        )
    db_course = crud.get_course(db, schedule_number=schedule_number)
    if db_course is None:
        raise HTTPException(status_code=404, detail="Course not found.")
    return db_course
