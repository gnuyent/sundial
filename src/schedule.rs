use crate::course::Course;
use crate::datetime::DateTime;
use crate::day::Day;
use crate::structures::{Meeting, Parameters};
use std::cmp::Ordering;
use std::collections::HashMap;
use time::Time;

/// A data structure holding schedule information which combines the [Meeting] and [Course] types.
#[derive(Debug, Default)]
pub struct Schedule {
    /// List of all courses within this schedule.
    courses: Vec<Course>,
    /// An integer representing how favorable this schedule is compared to all others.
    fitness: i32,
}

impl PartialOrd for Schedule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}

impl PartialEq for Schedule {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

impl Schedule {
    /// Creates a new [Schedule] instance.
    pub fn new(courses: Vec<Course>) -> Schedule {
        Schedule {
            courses,
            fitness: 0,
        }
    }

    /// A vector which includes every [Day] within this schedule.
    pub fn days(&self) -> Vec<Day> {
        self.courses
            .iter()
            .flat_map(|course| course.meetings.iter().map(|meeting| meeting.date.day))
            .collect()
    }

    /// A vector which includes every start time within this schedule.
    pub fn start_times(&self) -> Vec<Time> {
        self.courses
            .iter()
            .flat_map(|course| {
                course
                    .meetings
                    .iter()
                    .map(|meeting| meeting.date.start_time)
            })
            .collect()
    }

    /// A vector which includes every end time within this schedule.
    pub fn end_times(&self) -> Vec<Time> {
        self.courses
            .iter()
            .flat_map(|course| course.meetings.iter().map(|meeting| meeting.date.end_time))
            .collect()
    }

    /// Determines if the current schedule has overlapping times. An overlapping time means that
    /// the schedule is invalid.
    ///
    /// Overlapping times refer to when two classes have the same start/ending time or an end that
    /// runs past a start. A class that occurs on Monday from 0900-1000 overlaps with a class from
    /// 1000-1100. Similarly, a class that occurs on Friday from 1200-1300 overlaps with a class
    /// from 1250-1350.
    pub fn is_valid(self) -> bool {
        let mut times: Vec<DateTime> = Vec::new();
        // Retrieve all DateTime objects from the schedule's courses
        for course in self.courses {
            match course.overlaps {
                true => times.push(course.get_longest_overlap()),
                false => {
                    for meeting in course.meetings {
                        times.push(meeting.date);
                    }
                }
            };
        }

        let mut week: HashMap<Day, Vec<&DateTime>> = HashMap::new();
        week.insert(Day::Monday, Vec::new());
        week.insert(Day::Tuesday, Vec::new());
        week.insert(Day::Wednesday, Vec::new());
        week.insert(Day::Thursday, Vec::new());
        week.insert(Day::Friday, Vec::new());

        for dt in times.iter() {
            if let Some(v) = week.get_mut(&dt.day) {
                v.push(dt)
            };
        }

        for times in week.values_mut() {
            times.sort();
            for course_idx in 1..times.len() {
                let start_time = &times[course_idx].start_time;
                let end_time = &times[course_idx - 1].end_time;
                if end_time >= start_time {
                    return false;
                }
            }
        }

        true
    }

    /// Modifies the schedule's fitness fitness based on different parameters.
    ///
    /// This method uses the given schedule parameters as inputs to calculate teh schedule's
    /// fitness.
    pub fn calculate_fitness(mut self, schedule_parameters: Parameters) {
        self.fitness = 0; // reset to avoid undefined behavior
        self.avoid_day(schedule_parameters.bad_day);
        self.earliest_time(schedule_parameters.earliest_time);
        self.latest_time(schedule_parameters.latest_time);
        if schedule_parameters.prefer_no_waitlist {
            self.waitlist()
        }
    }

    /// Modifies the current schedule's fitness if it contains a day that the user wants avoided.
    fn avoid_day(&mut self, bad_days: Vec<Day>) {
        for bad_day in bad_days {
            match self.days().contains(&bad_day) {
                true => self.fitness -= 1,
                false => self.fitness += 1,
            };
        }
    }

    /// Modifies the current schedule's fitness by comparing each course's start time.
    fn earliest_time(&mut self, comparison_time: Time) {
        let start_times: Vec<Time> = self.start_times();
        for start_time in start_times {
            if start_time < comparison_time {
                self.fitness -= 1;
            }
        }
    }

    /// Modifies the current schedule's fitness by comparing each course's end time.
    fn latest_time(&mut self, comparison_time: Time) {
        let end_times: Vec<Time> = self.end_times();
        for end_time in end_times {
            if end_time > comparison_time {
                self.fitness -= 1;
            }
        }
    }

    /// Modifies the current schedule's fitness depending on if it has a waitlist or not.
    fn waitlist(&mut self) {
        for course in self.courses.iter() {
            match course.waitlist {
                true => self.fitness -= 1,
                false => self.fitness += 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schedule_is_not_valid() {
        let meeting_one: Meeting = Meeting::new(
            DateTime::new(
                Day::Monday,
                Time::try_from_hms(8, 0, 0).unwrap(),
                Time::try_from_hms(9, 0, 0).unwrap(),
            ),
            "m1".to_string(),
        );
        let course_one: Course = Course::new(
            "ONE-1".to_string(),
            "1".to_string(),
            vec![meeting_one],
            false,
            1,
            30,
            0,
            false,
        );
        let meeting_two: Meeting = Meeting::new(
            DateTime::new(
                Day::Monday,
                Time::try_from_hms(8, 0, 0).unwrap(),
                Time::try_from_hms(10, 0, 0).unwrap(),
            ),
            "m2".to_string(),
        );
        let course_two: Course = Course::new(
            "TWO-2".to_string(),
            "2".to_string(),
            vec![meeting_two],
            false,
            1,
            30,
            0,
            false,
        );
        let schedule: Schedule = Schedule::new(vec![course_one, course_two]);
        assert_eq!(false, schedule.is_valid());
    }

    #[test]
    fn schedule_is_valid() {
        let meeting_one: Meeting = Meeting::new(
            DateTime::new(
                Day::Monday,
                Time::try_from_hms(8, 0, 0).unwrap(),
                Time::try_from_hms(9, 0, 0).unwrap(),
            ),
            "m1".to_string(),
        );
        let course_one: Course = Course::new(
            "ONE-1".to_string(),
            "1".to_string(),
            vec![meeting_one],
            false,
            1,
            30,
            0,
            false,
        );
        let meeting_two: Meeting = Meeting::new(
            DateTime::new(
                Day::Monday,
                Time::try_from_hms(9, 1, 0).unwrap(),
                Time::try_from_hms(10, 0, 0).unwrap(),
            ),
            "m2".to_string(),
        );
        let course_two: Course = Course::new(
            "TWO-2".to_string(),
            "2".to_string(),
            vec![meeting_two],
            false,
            1,
            30,
            0,
            false,
        );
        let schedule: Schedule = Schedule::new(vec![course_one, course_two]);
        assert!(schedule.is_valid());
    }
}
