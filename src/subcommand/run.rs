use std::path::PathBuf;

use clap::ArgMatches;
use log::{error, info};

use crate::package::{package_from_string, Package};

pub enum RunError<'a> {
    InvalidOptions(&'a ArgMatches<'a>),
    PackageTypeIsNotImplemented,
}

struct CmdOption {
    package: Package,
}

fn parse_options<'a>(matches: &'a ArgMatches<'a>) -> Result<CmdOption, RunError<'a>> {
    // initialize with dummy values
    let mut opts = CmdOption {
        package: Package::Pkg("***".to_string()),
    };
    if let Ok(pkg) = package_from_string(String::from(matches.value_of("PACKAGE").unwrap())) {
        opts.package = pkg;
    } else {
        return Err(RunError::InvalidOptions(matches));
    }

    Ok(opts)
}

fn run_git_repo<'a>(mimium_dir: PathBuf, host: String, path: String) -> Result<(), RunError<'a>> {
    Ok(())
}

fn proc<'a>(mimium_dir: PathBuf, opt: CmdOption) -> Result<(), RunError<'a>> {
    match opt.package {
        Package::Git { host, path } => run_git_repo(mimium_dir, host, path),
        Package::Pkg(_name) => Err(RunError::PackageTypeIsNotImplemented),
        Package::Path(_path) => Err(RunError::PackageTypeIsNotImplemented),
    }
}

pub fn run<'a>(mimium_dir: PathBuf, matches: &'a ArgMatches<'a>) -> Result<(), RunError<'a>> {
    match parse_options(matches) {
        Ok(opt) => proc(mimium_dir, opt),
        Err(err) => Err(err),
    }
}
