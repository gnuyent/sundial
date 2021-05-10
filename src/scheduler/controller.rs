use crate::scraper::{Options, SdsuSpider};
use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fmt;

use super::{schedule::Schedule, Course, Parameters};

pub struct Controller {
    courses: HashMap<String, Vec<Course>>,
    schedules: Vec<Schedule>,
    parameters: Parameters,
}

impl fmt::Display for Controller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out_str = String::new();

        // FIXME: Error handle if 0 schedules.
        for course in &self.schedules[self.schedules.len() - 1].courses {
            out_str.push_str(&format!(
                "    Course.......: {}-{} ({}) - {}\n",
                course.course_subject, course.course_number, course.schedule_num, course.url
            ));
            for meeting in course.meetings.iter() {
                let meeting_time: String = match meeting.dates.len() {
                    0 => String::from("Asynchronous"),
                    _ => format!(
                        "{:0>2}{:0>2}-{:0>2}{:0>2}",
                        meeting.dates[0].start_time.hour(),
                        meeting.dates[0].start_time.minute(),
                        meeting.dates[0].end_time.hour(),
                        meeting.dates[0].end_time.minute()
                    ),
                };

                let meeting_location: String = match meeting.location.as_ref() {
                    "ON-LINE" => String::from("Online"),
                    _ => meeting.location.clone(),
                };

                out_str.push_str(&format!(
                    "    Meeting Type.: {}
    Instructor...: {}
    Location.....: {}
    Day(s).......: {:?}
    Time.........: {}
    Seats........: {}/{}\n\n",
                    meeting.meeting_type,
                    meeting.instructor.to_uppercase(),
                    meeting_location,
                    meeting.days(),
                    meeting_time,
                    course.seats_available,
                    course.seats_total,
                ));
            }
        }

        write!(f, "{}", out_str)
    }
}

impl Controller {
    pub fn new(parameters: Parameters) -> Self {
        let courses = match parameters.school.as_ref() {
            "SDSU" => SdsuSpider::new(Options::from_params(&parameters)).parse(),
            _ => panic!("Unable to create a spider."),
        }
        .unwrap();

        Self {
            courses,
            schedules: Vec::new(),
            parameters,
        }
    }

    pub fn generate_schedules(&mut self) -> Result<()> {
        let schedules_raw = self
            .courses
            .values()
            .into_iter()
            .map(IntoIterator::into_iter)
            .multi_cartesian_product()
            .collect_vec();

        info!("Generated {} schedules.", schedules_raw.len());

        for schedule in schedules_raw {
            if let Some(s) = Schedule::new(schedule) {
                s.calculate_fitness(&self.parameters)?;
                self.schedules.push(s);
            }
        }

        if self.parameters.include_professors.is_empty() {
            let mut num_matches = 1;
            if self.parameters.include_all_professors {
                num_matches = self.parameters.include_professors.len();
            }
            // Retrieve all professors from configuration file and make them uppercase.
            let professors: HashSet<String> = self
                .parameters
                .include_professors
                .clone()
                .into_iter()
                .map(|p| p.to_uppercase())
                .collect();

            // Only include schedules where it has at least one instructor matching the given list.
            self.schedules.retain(|s| {
                s.courses
                    .iter()
                    .flat_map(|c| c.meetings.iter())
                    .map(|m| {
                        // Create a set with all of this schedule's instructors.
                        m.instructor
                            .clone()
                            .split_whitespace()
                            .nth(1)
                            .unwrap_or("NO INSTRUCTOR")
                            .to_uppercase()
                    })
                    .collect::<HashSet<String>>()
                    // Perform a set intersection between this schedule's instructors and the
                    // configuration.
                    .intersection(&professors)
                    // Check if it includes the some or all of the professors.
                    .count()
                    >= num_matches
            })
        }

        info!("Validated {} schedules.", self.schedules.len());
        if self.schedules.len() == 0 {
            info!("Unable to generate any valid schedules. Exiting.");
            std::process::exit(1);
        }

        self.schedules.sort_by(|s1, s2| s1.fitness.cmp(&s2.fitness));

        Ok(())
    }
}
