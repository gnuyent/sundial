import itertools
import logging
from datetime import time
from typing import List

from sqlalchemy import create_engine, text

from hourglass.course import Course
from hourglass.datetime import DateTime, Day
from hourglass.meeting import Meeting
from hourglass.schedule import Schedule
from hourglass.schedule_parameters import ScheduleParameters


class ScheduleController:
    """
    ScheduleController is an abstraction for easy interaction with the database.

    In essence, ScheduleController provides an API for higher-level calls.

    Parameters
    ----------
    schedule_parameters : ScheduleParameters
        User-defined schedule parameters.
    course_list : List[str]
        List of strings which represent the courses to generate schedules with.
    """

    def __init__(
        self,
        schedule_parameters: ScheduleParameters = ScheduleParameters(),
        course_list: List[str] = [],
    ):
        self.schedule_parameters = schedule_parameters
        self.course_list = course_list
        self.schedules: List[Schedule] = []

    def generate_schedules(self):
        """Generate all potential, valid schedules."""
        all_classes: List[List[Course]] = []
        for course in self.course_list:
            c: List[Course] = self.generate_courses(course)
            all_classes.append(c)

        logging.debug(f"Found the following courses: {str(all_classes)}")

        for combination in itertools.product(*all_classes):
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
        engine = create_engine("sqlite:///classes.db")
        conn = engine.connect()
        query = text(
            """
            SELECT *
            FROM course
            WHERE course.course
            LIKE :course
            """
        )
        courses = conn.execute(query, course=course_string).fetchall()
        for course in courses:
            waitlist = course.seats_open == 0
            id = course.id
            # Meetings
            meetings = []
            query = text(
                """
                SELECT *
                FROM meeting
                WHERE meeting.course_id
                LIKE :course
                """
            )
            sql_meetings = conn.execute(query, course=id).fetchall()
            for meeting in sql_meetings:
                days: List[Day] = Day.parse_days(meeting.days)
                hours: List[time] = DateTime.parse_time(meeting.hours)
                for day in days:
                    dt: DateTime = DateTime(day=day, start=hours[0], end=hours[1])
                    m = Meeting(
                        date=dt,
                        instructor=meeting.instructor,
                        location=meeting.location,
                        meeting_id=meeting.meeting_id,
                        meeting_type=meeting.meeting_type,
                    )
                    meetings.append(m)
            # Handle meetings that overlap
            start_times = [meeting.date.start for meeting in meetings]
            if len(start_times) == len(set(start_times)):
                overlaps = False
            else:
                overlaps = True

            footnotes = {}
            query = text(
                """
                SELECT footnote.code, footnote.text
                FROM footnote
                WHERE footnote.course_id
                LIKE :course
                """
            )
            sql_footnotes = conn.execute(query, course=id).fetchall()
            for footnote in sql_footnotes:
                footnotes[footnote.code] = footnote.text

            c = Course(
                course_hours=course.course_hours,
                course_title=course.course_title,
                course=course.course,
                description=course.description,
                footnotes=footnotes,
                full_title=course.full_title,
                general_text=course.general_text,
                id=id,
                meetings=meetings,
                overlaps=overlaps,
                period=course.period,
                prerequisite=course.prerequisite,
                schedule_num=course.schedule_num,
                seats_available=course.seats_available,
                seats_open=course.seats_open,
                section=course.section,
                session=course.session,
                statement=course.statement,
                units=course.units,
                url=course.url,
                waitlist=waitlist,
            )
            all_courses.append(c)

        return all_courses

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
