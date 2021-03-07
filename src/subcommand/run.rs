use std::io;
use std::path::PathBuf;
use std::process::Command;

use clap::ArgMatches;
use log::info;

use crate::constant;
use crate::package::{Package, PackageDesignator};

pub enum RunError<'a> {
    InvalidOptions(&'a ArgMatches<'a>),
    MimiumFailed,
    IOError(io::Error),
}

struct CmdOption {
    package: Package,
}

fn parse_options<'a>(matches: &'a ArgMatches<'a>) -> Result<CmdOption, RunError<'a>> {
    // initialize with dummy values
    let mut opts = CmdOption {
        package: Package::Pkg("***".to_string()),
    };
    let pkg = PackageDesignator(String::from(matches.value_of("PACKAGE").unwrap()));
    if let Ok(pkg) = pkg.package() {
        opts.package = pkg;
    } else {
        return Err(RunError::InvalidOptions(matches));
    }

    Ok(opts)
}

fn run_package<'a>(mimium_dir: PathBuf, opt: CmdOption) -> Result<(), RunError<'a>> {
    info!("Run package {}.", opt.package.name());

    // TODO: Get from mmmp.toml
    let entrypoint = "test.mmm";
    let entrypoint_path = format!(
        "{}/{}/{}",
        mimium_dir.to_str().unwrap(),
        opt.package.path().to_str().unwrap(),
        entrypoint,
    );
    let args = &[entrypoint_path];

    info!("Run mimium with args: {:?}", args);

    match Command::new(constant::MIMIUM_EXECUTABLE)
        .args(args)
        .output()
    {
        Ok(output) => {
            if output.status.success() {
                Ok(())
            } else {
                println!("stderr:\n{}", std::str::from_utf8(&output.stderr).unwrap());
                Err(RunError::MimiumFailed)
            }
        }
        Err(err) => Err(RunError::IOError(err)),
    }
}

pub fn run<'a>(mimium_dir: PathBuf, matches: &'a ArgMatches<'a>) -> Result<(), RunError<'a>> {
    match parse_options(matches) {
        Ok(opt) => run_package(mimium_dir, opt),
        Err(err) => Err(err),
    }
}
