use std::fs;
use std::io;
use std::path::PathBuf;

use clap::ArgMatches;
use git2::{Error, Repository};
use log::{error, info};

use crate::package::{is_mimium_package, PackageDesignator};

pub enum InstallError<'a> {
    InvalidOptions(&'a ArgMatches<'a>),
    CannotCloneGitRepository(Error),
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

fn clone_git_repo<'a>(
    mimium_dir: PathBuf,
    pkg_dsn: PackageDesignator,
) -> Result<String, InstallError<'a>> {
    let repo_url = pkg_dsn.remote_url().unwrap();
    // TODO: Protection from the directory traversal attack?
    let pkg_path = format!(
        "{}/{}",
        mimium_dir.to_str().unwrap(),
        pkg_dsn.path().to_str().unwrap()
    );

    info!("Cloning into {:?}...", pkg_path);
    // NOTE: libgit2 cannot shallow clone...
    match Repository::clone(&repo_url, pkg_path.clone()) {
        Ok(_repo) => {
            info!("Successfuly cloned package as Git repository.");
            Ok(pkg_path)
        }
        Err(err) => {
            error!(
                "Cannot clone Git repository {:?} because {:?}",
                repo_url, err
            );
            Err(InstallError::CannotCloneGitRepository(err))
        }
    }
}

fn install_git_repo<'a>(
    mimium_dir: PathBuf,
    pkg_dsn: PackageDesignator,
) -> Result<(), InstallError<'a>> {
    if let PackageDesignator::Git { host, path } = pkg_dsn.clone() {
        info!("Install {:?} from {:?} as Git repository.", path, host);

        match clone_git_repo(mimium_dir, pkg_dsn) {
            Ok(pkg_path) => {
                let pkg_path = PathBuf::from(pkg_path);
                match is_mimium_package(&pkg_path) {
                    Ok(true) => Ok(()),
                    Ok(false) => {
                        error!("The repository {} is not a mimium package.", path.clone());

                        info!("Removing the repository {}...", path.clone());
                        if let Err(err) = fs::remove_dir_all(pkg_path.clone()) {
                            error!("Cannot remove repository because {}", err);
                        }
                        Err(InstallError::MalformedPackage)
                    }
                    Err(err) => Err(InstallError::IOError(err)),
                }
            }
            Err(err) => Err(err),
        }
    } else {
        Err(InstallError::Eden)
    }
}

fn install_package<'a>(mimium_dir: PathBuf, opt: CmdOption) -> Result<(), InstallError<'a>> {
    match opt.package_designator.clone() {
        PackageDesignator::Git { host: _, path: _ } => {
            install_git_repo(mimium_dir, opt.package_designator)
        }
        PackageDesignator::Pkg(_name) => Err(InstallError::PackageTypeIsNotImplemented),
        PackageDesignator::Path(_path) => Err(InstallError::PackageTypeIsNotImplemented),
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
