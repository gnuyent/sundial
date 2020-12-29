use crate::day::Day;
use anyhow::Result;
use std::cmp::Ordering;
use time::Time;

/// A wrapper around [Time] to include the [Day] enum and a start/end time.
#[derive(Copy, Clone, Debug, Eq, Hash)]
pub struct DateTime {
    pub day: Day,
    pub start_time: Time,
    pub end_time: Time,
}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start_time.cmp(&other.start_time)
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.start_time.partial_cmp(&other.start_time)
    }
}

impl PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.day == other.day
            && self.start_time == other.start_time
            && self.end_time == other.end_time
    }
}

impl DateTime {
    /// Creates a new [DateTime] instance.
    pub fn new(day: Day, start_time: Time, end_time: Time) -> Self {
        DateTime {
            day,
            start_time,
            end_time,
        }
    }
    /// Generates a [Time] object from a valid time string.
    ///
    /// A time string is defined as a [String] in `HHMM` format.
    ///
    /// # Examples
    /// ```
    /// let time_string: String = "1530".to_string();
    /// assert_eq!(parse_single_time(time_string), Time::try_from_hms(15, 30, 0)?);
    /// # Ok::<(), time::error:ComponentRange>(())
    /// ```
    /// ```
    /// let time_string: String = "-0001".to_string();
    /// assert!(parse_single_time(time_string).is_err());
    /// ```
    pub fn parse_single_time(time_string: String) -> Result<Time> {
        // TODO: Make minimums possible.
        let (hour, minute) = time_string.split_at(2);
        Ok(Time::try_from_hms(
            hour.to_string().parse::<u8>()?,
            minute.to_string().parse::<u8>()?,
            0,
        )?)
    }

    /// Generates a tuple of two [Time] objects from a valid range time string.
    ///
    /// A range time string is defined as a [String] in `HHMM-HHMM` format.
    pub fn parse_time(range_string: String) -> Result<(Time, Time)> {
        let (left_side, right_side) = range_string.split_at(4);

        let start_time = DateTime::parse_single_time(left_side.to_string())?;

        let end_time = DateTime::parse_single_time(right_side.to_string())?;

        Ok((start_time, end_time))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::Time;

    #[test]
    fn parse_valid_times() {
        assert_eq!(
            DateTime::parse_single_time("0540".to_string()).unwrap(),
            Time::try_from_hms(5, 40, 0).unwrap()
        );
        assert_eq!(
            DateTime::parse_single_time("1315".to_string()).unwrap(),
            Time::try_from_hms(13, 15, 0).unwrap()
        );
    }

    #[test]
    fn parse_invalid_times() {
        assert!(DateTime::parse_single_time("2400".to_string()).is_err());
        assert!(DateTime::parse_single_time("-0001".to_string()).is_err());
    }

    #[test]
    fn datetime_comparisons() {
        assert!(
            DateTime::parse_single_time(String::from("2300")).unwrap()
                < DateTime::parse_single_time(String::from("2359")).unwrap()
        );
    }
}
