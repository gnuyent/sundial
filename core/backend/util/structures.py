# Data containers
from datetime import datetime, time
from typing import Dict, Tuple

from core.backend.util.tools import average_time, day_separator, time_sorter


class Course:
    def __init__(self, id: str, course: str, days: str, time_range: str):
        self.id = id
        self.course = course
        self.days = days
        self.time_string = time_range
        hours = time_range.split("-")
        self.hour_start = time(int(hours[0][:2]), int(hours[0][2:]))
        self.hour_end = time(int(hours[1][:2]), int(hours[1][2:]))
        self.time_range = [self.hour_start, self.hour_end]

    def __eq__(self, other):
        return isinstance(other, self.__class__) and self.course == other.course

    def __lt__(self, other):
        return self.hour_start < other.hour_start

    def __gt__(self, other):
        return self.hour_start > other.hour_start

    def __repr__(self):
        zero_time = time()
        start_time = str(self.hour_start.isoformat(timespec="minutes"))
        end_time = str(self.hour_end.isoformat(timespec="minutes"))
        if self.hour_start == zero_time and self.hour_end == zero_time:
            return f"{self.course} NO_TIME"
        else:
            return f"{self.course} {self.days} {start_time}-{end_time}"


class Schedule:
    def __init__(self, courses):
        try:
            self.courses = list(courses)
        except TypeError:
            print("Expected tuple or list.")
            raise
        self.fitness = 0

    def overlaps(self) -> bool:
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
                c = Course(course.id, course.course, days[0], course.time_string)
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

    def calculate_fitness(self, schedule_parameters: Dict):
        self.fitness = 0
        self.around_time(
            schedule_parameters["around_time"],
            schedule_parameters["maximum_time_distance"],
        )
        self.bad_day((schedule_parameters["bad_day"]))
        self.earliest_time(schedule_parameters["earliest_time"])
        self.latest_time(schedule_parameters["latest_time"])

    def around_time(self, comparison_time: time, time_distance: time):
        start_time = time_sorter(self.courses, start=True)[0]
        end_time = time_sorter(self.courses, start=False)[0]
        average_times = [start_time, end_time]

        if len(self.courses) > 2:
            for course in self.courses[1:-1]:
                start = datetime(
                    2020, 1, 1, course.hour_start.hour, course.hour_start.minute
                )
                end = datetime(2020, 1, 1, course.hour_end.hour, course.hour_end.minute)
                midpoint = (start + (end - start) / 2).time()
                average_times.append(midpoint)

        average = average_time(average_times)
        comparison_time = datetime(
            2020, 1, 1, comparison_time.hour, comparison_time.minute
        )
        distance = abs((comparison_time - average).total_seconds())

        time_distance = (time_distance.hour * 60 * 60) + (time_distance.minute * 60)
        if distance <= time_distance:
            self.fitness += 1
        else:
            self.fitness -= 1

    def bad_day(self, days: str):
        days = day_separator(days)
        course_days = []
        [
            course_days.append(day_separator(course.days)) for course in self.courses
        ]  # Get array of days for all courses
        course_days = [
            day for day_array in course_days for day in day_array
        ]  # Flatten list
        for day in days:
            for course_day in course_days:
                if day == course_day:  # if unliked day matches a course schedule day
                    self.fitness -= 1

    def earliest_time(self, comparison_time: time):
        course_times = time_sorter(self.courses, start=True)
        for start_time in course_times:
            if start_time < comparison_time:  # start time is before specified
                self.fitness -= 1
            else:
                break  # Terminate early so not every class is visited

    # Determines if current schedule passes latest time or not
    def latest_time(self, comparison_time: time):
        course_times = time_sorter(self.courses, start=False)
        for end_time in course_times:
            if end_time > comparison_time:  # end time is later than specified
                self.fitness -= 1
            else:
                break

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
