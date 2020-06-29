use crate::course::Course;
use crate::meeting::Meeting;
use crate::schedule_parameters::ScheduleParameters;
use crate::tools::Days;
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt;
use std::iter::FromIterator;
use time::Time;

/// The `Schedule` type holds a list of courses within it and calculates a fitness score based upon those
/// courses.
pub struct Schedule {
    pub courses: Vec<Course>,
    pub fitness: i16,
}

impl Schedule {
    /// Creates a new instance of the `Schedule` object.
    pub fn new(courses: Vec<Course>, fitness: i16) -> Self {
        Schedule { courses, fitness }
    }

    /// Checks if schedule is valid (no overlapping times).
    pub fn is_valid(&self) -> bool {
        let unique_courses: Vec<String> = self
            .courses
            .iter()
            .map(|course| &course.course)
            .cloned()
            .collect();
        let temp = unique_courses.clone().into_iter().unique();
        let mut unique_count = 0;
        for unique in temp {
            unique_count += 1;
        }

        if unique_courses.len() != unique_count {
            return false;
        }

        const MONDAY: usize = 0;
        const TUESDAY: usize = 1;
        const WEDNESDAY: usize = 2;
        const THURSDAY: usize = 3;
        const FRIDAY: usize = 4;
        let mut week = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];

        let meetings: Vec<Meeting> = self
            .courses
            .iter()
            .flat_map(|c| &c.meetings)
            .cloned()
            .collect();

        for meeting in meetings {
            match meeting.day {
                Days::Monday => week[MONDAY].push(meeting),
                Days::Tuesday => week[TUESDAY].push(meeting),
                Days::Wednesday => week[WEDNESDAY].push(meeting),
                Days::Thursday => week[THURSDAY].push(meeting),
                Days::Friday => week[FRIDAY].push(meeting),
                _ => (),
            }
        }

        for i in 0..week.len() {
            week[i].sort();
        }

        for day in week.iter() {
            for course_index in 1..day.len() {
                let start_time = day[course_index].hours[0];
                let end_time = day[course_index].hours[1];
                if end_time >= start_time {
                    return false;
                }
            }
        }

