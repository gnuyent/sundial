use crate::datetime::DateTime;
use crate::day::Day;
use anyhow::Result;
use itertools::izip;
use scraper::{Html, Selector};

/// A data structure holding meeting information which matches the corresponding database table.
#[derive(Clone, Debug)]
pub struct Meeting {
    /// Date that the meeting occurs on.
    pub date: DateTime,
    /// Type of class meeting.
    pub mtype: String,
    /// Meeting location.
    pub location: String,
    /// Course Instructor
    pub instructor: String,
}

impl Meeting {
    /// Creates a new [Meeting] instance.
    pub fn new(
        mtype: &str,
        time_range: &str,
        day: &str,
        location: &str,
        instructor: &str,
    ) -> Meeting {
        let (start, end) = DateTime::parse_time(time_range).unwrap_or(DateTime::empty_time());
        let date: DateTime = DateTime::new(Day::parse_days(day)[0], start, end);

        let mtype = mtype.to_string();
        let location = location.to_string();
        let instructor = instructor.to_string();
        Meeting {
            date,
            mtype,
            location,
            instructor,
        }
    }

    /// Generates a vector of meetings from a given URL.
    pub fn from_url(course_url: &str) -> Result<Vec<Meeting>> {
        let response = reqwest::blocking::get(course_url)?.text()?;
        let fragment = Html::parse_fragment(&response);

        let meeting_type = Selector::parse(".sectionFieldType").unwrap();
        let meeting_time = Selector::parse(".sectionFieldTime").unwrap();
        let meeting_day = Selector::parse(".sectionFieldDay").unwrap();
        let meeting_loc = Selector::parse(".sectionFieldLocation").unwrap();
        let meeting_instr = Selector::parse(".sectionFieldInstructor > a").unwrap();

        let mut meetings = Vec::new();
        for (_type, _time, _day, _loc, _instr) in izip!(
            fragment.select(&meeting_type),
            fragment.select(&meeting_time),
            fragment.select(&meeting_day),
            fragment.select(&meeting_loc),
            fragment.select(&meeting_instr),
        ) {
            let _type = _type.text().next().unwrap_or("").trim();
            let _time = _time.text().next().unwrap_or("").trim();
            let _day = _day.text().next().unwrap_or("").trim();
            let _loc = _loc.text().next().unwrap_or("").trim();
            let _instr = _instr.text().next().unwrap_or("").trim();
            info!(
                "Parsed meeting {} {} {} {} {}",
                _type, _time, _day, _loc, _instr
            );
            let value = Meeting::new(_type, _time, _day, _loc, _instr);
            meetings.push(value);
        }

        Ok(meetings)
    }
}
