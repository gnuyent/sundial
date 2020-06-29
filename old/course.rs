use crate::meeting::Meeting;
use std::collections::HashMap;

/// The `Course` struct holds all important data regarding a certain course. It matches the column names within the database file.
#[derive(Clone, Debug)]
pub struct Course {
    pub course_hours: String,
    pub course_title: String,
    pub course: String,
    pub description: String,
    pub footnotes: HashMap<String, String>,
    pub full_title: String,
    pub general_text: String,
    pub id: String,
    pub meetings: Vec<Meeting>,
    pub period: i32,
    pub prerequisite: String,
    pub schedule_num: i32,
    pub seats_available: i32,
    pub seats_open: i32,
    pub section: i32,
    pub session: String,
    pub statement: String,
    pub units: f32,
    pub url: String,
    pub waitlist: bool,
}
