use crate::datetime::Day;
use time::Time;

/// The `ScheduleParameters` type is defined by different preferences made by the user.
pub struct ScheduleParameters {
    /// `time::Time` representing time to search around.
    pub around_time: Time,
    /// `Vec<Day>` of weekdays to avoid.
    pub bad_day: Vec<Day>,
    pub earliest_time: Time,
    pub include_courses: Vec<u16>,
    pub include_professors: Vec<String>,
    pub latest_time: Time,
    maximum_time_distance: i32,
    pub prefer_no_waitlist: bool,
}

impl Default for ScheduleParameters {
    fn default() -> Self {
        ScheduleParameters::new(
            Time::midnight(),
            vec![Day::Friday],
            Time::midnight(),
            Vec::new(),
            Vec::new(),
            Time::midnight(),
            0,
            true,
        )
    }
}

impl ScheduleParameters {
    pub fn new(
        around_time: Time,
        bad_day: Vec<Day>,
        earliest_time: Time,
        include_courses: Vec<u16>,
        include_professors: Vec<String>,
        latest_time: Time,
        maximum_time_distance: i32,
        prefer_no_waitlist: bool,
    ) -> Self {
        if maximum_time_distance >= 0 && maximum_time_distance <= 86340 {
            let sp = ScheduleParameters {
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
    use crate::datetime::Day;
    use crate::schedule_parameters::ScheduleParameters;
    use time::time;

    #[test]
    #[should_panic]
    fn negative_distance() {
        ScheduleParameters::new(
            time!(12:00),
            vec![Day::Online],
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
    fn overflow_distance() {
        ScheduleParameters::new(
            time!(12:00),
            vec![Day::Online],
            time!(12:00),
            Vec::new(),
            Vec::new(),
            time!(12:00),
            86341,
            true,
        );
    }
}
