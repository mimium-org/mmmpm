use clap::ArgMatches;

use crate::package::Package;

struct CmdOption {
    package: Package,
}

fn parse_option(matches: &ArgMatches) -> CmdOption {
    CmdOption {
        package: Package::Pkg(String::from(matches.value_of("PACKAGE").unwrap())),
    }
}

fn proc(opt: CmdOption) -> Result<(), ()> {
    println!("subcommand: install {:?}", opt.package);
    Ok(())
}

pub fn install(matches: &ArgMatches) -> Result<(), ()> {
    proc(parse_option(matches))
}
