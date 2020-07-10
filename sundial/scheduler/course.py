from datetime import timedelta
from typing import List

from sundial.scheduler.meeting import Meeting
from sundial.scheduler.period import DateTime, Day


class Course:
    """
    Course holds all information regarding a specific course.

    Parameters
    ----------
    course :
        Abbreviation and number of the course.
    id : str
        Unique ID assigned to the course.
    meetings : List[Meeting]
        All meetings associated with the course.
    overlaps : bool
        True if course contains overlapping times, False otherwise. Used to determine what time to use (the one with the longest duration) in calculating schedule overlap. # noqa: E501
    schedule_num : int
        Official schedule number.
    seats_total : int
        Maximum possible seats in the course.
    seats_available : int
        Available seats in the course.
    waitlist : int
        True if course is waitlisted, False otherwise.
    """

    def __init__(
        self,
        course: str = "",
        id: str = "",
        meetings: List[Meeting] = [Meeting()],
        overlaps: bool = False,
        schedule_num: int = 0,
        seats_available: int = 0,
        seats_total: int = 0,
        waitlist: bool = True,
    ):
        self.course = course
        self.id = id
        self.meetings = meetings
        self.overlaps = overlaps
        self.schedule_num = schedule_num
        self.seats_available = seats_available
        self.seats_total = seats_total
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

        Some courses contain times that are overlapping e.g. Monday 0800-0850, Monday 0800-0950. In this case, we want to determine what the longest time is within that overlap. From the example before, this method would return Monday 0800-0950. # noqa: E501

        Returns
        -------
        DateTime
            DateTime representing the greatest difference in time.
        """
        times = [meeting.date for meeting in self.meetings]
        highest_index = 0
        highest_diff: float = 0
        for idx, meeting in enumerate(times):
            start: timedelta = timedelta(
                hours=meeting.start.hour, minutes=meeting.start.minute
            )
            end: timedelta = timedelta(
                hours=meeting.end.hour, minutes=meeting.end.minute
            )
            difference = (end - start).total_seconds()
            if difference > highest_diff:
                highest_diff = difference
                highest_index = idx
        return self.meetings[highest_index].date
