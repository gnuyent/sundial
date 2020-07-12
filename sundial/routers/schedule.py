from typing import List

from fastapi import APIRouter, Depends, HTTPException
from sqlalchemy.orm import Session

from sundial.scheduler.controller import Controller
from sundial.scheduler.parameters import Parameters
from sundial.schemas import Course, ScheduleParameters
from sundial.database import get_db
from sundial import crud

router = APIRouter()


@router.get("", response_model=List[Course])
def generate_schedules(parameters: ScheduleParameters, db: Session = Depends(get_db)):
    sp = Parameters(
        parameters.around_time,
        parameters.bad_day,
        parameters.earliest_time,
        parameters.include_courses,
        parameters.include_professors,
        parameters.latest_time,
        parameters.maximum_time_distance,
        parameters.prefer_no_waitlist,
    )
    controller = Controller(schedule_parameters=sp, course_list=parameters.course_list)
    controller.generate_schedules()
    schedule_nums = [
        course.schedule_num for course in controller.best_schedule().courses
    ]
    db_courses = crud.get_course_multiple(db, schedule_nums)
    if db_courses is None:
        raise HTTPException(status_code=404, detail="No schedule could be generated.")
    return db_courses
