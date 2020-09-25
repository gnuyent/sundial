from typing import List, Optional

from pydantic import BaseModel


class Course(BaseModel):
    """Pydantic schema for the course table."""

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

    class Config:  # noqa: D106
        orm_mode = True


class Meeting(BaseModel):
    """Pydantic schema for the meeting table."""

    course_id: str
    meeting_id: str
    meeting_type: str
    hours: str
    days: str
    location: str
    instructor: str

    class Config:  # noqa: D106
        orm_mode = True


class ScheduleParameters(BaseModel):
    """Pydantic schema for the Parameters object."""

    around_time: Optional[str] = "0000"
    bad_day: Optional[str] = ""
    earliest_time: Optional[str] = "0000"
    include_courses: Optional[List[int]] = []
    include_professors: Optional[List[str]] = []
    latest_time: Optional[str] = "0000"
    maximum_time_distance: Optional[int] = 0
    prefer_no_waitlist: Optional[bool] = True
    course_list: Optional[List[str]] = []


class ScheduleList(BaseModel):
    schedules: List[List[Course]] = [[Course]]
