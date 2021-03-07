use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use log::{error, info};

use crate::constant;

// TODO: implement fmt::Display
#[derive(Debug, Clone)]
pub enum PackageDesignator {
    Pkg(String),
    Git { host: String, path: String },
    Path(Box<Path>),
}

impl PackageDesignator {
    pub fn from_str(s: String) -> Result<PackageDesignator, ()> {
        // TODO: Describe package designater convension

        if let Some(_) = s.find(':') {
            // if it's possibly a Git repository (including `:` like `github.com:mimium-org/mimium`
            let vec: Vec<&str> = s.splitn(2, ":").collect();
            if vec.len() == 2 {
                Ok(PackageDesignator::Git {
                    host: vec.get(0).unwrap().to_string(),
                    path: vec.get(1).unwrap().to_string(),
                })
            } else {
                error!("mulformed package designator");
                Err(())
            }
        } else {
            // if it's not a Git repos so it's wheather normal package or path
            // for now, all not-a-Git packages are normal packages
            Ok(PackageDesignator::Pkg(s.clone()))
        }
    }

    pub fn name(&self) -> String {
        match self {
            PackageDesignator::Pkg(name) => name.to_string(),
            PackageDesignator::Git { host: _, path } => {
                let path_buf = PathBuf::from(path.clone());
                path_buf.file_name().unwrap().to_str().unwrap().to_string()
            }
            PackageDesignator::Path(path) => {
                path.file_name().unwrap().to_str().unwrap().to_string()
            }
        }
    }

    pub fn path(&self) -> PathBuf {
        let name = self.name();

        match self.clone() {
            PackageDesignator::Pkg(_) => PathBuf::from(format!("mmmp/{}", name)),
            PackageDesignator::Git { host, path: _ } => PathBuf::from(format!("{}/{}", host, name)),
            PackageDesignator::Path(path) => path.to_path_buf(),
        }
    }

    pub fn remote_url(&self) -> Option<String> {
        match self {
            PackageDesignator::Pkg(_) => None,
            PackageDesignator::Git { host, path } => Some(format!("https://{}/{}.git", host, path)),
            PackageDesignator::Path(_) => None,
        }
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

pub struct Package {
    pub entrypoint: String,
}

impl Package {
    pub fn parse_path(_pkg_path: &Path) -> Result<Package, ()> {
        let pkg = Package {
            entrypoint: "test.mmm".to_string(),
        };
        Ok(pkg)
    }
}
