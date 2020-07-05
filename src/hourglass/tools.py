import sqlite3
from contextlib import contextmanager
from datetime import datetime, time, timedelta
from typing import List


@contextmanager
def connect_database(database_path: str, read_only: bool = True) -> None:
    connection = sqlite3.connect(f"{database_path}", uri=read_only)
    cursor = connection.cursor()
    yield cursor
    connection.close()


def flatten(arr: List[List]) -> List:
    """
    Flattens a 2-D array.

    :param arr: List[List] to flatten.
    :return: Flattened List.
    """
    for x in arr:
        if hasattr(x, "__iter__") and not isinstance(x, str):
            for y in flatten(x):
                yield y
        else:
            yield x


def day_separator(days: str) -> List[str]:
    """
    Separates days from a day string.

    :param days: Day string in the format of "MTWTHF"
    :return: List[str] of individual days.
    """
    if "TH" in days:
        split_days = days.partition("TH")
        split_days = list(filter(None, split_days))  # Remove empty indices
        try:
            if "TH" == split_days[1]:
                split_days[0] = list(split_days[0])
                split_days = list(flatten(split_days))
        except IndexError:
            pass
    else:
        split_days = list(days)

    return split_days


def time_sorter(courses: List, ascending: bool = False) -> List[time]:
    """
    Sort by start or end time.

    :param courses: List[Course] containing courses to sort by time.
    :param ascending: Boolean to sort times in ascending (True) or descending (False) order.
    :return: List[datetime.time] containing sorted course times in the specified order.
    """
    course_times = []
    if ascending is False:
        [course_times.append(course.hour_end) for course in courses]
    else:
        [course_times.append(course.hour_start) for course in courses]
    course_times.sort(reverse=not ascending)
    return course_times


def average_time(dates: List[time]) -> datetime:
    """
    Generates the average time from a list of times.

    :param dates: List[time] to find the average of.
    :return: datetime.datetime object that contains the average time from the list.
    """
    dates = [datetime(2020, 1, 1, time.hour, time.minute) for time in dates]
    reference_date = datetime(1900, 1, 1)
    return reference_date + sum(
        [date - reference_date for date in dates], timedelta()
    ) / len(dates)
