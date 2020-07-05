from datetime import datetime, time
from enum import Enum
from typing import Dict, Tuple

from hourglass.tools import average_time, day_separator, time_sorter


class DateTime:
    def __init__(
        self, day: Day = Day.Online, start: time = time.min, end: time = time.min,
    ):
        self.day = day
        self.start = start
        self.end = end

    def parse_time(self, hour_range: str) -> List(time):
        """
        `parse_time` generates a list of two `datetime.time` objects from a string that contains a start and end time separated with a '-'.

        :param hour_range: String representation of the time in the format of 'HHMM-HHMM'. (default: "0000-0000)
        """
        try:
            hour_range = int(hour_range.split("-"))
            start_hour = int(hour_range[0][0:2])
            start_minute = int(hour_range[0][2:])
            end_hour = int(hour_range[1][0:2])
            end_minute = int(hour_range[1][2:])
        except IndexError:
            raise IndexError(
                "Input was invalid. Ensure that your string is formatted correctly."
            )

            start_time: time = datetime.time(start_hour, start_minute)
        end_time: time = datetime.time(end_hour, end_minute)

        return [start_time, end_time]


class Day(Enum):
    """
    `Day` represents all valid days that courses can be held.
    """

    Monday = 0
    Tuesday = 1
    Wednesday = 2
    Thursday = 3
    Friday = 4
    Online = 5
    Arranged = 6

    @staticmethod
    def _match_day(focus: str) -> Day:
        """
        `match_day` is a private helper function for `parse_days`. It matches strings to `Day`.
        """
        days = {
            "M": Day.Monday,
            "T": Day.Tuesday,
            "W": Day.Wednesday,
            "H": Day.Thursday,  # 'H' is matched since we iterate backwards in `parse_days`
            "F": Day.Friday,
            "O": Day.Online,
            "A": Day.Arranged,
        }
        return days[focus]

    @staticmethod
    def parse_days(days: str) -> List(Day):
        days_list = list(days)
        return_list = []

        while True:
            try:
                focus = days_list.pop()
                focus = Day._match_day(focus)
            except IndexError:
                break

            if focus == Day.Thursday:
                days_list.pop()  # pop twice to get rid of the 'T' in 'TH'
            return_list.append(focus)

        return_list.reverse()
        return return_list
