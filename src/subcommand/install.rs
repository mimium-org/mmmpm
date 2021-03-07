use std::fs;
use std::io;
use std::path::PathBuf;

use clap::ArgMatches;
use git2::{Error, Repository};
use log::{error, info};

use crate::package::{is_mimium_package, package_from_string, Package};

pub enum InstallError<'a> {
    InvalidOptions(&'a ArgMatches<'a>),
    CannotCloneGitRepository(Error),
    MalformedPackage,
    PackageTypeIsNotImplemented,
    IOError(io::Error),
}

struct CmdOption {
    package: Package,
}

fn parse_options<'a>(matches: &'a ArgMatches<'a>) -> Result<CmdOption, InstallError<'a>> {
    // initialize with dummy values
    let mut opts = CmdOption {
        package: Package::Pkg("***".to_string()),
    };

    if let Ok(pkg) = package_from_string(String::from(matches.value_of("PACKAGE").unwrap())) {
        opts.package = pkg;
    } else {
        return Err(InstallError::InvalidOptions(matches));
    }

    Ok(opts)
}

fn clone_git_repo<'a>(
    mimium_dir: PathBuf,
    host: String,
    path: String,
) -> Result<String, InstallError<'a>> {
    let repo_url = format!("https://{}/{}.git", host, path);
    // TODO: Protection from the directory traversal attack?
    let path_buf = PathBuf::from(path.clone());
    let repo_name = path_buf.file_name().unwrap().to_str().unwrap();
    let pkg_path = format!("{}/{}/{}", mimium_dir.to_str().unwrap(), host, &repo_name);

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
    host: String,
    path: String,
) -> Result<(), InstallError<'a>> {
    info!("install {:?} from {:?} as Git repository.", path, host);
    match clone_git_repo(mimium_dir, host, path.clone()) {
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
}

fn install_package<'a>(mimium_dir: PathBuf, opt: CmdOption) -> Result<(), InstallError<'a>> {
    match opt.package {
        Package::Git { host, path } => install_git_repo(mimium_dir, host, path),
        Package::Pkg(_name) => Err(InstallError::PackageTypeIsNotImplemented),
        Package::Path(_path) => Err(InstallError::PackageTypeIsNotImplemented),
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
