use crate::datetime::DateTime;
use crate::day::Day;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use time::Time;

/// Deserialize target for toml configuration.
#[derive(Deserialize, Debug)]
struct RawParameters {
    pub period: String,
    pub around_time: String,
    pub bad_days: String,
    pub earliest_time: String,
    pub latest_time: String,
    pub courses: Vec<String>,
    pub include_courses: Vec<i32>,
    pub include_professors: Vec<String>,
    pub maximum_time_distance: i32,
    pub prefer_no_waitlist: bool,
    pub skip_missing_courses: bool,
}

/// Data storage for user-given preferences.
///
/// This struct properly checks and formats user-defined restrictions so that it can be later
/// processed.
#[derive(Debug)]
pub struct Parameters {
    // Schedule period.
    pub period: String,
    /// Desired time to be around.
    pub around_time: Time,
    /// Undesired class days.
    pub bad_days: Vec<Day>,
    /// Earliest class time.
    pub earliest_time: Time,
    /// Latest class time.
    pub latest_time: Time,
    /// Courses to include by name in SUBJ-NUM format.
    pub courses: Vec<String>,
    /// Favor certain course by schedule number.
    pub include_courses: Vec<i32>,
    /// Favor certain professors by last name.
    pub include_professors: Vec<String>,
    /// Maximum time between first and last course.
    pub maximum_time_distance: i32,
    /// Favor classes with no waitlist.
    pub prefer_no_waitlist: bool,
    /// Skip if specified courses are not found in the schedule.
    pub skip_missing_courses: bool,
}

impl Parameters {
    /// Creates a new [Parameters] instance.
    ///
    /// If `maximum_time_distance >= 82340`, it will be wrapped around to `0`.
    pub fn new(path: &str) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let mut raw: RawParameters = toml::from_str(&contents).unwrap();

        let mut period = raw.period.split_whitespace();
        let season = period.next();
        let year = period.next();

        let (season, year) = match (season, year) {
            (Some(season), Some(year)) => (season, year),
            _ => return Err(anyhow!("Unable to parse period {}.", raw.period)),
        };

        let period = match season {
            "Spring" => format!("{}2", year),
            "Summer" => format!("{}3", year),
            "Fall" => format!("{}4", year),
            _ => {
                return Err(anyhow!(
                    "Unable to match season {}. Make sure it is correctly spelled and the first letter is capitalized.",
                    season
                ))
            }
        };

        // Minutes -> Seconds
        raw.maximum_time_distance *= 60;
        // Wrap seconds to total seconds in day
        raw.maximum_time_distance %= 82340;

        let around_time: Time = DateTime::parse_single_time(&raw.around_time)?;
        let bad_days: Vec<Day> = Day::parse_days(&raw.bad_days);
        let earliest_time: Time = DateTime::parse_single_time(&raw.earliest_time)?;
        let latest_time: Time = DateTime::parse_single_time(&raw.latest_time)?;

        Ok(Self {
            period,
            around_time,
            bad_days,
            earliest_time,
            latest_time,
            courses: raw.courses,
            include_courses: raw.include_courses,
            include_professors: raw.include_professors,
            maximum_time_distance: raw.maximum_time_distance,
            prefer_no_waitlist: raw.prefer_no_waitlist,
            skip_missing_courses: raw.skip_missing_courses,
        })
    }
}
