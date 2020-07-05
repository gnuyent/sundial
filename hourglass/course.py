from datetime import datetime, time
from typing import Dict, Tuple

from hourglass.tools import average_time, day_separator, time_sorter


class Course:
    """`Course` is a data container from the database."""

    def __init__(
        self,
        course_hours="",
        course_title="",
        course="",
        description="",
        footnotes={},
        full_title="",
        general_text="",
        id="",
        meetings=[Meeting()],
        period=0,
        prerequisite="",
        schedule_num=0,
        seats_available=0,
        seats_open=0,
        section=0,
        session="",
        statement="",
        units=0.0,
        url="",
        waitlist=True,
    ):
        self.course_title = course_title
        self.course = course
        self.description = description
        self.footnotes = footnotes
        self.full_title = full_title
        self.general_text = general_text
        self.id = id
        self.meetings = meetings
        self.period = period
        self.prerequisite = prerequisite
        self.schedule_num = schedule_num
        self.seats_available = seats_available
        self.seats_open = seats_open
        self.section = section
        self.session = session
        self.statement = statement
        self.units = units
        self.url = url
        self.waitlist = waitlist

    def __repr__(self):
        zero_time = time()
        start_time = str(self.hour_start.isoformat(timespec="minutes"))
        end_time = str(self.hour_end.isoformat(timespec="minutes"))
        out_string = f"{self.course}"
        if self.waitlist:
            out_string += " (W)"
        if self.hour_start == zero_time and self.hour_end == zero_time:
            return f"{out_string} NO_TIME"
        else:
            return f"{out_string} {self.days} {start_time}-{end_time}"
