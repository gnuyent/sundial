from datetime import datetime, timedelta


# Common functions used by the controllers


def flatten(arr):
    for x in arr:
        if hasattr(x, '__iter__') and not isinstance(x, str):
            for y in flatten(x):
                yield y
        else:
            yield x


# Checks if schedule has the same course multiple times
def contains_duplicates(input_array):
    nonduplicated = []
    for value in input_array:
        if value in nonduplicated:
            return True
        else:
            nonduplicated.append(value)

    return False


# Checks if the same course has overlapping times.
def identical_overlap(focus, comparison):
    if focus.hour_start == comparison.hour_start and focus.days == comparison.days:
        return True
    else:
        return False


# Get the days from a certain day string
def day_separator(days):
    if "TH" in days:
        split_days = days.partition("TH")
        split_days = list(filter(None, split_days))  # Remove empty indices
        try:
            if "TH" == split_days[1]:
                split_days[0] = list(split_days[0])
                split_days = list(flatten(split_days))
        except IndexError:
            pass
    else:
        split_days = list(days)

    return split_days


# Sort by start or end time
def time_sorter(courses, start=False):
    course_times = []
    if start is False:
        [course_times.append(course.hour_end) for course in courses]
    else:
        [course_times.append(course.hour_start) for course in courses]
    course_times.sort(reverse=not start)
    return course_times


# Generate average time of datetime list. Returns datetime object
def average_time(dates):
    dates = [datetime(2020, 1, 1, time.hour, time.minute) for time in dates]
    reference_date = datetime(1900, 1, 1)
    return reference_date + sum([date - reference_date for date in dates], timedelta()) / len(dates)
