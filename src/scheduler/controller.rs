use crate::scraper::{Options, SDSUSpider};
use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use super::{schedule::Schedule, Course, Parameters};

pub struct Controller {
    courses: HashMap<String, Vec<Course>>,
    schedules: Vec<Schedule>,
    parameters: Parameters,
}

// FIXME: Error handle if 0 schedules.
// for course in &schedules[schedules.len() - 1].courses {
//     write!(
//         "Course: {}-{} ({}) - {}",
//         course.course_subject, course.course_number, course.schedule_num, course.url
//     );
//     for meeting in course.meetings.iter() {
//         write!(
//             "    This is a {} taught by {:?} {} on {:?} between {}{}-{}{}.",
//             meeting.meeting_type,
//             meeting.instructor,
//             meeting.location,
//             meeting.date.day,
//             meeting.date.start_time.hour(),
//             meeting.date.start_time.minute(),
//             meeting.date.end_time.hour(),
//             meeting.date.end_time.minute(),
//         );
//     }
//     write!(
//         "    There are {}/{} seats available.\n",
//         course.seats_available, course.seats_total
//     );
// }

impl Controller {
    pub fn new(parameters: Parameters) -> Self {
        let courses = match parameters.school.as_ref() {
            "SDSU" => SDSUSpider::new(Options::from_params(&parameters)).parse(),
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

        if self.parameters.include_professors.len() != 0 {
            let mut num_matches = 1;
            if self.parameters.include_all_professors {
                num_matches = self.parameters.include_professors.len();
            }
            // Retrieve all professors from configuration file and make them uppercase.
            let professors: HashSet<String> = HashSet::from_iter(
                self.parameters
                    .include_professors
                    .clone()
                    .into_iter()
                    .map(|p| p.to_uppercase()),
            );
            // Only include schedules where it has at least one instructor matching the given list.
            self.schedules.retain(|s| {
                HashSet::from_iter(s.courses.iter().flat_map(|c| c.meetings.iter()).map(|m| {
                    // Create a set with all of this schedule's instructors.
                    m.instructor
                        .clone()
                        .split_whitespace()
                        .nth(1)
                        .unwrap()
                        .to_uppercase()
                }))
                // Perform a set intersection between this schedule's instructors and the
                // configuration.
                .intersection(&professors)
                // Check if it includes the some or all of the professors.
                .count()
                    >= num_matches
            })
        }

        info!("Validated {} schedules.", self.schedules.len());

        self.schedules.sort_by(|s1, s2| s1.fitness.cmp(&s2.fitness));

        Ok(())
    }
}
