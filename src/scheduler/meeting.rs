use super::date::{Date, Day};
use anyhow::Result;
use time::Time;

#[derive(Clone, Debug)]
pub struct Meeting {
    pub dates: Vec<Date>,
    pub meeting_type: String,
    pub location: String,
    pub instructor: String,
}

impl Meeting {
    pub fn new(
        time_range: &str,
        days: &str,
        meeting_type: &str,
        location: &str,
        instructor: &str,
    ) -> Result<Self> {
        let mut times: [time::Time; 2] = [Time::midnight(); 2];
        // Parse time string to objects
        if !time_range.is_empty() {
            for (idx, time) in time_range.splitn(2, '-').enumerate() {
                let (hour, minute) = time.split_at(2);
                let hour = hour.to_string().parse::<u8>()?;
                let minute = minute.to_string().parse::<u8>()?;
                times[idx] = Time::try_from_hms(hour, minute, 0)?;
            }
        }

        let mut day_vec: Vec<char> = days.chars().collect();
        let mut dates: Vec<Date> = Vec::new();

        while let Some(i) = day_vec.pop() {
            let day: Day = Day::match_day(i.to_string().as_ref());

            if day == Day::Thursday {
                day_vec.pop(); // pop twice to get rid of "T" in "TH"
            }

            let date = Date {
                start_time: times[0],
                end_time: times[1],
                day,
            };

            dates.push(date);
        }

        dates.reverse();

        Ok(Self {
            // TODO: Implement instructors to Vec.
            dates,
            meeting_type: meeting_type.to_string(),
            location: location.to_string(),
            instructor: instructor.to_string(),
        })
    }

    pub fn days(&self) -> Vec<Day> {
        self.dates.iter().map(|d| d.day.clone()).collect()
    }
}
