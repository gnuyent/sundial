<h1 align="center">
Sundial
</h1>
<h4 align="center">
    A universal class scheduler.
</h4>

<p align="center">
    <a href="https://www.rust-lang.org/tools/install">
        <img src="https://badgen.net/badge/rust/1.42+/?color=orange" alt="Rust Version" />
    </a>
    <a href="https://github.com/gnuyent/sundial/blob/master/LICENSE">
        <img src="https://badgen.net/badge/license/GPL2/orange" alt="License" />
    </a>
</p>

<div align="center">
    <h4>
        <a href="#features">Features</a> |
        <a href="#installation">Installation</a> |
        <a href="#troubleshooting">Troubleshooting</a> |
        <a href="#contributing">Contributing</a>
    </h4>
</div>
<br>

Sundial is a powerful class scheduler built for speed and extensibility. It's quick, just like telling time.

## Features
* Generates parameterized class schedules.
* Intelligent caching for fast, updated information.

## Configuration
Create a `config.toml` file inside the same directory as the executable. Then, paste and configure the following:

```toml
# Configuration file for sundial.

school = "SDSU"
# School period to schedule with. This is formatted as S YYYY where YYYY is the year and
# S is the season. S can be Spring, Summer, or Fall.
period = "Spring 2021"
# The desired time to schedule around. This should ideally be the middle-most time.
around_time = "1200"
# Days to avoid.
bad_days = ["Thursday", "Friday"]
# The earliest time a class can be.
earliest_time = "1000"
# The latest time a class can be.
latest_time = "1700"
# Courses to include by name in SUBJ-NUM format.
# Ex: CS-370, RWS-305W
# Make sure to keep the spaces in the name.
# Ex: "A E-220" should stay the same.
courses = ["COMM-371", "CS-370", "CS-440", "CS-574", "RWS-305W"]
# True if a requested course isn't found, false to stop scheduling immediately.
skip_missing_courses = false
# Courses to add by schedule number. If the course matching the schedule number is found
# above, this schedule number will be ignored.
include_courses = ["22152"]
# Professors to include by last name.
include_professors = ["Bartoli", "Boyd"]
# True if you want every single professor above, false if you only want some. This
# option only works if there is at least one name above.
include_all_professors = false
# Maximum time between the first and last class (minutes).
maximum_time_distance = 600
# True if waitlist is not okay, false otherwise.
prefer_no_waitlist = true
```

## Installation
There are currently no automated builds.

To run locally, make sure you have Rust and Cargo installed. Then, once inside the repository, configure the `config.toml` file and run:

* `cargo run --release`

## Troubleshooting
sundial is a continuous work in progress. If you start facing problems, please submit an issue with as much relevant
information (logs, screenshots, etc) as possible.

## License
sundial is licensed under the [GPL2](LICENSE). Pull requests are welcome!

## Disclaimer
sundial does not guarantee that you meet all the prerequisites to take a class. Always consult an academic counselor
before registering for classes.

This project is not affiliated with or endorsed by the supported universities in any way. Be responsible when using this program.
