from datetime import time

from sundial.scheduler.course import Course
from sundial.scheduler.meeting import Meeting
from sundial.scheduler.period import DateTime, Day
from sundial.scheduler.schedule import Schedule


class TestSchedule:
    """Class for testing Schedule."""

    def course_setup(self):
        """Set up a Course with a meeting on Friday."""
        return Course(meetings=[Meeting(date=DateTime(day=Day.Friday))])

    def schedule_setup(self):
        """Set up a Schedule."""
        return Schedule(courses=[self.course_setup()])

    def test_is_valid(self):
        """Determine if is_valid works correctly."""
        pass

    def test_avoid_day(self):
        """Check if fitness is correctly modified for given days."""
        s = self.schedule_setup()
        s.avoid_day([Day.Friday])
        assert s.fitness == -1
        s = self.schedule_setup()
        s.avoid_day([Day.Tuesday])
        assert s.fitness == 1

    def test_earliest_time(self):
        """Check if fitness is correctly modified given input time."""
        s = self.schedule_setup()
        s.courses[0].meetings[0].date = DateTime(
            day=Day.Friday, start=time(12), end=time(12, 50)
        )
        s.earliest_time(time(8))
        assert s.fitness == 0
        s.earliest_time(time(12))
        assert s.fitness == 0
        s.earliest_time(time(12, 30))
        assert s.fitness == -1

    def test_latest_time(self):
        """Check if fitness is correctly modified given input time."""
        s = self.schedule_setup()
        s.courses[0].meetings[0].date = DateTime(
            day=Day.Friday, start=time(12), end=time(12, 50)
        )
        s.latest_time(time(8))
        assert s.fitness == -1
        s.latest_time(time(12))
        assert s.fitness == -2
        s.latest_time(time(13))
        assert s.fitness == -2

    def test_waitlist(self):
        """Check if fitness is correctly modified given waitlist status."""
        s = self.schedule_setup()
        s.waitlist()
        assert s.fitness == -1
        s = self.schedule_setup()
        s.courses[0].waitlist = False
        s.waitlist()
        assert s.fitness == 1
