use crate::datetime::DateTime;
use crate::meeting::Meeting;
use anyhow::Result;
use scraper::{Html, Selector};
use std::collections::HashMap;

/// A data structure holding course information which matches the corresponding database table.
#[derive(Clone, Debug, Default)]
pub struct Course {
    /// URL of the course.
    pub url: String,
    /// Abbreviation and number of the course (e.g. A E-100)
    pub course: String,
    /// Unique ID for the course.
    pub id: String,
    /// Vector containing every meeting that the course has.
    pub meetings: Vec<Meeting>,
    /// True if meeting has overlapping times, False otherwise. This value determines what time to
    /// use in calculating schedule overlap.
    pub overlaps: bool,
    /// Official schedule number according to the course catalog.
    pub schedule_num: i32,
    /// Maximum possible seats in the course.
    pub seats_total: i32,
    /// Available seats in the course.
    pub seats_available: i32,
    /// True if course is waitlisted (`seats_available == 0`), False otherwise.
    pub waitlist: bool,
}

impl Course {
    /// Creates a new [Course] instance.
    pub fn new(data_map: HashMap<&str, &str>) -> Course {
        let mut seats_available: i32 = 0;
        let mut seats_total: i32 = 0;
        match data_map.get("Seats") {
            Some(seats) => {
                let mut seats = seats.splitn(2, "/");
                seats_available = seats.next().unwrap().to_string().parse().unwrap();
                seats_total = seats.next().unwrap().to_string().parse().unwrap();
            }
            None => warn!("WARN: Unable to determine waitlist availability."),
        }

        Course {
            url: data_map.get("URL").unwrap().to_string(),
            course: data_map.get("Course").unwrap().to_string(),
            id: "1".to_string(),
            meetings: Vec::new(),
            overlaps: false,
            schedule_num: data_map
                .get("Schedule #")
                .unwrap()
                .to_string()
                .parse()
                .unwrap_or(0),
            seats_total,
            seats_available,
            waitlist: false,
        }
    }

    /// Calculates the longest time in an overlapping scenario.
    ///
    /// Some courses contain times that are overlapping (e.g. Monday 0800-0850, Monday 0800-0950).
    /// In this case, we want to determine what the largest is difference between all time ranges
    /// that overlap. From the previous example, this would be condensed to Monday 0800-0950.
    pub fn get_longest_overlap(&self) -> DateTime {
        let times: Vec<DateTime> = self
            .meetings
            .iter()
            .map(|x| x.date)
            .collect::<Vec<DateTime>>();

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

    pub fn from_url(course_url: &str) -> Result<Course> {
        let response = reqwest::blocking::get(course_url)?.text()?;
        let fragment = Html::parse_fragment(&response);

        //let label_selector = fragment.select(&Selector::parse("td.sectionDetailLabel").unwrap());
        //let content_selector = fragment.select(&Selector::parse("td.sectionDetailContent").unwrap());

        let label_selector = &Selector::parse("td.sectionDetailLabel").unwrap();
        let content_selector = &Selector::parse("td.sectionDetailContent").unwrap();
        //
        let mut course_details: HashMap<&str, &str> = HashMap::new();
        for (label, content) in fragment
            .select(&label_selector)
            .zip(fragment.select(&content_selector))
        {
            let label = label.text().next().unwrap();
            let content = content.text().next().unwrap();
            course_details.insert(label, content);
        }

        course_details.insert("URL", course_url.clone());

        info!(
            "Parsed {}-{} ({}) from {}.",
            course_details.get("Course").unwrap(),
            course_details.get("Section").unwrap(),
            course_details.get("Schedule #").unwrap(),
            course_details.get("URL").unwrap(),
        );

        Ok(Course::new(course_details))
    }
}

#[cfg(test)]
mod tests {
    //    use super::*;
    //    use crate::day::Day;
    //    use time::Time;
    //
    //    #[test]
    //    fn day_overlap() {
    //        let start_time = Time::try_from_hms(8, 0, 0).unwrap();
    //        let end_time_one = Time::try_from_hms(8, 50, 0).unwrap();
    //        let end_time_two = Time::try_from_hms(9, 50, 0).unwrap();
    //        let meeting_one = Meeting::new(
    //            DateTime::new(Day::Monday, start_time, end_time_one),
    //            String::from("A"),
    //        );
    //        let meeting_two = Meeting::new(
    //            DateTime::new(Day::Monday, start_time, end_time_two),
    //            String::from("B"),
    //        );
    //        let mut course = Course::default();
    //        course.meetings.push(meeting_one);
    //        course.meetings.push(meeting_two);
    //        assert_eq!(
    //            DateTime::new(
    //                Day::Monday,
    //                Time::try_from_hms(8, 0, 0).unwrap(),
    //                Time::try_from_hms(9, 50, 0).unwrap()
    //            ),
    //            course.get_longest_overlap()
    //        );
    //    }
}
