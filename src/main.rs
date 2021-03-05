extern crate clap;
extern crate dirs;
extern crate log;

extern crate git2;

use log::{error, info, LevelFilter};
use std::fs::create_dir;
use std::path::PathBuf;

const MIMIUM_DIR: &str = ".mimium";

fn determine_mimium_dir() -> Option<PathBuf> {
    match dirs::home_dir() {
        Some(path) => {
            let mut path = path.clone();
            path.push(MIMIUM_DIR);
            info!("mimium directory = {:?}", path);
            Some(path)
        }
        None => {
            error!("Cannot determine mimium directory.");
            None
        }
    }
}

fn ensure_dir(path: PathBuf) -> Result<(), std::io::Error> {
    let path = path.into_boxed_path();
    if !path.exists() {
        info!("{:?} is not found so is created.", path);
        create_dir(path)
    } else {
        error!("{:?} is found.", path);
        Ok(())
    }
}

fn main() {
    let yaml = clap::load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    env_logger::builder()
        .filter_level(if matches.is_present("verbose") {
            LevelFilter::Info
        } else {
            LevelFilter::Error
        })
        .format_level(false)
        .format_timestamp(None)
        .init();

    let mimium_dir = determine_mimium_dir();
    if let Some(path) = mimium_dir {
        match ensure_dir(path) {
            Ok(_) => (),
            err => {
                error!("Cannot ensure mimium directory because {:?}", err);
                return;
            }
        }
    }

    match matches.subcommand() {
        ("install", Some(sub_m)) => {
            println!("subcommand: install {}", sub_m.value_of("PACKAGE").unwrap())
        }
        ("list", Some(_)) => println!("subcommand: list"),
        ("run", Some(sub_m)) => {
            println!("subcommand: install {}", sub_m.value_of("PACKAGE").unwrap())
        }
        _ => println!("{}", matches.usage()),
    }
}
