from datetime import time
from typing import List

from hourglass.datetime import Day


class ScheduleParameters:
    """
    Data storage for user-given preferences.

    `ScheduleParameters` properly checks and formats data user-defined restrictions so
    that it can be processed later.

    Parameters
    ----------

    """

    def __init__(
        self,
        around_time: time = time.min,
        bad_day: List(Day) = [],
        earliest_time: time = time.min(),
        include_courses=[],
        include_professors=[],
        latest_time: time = time.min,
        maximum_time_distance=0,
        prefer_no_waitlist=True,
    ):
        if 0 <= maximum_time_distance <= 86340:
            self.around_time = around_time
            self.bad_day = bad_day
            self.earliest_time = earliest_time
            self.include_courses = include_courses
            self.include_professors = include_professors
            self.latest_time = latest_time
            self.maximum_time_distance = maximum_time_distance
            self.prefer_no_waitlist = prefer_no_waitlist
        else:
            raise ValueError(
                "Make sure the parameter 'maximum_time_distance' is within 0 and 86340."
            )
