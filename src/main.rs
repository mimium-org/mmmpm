extern crate clap;
extern crate dirs;
extern crate log;

use log::LevelFilter;

use mmmpm_package::UndeterminedPackage;

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

    // test code
    let pkg_name = UndeterminedPackage::new("github.com:t-sin/koto".to_string());
    match pkg_name.determine() {
        Some(pkgdsn) => {
            println!("{:?}", pkgdsn.name());
            let host = pkgdsn.host();
            let mut pkg_exists = false;
            match host.exists() {
                Ok(exists) => {
                    println!("result = {}", exists);
                    pkg_exists = true;
                }
                Err(err) => println!("error: {:?}", err),
            }

            if pkg_exists {
                match host.retrieve() {
                    Ok(archive) => println!("{:?}", archive),
                    Err(err) => println!("error: {:?}", err),
                }
            }
        }
        None => println!("none!!"),
    }
}
