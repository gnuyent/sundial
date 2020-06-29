use crate::meeting::Meeting;
use sqlx::FromRow;
use std::collections::HashMap;

/// `Course` is a correctly parsed and formatted version of `SqlCourse`.
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
    pub period: u16,
    pub prerequisite: String,
    pub schedule_num: u16,
    pub seats_available: i32,
    pub seats_open: i32,
    pub section: u8,
    pub session: String,
    pub statement: String,
    pub units: f32,
    pub url: String,
    pub waitlist: bool,
}

impl Default for Course {
    fn default() -> Self {
        Course {
            course_hours: String::from(""),
            course_title: String::from(""),
            course: String::from(""),
            description: String::from(""),
            footnotes: HashMap::new(),
            full_title: String::from(""),
            general_text: String::from(""),
            id: String::from(""),
            meetings: vec![Meeting::default()],
            period: 0,
            prerequisite: String::from(""),
            schedule_num: 0,
            seats_available: 0,
            seats_open: 0,
            section: 0,
            session: String::from(""),
            statement: String::from(""),
            units: 0.0,
            url: String::from(""),
            waitlist: true,
        }
    }
}

/// `SqlCourse` directly corresponds to the database and allows for immediate extraction from
/// queries.
#[derive(FromRow)]
pub struct SqlCourse {
    pub course_hours: String,
    pub course_title: String,
    pub course: String,
    pub description: String,
    pub full_title: String,
    pub general_text: String,
    pub id: String,
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
}

impl SqlCourse {
    pub fn into_course(
        original: Self,
        footnotes: HashMap<String, String>,
        meetings: Vec<Meeting>,
    ) -> Course {
        let waitlist = original.seats_open == 0; // all seats are taken

        Course {
            course_hours: original.course_hours,
            course_title: original.course_title,
            course: original.course,
            description: original.description,
            footnotes,
            full_title: original.full_title,
            general_text: original.general_text,
            id: original.id,
            meetings,
            period: original.period as u16,
            prerequisite: original.prerequisite,
            schedule_num: original.schedule_num as u16,
            seats_available: original.seats_available,
            seats_open: original.seats_open,
            section: original.section as u8,
            session: original.session,
            statement: original.statement,
            units: original.units,
            url: original.url,
            waitlist,
        }
    }
}
