import pytest

from sundial.scheduler.parameters import Parameters


class TestParameters:
    """Class for testing schedule parameters."""

    def test_fails_on_negative_time(self):
        """Fail if time is negative."""
        with pytest.raises(ValueError):
            Parameters(maximum_time_distance=-1)

    def test_fails_on_overflowed_time(self):
        """Fail if time exceeds one day."""
        with pytest.raises(ValueError):
            Parameters(maximum_time_distance=86341)
