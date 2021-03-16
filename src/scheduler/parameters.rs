use anyhow::Result;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Parameters {
    pub school: String,
    pub period: String,
    pub around_time: String,
    pub bad_days: Vec<String>,
    pub earliest_time: String,
    pub latest_time: String,
    pub courses: Vec<String>,
    pub skip_missing_courses: bool,
    pub include_courses: Vec<String>,
    pub include_professors: Vec<String>,
    pub include_all_professors: bool,
    pub maximum_time_distance: i32,
    pub prefer_no_waitlist: bool,
}

impl Parameters {
    pub fn new(path: &str) -> Result<Self> {
        let config: String = fs::read_to_string(path)?;
        Ok(toml::from_str(&config)?)
    }
}
