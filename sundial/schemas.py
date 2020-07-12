from typing import List, Optional

from pydantic import BaseModel


class Course(BaseModel):
    id: str
    url: str
    period: int
    course: str
    course_title: str
    section: int
    schedule_num: int
    units: float
    session: str
    seats_available: int
    seats_total: int
    full_title: str
    description: Optional[str] = None
    prerequisite: Optional[str] = None
    course_hours: Optional[str] = None
    statement: Optional[str] = None
    general_text: Optional[str] = None

    class Config:
        orm_mode = True


class Schedule(BaseModel):
    schedules: List[int]


class ScheduleParameters(BaseModel):
    around_time: Optional[str] = "0000"
    bad_day: Optional[str] = ""
    earliest_time: Optional[str] = "0000"
    include_courses: Optional[List[int]] = []
    include_professors: Optional[List[str]] = []
    latest_time: Optional[str] = "0000"
    maximum_time_distance: Optional[int] = 0
    prefer_no_waitlist: Optional[bool] = True
