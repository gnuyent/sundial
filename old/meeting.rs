use crate::tools::Days;
use std::cmp::Ordering;
use time::Time;

/// The `Meeting` type holds individual meeting information that is retrieved from the SQLite database.
#[derive(Clone, Debug, Eq, Ord, PartialEq)]
pub struct Meeting {
    pub day: Days,
    pub hours: [Time; 2],
    pub instructor: String,
    pub location: String,
    pub meeting_id: String,
    pub meeting_type: String,
}

impl PartialOrd for Meeting {
    fn partial_cmp(&self, other: &Meeting) -> Option<Ordering> {
        Some(self.hours[0].cmp(&other.hours[0]))
    }
}
