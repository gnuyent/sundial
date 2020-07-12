import itertools
import logging
from datetime import time
from typing import List, Tuple

from sundial import models
from sundial.database import get_db
from sundial.scheduler.course import Course
from sundial.scheduler.meeting import Meeting
from sundial.scheduler.parameters import Parameters
from sundial.scheduler.period import DateTime, Day
from sundial.scheduler.schedule import Schedule


class Controller:
    """Controller is an API abstraction for easy interaction with the database.

    Parameters
    ----------
    schedule_parameters : Parameters
        User-defined schedule parameters.
    course_list : List[str]
        List of strings which represent the courses to generate schedules with.
    """

    def __init__(
        self, schedule_parameters: Parameters, course_list: List[str] = [],
    ):
        self.schedule_parameters: Parameters = schedule_parameters
        self.course_list: List[str] = course_list
        self.schedules: List[Schedule] = []
        self.db = next(get_db())

    def generate_schedules(self):
        """Generate all potential, valid schedules."""
        all_classes: List[List[Course]] = []
        for course in self.course_list:
            c: List[Course] = self.generate_courses(course)
            all_classes.append(c)

        logging.debug(f"Found the following courses: {str(all_classes)}")

        for combination in itertools.product(*all_classes):  # * unpacks list
            s: Schedule = Schedule(combination)
            if s.is_valid():
                self.schedules.append(s)

    def generate_courses(self, course_string: str) -> List[Course]:
        """
        Generate Course objects for every course matching the given input.

        Parameters
        ----------
        course_string : str
            The name of the course to create a Course for.

        Returns
        -------
        List[Course]
            List of all courses matching the given input string.
        """
        all_courses = []
        courses = (
            self.db.query(
                models.Course.course,
                models.Course.id,
                models.Course.schedule_num,
                models.Course.seats_available,
                models.Course.seats_total,
            )
            .filter(models.Course.course == course_string)
            .all()
        )
        for course in courses:
            waitlist = course.seats_available == 0
            id = course.id
            meetings, overlaps = self.generate_meetings(id)
            c = Course(
                course.course,
                id,
                meetings,
                overlaps,
                course.schedule_num,
                course.seats_available,
                course.seats_total,
                waitlist,
            )
            all_courses.append(c)

        return all_courses

    def generate_meetings(self, course_id: str) -> Tuple[List[Meeting], bool]:
        all_meetings = []
        meetings = (
            self.db.query(
                models.Meeting.days, models.Meeting.hours, models.Meeting.meeting_id
            )
            .filter(models.Meeting.course_id == course_id)
            .all()
        )
        for meeting in meetings:
            days: List[Day] = Day.parse_days(meeting.days)
            hours: List[time] = DateTime.parse_time(meeting.hours)
            for day in days:
                dt: DateTime = DateTime(day=day, start=hours[0], end=hours[1])
                m = Meeting(date=dt, meeting_id=meeting.meeting_id)
                all_meetings.append(m)
        # Handle meetings that overlap
        start_times = [meeting.date.start for meeting in all_meetings]
        if len(start_times) == len(set(start_times)):
            overlaps = False
        else:
            overlaps = True

        return all_meetings, overlaps

    def best_schedule(self) -> Schedule:
        [
            schedule.calculate_fitness(self.schedule_parameters)
            for schedule in self.schedules
        ]
        self.schedules.sort(reverse=True)
        return self.schedules[0]

    def __str__(self):
        output = ""
        for schedule in self.schedules:
            output += str(schedule) + "\n"
        return output
