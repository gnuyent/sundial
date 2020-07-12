from typing import List

from fastapi import APIRouter, Depends, HTTPException
from sqlalchemy.orm import Session

from sundial import crud, schemas
from sundial.database import get_db

router = APIRouter()


@router.get("/{subject}", response_model=List[schemas.Course])
def read_subject(subject: str, db: Session = Depends(get_db)):
    db_courses = crud.get_subject(db, subject=subject)
    if db_courses is None:
        raise HTTPException(status_code=404, detail=f"No courses found in {subject}")
    return db_courses
