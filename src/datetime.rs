use crate::error::ParserError;
use time::Time;

// A common collection of utilities used by the sundial API.

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DateTime {
    pub day: Day,
    pub start: Time,
    pub end: Time,
}

impl Default for DateTime {
    fn default() -> Self {
        DateTime {
            day: Day::Online,
            start: Time::midnight(),
            end: Time::midnight(),
        }
    }
}

impl DateTime {
    pub fn new(day: Day, time_string: &str) -> Self {
        let times: (Time, Time) =
            DateTime::parse_time(time_string).unwrap_or((Time::midnight(), Time::midnight()));
        DateTime {
            day,
            start: times.0,
            end: times.1,
        }
    }

    /// `parse_time` generates an array of two `time::Time` objects from a string that contains a start and end time separated with a '-'.
    ///
    /// # Arguments
    ///
    /// * `hour_range` - A string slice that contains the times to parse.
    fn parse_time(hour_range: &str) -> Result<(Time, Time), ParserError> {
        let hour_range: Vec<&str> = hour_range.split('-').into_iter().collect();
        let start_hours: u8 = hour_range[0][0..2].parse()?;
        let start_minutes: u8 = hour_range[0][2..].parse()?;
        let end_hours: u8 = hour_range[1][0..2].parse()?;
        let end_minutes: u8 = hour_range[1][2..].parse()?;

        let start_time: Time = Time::try_from_hms(start_hours, start_minutes, 0)?;
        let end_time: Time = Time::try_from_hms(end_hours, end_minutes, 0)?;
        Ok((start_time, end_time))
    }
}

/// `Day` represents all valid days that courses can be held.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Online,
    Arranged,
}

impl Day {
    /// `match_day` is a private helper function for `parse_days`. It matches valid `char` types to `Day`.
    ///
    /// # Arguments
    ///
    /// * `focus` - A `char` to parse and match to its respective `Day` enum.
    fn match_day(focus: char) -> Day {
        match focus {
            'M' => Day::Monday,
            'T' => Day::Tuesday,
            'W' => Day::Wednesday,
            'H' => Day::Thursday, // 'H' is matched since we iterate backwards in `parse_days`
            'F' => Day::Friday,
            _ => Day::Online,
        }
    }

    /// `parse_days` generates a Vec<Day> based upon a valid input string.
    ///
    /// # Arguments
    ///
    /// * `days` - A string slice that contains the data to parse.
    pub fn parse_days(days: &str) -> Vec<Day> {
        let mut days_vector: Vec<char> = days.chars().collect();
        let mut return_vector: Vec<Day> = Vec::new();
        loop {
            let focus: Day = match days_vector.pop() {
                Some(day) => Day::match_day(day),
                None => break, // break out of loop if vector is empty (len == 0)
            };
            match focus {
                Day::Thursday => {
                    days_vector.pop(); // pop twice to get rid of the 'T' char in 'TH'
                    return_vector.push(focus);
                }
                _ => return_vector.push(focus),
            }
        }
        return_vector.reverse();

        return_vector
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ParserError;
    use time::*;

    #[test]
    fn regular_days() {
        let day_string: String = String::from("MWF");
        let day_vec: Vec<Day> = Day::parse_days(&day_string);
        assert!(matches!(day_vec[0], Day::Monday));
        assert!(matches!(day_vec[1], Day::Wednesday));
        assert!(matches!(day_vec[2], Day::Friday));
    }

    #[test]
    fn tuesday_thursday() {
        let day_string: String = String::from("TTH");
        let day_vec: Vec<Day> = Day::parse_days(&day_string);
        assert!(matches!(day_vec[0], Day::Tuesday));
        assert!(matches!(day_vec[1], Day::Thursday));
    }

    #[test]
    fn online() {
        let day_string: String = String::from("O");
        let day_vec: Vec<Day> = Day::parse_days(&day_string);
        assert!(matches!(day_vec[0], Day::Online));
    }

    #[test]
    fn empty() {
        let day_string: String = String::from("");
        let day_vec: Vec<Day> = Day::parse_days(&day_string);
        assert_eq!(day_vec.len(), 0);
    }

    #[test]
    fn normal_times() {
        let hour_range = "1200-1250";
        assert_eq!(
            DateTime::parse_time(hour_range).unwrap(),
            (time!(12:00), time!(12:50))
        );
    }

    #[test]
    fn malformed_time() {
        let hour_range = DateTime::parse_time("xxxx-xxxx").map(|hr| hr);
        assert!(hour_range.is_err());
    }
}
