from datetime import timedelta
from typing import Dict, List

from hourglass.datetime import Day, DateTime
from hourglass.meeting import Meeting


class Course:
    """
    Course holds all information regarding a specific course.

    Parameters
    ----------
    course_hours : str
        Represents the weekly time needed for the course.
    course_title :
        Name of the course.
    course :
        Abbreviation and number of the course.
    description :
        Given description of the course.
    footnotes : Dict[str, str]
        Footnote notices about the course.
    full_title : str
        Expanded title of the course.
    general_text : str
        More general information about the course.
    id : str
        Unique ID assigned to the course.
    meetings : List[Meeting]
        All meetings associated with the course.
    overlaps : bool
        True if course contains overlapping times, False otherwise.
    period : int
        Time period of when the course exists.
    prerequisite : str
        Prerequisites to take the course.
    schedule_num : int
        Official schedule number.
    seats_available : int
        Maximum possible seats in the course.
    seats_open : int
        Available seats in the course.
    section : int
        Section of the course.
    session : str
        Session of the course. Typically the season.
    statement : str
        Course statement by the department.
    units : float
        Units given for the course.
    url : int
        Absolute URL for the course.
    waitlist : int
        True if course is waitlisted, False otherwise.
    """

    def __init__(
        self,
        course_hours: str = "",
        course_title: str = "",
        course: str = "",
        description: str = "",
        footnotes: Dict[str, str] = {},
        full_title: str = "",
        general_text: str = "",
        id: str = "",
        meetings: List[Meeting] = [Meeting()],
        overlaps: bool = False,
        period: int = 0,
        prerequisite: str = "",
        schedule_num: int = 0,
        seats_available: int = 0,
        seats_open: int = 0,
        section: int = 0,
        session: str = "",
        statement: str = "",
        units: float = 0.0,
        url: str = "",
        waitlist: bool = True,
    ):
        self.course_title = course_title
        self.course = course
        self.description = description
        self.footnotes = footnotes
        self.full_title = full_title
        self.general_text = general_text
        self.id = id
        self.meetings = meetings
        self.overlaps = overlaps
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
        """Format the output to include course name, start/end time, and waitlist status. # noqa: D400, E501"""
        out_string = f"{str(self.course)} {self.schedule_num}"
        if self.waitlist:
            out_string += " (W)"

        times: List[DateTime] = [meeting.date for meeting in self.meetings]
        times.sort()
        days = ""

        # The mess of code below combines days with the same time to one output.
        # Example: M 0800-0850, T 0800-0850 -> MT 0800-0850
        for time_idx in range(len(times)):
            focus = times[time_idx]
            if focus.day == Day.Thursday:
                days += "TH"
            elif focus.day == Day.Online:
                days += "ONLINE"
            elif focus.day == Day.Arranged:
                days += "ARRANGED"
            else:
                days += str(focus.day.name)[0]
            try:
                next = times[time_idx + 1]
                if focus.start == next.start and focus.end == next.end:
                    continue
            except IndexError:
                pass

            if focus.day == Day.Online or focus.day == Day.Arranged:
                out_string += f" {days}"
            else:
                start_time = str(focus.start.isoformat(timespec="minutes"))
                end_time = str(focus.end.isoformat(timespec="minutes"))
                out_string += f" {days} {start_time}-{end_time}"
            days = ""

        return out_string

    def get_longest_overlap(self) -> DateTime:
        """
        Calculate the longest time in an overlapping scenario.

        Returns
        -------
        DateTime
            DateTime representing the greatest difference in time.
        None
            If course does not overlap.

        """
        times = [meeting.date for meeting in self.meetings]
        highest_index = 0
        highest_diff: float = 0
        for idx in range(len(self.meetings)):
            start: timedelta = timedelta(
                hours=times[idx].start.hour, minutes=times[idx].start.minute
            )
            end: timedelta = timedelta(
                hours=times[idx].end.hour, minutes=times[idx].end.minute
            )
            difference = (end - start).total_seconds()
        if difference > highest_diff:
            highest_diff = difference
            highest_index = idx
        return self.meetings[highest_index].date
