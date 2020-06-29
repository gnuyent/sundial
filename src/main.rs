mod course;
mod database;
mod datetime;
mod error;
mod meeting;
mod schedule;
mod schedule_controller;
mod schedule_parameters;
use crate::datetime::Day;
use crate::schedule_controller::ScheduleController;
use crate::schedule_parameters::ScheduleParameters;
#[macro_use]
use time::time;
use warp::Filter;

// #[tokio::main]
// async fn main() {
//     let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
//
//     warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
// }

fn main() {
    let sp = ScheduleParameters::new(
        time!(12:00),
        vec![Day::Friday],
        time!(10:00),
        vec![],
        vec![],
        time!(15:00),
        10800,
        true,
    );

    let mut sc = ScheduleController::new(
        sp,
        vec![
            "A S-92A".to_string(),
            "A S-200A".into(),
            "CS-310".into(),
            "CS-320".into(),
            "ENS-331".into(),
            "MATH-245".into(),
            "MATH-254".into(),
        ],
    )
    .unwrap();

    sc.generate_schedules().unwrap();

    for schedule in sc.schedules {
        println!("{:#?}", schedule);
    }
}
