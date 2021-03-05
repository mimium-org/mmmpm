use std::path::PathBuf;

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

fn proc(mimium_dir: PathBuf, opt: CmdOption) -> Result<(), ()> {
    match opt.package {
        Package::Git { host, path } => {
            info!("install {:?} from {:?} as Git repository.", path, host);

            let repo_url = format!("https://{}/{}.git", host, path);
            // TODO: Protection from the directory traversal attack?
            let pkg_dir = format!("{}/{}/{}", mimium_dir.to_str().unwrap(), host, path);

            info!("Cloning into {:?}...", pkg_dir);
            match Repository::clone(&repo_url, pkg_dir) {
                Ok(repo) => {
                    info!("Successfuly cloned package as Git repository.");
                    Ok(())
                }
                Err(e) => {
                    error!("Cannot clone Git repository {:?}", repo_url);
                    Err(())
                }
            }
        }
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
