from datetime import time

from sundial.scheduler.controller import Controller
from sundial.scheduler.parameters import Parameters
from sundial.scheduler.period import Day


class TestController:
    """Class for testing Controller."""

    def test_generate_schedules(self):
        sp = Parameters(
            bad_day=Day.parse_days("TTH"), earliest_time=time(10), latest_time=time(18)
        )
        controller = Controller(
            sp,
            [
                "A S-92A",
                "A S-200A",
                "CS-310",
                "CS-320",
                "ENS-331",
                "MATH-245",
                "MATH-254",
            ],
        )

        controller.generate_schedules()
        best = controller.best_schedule().courses
        assert len(best), 7
