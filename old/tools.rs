use time::Time;

// A common collection of utilities used by the sundial API.

/// The `Days` type represents all valid days that courses can be held.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Days {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Online,
}

/// `match_day` is a private helper function for `parse_days`. It matches valid `char` types to the `Days` type.
///
/// # Arguments
///
/// * `focus` - A `char` to parse and match to its respective `Days` enum.
fn match_day(focus: char) -> Days {
    match focus {
        'M' => Days::Monday,
        'T' => Days::Tuesday,
        'W' => Days::Wednesday,
        'H' => Days::Thursday, // 'H' is matched since we iterate backwards in `parse_days`
        'F' => Days::Friday,
        _ => Days::Online,
    }
}

/// `parse_days` generates a Vec<Days> based upon a valid input string.
///
/// # Arguments
///
/// * `days` - A string slice that contains the data to parse.
pub fn parse_days(days: &str) -> Vec<Days> {
    let mut days_vector: Vec<char> = days.chars().collect();
    let mut return_vector: Vec<Days> = Vec::new();
    loop {
        let focus: Days = match days_vector.pop() {
            Some(day) => match_day(day),
            None => break, // break out of loop if vector is empty (len == 0)
        };
        match focus {
            Days::Thursday => {
                days_vector.pop(); // pop twice to get rid of the 'T' char in 'TH'
                return_vector.push(focus);
            }
            _ => return_vector.push(focus),
        }
    }
    return_vector.reverse();

    return_vector
}

/// `parse_time` generates an array of two `time::Time` objects from a string that contains a start and end time separated with a '-'.
///
/// # Arguments
///
/// * `hour_range` - A string slice that contains the times to parse.
pub fn parse_time(hour_range: &str) -> [Time; 2] {
    //TODO
    let start_time: Time = Time::try_from_hms(0, 0, 0)
        .expect("Received incorrect time from database. Try rescraping.");
    let end_time: Time = Time::try_from_hms(0, 0, 0)
        .expect("Received incorrect time from database. Try rescraping.");
    [start_time, end_time]
}

#[cfg(test)]
mod tests {
    use crate::tools::{parse_days, Days};

    #[test]
    fn regular_days() {
        let day_string: String = String::from("MWF");
        let day_vec: Vec<Days> = parse_days(&day_string);
        assert!(matches!(day_vec[0], Days::Monday));
        assert!(matches!(day_vec[1], Days::Wednesday));
        assert!(matches!(day_vec[2], Days::Friday));
    }

    #[test]
    fn tuesday_thursday() {
        let day_string: String = String::from("TTH");
        let day_vec: Vec<Days> = parse_days(&day_string);
        assert!(matches!(day_vec[0], Days::Tuesday));
        assert!(matches!(day_vec[1], Days::Thursday));
    }

    #[test]
    fn online() {
        let day_string: String = String::from("O");
        let day_vec: Vec<Days> = parse_days(&day_string);
        assert!(matches!(day_vec[0], Days::Online));
    }

    #[test]
    fn empty() {
        let day_string: String = String::from("");
        let day_vec: Vec<Days> = parse_days(&day_string);
        assert_eq!(day_vec.len(), 0);
    }
}
