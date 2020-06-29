use crate::tools::Days;
use time::Time;

/// The `ScheduleParameters` type is defined by different preferences made by the user.
pub struct ScheduleParameters {
    /// `time::Time` representing time to search around.
    pub around_time: Time,
    /// `Vec<Days>` of weekdays to avoid.
    pub bad_day: Vec<Days>,
    pub earliest_time: Time,
    pub include_courses: Vec<u16>,
    pub include_professors: Vec<String>,
    pub latest_time: Time,
    maximum_time_distance: i32,
    pub prefer_no_waitlist: bool,
}

impl ScheduleParameters {
    pub fn new(
        around_time: Time,
        bad_day: Vec<Days>,
        earliest_time: Time,
        include_courses: Vec<u16>,
        include_professors: Vec<String>,
        latest_time: Time,
        maximum_time_distance: i32,
        prefer_no_waitlist: bool,
    ) -> Self {
        if maximum_time_distance >= 0 && maximum_time_distance <= 86340 {
            let mut sp = ScheduleParameters {
                around_time,
                bad_day,
                earliest_time,
                include_courses,
                include_professors,
                latest_time,
                maximum_time_distance,
                prefer_no_waitlist,
            };
            return sp;
        } else {
            panic!("Make sure the parameter 'maximum_time_distance' is within 0 and 86340.");
        }
    }

    pub fn maximum_time_distance(&self) -> i32 {
        self.maximum_time_distance
    }
}

#[cfg(test)]
mod tests {
    use crate::schedule_parameters::ScheduleParameters;
    use crate::tools::Days;
    use time::time;

    #[test]
    #[should_panic]
    fn test_negative_distance() {
        ScheduleParameters::new(
            time!(12:00),
            vec![Days::Online],
            time!(12:00),
            Vec::new(),
            Vec::new(),
            time!(12:00),
            -1,
            true,
        );
    }

    #[test]
    #[should_panic]
    fn test_overflow_distance() {
        ScheduleParameters::new(
            time!(12:00),
            vec![Days::Online],
            time!(12:00),
            Vec::new(),
            Vec::new(),
            time!(12:00),
            86341,
            true,
        );
    }
}
