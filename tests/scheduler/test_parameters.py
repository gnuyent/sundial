import pytest

from sundial.scheduler.parameters import Parameters


class TestParameters:
    def test_fails_on_negative_time(self):
        with pytest.raises(ValueError):
            Parameters(maximum_time_distance=-1)

    def test_fails_on_overflowed_time(self):
        with pytest.raises(ValueError):
            Parameters(maximum_time_distance=86341)
