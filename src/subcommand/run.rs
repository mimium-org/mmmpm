use std::io;
use std::path::PathBuf;
use std::process::Command;

use clap::ArgMatches;
use log::info;

use crate::constant::MIMIUM_EXECUTABLE;
use crate::package::{MmmPackage, Package, PackageDesignator};

#[derive(Debug)]
pub enum RunError<'a> {
    InvalidOptions(&'a ArgMatches<'a>),
    CannotDeterminePackageType,
    CannotFoundPackageFile,
    MalformedPackage,
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

fn run_package<'a>(
    mimium_dir: PathBuf,
    mmm: MmmPackage,
    opt: CmdOption,
) -> Result<(), RunError<'a>> {
    info!("Run package {}.", opt.package_designator.name());

    let entrypoint_path = format!(
        "{}/{}/{}",
        mimium_dir.to_str().unwrap(),
        opt.package_designator.path().unwrap().to_str().unwrap(),
        mmm.package.entrypoint,
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
        Ok(mut opt) => match opt.package_designator.determine(mimium_dir.clone()) {
            Ok(pkg_dsn) => {
                opt.package_designator = pkg_dsn.clone();

                if !pkg_dsn.exists(mimium_dir.clone()) {
                    Err(RunError::CannotFoundPackageFile)
                } else {
                    let mut pkg_path = mimium_dir.clone();
                    pkg_path.extend(&[pkg_dsn.package_file_path().unwrap()]);
                    match MmmPackage::from_path(&pkg_path) {
                        Ok(pkg) => run_package(mimium_dir, pkg, opt),
                        Err(_) => Err(RunError::MalformedPackage),
                    }
                }
            }
            Err(_) => Err(RunError::CannotDeterminePackageType),
        },
        Err(err) => Err(err),
    }
}
