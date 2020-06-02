<h1 align="center">
    ⏰🌞 Sundial
</h1>
<h4 align="center">
    A class scheduler for <a href="https://www.sdsu.edu/">San Diego State University</a>.
</h4>

<p align="center">
    <!-- Python Version -->
    <a href="https://www.python.org/downloads/">
        <img src="https://badgen.net/badge/python/3.8.3/?color=green" alt="Python Version" />
    </a>
    <a href="https://github.com/gnuyent/sundial/blob/master/LICENSE.md">
        <img src="https://badgen.net/badge/license/MIT/blue" alt="License" />
    </a>
</p>

<div align="center">
    <h4>
        <a href="#features">Features</a> |
        <a href="#quick-start">Quick Start</a> |
        <a href="#installation">Installation</a> |
        <a href="#troubleshooting">Troubleshooting</a>
    </h4>
</div>

<div align="center">
    <sub>Built with ❤ by <a href="https://github.com/gnuyent">Brandon Nguyen</a>
    </sub>
</div>
<br>

Sundial is a class scheduler for San Diego State University. Sundial is light, speedy, and extremely customizable. It is
able to generate a schedule for any course list and time period available on the 
[scheduling website](https://sunspot.sdsu.edu/schedule/search). Simplicity is the focus, just like telling time.

## Features
* 🏃 Quickly generates schedules.
* 🙆 Flexible options based on different requirements:
    * ⌚ Set a time to schedule around.
    * ❌ Reject certain days.
    * 🌄 Set the earliest time.
    * 🌙 Set the latest time.
    * 📋 Avoid waitlists.
    * 👩‍ Include certain professors*.

<small>*not currently implemented.</small>
    
## Quick Start
1. Download the latest release on the releases page.
2. Extract the folder and install the Python requirements with `pip install -r requirements.txt`
3. Configure `sundial.py`
3. Run `python sundial.py`

## Installation
1. Download the latest release on the [releases](https://github.com/gnuyent/sundial/releases/) page.
2. Install dependencies (`pip install -r requirements.txt`):
    * [Python 3](https://www.python.org/downloads/) (tested on Python 3.8.3)
3. Generate course database
    * Navigate to the `scraper/` directory in your preferred terminal.
    * Run the command `scrapy crawl allcourses --loglevel WARNING`
        * This command will take a few minutes to execute.
    * Make sure that `classes.db` is in the top-level `sundial/` directory.
3. Configure `sundial.py`
    * Set schedule parameters to desired fields. The algorithm will still generate schedules that do not meet these 
    guidelines.
        * `around_time` attempts to generate a schedule around this time.
        * `maximum_time_distance` the furthest time (hours and minutes) from `around_time` that you want to have a 
        class.
        * `bad_day` day(s) that you do not want to take a class
        * `earliest_time` the earliest time you are willing to take a class.
        * `latest_time` the latest time you are willing for a class to end.
        * `prefer-no-waitlist` prefer classes that do not have a waitlist.
4. Edit line 14 with your classes
    * Make sure classes are comma-separated and formatted correctly.
5. Run `python sundial.py`

## Troubleshooting
sundial is a continuous work in progress. If you start facing problems, please submit an issue with as much relevant 
information (logs, screenshots, etc) as possible.

## Contributing
sundial is licensed under the MIT license. Pull requests are welcome.

sundial does not guarantee that you meet all the prerequisites to take a class. Always consult an academic counselor
before registering for classes.

This project is not affiliated with or endorsed by San Diego State University in any way. Be responsible when scraping
or use the included database on the [releases](https://github.com/gnuyent/sundial/releases) page.
