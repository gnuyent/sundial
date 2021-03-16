use crate::scraper::{Options, SDSUSpider};
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

use super::{schedule::Schedule, Parameters};

pub struct Controller {
    schedules: Vec<Schedule>,
    parameters: Parameters,
}

impl Controller {
    pub fn new(parameters: Parameters) -> Self {
        Self {
            schedules: Vec::new(),
            parameters,
        }
    }

    pub fn generate_schedules(&self) {
        let courses = match self.parameters.school.as_ref() {
            "SDSU" => SDSUSpider::new(Options::from_params(&self.parameters)).parse(),
            _ => panic!("Unable to create a spider."),
        }
        .unwrap();

        let schedules_raw = courses
            .values()
            .into_iter()
            .map(IntoIterator::into_iter)
            .multi_cartesian_product()
            .collect_vec();

        info!("Generated {} schedules.", schedules_raw.len());

        let mut schedules: Vec<Schedule> = Vec::new();

        for schedule in schedules_raw {
            let mut s = Schedule::new(schedule);
            if s.is_valid() {
                s.calculate_fitness(&self.parameters);
                schedules.push(s);
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
            schedules = schedules
                .into_iter()
                .filter(|s| {
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
                    .collect::<HashSet<_>>()
                    // Check if it includes the some or all of the professors.
                    .len()
                        >= num_matches
                })
                .collect();
        }

        info!("Validated {} schedules.", schedules.len());
        // FIXME: Error handle if 0 schedules.

        schedules.sort_by(|s1, s2| s1.fitness.cmp(&s2.fitness));

        for course in &schedules[schedules.len() - 1].courses {
            println!(
                "Course: {}-{} ({}) - {}",
                course.course_subject, course.course_number, course.schedule_num, course.url
            );
            for meeting in course.meetings.iter() {
                println!(
                    "    This is a {} taught by {:?} {} on {:?} between {}{}-{}{}.",
                    meeting.meeting_type,
                    meeting.instructor,
                    meeting.location,
                    meeting.date.day,
                    meeting.date.start_time.hour(),
                    meeting.date.start_time.minute(),
                    meeting.date.end_time.hour(),
                    meeting.date.end_time.minute(),
                );
            }
            println!(
                "    There are {}/{} seats available.\n",
                course.seats_available, course.seats_total
            );
        }
    }
}