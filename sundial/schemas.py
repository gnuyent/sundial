from typing import List, Optional

from pydantic import BaseModel


class Course(BaseModel):
    id: str
    url: str
    period: int
    course: str
    course_title: str
    section: str
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


class Meeting(BaseModel):
    course_id: str
    meeting_id: str
    meeting_type: str
    hours: str
    days: str
    location: str
    instructor: str

    class Config:
        orm_mode = True


class ScheduleParameters(BaseModel):
    # Do optional blocks
    around_time: str = "0000"
    bad_day: str = ""
    earliest_time: str = "0000"
    include_courses: List[int] = []
    include_professors: List[str] = []
    latest_time: str = "0000"
    maximum_time_distance: int = 0
    prefer_no_waitlist: bool = True
    course_list: List[str]
