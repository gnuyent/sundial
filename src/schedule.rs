use crate::course::Course;
use crate::datetime::Day;
use std::collections::HashSet;

/// The `Schedule` type holds a list of courses within it and calculates a fitness score based upon those
/// courses.
#[derive(Debug)]
pub struct Schedule {
    pub courses: Vec<Course>,
    pub fitness: i16,
}

impl Default for Schedule {
    fn default() -> Self {
        Schedule {
            courses: vec![Course::default()],
            fitness: 0,
        }
    }
}

impl Schedule {
    pub fn new(courses: Vec<Course>, fitness: i16) -> Self {
        Schedule { courses, fitness }
    }

    /// Determines if the current schedule does not have overlapping times.
    pub fn is_valid(&self) -> bool {
        //TODO: Implement this method.
        true
    }

    /// Determines if the current schedule
    ///
    /// # Arguments
    ///
    /// * `days` - `Vec<Day>` that contains days to avoid.
    fn bad_day(&mut self, bad_days: Vec<Day>) -> bool {
        // The code block below retrieves every day that the schedule has a class on.
        let current_schedule_days: HashSet<Day> = self
            .courses
            .iter()
            .flat_map(|c| &c.meetings)
            .map(|d| &d.date.day)
            .cloned()
            .collect();

        for day in bad_days.iter() {
            if current_schedule_days.contains(day) {
                return true;
            }
        }
        false
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
    //    use crate::course::Course;
    //    use crate::meeting::Meeting;
    //    use crate::schedule::Schedule;
    //    use crate::schedule_parameters::ScheduleParameters;
    //    use crate::tools::Day;
    //    use std::collections::HashMap;
    //    use time::Time;
    //
    //    #[test]
    //    fn test_bad_day() {
    //        let mut s: Schedule = Schedule::default();
    //        let mut m = setup_meeting();
    //        m.day = Day::Friday;
    //        s.courses[0].meetings.push(m);
    //        s.bad_day(vec![Day::Friday]);
    //        assert_eq!(s.fitness, -1);
    //    }
    //
    //    #[test]
    //    fn test_latest_time() {
    //        let mut c: Course = setup_course();
    //        let m: Meeting = setup_meeting();
    //        c.meetings.push(m);
    //        let mut s: Schedule = Schedule {
    //            courses: vec![c],
    //            fitness: 0,
    //        };
    //        s.latest_time(Time::try_from_hms(11, 0, 0).unwrap());
    //        assert_eq!(s.fitness, -1);
    //        s.fitness = 0;
    //        s.latest_time(Time::try_from_hms(15, 0, 0).unwrap());
    //        assert_eq!(s.fitness, 0);
    //    }
    //
    //    #[test]
    //    fn test_waitlist() {
    //        let mut s: Schedule = Schedule {
    //            courses: vec![setup_course()],
    //            fitness: 0,
    //        };
    //        s.waitlist();
    //        assert_eq!(s.fitness, -1);
    //        s.courses[0].waitlist = false;
    //        s.fitness = 0;
    //        s.waitlist();
    //        assert_eq!(s.fitness, 1);
    //    }
}
