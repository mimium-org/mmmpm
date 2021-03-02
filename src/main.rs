extern crate clap;
extern crate git2;

fn main() {
    let yaml = clap::load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();

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
