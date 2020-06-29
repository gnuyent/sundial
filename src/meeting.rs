use crate::datetime::{DateTime, Day};
use sqlx::FromRow;
use std::cmp::Ordering;

/// `Meeting` holds individual meeting information parsed from `SqlMeeting`.
#[derive(Clone, Debug, Eq, Ord, PartialEq)]
pub struct Meeting {
    pub date: DateTime,
    pub instructor: String,
    pub location: String,
    pub meeting_id: String,
    pub meeting_type: String,
}

impl PartialOrd for Meeting {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare by start times
        self.date.start.partial_cmp(&other.date.start)
    }
}

impl Default for Meeting {
    fn default() -> Self {
        Meeting {
            date: DateTime::default(),
            instructor: String::from(""),
            location: String::from(""),
            meeting_id: String::from(""),
            meeting_type: String::from(""),
        }
    }
}

/// `SqlMeeting` directly corresponds to the database and allows for immediate extraction from
/// queries.
#[derive(FromRow)]
pub struct SqlMeeting {
    pub course_id: String,
    pub days: String,
    pub hours: String,
    pub instructor: String,
    pub location: String,
    pub meeting_id: String,
    pub meeting_type: String,
}

impl SqlMeeting {
    /// `into_meeting` converts `SqlMeeting` to `Meeting`. This method is typically only used
    /// internally for easier conversion.
    pub fn into_meeting(original: Self) -> Vec<Meeting> {
        let days: Vec<Day> = Day::parse_days(&original.days);
        let mut converted_meetings: Vec<Meeting> = Vec::new();
        for day in days {
            let dt = DateTime::new(day, &original.hours);
            let meeting = Meeting {
                date: dt,
                instructor: original.instructor.clone(),
                location: original.location.clone(),
                meeting_id: original.meeting_id.clone(),
                meeting_type: original.meeting_type.clone(),
            };
            converted_meetings.push(meeting);
        }
        converted_meetings
    }
}
