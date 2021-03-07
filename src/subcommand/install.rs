use std::fs;
use std::path::{Path, PathBuf};

use clap::ArgMatches;
use git2::Repository;
use log::{error, info};

use crate::package::{package_from_string, Package};

struct CmdOption {
    package: Package,
}

fn parse_option(matches: &ArgMatches) -> Result<CmdOption, ()> {
    if let Ok(pkg) = package_from_string(String::from(matches.value_of("PACKAGE").unwrap())) {
        Ok(CmdOption { package: pkg })
    } else {
        Err(())
    }
}

fn clone_git_repo(mimium_dir: PathBuf, host: String, path: String) -> Result<String, ()> {
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
        Err(e) => {
            error!("Cannot clone Git repository {:?} because {:?}", repo_url, e);
            Err(())
        }
    }
}

fn is_mimium_package(pkg_path: &PathBuf) -> bool {
    info!("Validating repository cloned is a mimium package.");
    match fs::read_dir(pkg_path) {
        Ok(mut entries) => {
            if let Some(_) = entries.find(|e| e.as_ref().unwrap().file_name() == "mmmp.toml") {
                info!("'mmmp.toml' is found.");
                // TODO: is this package loadable?
                true
            } else {
                info!("'mmmp.toml' is not found.");
                false
            }
        }
        Err(err) => {
            error!("Read error: {:?}", err);
            false
        }
    }
}

fn install_git_repo(mimium_dir: PathBuf, host: String, path: String) -> Result<(), ()> {
    info!("install {:?} from {:?} as Git repository.", path, host);
    match clone_git_repo(mimium_dir, host, path.clone()) {
        Ok(pkg_path) => {
            let pkg_path = PathBuf::from(pkg_path);
            if is_mimium_package(&pkg_path) {
                Ok(())
            } else {
                error!("The repository {} is not a mimium package.", path.clone());
                info!("Removing the repository {}...", path.clone());
                if let Err(err) = fs::remove_dir_all(pkg_path.clone()) {
                    error!("Cannot remove repository because {}", err);
                }
                Err(())
            }
        }
        err => {
            error!("Clone error: {:?}", err);
            Err(())
        }
    }
}

fn proc(mimium_dir: PathBuf, opt: CmdOption) -> Result<(), ()> {
    match opt.package {
        Package::Git { host, path } => install_git_repo(mimium_dir, host, path),
        Package::Pkg(_name) => Err(()),
        Package::Path(_path) => Err(()),
    }
}

pub fn install(mimium_dir: PathBuf, matches: &ArgMatches) -> Result<(), ()> {
    if let Ok(opt) = parse_option(matches) {
        proc(mimium_dir, opt)
    } else {
        Err(())
    }
}
