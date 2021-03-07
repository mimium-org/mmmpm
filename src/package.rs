use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use log::{error, info};

use crate::constant;

// TODO: implement fmt::Display
#[derive(Debug, Clone)]
pub enum Package {
    Pkg(String),
    Git { host: String, path: String },
    Path(Box<Path>),
}

impl Package {
    pub fn name(&self) -> String {
        match self {
            Package::Pkg(name) => name.to_string(),
            Package::Git { host: _, path } => {
                let path_buf = PathBuf::from(path.clone());
                path_buf.file_name().unwrap().to_str().unwrap().to_string()
            }
            Package::Path(path) => path.file_name().unwrap().to_str().unwrap().to_string(),
        }
    }

    pub fn path(&self) -> PathBuf {
        let name = self.name();

        match self.clone() {
            Package::Pkg(_) => PathBuf::from(format!("mmmp/{}", name)),
            Package::Git { host, path: _ } => PathBuf::from(format!("{}/{}", host, name)),
            Package::Path(path) => path.to_path_buf(),
        }
    }

    pub fn remote_url(&self) -> Option<String> {
        match self {
            Package::Pkg(_) => None,
            Package::Git { host, path } => Some(format!("https://{}/{}.git", host, path)),
            Package::Path(_) => None,
        }
    }
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
            if let Some(_) =
                entries.find(|e| e.as_ref().unwrap().file_name() == constant::MMMPM_PACKAGE_FILE)
            {
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