        true
    }

    /// Calculates fitness for the current schedule based on its courses.
    ///
    /// # Arguments
    ///
    /// * `schedule_parameters` - `ScheduleParameters` containing different user-defined parameters
    /// for calculating fitness.
    pub fn calculate_fitness(&mut self, schedule_parameters: ScheduleParameters) {
        self.fitness = 0;
        self.around_time(
            schedule_parameters.around_time,
            schedule_parameters.maximum_time_distance(),
        );
        self.bad_day(schedule_parameters.bad_day);
        self.earliest_time(schedule_parameters.earliest_time);
        self.latest_time(schedule_parameters.latest_time);
        if schedule_parameters.prefer_no_waitlist {
            self.waitlist();
        }
    }

    /// Modifies fitness if average course time is within `time_distance` from `comparison_time`.
    ///
    /// # Arguments
    ///
    /// * `average_time` - `time::Time` object to compare the current `Schedule` to.
    /// * `time_distance` - `i32` seconds representing the maximum distance away from `comparison_time`.
    fn around_time(&mut self, average_time: Time, time_distance: i32) {
        // Retrieves every time that a class meets.
        let all_hours: Vec<[Time; 2]> = self
            .courses
            .iter()
            .flat_map(|c| &c.meetings)
            .map(|h| &h.hours)
            .cloned()
            .collect();
        let mut start_time: Vec<Time> = Vec::new();
        let mut end_time: Vec<Time> = Vec::new();
        for hour_array in all_hours {
            start_time.push(hour_array[0]);
            end_time.push(hour_array[1]);
        }
        start_time.sort_unstable(); // unstable is ok because we only need the first index.
        end_time.sort_unstable();
        end_time.reverse();

        let start_time: u16 = start_time[0]
            .format("%H%M")
            .parse()
            .expect("Unable to parse start time '{:#?}'");
        let end_time: u16 = end_time[0]
            .format("%H%M")
            .parse()
            .expect("Unable to parse end time '{:#?}'");

        // Calculates the average time from two, 3-4 digit long integers and splits it into seconds
        let comparison_time: u32 = ((start_time + end_time) / 2) as u32;
        let hour: u32 = (&comparison_time / 100) * 60 * 60; // hours to seconds
        let minute: u32 = (&comparison_time % 100) * 60; // minutes to seconds
        let comparison_time: i32 = (hour + minute) as i32; // total seconds of average time from midnight (0)

        let comparison_distance: i32 = (comparison_time - &time_distance).abs(); // distance from time in seconds
        if comparison_distance <= time_distance {
            self.fitness += 1;
        } else {
            self.fitness -= 1;
        }
    }

    /// Decreases fitness if current schedule contains a specified date.
    ///
    /// # Arguments
    ///
    /// * `days` - `Vec<Days>` that contains days to avoid.
    fn bad_day(&mut self, bad_days: Vec<Days>) {
        // The code block below retrieves every day that the schedule has a class on.
        let current_schedule_days: Vec<Days> = self
            .courses
            .iter()
            .flat_map(|c| &c.meetings)
            .map(|m| &m.day)
            .cloned()
            .collect();
        let current_schedule_days: HashSet<Days> = HashSet::from_iter(current_schedule_days);
        for day in bad_days.iter() {
            if current_schedule_days.contains(day) {
                self.fitness -= 1;
            }
        }
    }

    /// Decreases fitness if current schedule runs before the specified time.
    ///
    /// # Arguments
    ///
    /// * `earliest_time` - `time::Time` object that represents the earliest time a class can begin (inclusive).
    fn earliest_time(&mut self, earliest_time: Time) {
        for course in self.courses.iter() {
            for meeting in course.meetings.iter() {
                let comparison_time: time::Time = meeting.hours[0];
                if earliest_time > comparison_time {
                    self.fitness -= 1;
                }
            }
        }
    }

    /// Decreases fitness if current schedule runs after the specified time.
    ///
    /// # Arguments
    ///
    /// * `latest_time` - `time::Time` object that represents the latest time a class can end (inclusive).
    fn latest_time(&mut self, latest_time: Time) {
        let all_ending_times: Vec<Time> = self
            .courses
            .iter()
            .flat_map(|c| &c.meetings)
            .map(|m| &m.hours[1])
            .cloned()
            .collect();
        for comparison_time in all_ending_times {
            if latest_time < comparison_time {
                self.fitness -= 1;
            }
        }
    }

    /// Modifies fitness for each course depending on its waitlist status.
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
    use crate::course::Course;
    use crate::meeting::Meeting;
    use crate::schedule::Schedule;
    use crate::schedule_parameters::ScheduleParameters;
    use crate::tools::Days;
    use std::collections::HashMap;
    use time::Time;

    fn setup_course() -> Course {
        Course {
            course_hours: String::from(""),
            course_title: String::from(""),
            course: String::from(""),
            description: String::from(""),
            footnotes: HashMap::new(),
            full_title: String::from(""),
            general_text: String::from(""),
            id: String::from(""),
            meetings: Vec::new(),
            period: 0,
            prerequisite: String::from(""),
            schedule_num: 0,
            seats_available: 0,
            seats_open: 0,
            section: 0,
            session: String::from(""),
            statement: String::from(""),
            units: 0.0,
            url: String::from(""),
            waitlist: true,
        }
    }

    fn setup_meeting() -> Meeting {
        Meeting {
            day: Days::Online,
            hours: [
                Time::try_from_hms(12, 0, 0).unwrap(),
                Time::try_from_hms(12, 50, 0).unwrap(),
            ],
            instructor: String::from(""),
            location: String::from(""),
            meeting_id: String::from(""),
            meeting_type: String::from(""),
        }
    }

    fn setup_schedule_parameters() -> ScheduleParameters {
        ScheduleParameters::new(
            Time::try_from_hms(0, 0, 0).unwrap(),
            vec![Days::Friday],
            Time::try_from_hms(0, 0, 0).unwrap(),
            Vec::new(),
            Vec::new(),
            Time::try_from_hms(0, 0, 0).unwrap(),
            0,
            true,
        )
    }

    #[test]
    fn test_bad_day() {
        let mut s: Schedule = Schedule::new(vec![setup_course(), setup_course()], 0);
        let mut m = setup_meeting();
        m.day = Days::Friday;
        s.courses[0].meetings.push(m);
        s.bad_day(vec![Days::Friday]);
        assert_eq!(s.fitness, -1);
    }

    #[test]
    fn test_latest_time() {
        let mut c: Course = setup_course();
        let m: Meeting = setup_meeting();
        c.meetings.push(m);
        let mut s: Schedule = Schedule::new(vec![c], 0);
        s.latest_time(Time::try_from_hms(11, 0, 0).unwrap());
        assert_eq!(s.fitness, -1);
        s.fitness = 0;
        s.latest_time(Time::try_from_hms(15, 0, 0).unwrap());
        assert_eq!(s.fitness, 0);
    }

    #[test]
    fn test_waitlist() {
        let mut s: Schedule = Schedule::new(vec![setup_course()], 0);
        s.waitlist();
        assert_eq!(s.fitness, -1);
        s.courses[0].waitlist = false;
        s.fitness = 0;
        s.waitlist();
        assert_eq!(s.fitness, 1);
    }
}
