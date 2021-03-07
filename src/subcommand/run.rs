use std::io;
use std::path::PathBuf;
use std::process::Command;

use clap::ArgMatches;
use log::info;

use crate::constant::{MIMIUM_EXECUTABLE, MMMPM_PACKAGE_FILE};
use crate::package::{Package, PackageDesignator};

pub enum RunError<'a> {
    InvalidOptions(&'a ArgMatches<'a>),
    MalformedPackageConfig,
    MimiumFailed,
    IOError(io::Error),
}

struct CmdOption {
    package_designator: PackageDesignator,
}

fn parse_options<'a>(matches: &'a ArgMatches<'a>) -> Result<CmdOption, RunError<'a>> {
    // initialize with dummy values
    let mut opts = CmdOption {
        package_designator: PackageDesignator::Pkg("***".to_string()),
    };
    let pkg_str = String::from(matches.value_of("PACKAGE").unwrap());
    if let Ok(pkg_dsn) = PackageDesignator::from_str(pkg_str) {
        opts.package_designator = pkg_dsn;
    } else {
        return Err(RunError::InvalidOptions(matches));
    }

    Ok(opts)
}

fn run_package<'a>(mimium_dir: PathBuf, pkg: Package, opt: CmdOption) -> Result<(), RunError<'a>> {
    info!("Run package {}.", opt.package_designator.name());

    let entrypoint_path = format!(
        "{}/{}/{}",
        mimium_dir.to_str().unwrap(),
        opt.package_designator.path().to_str().unwrap(),
        pkg.entrypoint,
    );
    let args = &[entrypoint_path];

    info!("Run mimium with args: {:?}", args);

    match Command::new(MIMIUM_EXECUTABLE).args(args).output() {
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
        Ok(opt) => {
            let pkg_path = PathBuf::from(format!(
                "{}/{}/{}",
                mimium_dir.to_str().unwrap(),
                opt.package_designator.path().to_str().unwrap(),
                MMMPM_PACKAGE_FILE,
            ));
            match Package::from_path(&pkg_path) {
                Ok(pkg) => run_package(mimium_dir, pkg, opt),
                Err(_) => Err(RunError::MalformedPackageConfig),
            }
        }
        Err(err) => Err(err),
    }
}
