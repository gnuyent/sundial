import pytest

from hourglass.schedule_parameters import ScheduleParameters


class TestScheduleParameters:
    def test_fails_on_negative_time(self):
        with pytest.raises(ValueError):
            ScheduleParameters(maximum_time_distance=-1)

    def test_fails_on_overflowed_time(self):
        with pytest.raises(ValueError):
            ScheduleParameters(maximum_time_distance=86341)
