from datetime import time
from core.backend.util.controllers import ScheduleController

# Schedule parameters to generate fitness scores on. No parameter is firm and will not
# immediately dismiss a schedule because it does not fit in a certain parameter.
schedule_parameters = {
    "around_time": time(12),  # datetime.time object in hours, minutes of time to search around
    "maximum_time_distance": time(
        4
    ),  # datetime.time object in hours, minutes of maximum distance (inclusive)
    "bad_day": "TTH",  # date string in chronological order to dislike
    "earliest_time": time(
        10
    ),  # datetime.time object in hours, minutes of earliest start time (exclusive)
    "latest_time": time(
        18
    ),  # datetime.time object in hours, minutes of latest end time (exclusive)
    "prefer-no-waitlist": True,  # Prefer non-waitlisted classes
    "include-professors:": [],  # include certain professors
    "include-courses": [],  # include certain courses (input schedule number)
}

controller = ScheduleController(
    schedule_parameters,
    "A S-92A",
    "A S-200A",
    "CS-310",
    "CS-320",
    "ENS-331",
    "MATH-245",
    "MATH-254",
)

# Do not edit below here!
controller.generate_schedules()
controller.iterate()
best = controller.best_schedule()
if best is not None:
    print(f"{best} \nFitness: {best.fitness}")
    print("------ additional results ------")
    for schedule in controller.schedules[1:]:
        print(f"{schedule} \nFitness: {schedule.fitness}")
    print(len(controller.schedules))
else:
    print("No schedules!")
