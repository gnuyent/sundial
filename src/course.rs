use crate::datetime::DateTime;
use crate::structures::Meeting;

/// A data structure holding course information which matches the corresponding database table.
#[derive(Debug, Default)]
pub struct Course {
    /// Abbreviation and number of the course (e.g. A E-100)
    pub course: String,
    /// Unique ID for the course.
    pub id: String,
    /// Vector containing every meeting that the course has.
    pub meetings: Vec<Meeting>,
    /// True if meeting has overlapping times, False otherwise. This value determines what time to
    /// use in calculating schedule overlap.
    pub overlaps: bool,
    /// Official schedule number according to the course catalog.
    pub schedule_num: u16,
    /// Maximum possible seats in the course.
    pub seats_total: u32,
    /// Available seats in the course.
    pub seats_available: u32,
    /// True if course is waitlisted (`seats_available == 0`), False otherwise.
    pub waitlist: bool,
}

impl Course {
    /// Creates a new [Course] instance.
    pub fn new(
        course: String,
        id: String,
        meetings: Vec<Meeting>,
        overlaps: bool,
        schedule_num: u16,
        seats_total: u32,
        seats_available: u32,
        waitlist: bool,
    ) -> Course {
        Course {
            course,
            id,
            meetings,
            overlaps,
            schedule_num,
            seats_total,
            seats_available,
            waitlist,
        }
    }

    /// Calculates the longest time in an overlapping scenario.
    ///
    /// Some courses contain times that are overlapping (e.g. Monday 0800-0850, Monday 0800-0950).
    /// In this case, we want to determine what the largest is difference between all time ranges
    /// that overlap. From the previous example, this would be condensed to Monday 0800-0950.
    pub fn get_longest_overlap(self) -> DateTime {
        let times: Vec<DateTime> = self
            .meetings
            .iter()
            .map(|x| x.date)
            .collect::<Vec<DateTime>>();

        let mut highest_idx = 0;
        let mut highest_diff = 0;

        for (idx, meeting) in times.iter().enumerate() {
            let start = meeting.start_time;
            let end = meeting.end_time;
            let difference = (end - start).whole_seconds();
            if difference > highest_diff {
                highest_diff = difference;
                highest_idx = idx;
            }
        }
        self.meetings[highest_idx].date
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day::Day;
    use time::Time;

    #[test]
    fn day_overlap() {
        let start_time = Time::try_from_hms(8, 0, 0).unwrap();
        let end_time_one = Time::try_from_hms(8, 50, 0).unwrap();
        let end_time_two = Time::try_from_hms(9, 50, 0).unwrap();
        let meeting_one = Meeting::new(
            DateTime::new(Day::Monday, start_time, end_time_one),
            String::from("A"),
        );
        let meeting_two = Meeting::new(
            DateTime::new(Day::Monday, start_time, end_time_two),
            String::from("B"),
        );
        let mut course = Course::default();
        course.meetings.push(meeting_one);
        course.meetings.push(meeting_two);
        assert_eq!(
            DateTime::new(
                Day::Monday,
                Time::try_from_hms(8, 0, 0).unwrap(),
                Time::try_from_hms(9, 50, 0).unwrap()
            ),
            course.get_longest_overlap()
        );
    }
}
