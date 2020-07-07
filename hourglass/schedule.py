from datetime import time
from typing import Dict, List

from hourglass.course import Course
from hourglass.datetime import Day, DateTime
from hourglass.schedule_parameters import ScheduleParameters


class Schedule:
    """
    Schedule retains information that will eventually be processed.

    Parameters
    ----------
    courses : List[Course]
        The list of courses to include in the schedule.
        (default: [Course()])
    fitness : int
        Fitness score to initially assign.
        (default: 0)
    """

    def __init__(self, courses: List[Course] = [Course()], fitness: int = 0):
        self.courses: List[Course] = courses
        self.fitness: int = fitness

    def __lt__(self, other):
        return self.fitness < other.fitness

    def is_valid(self) -> bool:
        """
        Determine if the current schedule does not have overlapping times.

        Returns
        -------
        bool
            True if the current schedule is valid, False otherwise.
        """
        times: List[DateTime] = []
        # retrieve all DateTime from the schedule's courses
        for course in self.courses:
            if course.overlaps:
                times.append(course.get_longest_overlap())
            else:
                for meeting in course.meetings:
                    times.append(meeting.date)

        week: Dict[Day, List[DateTime]] = {
            Day.Monday: [],
            Day.Tuesday: [],
            Day.Wednesday: [],
            Day.Thursday: [],
            Day.Friday: [],
        }
        # Separate DateTime by day
        for dt in times:
            try:
                week[dt.day].append(dt)
            except KeyError:
                continue  # Skip Online and Arranged courses

        # Determine overlap by sorting and seeing if end A is after start B
        for day, times in week.items():
            times.sort()
            for course_index in range(1, len(times)):
                start_time = times[course_index].start
                end_time = times[course_index - 1].end
                if end_time >= start_time:
                    return False

        return True

    def calculate_fitness(self, schedule_parameters: ScheduleParameters):
        """
        Modify the schedule's fitness based on different parameters.

        Uses the given schedule parameters as inputs to calculate the schedule's
        fitness.

        Parameters
        ----------
        schedule_parameters : ScheduleParameters
            User-defined schedule parameters to calculate.
        """
        self.fitness = 0  # reset to avoid undefined behavior
        self.avoid_day(schedule_parameters.bad_day)
        self.earliest_time(schedule_parameters.earliest_time)
        self.latest_time(schedule_parameters.latest_time)
        if schedule_parameters.prefer_no_waitlist:
            self.waitlist()

    def avoid_day(self, bad_days: List[Day]):
        """
        Modify the current schedule's fitness if it contains a day that the user wants avoided.  # noqa: D400, E501

        Parameters
        ----------
        bad_days : List[Day]
            List that contains days to avoid.
        """
        # retrieves every meeting day in the schedule

        current_schedule_days: List[Day] = [
            meeting.date.day for course in self.courses for meeting in course.meetings
        ]

        for bad_day in bad_days:
            if bad_day in current_schedule_days:
                self.fitness -= 1
            else:
                self.fitness += 1

    def earliest_time(self, comparison_time: time):
        """
        Modify the current schedule's fitness by comparing each course's start time.

        Parameters
        ----------
        comparison_time : time
            Time representing the earliest time (inclusive) that the class can end.
        """
        start_times: List[time] = [
            meeting.date.start for course in self.courses for meeting in course.meetings
        ]

        for start_time in start_times:
            if start_time < comparison_time:
                self.fitness -= 1

    def latest_time(self, comparison_time: time):
        """
        Modify the current schedule's fitness by comparing each course's end time.

        Parameters
        ----------
        comparison_time : time
            Time representing the latest time (inclusive) that the class can be.
        """
        end_times: List[time] = [
            meeting.date.end for course in self.courses for meeting in course.meetings
        ]

        for end_time in end_times:
            if end_time > comparison_time:
                self.fitness -= 1

    def waitlist(self):
        """Modify the current schedule's fitness depending on if it has a waitlist."""
        for course in self.courses:
            if course.waitlist:
                self.fitness -= 1
            else:
                self.fitness += 1

    def __repr__(self):
        """Return every output of the courses within the schedule."""
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
