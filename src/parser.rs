use crate::course::Course;
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
        let course = parse_single_course(&course_url)?;
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

    info!(
        "Successfully found {} subjects for {}.",
        urls.capacity(),
        period
    );

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

fn parse_single_course(course_url: &str) -> Result<Course> {
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

    info!(
        "Parsed {}-{} ({}) from {}.",
        course_details.get("Course").unwrap(),
        course_details.get("Section").unwrap(),
        course_details.get("Schedule #").unwrap(),
        course_url
    );

    Ok(Course::new(course_details))
}
