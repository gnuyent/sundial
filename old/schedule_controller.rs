use crate::course::Course;
use crate::meeting::Meeting;
use crate::schedule::Schedule;
use crate::schedule_parameters::ScheduleParameters;
use crate::tools::{parse_days, parse_time, Days};
use futures::executor::block_on;
use itertools::Itertools;
use sqlx::SqlitePool;
use std::collections::HashMap;
use time::Time;

/// The `ScheduleController` type is an entry point for easy API interaction without any
/// manual calls to subclasses.
pub struct ScheduleController {
    pub schedules: Vec<Schedule>,
    course_list: Vec<String>,
    schedule_parameters: ScheduleParameters,
}

impl ScheduleController {
    pub fn new(schedule_parameters: ScheduleParameters, course_list: Vec<String>) -> Self {
        ScheduleController {
            schedules: Vec::new(),
            course_list,
            schedule_parameters,
        }
    }

    /// Generates all potential valid and non-overlapping course schedules.
    pub fn generate_schedules(&mut self) {
        let all_classes: Vec<Course> = match block_on(self.retrieve_all_classes()) {
            Ok(val) => val,
            Err(_) => panic!("Unable to generate schedules!"),
        };
        // Generate every possible combination of schedules (including overlapping)
        let mut combinations = all_classes.into_iter().combinations(self.course_list.len());

        // Filter schedules to ones that do not overlap
        for schedule in combinations {
            let s = Schedule::new(schedule, 0);
            if s.is_valid() {
                self.schedules.push(s);
            }
        }
    }

    // Private helper function for `generate_schedules`. Generates a list of all courses with correct metadata.
    async fn retrieve_all_classes(&self) -> Result<Vec<Course>, sqlx::Error> {
        let pool = SqlitePool::new("sqlite://src/classes.db").await?;
        let mut all_classes: Vec<Course> = Vec::new();

        for course in self.course_list.iter() {
            let mut cursor = sqlx::query!(
                "
                SELECT course.*
                FROM course
                WHERE course.course
                LIKE ?
                ",
                course
            )
            // .bind(course)
            .fetch_all(&pool)
            .await?;

            for row in cursor {
                let row_id: String = row.id.clone();
                println!("{}", row_id);

                // Generate the meetings array
                let mut course_meetings = sqlx::query!(
                    "
                    SELECT meeting.meeting_id, meeting.meeting_type, meeting.hours, meeting.days, meeting.location, meeting.instructor
                    FROM meeting
                    WHERE meeting.course_id = ?
                    ",
                    row_id.clone()
                )
                .fetch_all(&pool)
                .await?;
                let mut meetings: Vec<Meeting> = Vec::new();
                for meeting in course_meetings {
                    // Variables are defined here so they do not need to be recreated on the for loop below
                    let parsed_days: Vec<Days> = parse_days(&meeting.days.unwrap());
                    let meeting_id: String = meeting.meeting_id;
                    let meeting_type: String = meeting.meeting_type.unwrap();
                    let hours: [Time; 2] = parse_time(&meeting.hours.unwrap());
                    let location: String = meeting.location.unwrap();
                    let instructor: String = meeting.instructor.unwrap();
                    for individual_day in parsed_days {
                        let m: Meeting = Meeting {
                            meeting_id: meeting_id.clone(),
                            meeting_type: meeting_type.clone(),
                            hours,
                            day: individual_day,
                            location: location.clone(),
                            instructor: instructor.clone(),
                        };

                        meetings.push(m);
                    }
                }

                // Generate the footnotes array
                let mut course_footnotes = sqlx::query!(
                    "
                    SELECT footnote.code, footnote.text
                    FROM footnote
                    WHERE footnote.course_id = ?
                    ",
                    row_id.clone()
                )
                .fetch_all(&pool)
                .await?;
                let mut footnotes: HashMap<String, String> = HashMap::new();
                for footnote in course_footnotes {
                    footnotes.insert(footnote.code.unwrap(), footnote.text.unwrap());
                }
                let seats_open: i32 = row.seats_open.unwrap();
                let waitlist: bool = &seats_open == &0;
                let new_course = Course {
                    course_hours: row.course_hours.unwrap(),
                    course_title: row.course_title.unwrap(),
                    course: row.course.unwrap(),
                    description: row.description.unwrap(),
                    footnotes,
                    full_title: row.full_title.unwrap(),
                    general_text: row.general_text.unwrap(),
                    id: row.id,
                    meetings,
                    period: row.period.unwrap(),
                    prerequisite: row.prerequisite.unwrap(),
                    schedule_num: row.schedule_num.unwrap(),
                    seats_available: row.seats_available.unwrap(),
                    seats_open,
                    section: row.section.unwrap(),
                    session: row.session.unwrap(),
                    statement: row.statement.unwrap(),
                    units: row.units.unwrap(),
                    url: row.url.unwrap(),
                    waitlist,
                };
                all_classes.push(new_course);
            }
        }

        Ok(all_classes)
    }

    /// Returns the schedule with the highest fitness score.
    pub fn best_schedule(&self) -> Option<&Schedule> {
        match self.schedules.get(0) {
            Some(s) => return Some(s),
            None => None,
        }
    }
}
