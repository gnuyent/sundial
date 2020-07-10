import pytest

from sundial.scheduler.period import Day


class TestDay:
    """Class for testing Day."""

    def test_parse_days(self):
        """Test combinations of different days and ensure correct output."""
        assert Day.parse_days("MWF") == [Day.Monday, Day.Wednesday, Day.Friday]
        assert Day.parse_days("TH") == [Day.Thursday]
        assert Day.parse_days("TTH") == [Day.Tuesday, Day.Thursday]
        assert Day.parse_days("") == [Day.Online]
        assert Day.parse_days("Arranged") == [Day.Arranged]
        with pytest.raises(ValueError):
            Day.parse_days("P")

    def test_match_day(self):
        """Make sure days are correctly matched."""
        assert Day._match_day("M") == Day.Monday
        assert Day._match_day("T") == Day.Tuesday
        assert Day._match_day("W") == Day.Wednesday
        assert Day._match_day("H") == Day.Thursday
        assert Day._match_day("F") == Day.Friday
