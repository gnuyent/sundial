/// All possible days that can be matched.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Online,
    Arranged,
    Unknown,
}

impl Day {
    /// Matches a given char to a [Day] enum type.
    ///
    /// If the given char does not match, [Day::Unknown] will be returned.
    ///
    /// # Examples
    /// ```
    /// let day: char = 'F';
    /// let match: Day = match_day(day);
    ///
    /// assert_eq!(Day::Friday, match);
    /// ```
    fn match_day(focus: char) -> Day {
        match focus {
            'M' => Day::Monday,
            'T' => Day::Tuesday,
            'W' => Day::Wednesday,
            'H' => Day::Thursday,
            'F' => Day::Friday,
            'S' => Day::Saturday,
            'O' => Day::Online,
            'A' => Day::Arranged,
            _ => Day::Unknown,
        }
    }

    /// Converts &str representing days to a vector of [Day] objects.
    ///
    /// # Examples
    ///
    /// ```
    /// let days: str = "MWF";
    /// let answer: Vec<Day> = parse_days(days);
    ///
    /// assert_eq!(vec![Day::Monday, Day::Wednesday, Day::Friday]);
    /// ```
    ///
    /// If an invalid value is passed, an empty vector will be returned.
    /// ```
    /// let days: str = "Z";
    /// let answer: Vec<Day> = parse_days(days);
    ///
    /// assert_eq!(vec![], answer);
    /// ```
    ///
    pub fn parse_days(days: &str) -> Vec<Day> {
        if days == "" {
            return vec![Day::Online];
        } else if days == "Arranged" {
            return vec![Day::Arranged];
        }

        let mut day_vec: Vec<char> = days.chars().collect();
        let mut return_vec: Vec<Day> = Vec::new();

        loop {
            let focus: Day;
            match day_vec.pop() {
                Some(i) => focus = Day::match_day(i),
                None => break,
            };

            if focus == Day::Thursday {
                day_vec.pop(); // pop twice to get rid of the "T" in "TH"
            } else if focus == Day::Unknown {
                return vec![];
            }
            return_vec.push(focus);
        }

        return_vec.reverse();
        return_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_day() {
        assert_eq!(Day::match_day('M'), Day::Monday);
        assert_eq!(Day::match_day('T'), Day::Tuesday);
        assert_eq!(Day::match_day('W'), Day::Wednesday);
        assert_eq!(Day::match_day('H'), Day::Thursday);
        assert_eq!(Day::match_day('F'), Day::Friday);
        assert_eq!(Day::match_day('S'), Day::Saturday);
        assert_eq!(Day::match_day('O'), Day::Online);
        assert_eq!(Day::match_day('A'), Day::Arranged);
        assert_eq!(Day::match_day('Z'), Day::Unknown);
    }

    #[test]
    fn parse_valid_days() {
        assert_eq!(
            Day::parse_days("MTWTH"),
            vec![Day::Monday, Day::Tuesday, Day::Wednesday, Day::Thursday]
        );
        assert_eq!(
            Day::parse_days("MWF"),
            vec![Day::Monday, Day::Wednesday, Day::Friday]
        );
        assert_eq!(Day::parse_days(""), vec![Day::Online]);
        assert_eq!(Day::parse_days("Arranged"), vec![Day::Arranged]);
    }

    #[test]
    fn parse_invalid_days() {
        assert_eq!(Day::parse_days("ZZZ"), vec![]);
    }
}
