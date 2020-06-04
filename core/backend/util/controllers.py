import itertools
import os
from typing import Dict, List

from core.backend.util.structures import Course, Schedule
from core.backend.util.tools import connect_database


class ScheduleController:
    """
    Controller for generating and interacting with schedules.
    """

    def __init__(self, schedule_parameters: Dict, *args: str) -> None:
        """
        Constructs the ScheduleController object.

        :param schedule_parameters: Dict of specified parameters to calculate fitness.
        :param args: Comma-separated strings of classes in the format of 'ABBREVIATION-NUMBER'.
        """
        self.schedules = []  # all possible schedules
        """List of Schedule objects that are valid."""
        self.course_list = []  # courses requested
        """List of strings representing all classes requested."""
        self.schedule_parameters = schedule_parameters
        for course in args:
            self.course_list.append(course)

    def generate_schedules(self) -> None:
        """
        Generates all potential valid and non-overlapping course schedules.
        """
        all_classes = self._retrieve_all_classes()

        # Remove duplicate courses which have identical times
        idx = 0
        while idx < len(all_classes) - 1:
            focus_class = all_classes[idx]
            comparison_class = all_classes[idx + 1]
            if focus_class == comparison_class:
                all_classes.pop(idx)
            idx += 1

        # Generate every possible combination of schedules (including duplicate courses)
        schedule_tuples = list(itertools.combinations(all_classes, len(self.course_list)))

        for schedule in schedule_tuples:
            s = Schedule(schedule)
            # Add schedules that do not overlap and do not contain the same course multiple times
            if not s.overlaps() and not s.contains_duplicate_courses():
                self.schedules.append(s)

        # check for force include course option
        if len(self.schedule_parameters["include-courses"]) > 0:
            for schedule_number in self.schedule_parameters["include-courses"]:
                for schedule in list(self.schedules):  # create duplicate of list to modify in place
                    # if the specified schedule number is NOT in the schedule
                    if not any(str(schedule_number) in course.id for course in schedule.courses):
                        self.schedules.remove(schedule)

    def _retrieve_all_classes(self) -> List[Course]:
        """
        Retrieves all classes from the database that correspond with the given list.

        :return: List[Course] containing all matching classes.
        """
        with connect_database(
            f"{os.getcwd()}/classes.db", read_only=True
        ) as cursor:  # TODO: Integrate this with a config file
            all_classes = []  # All meetings of each course
            for course in self.course_list:
                meetings = cursor.execute(
                    f"""SELECT course.id, meeting.days, meeting.hours, course.seats_available
                                    FROM course, meeting
                                    WHERE meeting.course_id = course.id
                                    AND course.course LIKE ('{course}')"""
                ).fetchall()
                for course_id, meeting_days, meeting_hours, seats_available in meetings:
                    if seats_available == 0:
                        waitlist = True
                    else:
                        waitlist = False

                    if meeting_hours == "":  # Classes without times (e.g. online)
                        meeting_hours = "0000-0000"

                    new_meeting = Course(
                        id=course_id,
                        course=course,
                        days=meeting_days,
                        time_range=meeting_hours,
                        waitlist=waitlist,
                    )
                    all_classes.append(new_meeting)

        return all_classes

    def iterate(self) -> None:
        """
        Calculates (and resets) the fitness for every schedule and sorts the schedules by their new fitness level.
        """
        [schedule.calculate_fitness(self.schedule_parameters) for schedule in self.schedules]
        self.schedules.sort(key=lambda x: x.fitness, reverse=True)

    def best_schedule(self) -> Schedule or None:
        """
        Returns the best schedule.

        :return: Schedule object that has the highest fitness.
        """
        if len(self.schedules) > 0:
            return self.schedules[0]
        else:
            return None

    def __str__(self):
        output = ""
        for schedule in self.schedules:
            output += str(schedule) + "\n"
        return output
