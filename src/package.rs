use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use log::{error, info};

// TODO: implement fmt::Display
#[derive(Debug)]
pub enum Package {
    Pkg(String),
    Git { host: String, path: String },
    Path(Box<Path>),
}

pub fn package_from_string(pkg: String) -> Result<Package, ()> {
    if let Some(_) = pkg.find(':') {
        let vec: Vec<&str> = pkg.splitn(2, ":").collect();
        if vec.len() == 2 {
            Ok(Package::Git {
                host: vec.get(0).unwrap().to_string(),
                path: vec.get(1).unwrap().to_string(),
            })
        } else {
            error!("mulformed package designator");
            Err(())
        }
    } else {
        error!("not supported!");
        Err(())
    }
}

pub fn is_mimium_package(pkg_path: &PathBuf) -> Result<bool, io::Error> {
    info!("Validating repository cloned is a mimium package.");
    match fs::read_dir(pkg_path) {
        Ok(mut entries) => {
            if let Some(_) = entries.find(|e| e.as_ref().unwrap().file_name() == "mmmp.toml") {
                info!("'mmmp.toml' is found.");
                // TODO: is this package loadable?
                Ok(true)
            } else {
                info!("'mmmp.toml' is not found.");
                Ok(false)
            }
        }
        Err(err) => {
            error!("Read error: {:?}", err);
            Err(err)
        }
    }
}
