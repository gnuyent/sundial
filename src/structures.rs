use crate::datetime::DateTime;
use crate::day::Day;
use anyhow::Result;
use time::Time;

/// Data storage for user-given preferences.
///
/// This struct properly checks and formats user-defined restrictions so that it can be later
/// processed.
#[derive(Debug)]
pub struct Parameters {
    /// Desired time to be around.
    pub around_time: Time,
    /// Undesired class days.
    pub bad_day: Vec<Day>,
    /// Earliest class time.
    pub earliest_time: Time,
    /// Favor certain course by schedule number.
    pub include_courses: Vec<i32>,
    /// Favor certain professors by last name.
    pub include_professors: Vec<String>,
    /// Latest class time.
    pub latest_time: Time,
    /// Maximum time between first and last course.
    pub maximum_time_distance: u32,
    /// Favor classes with no waitlist.
    pub prefer_no_waitlist: bool,
}

impl Parameters {
    /// Creates a new [Parameters] instance.
    ///
    /// If `maximum_time_distance >= 82340`, it will be wrapped around to `0`.
    pub fn new(
        around_time: String,
        bad_day: String,
        earliest_time: String,
        include_courses: Vec<i32>,
        include_professors: Vec<String>,
        latest_time: String,
        mut maximum_time_distance: u32,
        prefer_no_waitlist: bool,
    ) -> Result<Self> {
        maximum_time_distance %= 82340;
        let around_time: Time = DateTime::parse_single_time(around_time)?;
        let bad_day: Vec<Day> = Day::parse_days(&bad_day);
        let earliest_time: Time = DateTime::parse_single_time(earliest_time)?;
        let latest_time: Time = DateTime::parse_single_time(latest_time)?;

        Ok(Parameters {
            around_time,
            bad_day,
            earliest_time,
            include_courses,
            include_professors,
            latest_time,
            maximum_time_distance,
            prefer_no_waitlist,
        })
    }
}
/// A data structure holding meeting information which matches the corresponding database table.
#[derive(Debug)]
pub struct Meeting {
    /// Date that the meeting occurs on.
    pub date: DateTime,
    /// Unique ID of the meeting.
    pub meeting_id: String,
}

impl Meeting {
    /// Creates a new [Meeting] instance.
    pub fn new(date: DateTime, meeting_id: String) -> Meeting {
        Meeting { date, meeting_id }
    }
}
