use std::ops::Mul;

use chrono::{DateTime, Months, Utc};
use clap::{Command, arg};
use inquire::Confirm;

use crate::parsers::{Duration, duration_parser, positive_integer_parser};

pub mod parsers;

fn met_cli() -> Command {
    Command::new("met")
        .about("A personal cli to generate fake data for our metered usage system.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("generate")
                .about("Generate a folder with all fake event data in parque format.")
                .arg(arg!(--events <VALUE> "Number of events generated per second.").value_parser(positive_integer_parser).required(true))
                .arg(
                    arg!(--"number-customers" <VALUE> "Number of customers to be generated. By default it's 10_000.")
                        .default_value("10000")
                        .value_parser(positive_integer_parser)
                        .required(false),
                )
                .arg(
                    arg!(--"time-range" <VALUE> "The span of time to be used to create random events. The values must have the format <number>y or <number>m")
                        .default_value("1y")
                        .value_parser(duration_parser)
                        .required(false),
                )
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = met_cli().get_matches();

    match matches.subcommand() {
        Some(("generate", sub_matches)) => {
            let generate_subcommand_state = calculate_generate_cli_state(
                sub_matches
                    .get_one("events")
                    .expect("The events argument is required"),
                sub_matches
                    .get_one("time-range")
                    .expect("We can't generate data without a time range"),
            );

            println!(
                "'generate' was used, the total of events to be generated are: {:?}. The initial start date is {:?} and end date is {:?}. Splitted between {:?} customers.",
                generate_subcommand_state.total_events,
                generate_subcommand_state.start_date,
                generate_subcommand_state.end_date,
                sub_matches
                    .get_one::<u16>("number-customers")
                    .expect("We need at least 1 customer"),
            );

            let confirmation = Confirm::new("Are you sure to proceed?")
                .with_default(false)
                .with_help_message("The data is going to be saved in multiple parquets files")
                .prompt();

            match confirmation {
                Ok(true) => println!("TODO generate data"),
                Ok(false) => println!("No data is going to be generated"),
                Err(_) => println!("Error confirming the prompt"),
            }
        }
        _ => {
            panic!("command doesn't exist")
        }
    }
}

struct GenerationCLIState {
    total_events: i64,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
}

fn calculate_generate_cli_state(
    events_per_second: &u16,
    time_range: &Duration,
) -> GenerationCLIState {
    let end_date = Utc::now();

    let start_date = match time_range {
        Duration::Month(duration) => end_date.clone() - Months::new(duration.clone().into()),
        Duration::Year(duration) => end_date.clone() - Months::new((duration * 12).into()),
    };

    let total_events = (end_date - start_date)
        .num_seconds()
        .mul(i64::from(events_per_second.clone()));

    return GenerationCLIState {
        total_events,
        start_date,
        end_date,
    };
}
