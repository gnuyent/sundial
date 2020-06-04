# Data containers
from datetime import datetime, time
from typing import Dict, Tuple

from core.backend.util.tools import average_time, day_separator, time_sorter


class Course:
    """
    The Course object holds important course information that needs to be quickly accessed.
    """

    def __init__(
        self,
        id: str = "N/A",
        course: str = "N/A",
        days: str = "",
        time_range: str = "0000-0000",
        waitlist: bool = False,
    ) -> None:
        """
        Constructs the Course object.

        :param id: Unique ID to reference the course within the database. (default: "N/A")
        :param course: Title of the course in the format of 'ABBREVIATION-NUMBER'. (default: "N/A")
        :param days: Valid day string of a single meeting. (default: "")
        :param time_range: String representation of the course time of a single meeting in the format of 'HHMM-HHMM'. (default: "0000-0000")
        :param waitlist: Boolean specifying if the course contains a waitlist. (default: False)
        """

        self.id = id
        self.course = course
        self.days = days
        self.time_string = time_range
        hours = time_range.split("-")
        self.hour_start = time(int(hours[0][:2]), int(hours[0][2:]))
        """datetime.time object of the start time."""
        self.hour_end = time(int(hours[1][:2]), int(hours[1][2:]))
        """datetime.time object of the end time."""
        self.time_range = [self.hour_start, self.hour_end]
        """List containing the self.hour_start (index: 0) and self.hour_end (index: 1) values."""
        self.waitlist = waitlist

    def __eq__(self, other):
        """
        :param other: Object to compare with.
        :return: True if both classes are Course objects, Course.course/Course.days/Course.hour_start are the same, else False.
        """
        return (
            isinstance(other, self.__class__)
            and self.course == other.course
            and self.days == other.days
            and self.hour_start == other.hour_start
        )

    def __lt__(self, other):
        """
        :param other: Course object to compare with.
        :return: True if self.hour_start < other.hour_start, else False.
        """
        return self.hour_start < other.hour_start

    def __gt__(self, other):
        """
        :param other: Course object to compare with.
        :return: True if self.hour_start > other.hour_start, else False.
        """
        return self.hour_start > other.hour_start

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


class Schedule:
    """
    The Schedule object maintains a list of Course objects as well as the current fitness score.
    """

    def __init__(self, courses: Tuple[Course] = (Course())) -> None:
        """
        Constructs the Schedule object.

        :param courses: Tuple of Course objects within the schedule. (default: (Course()))
        """
        try:
            self.courses = list(courses)
        except TypeError:
            raise TypeError("Expected tuple or list.")
        self.fitness = 0

    def contains_duplicate_courses(self) -> bool:
        """
        Checks if schedule has the same course multiple times (only name).

        :return: True if schedule contains duplicates, else False.
        """
        unique_courses = [x.course for x in self.courses]
        for value in self.courses:
            if unique_courses.count(value.course) >= 2:
                return True

        return False

    def overlaps(self) -> bool:
        """
        Determines if the current schedule has courses that overlap in time.

        :return: True if schedule overlaps. False if schedule does not overlap.
        """
        week = [
            [],  # Monday
            [],  # Tuesday
            [],  # Wednesday
            [],  # Thursday
            [],  # Friday
        ]
        for course in self.courses:
            days = day_separator(course.days)
            while len(days) > 0:
                c = Course(course.id, course.course, days[0], course.time_string, course.waitlist,)
                if days[0] == "M":
                    week[0].append(c)
                elif days[0] == "T":
                    week[1].append(c)
                elif days[0] == "W":
                    week[2].append(c)
                elif days[0] == "TH":
                    week[3].append(c)
                elif days[0] == "F":
                    week[4].append(c)
                days.pop(0)

        # Sort by start times
        for idx in range(0, len(week)):
            week[idx].sort()

        for day in week:
            for course_index in range(1, len(day)):
                start_time = day[course_index].hour_start
                end_time = day[course_index - 1].hour_end
                if end_time >= start_time:
                    return True

        return False

    def calculate_fitness(self, schedule_parameters: Dict) -> None:
        """
        Calculates fitness for the current schedule.

        :param schedule_parameters: Dictionary containing different schedule parameters for calculating fitness.
        """
        self.fitness = 0
        self._around_time(
            schedule_parameters["around_time"], schedule_parameters["maximum_time_distance"],
        )
        self._bad_day((schedule_parameters["bad_day"]))
        self._earliest_time(schedule_parameters["earliest_time"])
        self._latest_time(schedule_parameters["latest_time"])
        if schedule_parameters["prefer-no-waitlist"]:
            self._waitlist()

    def _around_time(self, comparison_time: time, time_distance: time = time(4)) -> None:
        """
        Modifies fitness if average course time is within time_distance from comparison_time.

        :param comparison_time: datetime.time object
        :param time_distance: datetime.time object
        """
        start_time = time_sorter(self.courses, ascending=True)[0]
        end_time = time_sorter(self.courses, ascending=False)[0]
        average_times = [start_time, end_time]

        if len(self.courses) > 2:
            for course in self.courses[1:-1]:
                start = datetime(2020, 1, 1, course.hour_start.hour, course.hour_start.minute)
                end = datetime(2020, 1, 1, course.hour_end.hour, course.hour_end.minute)
                midpoint = (start + (end - start) / 2).time()
                average_times.append(midpoint)

        average = average_time(average_times)
        comparison_time = datetime(2020, 1, 1, comparison_time.hour, comparison_time.minute)
        distance = abs((comparison_time - average).total_seconds())

        time_distance = (time_distance.hour * 60 * 60) + (time_distance.minute * 60)
        if distance <= time_distance:
            self.fitness += 1
        else:
            self.fitness -= 1

    def _bad_day(self, days: str) -> None:
        """
        Decreases fitness if current schedule contains a specified date.

        :param days: str that contains a valid day string.
        """
        days = day_separator(days)
        course_days = []
        [
            course_days.append(day_separator(course.days)) for course in self.courses
        ]  # Get array of days for all courses
        course_days = [day for day_array in course_days for day in day_array]  # Flatten list
        for day in days:
            for course_day in course_days:
                if day == course_day:  # if unliked day matches a course schedule day
                    self.fitness -= 1

    def _earliest_time(self, comparison_time: time) -> None:
        """
        Decreases fitness if current schedule runs before the specified time.

        :param comparison_time: datetime.time object that represents the earliest time a class can begin (inclusive).
        """
        course_times = time_sorter(self.courses, ascending=True)
        for start_time in course_times:
            if start_time < comparison_time:  # start time is before specified
                self.fitness -= 1
            else:
                break  # Terminate early so not every class is visited

    def _latest_time(self, comparison_time: time) -> None:
        """
        Decreases fitness if current schedule runs after the specified time.

        :param comparison_time: datetime.time object that represents the latest time a class can end (inclusive).
        """
        course_times = time_sorter(self.courses, ascending=False)
        for end_time in course_times:
            if end_time > comparison_time:  # end time is later than specified
                self.fitness -= 1
            else:
                break  # Terminate early so not every class is visited

    def _waitlist(self) -> None:
        """
        Modifies fitness for each course depending on its waitlist status.
        """
        for course in self.courses:
            if course.waitlist:
                self.fitness -= 1
            else:
                self.fitness += 1

    def __repr__(self):
        out_string = ""
        if len(self.courses) > 1:
            for course_idx in range(0, len(self.courses) - 1):
                out_string += str(self.courses[course_idx]) + ", "
            out_string += str(self.courses[-1])
            return out_string
        elif len(self.courses) == 1:
            return str(self.courses[0])
        else:
            return "No courses in schedule."
