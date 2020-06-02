import itertools
import os
import sqlite3
from typing import Dict

from core.backend.util.structures import Course, Schedule
from core.backend.util.tools import identical_overlap, contains_duplicates


# Abstracted controller for interacting with schedules


class ScheduleController:
    def __init__(self, schedule_parameters: Dict, *args: str):
        self.schedules = []  # all possible schedules
        self.course_list = []  # courses requested
        self.simulations = 0
        self.schedule_parameters = schedule_parameters
        for course in args:
            self.course_list.append(course)

    """
    Generates all potential valid and non-overlapping course schedules. Runs in O(n^2) but low number of inputs keep
    runtime low.
    """

    def generate_schedules(self):
        connection = sqlite3.connect(f"{os.getcwd()}/classes.db", uri=True)
        cursor = connection.cursor()
        all_classes = []  # All meetings of each course
        for course in self.course_list:
            meetings = cursor.execute(
                f"""SELECT course.id, meeting.days, meeting.hours, course.seats_available
                        FROM course, meeting
                        WHERE meeting.course_id = course.id
                        AND course.course LIKE ('{course}')"""
            ).fetchall()  # Get every day/time course meets
            for course_id, meeting_days, meeting_hours, seats_available in meetings:
                if seats_available == 0:
                    waitlist = True
                else:
                    waitlist = False
                if meeting_hours == "":  # Classes without times
                    meeting_hours = "0000-0000"
                new_meeting = Course(
                    id=course_id,
                    course=course,
                    days=meeting_days,
                    time_range=meeting_hours,
                    waitlist=waitlist,
                )
                all_classes.append(new_meeting)

        connection.close()

        # Remove duplicate courses which have identical times
        idx = 0
        while idx < len(all_classes) - 1:
            focus_class = all_classes[idx]
            comparison_class = all_classes[idx + 1]
            if focus_class == comparison_class and identical_overlap(
                focus_class, comparison_class
            ):
                all_classes.pop(idx)
            idx += 1

        schedule_tuples = list(
            itertools.combinations(all_classes, len(self.course_list))
        )
        schedule_tuples[:] = [
            unique_schedule
            for unique_schedule in schedule_tuples
            if not contains_duplicates(unique_schedule)
        ]  # Remove same course from same schedule

        for schedule in schedule_tuples:
            s = Schedule(schedule)
            if not s.overlaps():
                self.schedules.append(s)

        # check for force include course option
        if len(self.schedule_parameters["include-courses"]) > 0:
            for schedule_number in self.schedule_parameters["include-courses"]:
                for schedule in list(
                    self.schedules
                ):  # create duplicate of list to modify in place
                    # if the specified schedule number is NOT in the schedule
                    if not any(
                        str(schedule_number) in course.id for course in schedule.courses
                    ):
                        self.schedules.remove(schedule)

    def iterate(self):
        [
            schedule.calculate_fitness(self.schedule_parameters)
            for schedule in self.schedules
        ]
        self.schedules.sort(key=lambda x: x.fitness, reverse=True)

    def best_schedule(self) -> Schedule or None:
        if len(self.schedules) > 0:
            return self.schedules[0]
        else:
            return None

    def __str__(self):
        output = ""
        for schedule in self.schedules:
            output += str(schedule) + "\n"
        return output
