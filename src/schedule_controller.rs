use crate::course::{Course, SqlCourse};
use crate::database::DatabaseConnection;
use crate::meeting::{Meeting, SqlMeeting};
use crate::schedule::Schedule;
use crate::schedule_parameters::ScheduleParameters;
use itertools::Itertools;
use std::collections::HashMap;
use tokio::join;
use tokio::runtime::Runtime;

/// The `ScheduleController` type is an abstraction for easy interaction with the database
/// with powerful data manipulation capabilities.
pub struct ScheduleController {
    pub schedules: Vec<Schedule>,
    course_list: Vec<String>,
    db: DatabaseConnection,
    schedule_parameters: ScheduleParameters,
}

impl ScheduleController {
    pub fn new(
        schedule_parameters: ScheduleParameters,
        course_list: Vec<String>,
    ) -> anyhow::Result<Self> {
        let sc = ScheduleController {
            schedules: Vec::new(),
            course_list,
            db: DatabaseConnection::new()?,
            schedule_parameters,
        };
        Ok(sc)
    }

    pub fn generate_schedules(&mut self) -> anyhow::Result<()> {
        let mut all_classes: Vec<Vec<Course>> = Vec::new();
        for course in &self.course_list {
            let c = self.generate_courses(course)?;
            all_classes.push(c);
        }

        let mut combinations: Vec<Course> = all_classes.into_iter().flatten().collect(); // flattens to Vec<Course>

        let mut combinations = combinations
            .into_iter()
            .combinations(self.course_list.len());

        // Create and add only valid schedules
        combinations
            .into_iter()
            .map(|s| Schedule::new(s, 0))
            .filter(|s| s.is_valid())
            .map(|s| self.schedules.push(s));
        println!("{}", self.schedules.len());
        Ok(())
    }

    #[tokio::main]
    async fn generate_courses(&self, course_string: &str) -> anyhow::Result<Vec<Course>> {
        let mut conn = self.db.pool().acquire().await?;
        let sql_courses: Vec<SqlCourse> = sqlx::query_as!(
            SqlCourse,
            "
            SELECT *
            FROM course
            WHERE course.course
            LIKE ?
            ",
            course_string
        )
        .fetch_all(&mut conn)
        .await?;

        let mut converted_courses: Vec<Course> = Vec::new();
        // within this loop, we will handle meeting/footnote generation as well as conversion from
        // `SqlCourse` to `Course`
        for sql_course in sql_courses {
            let (meetings, footnotes) = join![
                self.generate_meetings(&sql_course.id),
                self.generate_footnotes(&sql_course.id),
            ];

            converted_courses.push(SqlCourse::into_course(sql_course, footnotes?, meetings?));
        }

        Ok(converted_courses)
    }

    ///
    async fn generate_meetings(&self, course_id: &str) -> anyhow::Result<Vec<Meeting>> {
        let mut conn = self.db.pool().acquire().await?;
        let sql_meetings: Vec<SqlMeeting> = sqlx::query_as!(
            SqlMeeting,
            "
            SELECT *
            FROM meeting
            WHERE meeting.course_id
            LIKE ?
            ",
            course_id
        )
        .fetch_all(&mut conn)
        .await?;

        let mut converted_meetings = vec![Vec::new()];
        for sql_meeting in sql_meetings {
            converted_meetings.push(SqlMeeting::into_meeting(sql_meeting));
        }
        Ok(converted_meetings.into_iter().flatten().collect())
    }

    async fn generate_footnotes(&self, course_id: &str) -> anyhow::Result<HashMap<String, String>> {
        let mut conn = self.db.pool().acquire().await?;
        let mut footnotes: HashMap<String, String> = HashMap::new();
        let queried_footnotes = sqlx::query!(
            "
            SELECT footnote.code, footnote.text
            FROM footnote
            WHERE footnote.course_id
            LIKE ?
            ",
            course_id
        )
        .fetch_all(&mut conn)
        .await?;

        for q in queried_footnotes {
            footnotes.insert(q.code.unwrap(), q.text.unwrap());
        }

        Ok(footnotes)
    }
}
