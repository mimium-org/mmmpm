use std::fs;
use std::io;
use std::path::PathBuf;

use clap::ArgMatches;
use log::{error, info};

use crate::package::{is_mimium_package, PackageDesignator};

pub enum InstallError<'a> {
    InvalidOptions(&'a ArgMatches<'a>),
    MalformedPackage,
    PackageTypeIsNotImplemented,
    IOError(io::Error),
    // This error never occurs.
    Eden,
}

struct CmdOption {
    package_designator: PackageDesignator,
}

fn parse_options<'a>(matches: &'a ArgMatches<'a>) -> Result<CmdOption, InstallError<'a>> {
    // initialize with dummy values
    let mut opts = CmdOption {
        package_designator: PackageDesignator::Pkg("***".to_string()),
    };

    let pkg_str = String::from(matches.value_of("PACKAGE").unwrap());
    if let Ok(pkg_dsn) = PackageDesignator::from_str(pkg_str) {
        opts.package_designator = pkg_dsn;
    } else {
        return Err(InstallError::InvalidOptions(matches));
    }

    Ok(opts)
}

fn install_github_repo<'a>(
    mimium_dir: PathBuf,
    pkg_dsn: PackageDesignator,
) -> Result<(), InstallError<'a>> {
    if let PackageDesignator::Git { host, path } = pkg_dsn.clone() {
        info!("Install {:?} from {:?} as Git repository.", path, host);
        Ok(())
    } else {
        Err(InstallError::Eden)
    }
}

fn install_package<'a>(mimium_dir: PathBuf, opt: CmdOption) -> Result<(), InstallError<'a>> {
    match opt.package_designator.clone() {
        PackageDesignator::Git { host: _, path: _ } => {
            install_github_repo(mimium_dir, opt.package_designator)
        }
        PackageDesignator::Pkg(_name) => Err(InstallError::PackageTypeIsNotImplemented),
        PackageDesignator::Path(_path) => Err(InstallError::PackageTypeIsNotImplemented),
        PackageDesignator::Indeterminated(_path) => Err(InstallError::PackageTypeIsNotImplemented),
    }
}

pub fn install<'a>(
    mimium_dir: PathBuf,
    matches: &'a ArgMatches<'a>,
) -> Result<(), InstallError<'a>> {
    match parse_options(matches) {
        Ok(opts) => install_package(mimium_dir, opts),
        Err(err) => Err(err),
    }
}
