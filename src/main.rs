mod course;
mod datetime;
mod day;
mod parameters;
mod parser;
mod schedule;
use course::Course;
use std::collections::HashMap;
extern crate clap;
use clap::Clap;
use itertools::Itertools;
use schedule::Schedule;

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;

/// An automatic class scheduler for San Diego State University.
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Brandon N. <gnuyent@protonmail.com>")]
struct Opts {
    /// Sets a custom config file
    #[clap(
        short,
        long,
        default_value = "config.toml",
        value_name = "FILE",
        takes_value = true
    )]
    config: String,
    /// Level of verbosity, can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}

//#[tokio::main]
fn main() {
    let opts: Opts = Opts::parse();

    let level: LevelFilter;

    match opts.verbose {
        0 => level = LevelFilter::Off,
        1 => level = LevelFilter::Error,
        2 => level = LevelFilter::Warn,
        3 | _ => level = LevelFilter::Info,
    }

    let mut config: ConfigBuilder = ConfigBuilder::new();
    config.set_time_to_local(true);
    let config: Config = config.build();

    CombinedLogger::init(vec![
        TermLogger::new(level, config, TerminalMode::Mixed).unwrap()
    ])
    .unwrap();

    let params = match parameters::Parameters::new(&opts.config) {
        Ok(p) => {
            info!("Configuration file successfully parsed.");
            p
        }
        Err(e) => {
            error!("Error parsing configuration file.");
            panic!("{}", e);
        }
    };

    //let subject_urls: HashMap<String, String> = match parser::get_subject_urls(&params.period) {
    //    Ok(s) => s,
    //    Err(e) => {
    //        error!("Unable to retrieve subject URLs. Did the website change?");
    //        panic!("{}", e);
    //    }
    //};

    //let mut all_courses: Vec<Vec<Course>> = Vec::new();

    //for course in params.courses.iter() {
    //    match parser::parse_courses(&course, &params.period, &subject_urls) {
    //        Ok(c) => {
    //            all_courses.push(c);
    //            info!("Successfully parsed {}.", course);
    //        }
    //        Err(e) => {
    //            if params.skip_missing_courses {
    //                warn!("Unable to parse courses for {}.", course);
    //                continue;
    //            } else {
    //                error!("Parsing {} was unsuccessful.", course);
    //                error!("Make sure that this course is available in the current season.");
    //                panic!("{}", e);
    //            }
    //        }
    //    };
    //}

    course::Meeting::from_url("https://sunspot.sdsu.edu/schedule/sectiondetails?scheduleNumber=21127&period=20212&admin_unit=R").unwrap();

    //info!("Generating schedules...");
    //let schedules_raw = all_courses
    //    .into_iter()
    //    .map(IntoIterator::into_iter)
    //    .multi_cartesian_product()
    //    .collect_vec();

    //info!("Generated {} schedules.", schedules_raw.len());

    //let mut schedules: Vec<Schedule> = Vec::new();

    //let mut discard_counter: i32 = 0;

    //for schedule in schedules_raw {
    //    let mut s = Schedule::new(schedule);
    //    if s.is_valid() {
    //        s.calculate_fitness(&params);
    //        schedules.push(s);
    //    } else {
    //        discard_counter += 1;
    //    }
    //}

    //info!("Validated {} schedules.", schedules.len());
    //info!("Discarded {} schedules.", discard_counter);

    //println!("{:#?}", schedules);
}
