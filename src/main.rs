mod scheduler;
mod scraper;

extern crate clap;
use clap::Clap;

#[macro_use]
extern crate log;
extern crate simplelog;

use simplelog::*;

/// An automatic class scheduler for San Diego State University.
#[derive(Clap)]
#[clap(version = "0.1.0")]
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

fn main() {
    let opts: Opts = Opts::parse();

    let level = match opts.verbose {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 | _ => LevelFilter::Info,
    };

    let config: Config = ConfigBuilder::new().set_time_to_local(true).build();
    TermLogger::init(level, config, TerminalMode::Mixed).expect("Unable to create logger.");

    let params = scheduler::Parameters::new(&opts.config).expect("Unable to parse configuration file.");

    scheduler::Controller::new(params).generate_schedules().expect("Unable to generate schedules.");
}
