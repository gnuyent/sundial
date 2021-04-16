use super::date::{Date, Day};
use super::Course;
use super::Parameters;
use std::collections::HashMap;
use time::Time;

#[derive(Debug)]
pub struct Schedule {
    /// List of all courses within this schedule.
    pub courses: Vec<Course>,
    /// An integer representing how favorable this schedule is compared to all others.
    pub fitness: i32,
}

impl Schedule {
    /// Creates a new [Schedule] instance.
    pub fn new(courses: Vec<&Course>) -> Self {
        let courses: Vec<Course> = courses.into_iter().cloned().collect();
        Self {
            courses,
            fitness: 0,
        }
    }

    /// Determines if the current schedule has overlapping times. An overlapping time means that
    /// the schedule is invalid.
    ///
    /// Overlapping times refer to when two classes have the same start/ending time or an end that
    /// runs past a start. A class that occurs on Monday from 0900-1000 overlaps with a class from
    /// 1000-1100. Similarly, a class that occurs on Friday from 1200-1300 overlaps with a class
    /// from 1250-1350.
    pub fn is_valid(&self) -> bool {
        let mut times: Vec<Date> = Vec::new();

        for course in &self.courses {
            if course.overlaps() {
                times.push(course.get_longest_overlap())
            } else {
                for meeting in &course.meetings {
                    times.push(meeting.date)
                }
            }
        }

        let mut week: HashMap<Day, Vec<&Date>> = HashMap::new();

        for date in times.iter() {
            if let Some(v) = week.get_mut(&date.day) {
                v.push(date);
            } else {
                week.insert(date.day.clone(), vec![date]);
            }
        }

        for times in week.values_mut() {
            times.sort();
            for idx in 1..times.len() {
                let start = &times[idx].start_time;
                let end = &times[idx - 1].end_time;
                if end >= start {
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
    /// TODO: Consider using a bitfield for conditions.
    pub fn calculate_fitness(&mut self, schedule_parameters: &Parameters) {
        self.fitness = 0; // reset to avoid undefined behavior
        self.avoid_day(&schedule_parameters.bad_days);
        self.earliest_time(&schedule_parameters.earliest_time);
        self.latest_time(&schedule_parameters.latest_time);
        if schedule_parameters.prefer_no_waitlist {
            self.waitlist()
        }
    }

    /// Modifies the current schedule's fitness if it contains a day that the user wants avoided.
    fn avoid_day(&mut self, bad_days: &Vec<String>) {
        let bad_days: Vec<Day> = bad_days.iter().map(|d| Day::match_day(d)).collect();
        let days: Vec<Day> = self
            .courses
            .iter()
            .flat_map(|course| course.meetings.iter().map(|m| m.date.day))
            .collect();
        for bad_day in bad_days {
            match days.contains(&bad_day) {
                true => self.fitness -= 1,
                false => self.fitness += 1,
            };
        }
    }

    /// Modifies the current schedule's fitness by comparing each course's start time.
    fn earliest_time(&mut self, comparison_time: &str) {
        let (hour, minute) = comparison_time.split_at(2);
        let comparison_time: Time = Time::try_from_hms(
            hour.parse::<u8>().unwrap(),
            minute.parse::<u8>().unwrap(),
            0,
        )
        .unwrap();
        let start_times: Vec<Time> = self
            .courses
            .iter()
            .flat_map(|c| c.meetings.iter().map(|m| m.date.start_time))
            .collect();
        for start_time in start_times {
            if start_time < comparison_time {
                self.fitness -= 1;
            }
        }
    }

    /// Modifies the current schedule's fitness by comparing each course's end time.
    fn latest_time(&mut self, comparison_time: &str) {
        let (hour, minute) = comparison_time.split_at(2);
        let comparison_time: Time = Time::try_from_hms(
            hour.parse::<u8>().unwrap(),
            minute.parse::<u8>().unwrap(),
            0,
        )
        .unwrap();
        let end_times: Vec<Time> = self
            .courses
            .iter()
            .flat_map(|c| c.meetings.iter().map(|m| m.date.end_time))
            .collect();
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
