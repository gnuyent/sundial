"""Support for returning Day type in static method."""
from __future__ import annotations

from datetime import time
from enum import Enum
from typing import List


class Day(Enum):
    """`Day` represents all valid days that courses can be held."""

    Monday = 0
    Tuesday = 1
    Wednesday = 2
    Thursday = 3
    Friday = 4
    Online = 5
    Arranged = 6

    @staticmethod
    def _match_day(focus: str) -> Day:
        """Match strings to `Day`."""
        days = {
            "M": Day.Monday,
            "T": Day.Tuesday,
            "W": Day.Wednesday,
            "H": Day.Thursday,  # 'H' is matched since we iterate backwards in `parse_days` # noqa: E501
            "F": Day.Friday,
        }
        try:
            return days[focus]
        except KeyError:
            raise ValueError(
                f"{focus} was unable to be parsed. Ensure that the input is valid."
            )

    @staticmethod
    def parse_days(days: str) -> List[Day]:
        """
        Convert a string with different day abbreviations to a list of `Day`s.

        Parameters
        ----------
        days : str
               The string of days to convert from `["M", "T", "W", "TH", "F", "Online", "Arranged"]`. # noqa: E501
        """
        days_list = list(days)
        return_list = []

        if days == "":
            return [Day.Online]
        elif days == "Arranged":
            return [Day.Arranged]
        while True:
            try:
                focus: Day = Day._match_day(days_list.pop())
            except IndexError:
                break

            if focus == Day.Thursday:
                days_list.pop()  # pop twice to get rid of the 'T' in 'TH'
            return_list.append(focus)

        return_list.reverse()
        return return_list


class DateTime:
    """
    A wrapper around `datetime` to include the `Day` type and a start/end time.

    Parameters
    ----------
    day : Day
          `Day` to include.
    start : datetime.time
            Starting time.
    end : datetime.time
          Ending time.
    """

    def __init__(
        self, day: Day = Day.Online, start: time = time.min, end: time = time.min,
    ):
        self.day = day
        self.start = start
        self.end = end

    def __lt__(self, other):
        """True if self.start < other.start, False otherwise."""  # noqa: D401
        return self.start < other.start

    def __repr__(self):
        """Display start time, end time, and day."""
        start_time = str(self.start.isoformat(timespec="minutes"))
        end_time = str(self.end.isoformat(timespec="minutes"))
        out_string = f"{self.day.name} {start_time}-{end_time}"
        return out_string

    @staticmethod
    def parse_single_time(time_string: str) -> time:
        """
        Generate a `datetime.time` object from a valid string.

        Parameters
        ----------
        time_string : str
            String representation of the time in 24 hour format 'HHMM'.

        Returns
        -------
        time
            Time representing the input.
        """
        try:
            hour = int(time_string[0:2])
            minute = int(time_string[2:])
        except ValueError:
            return time.min
        except IndexError:
            raise IndexError(
                "Input was invalid. Ensure that your string is formatted correctly."
            )

        return time(hour, minute)

    @staticmethod
    def parse_time(range_string: str = "0000-0000") -> List[time]:
        """
        Generate a list of two `datetime.time` objects from a string.

        The string must be formatted so that it contains a start and end time separated
        with a '-'.

        Parameters
        ----------
        hour_range_string : str
            String representation of the time in the format of 'HHMM-HHMM'.
            (default: "0000-0000")

        Returns
        -------
        List[time]
            List of `datetime.time` in the same order as the given string.
        """
        try:
            hour_range: List[str] = range_string.split("-")
            start_hour = int(hour_range[0][0:2])
            start_minute = int(hour_range[0][2:])
            end_hour = int(hour_range[1][0:2])
            end_minute = int(hour_range[1][2:])
        except ValueError:
            return [time.min, time.min]
        except IndexError:
            raise IndexError(
                "Input was invalid. Ensure that your string is formatted correctly."
            )

        start_time: time = time(start_hour, start_minute)
        end_time: time = time(end_hour, end_minute)

        return [start_time, end_time]
