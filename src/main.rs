extern crate clap;
extern crate dirs;
extern crate log;

extern crate git2;

mod constant;
mod package;
mod subcommand;

use log::{error, info, LevelFilter};
use std::fs::create_dir;
use std::path::PathBuf;

fn determine_mimium_dir() -> Option<PathBuf> {
    match dirs::home_dir() {
        Some(path) => {
            let mut path = path.clone();
            path.push(constant::MMMPM_DIR);
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
        match ensure_dir(path.clone()) {
            Ok(_) => match matches.subcommand() {
                ("install", Some(sub_m)) => {
                    let _ = subcommand::install(path, sub_m);
                }

                ("list", Some(_)) => println!("subcommand: list"),
                ("run", Some(sub_m)) => {
                    let _ = subcommand::run(path, sub_m);
                }
                _ => println!("{}", matches.usage()),
            },
            err => {
                error!("Cannot ensure mimium directory because {:?}", err);
                return;
            }
        }
    }
}
