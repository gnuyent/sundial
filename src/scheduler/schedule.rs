use super::date::{Date, Day};
use super::{Course, Meeting, Parameters};
use anyhow::Result;
use std::{cell::Cell, collections::HashMap};
use time::Time;

#[derive(Debug)]
pub struct Schedule {
    /// List of all courses within this schedule.
    pub courses: Vec<Course>,
    /// An integer representing how favorable this schedule is compared to all others.
    pub fitness: Cell<i32>,
}

impl Schedule {
    /// Creates a new [Schedule] instance.
    pub fn new(courses: Vec<&Course>) -> Option<Self> {
        let courses: Vec<Course> = courses.into_iter().cloned().collect();

        if Schedule::is_valid(&courses) {
            return Some(Self {
                courses,
                fitness: Cell::new(0),
            });
        }

        None
    }

    fn inc_fitness(&self) {
        self.fitness.set(self.fitness.get() + 1);
    }

    fn dec_fitness(&self) {
        self.fitness.set(self.fitness.get() + 1);
    }

    /// Determines if the current schedule has overlapping times. An overlapping time means that
    /// the schedule is invalid.
    ///
    /// Overlapping times refer to when two classes have the same start/ending time or an end that
    /// runs past a start. A class that occurs on Monday from 0900-1000 overlaps with a class from
    /// 1000-1100. Similarly, a class that occurs on Friday from 1200-1300 overlaps with a class
    /// from 1250-1350.
    fn is_valid(courses: &Vec<Course>) -> bool {
        let mut times: Vec<Date> = Vec::new();

        for course in courses {
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
    pub fn calculate_fitness(&self, schedule_parameters: &Parameters) -> Result<()> {
        self.fitness.set(0); // reset to avoid undefined behavior
        self.avoid_day(&schedule_parameters.bad_days);
        self.early_late_times(
            &schedule_parameters.earliest_time,
            &schedule_parameters.latest_time,
        )?;
        if schedule_parameters.prefer_no_waitlist {
            self.waitlist()
        }

        Ok(())
    }

    /// Modifies the current schedule's fitness if it contains a day that the user wants avoided.
    fn avoid_day(&self, bad_days: &Vec<String>) {
        let bad_days: Vec<Day> = bad_days.iter().map(|d| Day::match_day(d)).collect();
        let days: Vec<Day> = self
            .courses
            .iter()
            .flat_map(|course| course.meetings.iter().map(|m| m.date.day))
            .collect();
        for bad_day in bad_days {
            match days.contains(&bad_day) {
                true => self.dec_fitness(),
                false => self.inc_fitness(),
            };
        }
    }

    /// Modifies the current schedule's fitness by comparing each course's start time.
    fn early_late_times(&self, early_time: &str, late_time: &str) -> Result<()> {
        let (hour, minute) = early_time.split_at(2);
        let early_time: Time = Time::try_from_hms(hour.parse::<u8>()?, minute.parse::<u8>()?, 0)?;

        let (hour, minute) = late_time.split_at(2);
        let late_time: Time = Time::try_from_hms(hour.parse::<u8>()?, minute.parse::<u8>()?, 0)?;

        self.courses.iter().for_each(|Course { meetings, .. }| {
            meetings.iter().for_each(|Meeting { date, .. }| {
                if date.start_time < early_time {
                    self.dec_fitness();
                }

                if date.end_time > late_time {
                    self.dec_fitness();
                }
            })
        });

        Ok(())
    }

    /// Modifies the current schedule's fitness depending on if it has a waitlist or not.
    fn waitlist(&self) {
        for course in &self.courses {
            match course.waitlist {
                true => self.dec_fitness(),
                false => self.inc_fitness(),
            }
        }
    }
}
