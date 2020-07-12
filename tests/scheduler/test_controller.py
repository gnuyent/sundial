from sundial.scheduler.controller import Controller
from sundial.scheduler.parameters import Parameters


class TestController:
    """Class for testing Controller."""

    def test_generate_schedules(self):
        sp = Parameters(bad_day="TTH", earliest_time="1000", latest_time="18")
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
