from typing import List

from hourglass.course import Course
from hourglass.datetime import Day


class Schedule:
    def __init__(self, courses: List[Course] = [Course()], fitness=0):
        self.courses = courses
        self.fitness = fitness

    def is_valid(self) -> bool:
        """Determines if the current schedules does not have overlapping times."""
        # TODO: Implement this method
        return True

    def avoid_day(self, bad_days: List[Day]) -> bool:
        """
        Modify the current schedule's fitness if it contains a day that the user wants avoided.

        :param bad_days: `List(Day)` that contains days to avoid.
        """
        current_schedule_days = [
            day
            for date.day in date
            for meeting.date in meeting
            for course.meeting in self.courses
        ]

        for bad_day in bad_days:
            if bad_day in current_schedule_days:
                return True

        return False

    def waitlist(self):
        for course in self.courses:
            if course.waitlist:
                self.fitness -= 1
            else:
                self.fitness += 1
