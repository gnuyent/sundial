use super::date::{Date, Day};
use anyhow::Result;
use time::Time;

#[derive(Clone, Debug)]
pub struct Meeting {
    pub date: Date,
    pub meeting_type: String,
    pub location: String,
    pub instructor: String,
}

impl Meeting {
    pub fn new(
        time_range: &str,
        day: &str,
        meeting_type: &str,
        location: &str,
        instructor: &str,
    ) -> Result<Self> {
        let mut times: [time::Time; 2] = [Time::midnight(); 2];
        // Parse time string to objects
        if time_range != "" {
            for (idx, time) in time_range.splitn(2, "-").enumerate() {
                let (hour, minute) = time.split_at(2);
                let hour = hour.to_string().parse::<u8>()?;
                let minute = minute.to_string().parse::<u8>()?;
                times[idx] = Time::try_from_hms(hour, minute, 0)?;
            }
        }

        let date = Date {
            start_time: times[0],
            end_time: times[1],
            day: Day::match_day(day),
        };

        Ok(Self {
            date,
            meeting_type: meeting_type.to_string(),
            location: location.to_string(),
            // TODO: Implement instructors to Vec.
            instructor: instructor.to_string(),
        })
    }
}
