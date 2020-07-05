import pytest

from hourglass.datetime import DateTime, Day


class TestDay:
    def test_parse_time(self):
        pass

    def test_match_day(self):
        assert Day._match_day("M") == Day.Monday
        assert Day._match_day("T") == Day.Tuesday
        assert Day._match_day("W") == Day.Wednesday
        assert Day._match_day("H") == Day.Thursday
        assert Day._match_day("F") == Day.Friday
        assert Day._match_day("O") == Day.Online
        assert Day._match_day("A") == Day.Arranged
