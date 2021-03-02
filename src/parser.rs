use crate::course::Course;
use crate::meeting::Meeting;
use anyhow::{anyhow, Result};
use scraper::{Html, Selector};
use std::collections::HashMap;

const BASE_URL: &str = "https://sunspot.sdsu.edu";

// TODO: Cache results. Async?
pub fn parse_courses(
    course_name: &str,
    period: &str,
    subject_urls: &HashMap<String, String>,
) -> Result<Vec<Course>> {
    let search_subject = course_name.split("-").collect::<Vec<&str>>()[0];
    let course_urls: Vec<String>;
    if let Some(subject) = subject_urls.get(search_subject) {
        course_urls = get_course_urls(course_name, subject, period)?;
        if course_urls.len() == 0 {
            return Err(anyhow!("Found no courses matching {}.", course_name));
        }
    } else {
        panic!("Unknown subject '{}'", search_subject);
    }

    // 6 classes * 3 units per course = 18 units before allocating to heap
    let mut all_courses: Vec<Course> = Vec::new();
    for course_url in course_urls {
        let mut course = Course::from_url(&course_url)?;
        let meetings = Meeting::from_url(&course_url)?;
        course.meetings = meetings;
        all_courses.push(course);
    }

    Ok(all_courses)
}

pub fn get_subject_urls(period: &str) -> Result<HashMap<String, String>> {
    let search_url = format!(
        "{}/schedule/search?mode=browse_by_subject&category=browse_by_subject&period={}",
        BASE_URL, period
    );
    let response = reqwest::blocking::get(&search_url)?.text()?;

    let fragment = Html::parse_fragment(&response);
    let selector = Selector::parse("#browseContainer > ul > li > a").unwrap();

    let mut urls = HashMap::new();

    for element in fragment.select(&selector) {
        let href = element.value().attr("href").unwrap();
        let subject = href.split("=").collect::<Vec<&str>>()[2].to_string();
        let subject_url = format!("{}/schedule/{}&period={}", BASE_URL, href, period);
        urls.insert(subject, subject_url);
    }

    info!("Found {} subjects for {}.", urls.capacity(), period);

    Ok(urls)
}

fn get_course_urls(course: &str, subject_url: &str, period: &str) -> Result<Vec<String>> {
    let response = reqwest::blocking::get(subject_url)?.text()?;
    let fragment = Html::parse_fragment(&response);
    let selector = Selector::parse("div.sectionFieldCourse > a").unwrap();

    let mut course_urls = Vec::new();

    for element in fragment.select(&selector) {
        let focus_course = element.inner_html();
        if focus_course == course {
            let href = element.value().attr("href").unwrap();
            let course_url = format!("{}/schedule/{}&period={}", BASE_URL, href, period);

            course_urls.push(course_url);
        }
    }

    Ok(course_urls)
}
