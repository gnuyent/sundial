from datetime import time

from hourglass.datetime import Day
from hourglass.schedule_controller import ScheduleController
from hourglass.schedule_parameters import ScheduleParameters

sp = ScheduleParameters(
    bad_day=Day.parse_days("TTH"), earliest_time=time(10), latest_time=time(18),
)

controller = ScheduleController(
    sp, ["A S-92A", "A S-200A", "CS-310", "CS-320", "ENS-331", "MATH-245", "MATH-254"]
)

controller.generate_schedules()
best = controller.best_schedule()
print(controller.best_schedule())
print("Fitness: " + str(best.fitness))
print("------ additional results ------")
for schedule in controller.schedules[1:]:
    print(schedule)
    print("Fitness: " + str(schedule.fitness))
