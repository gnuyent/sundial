use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Online,
    Unknown,
}

impl Day {
    pub fn match_day(day: &str) -> Day {
        match day {
            "M" | "Monday" => Day::Monday,
            "T" | "Tuesday" => Day::Tuesday,
            "W" | "Wednesday" => Day::Wednesday,
            "H" | "TH" | "Thursday" => Day::Thursday,
            "F" | "Friday" => Day::Friday,
            "" | "Online" | "ON-LINE" => Day::Online,
            _ => {
                warn!("Unable to determine meeting day from {}.", day);
                Day::Unknown
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq)]
pub struct Date {
    pub start_time: time::Time,
    pub end_time: time::Time,
    pub day: Day,
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start_time.cmp(&other.start_time)
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.start_time.partial_cmp(&other.start_time)
    }
}

impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day
            && self.start_time == other.start_time
            && self.end_time == other.end_time
    }
}
