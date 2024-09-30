mod api;
mod images;
mod utils;

use clap::{arg, command, Command};
use std::process::exit;
use viuer::Config;

pub const MAX_LINES_FOR_QUOTE: usize = 5;
pub const HEIGHT: u8 = 10;
pub const WIDTH: u8 = 20;
pub const TEXT_COLOR: &'static str = "FEF7DB";
fn main() {
    let manager = match api::APIManager::new() {
        Ok(manager) => manager,
        Err(err) => {
            eprintln!("{}", err);
            exit(1)
        }
    };

    let viuer_conf = Config {
        width: Some(WIDTH as u32),
        height: Some(HEIGHT as u32),
        absolute_offset: false,
        transparent: true,
        ..Default::default()
    };

    let matches = command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("random").about("Get a random quote"))
        .subcommand(
            Command::new("character")
                .about("Get a quote from a specfic character")
                .arg(arg!([value] "A character from Avatar: The Last Airbender").required(true)),
        )
        .subcommand(
            Command::new("nation")
                .about("Get a quote from a character from a specfic nation")
                .arg(arg!([value] "A nation from Avatar: The Last Airbender").required(true)),
        )
        .subcommand(
            Command::new("bending")
                .about("Get a quote from a character with specfic bending ability")
                .arg(arg!([value] "A bending ability from Avatar: The Last Airbender").required(true)),
        )
        .subcommand(
            Command::new("episode")
                .about("Get a quote from a specfic episode")
                .arg(arg!([value] "An episode from Avatar: The Last Airbender").required(true)),
        )
        .subcommand(
            Command::new("book")
                .about("Get a quote from a specfic book")
                .arg(arg!([value] "A book from Avatar: The Last Airbender").required(true)),
        )
        .subcommand(
            Command::new("valid")
                .about("Get all valid inputs for any given filter above")
                .arg(arg!([filter] "Choose from: character, nation, bending, episode, book").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("random", _)) => {
            let quotes = match manager.random() {
                Ok(quotes) => quotes,
                Err(err) => {
                    eprintln!("Error getting random quotes: {}", err);
                    exit(1);
                }
            };
            let (quote, width) = utils::get_quote_and_width(&quotes);
            utils::print_image_and_quote(quote, width, &viuer_conf);
        }
        Some(("character", sub_matches)) => {
            let value = sub_matches.get_one::<String>("value").expect("required");
            let quotes = match manager.filter("character", &value) {
                Ok(quotes) => quotes,
                Err(err) => {
                    eprintln!("Error getting quotes from {}: {}", value, err);
                    exit(1);
                }
            };
            let (quote, width) = utils::get_quote_and_width(&quotes);
            utils::print_image_and_quote(quote, width, &viuer_conf);
        }
        Some(("nation", sub_matches)) => {
            let value = sub_matches.get_one::<String>("value").expect("required");
            let quotes = match manager.filter("nation", &value) {
                Ok(quotes) => quotes,
                Err(err) => {
                    eprintln!("Error getting quotes from {}: {}", value, err);
                    exit(1);
                }
            };
            let (quote, width) = utils::get_quote_and_width(&quotes);
            utils::print_image_and_quote(quote, width, &viuer_conf);
        }
        Some(("bending", sub_matches)) => {
            let value = sub_matches.get_one::<String>("value").expect("required");
            let quotes = match manager.filter("bending", &value) {
                Ok(quotes) => quotes,
                Err(err) => {
                    eprintln!("Error getting quotes from {}: {}", value, err);
                    exit(1);
                }
            };
            let (quote, width) = utils::get_quote_and_width(&quotes);
            utils::print_image_and_quote(quote, width, &viuer_conf);
        }
        Some(("episode", sub_matches)) => {
            let value = sub_matches.get_one::<String>("value").expect("required");
            let quotes = match manager.filter("episode", &value) {
                Ok(quotes) => quotes,
                Err(err) => {
                    eprintln!("Error getting quotes from {}: {}", value, err);
                    exit(1);
                }
            };
            let (quote, width) = utils::get_quote_and_width(&quotes);
            utils::print_image_and_quote(quote, width, &viuer_conf);
        }
        Some(("book", sub_matches)) => {
            let value = sub_matches.get_one::<String>("value").expect("required");
            let quotes = match manager.filter("book", &value) {
                Ok(quotes) => quotes,
                Err(err) => {
                    eprintln!("Error getting quotes from {}: {}", value, err);
                    exit(1);
                }
            };
            let (quote, width) = utils::get_quote_and_width(&quotes);
            utils::print_image_and_quote(quote, width, &viuer_conf);
        }

        Some(("valid", sub_matches)) => {
            let filter = sub_matches.get_one::<String>("filter").expect("required");
            let valid_filters = ["character", "nation", "bending", "episode", "book"];

            if !valid_filters.contains(&filter.as_str()) {
                eprintln!("Invalid Filter");
                exit(1);
            }
            let values = match manager.valid_inputs(&filter) {
                Ok(values) => values,
                Err(err) => {
                    eprintln!("Error getting valid inputs for {}: {}", filter, err);
                    exit(1);
                }
            };
            for value in values.iter() {
                println!("{}", value);
            }
        }
        _ => unreachable!(),
    }
}
