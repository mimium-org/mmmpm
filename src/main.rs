extern crate clap;
extern crate dirs;
extern crate log;

use log::LevelFilter;

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

    // TODO: create filesystem storage from `~/.mimium`
    match matches.subcommand() {
        ("install", Some(_)) => println!("subcommand: install"),

        ("list", Some(_)) => println!("subcommand: list"),
        ("run", Some(_)) => println!("subcommand: run"),
        _ => println!("{}", matches.usage()),
    }
}
