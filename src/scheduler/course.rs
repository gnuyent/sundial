use super::date::Date;
use super::meeting::Meeting;
use std::collections::HashMap;
use std::fmt;
use time::Time;

/// A data structure holding course informatino which matches the corresponding database table.
#[derive(Clone, Debug, Default)]
pub struct Course {
    /// Unique identifier for the course.
    pub id: String,
    pub url: String,
    /// Full, spelled out, time period the course takes place in.
    pub period: String,
    pub course_subject: String,
    pub course_number: String,
    /// Subject and number of the course. Separated by a `-`.
    pub course_title: String,
    /// Section of the course. This is always 3 characters wide, filled with zeroes if empty.
    pub section: String,
    pub schedule_num: String,
    pub units: String,
    pub session: String,
    pub seats_available: String,
    pub seats_total: String,
    pub description: String,
    pub prerequisite: String,
    pub meetings: Vec<Meeting>,
    /// Miscellaneous data of the course.
    pub miscellaneous: HashMap<String, String>,
    pub waitlist: bool,
}

impl fmt::Display for Course {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}-{} Section {}",
            self.course_subject, self.course_number, self.section
        )
    }
}

impl Course {
    pub fn overlaps(&self) -> bool {
        let starts = self
            .meetings
            .iter()
            .map(|x| x.date.start_time)
            .collect::<Vec<Time>>();

        // Create a 'set' from starts
        let mut uniq = starts.to_owned();
        uniq.sort_unstable();
        uniq.dedup();

        if starts.iter().count() != uniq.iter().count() {
            return true;
        }

        false
    }

    pub fn get_longest_overlap(&self) -> Date {
        let times: Vec<Date> = self.meetings.iter().map(|x| x.date).collect::<Vec<Date>>();

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
