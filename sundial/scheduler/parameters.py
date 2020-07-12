from datetime import time
from typing import List

from sundial.scheduler.period import DateTime, Day


class Parameters:
    """Data storage for user-given preferences.

    `Parameters` properly checks and formats data user-defined restrictions so
    that it can be processed later.

    Parameters
    ----------
    around_time : str
        Time to check.
    """

    def __init__(
        self,
        around_time: str = "0000",
        bad_day: str = "",
        earliest_time: str = "0000",
        include_courses: List[int] = [],
        include_professors: List[str] = [],
        latest_time: str = "0000",
        maximum_time_distance: int = 0,
        prefer_no_waitlist: bool = True,
    ):
        if 0 <= maximum_time_distance <= 86340:
            self.around_time: time = DateTime.parse_single_time(around_time)
            self.bad_day: List[Day] = Day.parse_days(bad_day)
            self.earliest_time: time = DateTime.parse_single_time(earliest_time)
            self.include_courses: List[int] = include_courses
            self.include_professors: List[str] = include_professors
            self.latest_time: time = DateTime.parse_single_time(earliest_time)
            self.maximum_time_distance: int = maximum_time_distance
            self.prefer_no_waitlist: bool = prefer_no_waitlist
        else:
            raise ValueError(
                "Make sure the parameter 'maximum_time_distance' is within 0 and 86340."
            )
