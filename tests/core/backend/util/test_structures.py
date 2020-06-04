from datetime import time

import pytest

from core.backend.util.structures import Course, Schedule


@pytest.fixture
def c1():
    c1 = Course(
        id="20204-ACCTG201-01-20042",
        course="ACCTG-201",
        days="M",
        time_range="1200-1315",
        waitlist=False,
    )
    return c1


@pytest.fixture
def c2():
    c2 = Course(
        id="20204-ACCTG201-01-20042",
        course="ACCTG-201",
        days="M",
        time_range="0000-0000",
        waitlist=False,
    )
    return c2


@pytest.fixture
def c3():
    c3 = Course(
        id="20204-ART100-05-20209",
        course="ART-100",
        days="TTH",
        time_range="1230-1510",
        waitlist=False,
    )
    return c3


@pytest.fixture
def c4():
    c4 = Course(
        id="blank", course="blank", days="MW", time_range="1200-1250", waitlist=False
    )
    return c4


@pytest.fixture
def c5():
    c5 = Course(
        id="blank", course="blank", days="WTH", time_range="1100-1200", waitlist=False
    )
    return c5


@pytest.fixture
def c6():
    c6 = Course(
        id="blank", course="blank", days="WTH", time_range="1800-1900", waitlist=False
    )
    return c6


@pytest.fixture
def schedule_parameters():
    schedule_parameters = {
        "around_time": time(12),
        "maximum_time_distance": time(4),
        "bad_day": "TTH",
        "earliest_time": time(10),
        "latest_time": time(18),
        "prefer-no-waitlist": True,
        "include-professors:": [],
        "include-courses": [],
    }
    return schedule_parameters


class TestStructures:
    def test_course(self, c1, c2, c3):
        # Regular class with time
        assert c1.hour_start == time(12)
        assert c1.hour_end == time(13, 15)

        # Online classes (start and end at midnight)
        assert c2.hour_start == time(0)
        assert c2.hour_end == time(0)

        # __eq__: true if both are Course classes and course names are the same
        assert c1 == c2
        assert c1 != c3

        # __lt__: true if class A starts before class B
        # __gt__: true if class A starts after class B
        assert c2 < c1
        assert c1 < c3
        assert c2 < c3

    def test_schedule(self, c4, c5):
        s = Schedule((c4, c5))
        # Ensures input is not String
        assert not isinstance(s.courses, str)
        # Error out with incorrect input type
        with pytest.raises(TypeError):
            Schedule(15)
        # Makes sure fitness is 0
        assert s.fitness == 0

    def test_schedule_overlaps(self, c1, c2, c3, c4, c5):
        s = Schedule((c4, c5))
        assert s.overlaps() is True
        assert Schedule((c1, c2, c3)).overlaps() is False

    def test_schedule_calculate_fitness(self, c1, c6, schedule_parameters):
        s1 = Schedule((c1, c6))
        s1.calculate_fitness(schedule_parameters)
        assert s1.fitness == 1
