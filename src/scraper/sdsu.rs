use super::Options;
use crate::scheduler::{Course, Meeting};
use anyhow::Result;
use scraper::{Html, Selector};
use std::collections::HashMap;

/// Scraper for San Diego State University
pub struct SDSUSpider {
    opts: Options,
    base_url: String,
}

impl SDSUSpider {
    pub fn new(opts: Options) -> Self {
        let base_url = String::from("https://sunspot.sdsu.edu/schedule/search?mode=search");
        Self { opts, base_url }
    }

    pub fn parse(&self) -> Result<HashMap<String, Vec<Course>>> {
        let runtime = tokio::runtime::Runtime::new().expect("unable to create a runtime.");
        let client = reqwest::Client::new();

        let mut courses: HashMap<String, Vec<Course>> = HashMap::new();
        let course_urls = runtime.block_on(self.parse_courses_urls(&client))?;

        for url in course_urls.iter() {
            let c = runtime.block_on(self.parse_course_information(&client, url))?;
            let course_identifier = format!("{}-{}", c.course_subject, c.course_number);
            info!("Parsed {}.", c);
            if let Some(arr) = courses.get_mut(&course_identifier) {
                arr.push(c)
            } else {
                courses.insert(course_identifier, vec![c]);
            }
        }

        Ok(courses)
    }

    async fn parse_courses_urls(&self, client: &reqwest::Client) -> Result<Vec<String>> {
        let mut course_urls: Vec<String> = Vec::new();

        for course in self.opts.courses.iter() {
            let mut t = course.splitn(2, "-");
            let subject = t.next().unwrap();
            let number = t.next().unwrap();
            let url = format!(
                "{}&period={}&abbrev={}&number={}",
                self.base_url, self.opts.period_num, subject, number
            );

            let body = client.get(&url).send().await?.text().await?;
            let body = Html::parse_document(&body);
            let selector = Selector::parse(".sectionFieldCourse > a").unwrap();

            for element in body.select(&selector) {
                let full_url = format!(
                    "https://sunspot.sdsu.edu/schedule/{}",
                    element.value().attr("href").unwrap()
                );
                course_urls.push(full_url);
            }
        }

        Ok(course_urls)
    }

    async fn parse_course_information(
        &self,
        client: &reqwest::Client,
        url: &str,
    ) -> Result<Course> {
        let body = client.get(url).send().await?.text().await?;
        let body = Html::parse_document(&body);

        let mut item = Course::default();
        item.miscellaneous = HashMap::new();
        let mut course_details: HashMap<String, String> = HashMap::new();

        for tr in body.select(&Selector::parse("tr").unwrap()) {
            let label = tr
                .select(&Selector::parse(".sectionDetailLabel").unwrap())
                .next()
                .unwrap()
                .text()
                .collect::<String>();
            let content = tr
                .select(&Selector::parse(".sectionDetailContent").unwrap())
                .next()
                .unwrap()
                .text()
                .collect::<String>();
            course_details.insert(label, content);
        }

        // Insert course details.
        for (label, content) in course_details.into_iter() {
            match label.as_ref() {
                "Period" => item.period = self.opts.period.clone(),
                "Course" => {
                    let mut split_course = content.splitn(2, "-");
                    item.course_subject = split_course.next().unwrap().to_string();
                    item.course_number = split_course.next().unwrap().to_string();
                }
                "Section" => item.section = format!("{:0>3}", content),
                "Schedule #" => item.schedule_num = content,
                "Units" => item.units = content,
                "Session" => item.session = content,
                "Seats" => {
                    let mut seats = content.splitn(2, "/");
                    item.seats_available = seats.next().unwrap().to_string();
                    item.seats_total = seats.next().unwrap().to_string();
                }
                "Full Title" => item.course_title = content,
                "Description" => item.description = content,
                "Prerequisite" => item.prerequisite = content,
                "Meetings" => item.meetings = SDSUSpider::parse_meetings(&body),
                _ => {
                    let mut key = label.clone();
                    key.make_ascii_lowercase();
                    key = label.replace(" ", "_");
                    drop(item.miscellaneous.insert(key, content)); // don't need value returned
                }
            }
        }

        Ok(item)
    }

    fn parse_meetings(body: &Html) -> Vec<Meeting> {
        let types = SDSUSpider::div_to_vec(body, ".sectionFieldType");
        let times = SDSUSpider::div_to_vec(body, ".sectionFieldTime");
        let days = SDSUSpider::div_to_vec(body, ".sectionFieldDay");
        let locations = SDSUSpider::div_to_vec(body, ".sectionFieldLocation");
        let instructors = SDSUSpider::div_to_vec(body, ".sectionFieldInstructor");

        let mut meetings: Vec<Meeting> = Vec::new();

        for idx in 0..types.len() {
            let meeting = Meeting::new(
                &times[idx],
                &days[idx],
                &types[idx],
                &locations[idx],
                &instructors[idx],
            ).unwrap();
            meetings.push(meeting);
        }

        meetings
    }

    fn div_to_vec(body: &Html, div: &str) -> Vec<String> {
        body.select(&Selector::parse(div).unwrap())
            .into_iter()
            .map(|t| t.text().collect::<String>().trim().to_owned())
            .collect()
    }
}
