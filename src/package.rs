use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use log::{error, info};

use crate::constant::{MMMPM_GIT_DIR, MMMPM_PACKAGE_DIR, MMMPM_PACKAGE_FILE};

// TODO: implement fmt::Display
#[derive(Debug, Clone)]
pub enum PackageDesignator {
    Pkg(String),
    Git { host: String, path: String },
    Path(Box<Path>),
    Indeterminated(String),
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
            Ok(PackageDesignator::Indeterminated(s.clone()))
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
            PackageDesignator::Indeterminated(name) => name.to_string(),
        }
    }

    pub fn path(&self) -> Result<PathBuf, ()> {
        let name = self.name();

        match self.clone() {
            PackageDesignator::Pkg(_) => {
                Ok(PathBuf::from(format!("{}/{}", MMMPM_PACKAGE_DIR, name)))
            }
            PackageDesignator::Git { host, path: _ } => Ok(PathBuf::from(format!(
                "{}/{}/{}",
                MMMPM_GIT_DIR, host, name
            ))),
            PackageDesignator::Path(path) => Ok(path.to_path_buf()),
            PackageDesignator::Indeterminated(_) => Err(()),
        }
    }

    pub fn package_file_path(&self) -> Result<PathBuf, ()> {
        if let Ok(path) = self.path() {
            let mut path = path.clone();
            path.extend(&[MMMPM_PACKAGE_FILE.parse::<PathBuf>().unwrap()]);
            Ok(path)
        } else {
            Err(())
        }
    }

    pub fn remote_url(&self) -> Option<String> {
        match self {
            PackageDesignator::Pkg(_) => None,
            PackageDesignator::Git { host, path } => Some(format!("https://{}/{}.git", host, path)),
            PackageDesignator::Path(_) => None,
            PackageDesignator::Indeterminated(_) => None,
        }
    }

    pub fn determine(&self, root_dir: PathBuf) -> Result<Self, ()> {
        match self.clone() {
            PackageDesignator::Indeterminated(name) => {
                info!("Determine package type of {}", name.clone());

                // First, check if it's a normal package so search in `mmmp` directory
                info!("Check if it'a mmmpm package");
                let mut mmmpm_dir = root_dir.clone();
                mmmpm_dir.extend(&[MMMPM_PACKAGE_DIR]);
                if let Ok(mut entries) = fs::read_dir(mmmpm_dir) {
                    // Find same name directory, it's a package
                    if let Some(_) = entries.find(|e| {
                        let e = e.as_ref().unwrap();
                        e.file_type().unwrap().is_dir() && name == e.file_name().to_str().unwrap()
                    }) {
                        info!("It's a mmmpm package");
                        return Ok(PackageDesignator::Pkg(name.clone()));
                    }
                }

                // Second, check if it's a Git repo so search in Git repos.
                info!("Check if it'a git package");
                let mut git_dir = root_dir.clone();
                git_dir.extend(&[MMMPM_PACKAGE_DIR]);
                if let Ok(entries) = fs::read_dir(git_dir) {
                    // For all Git repository host directories ...
                    for entry in
                        entries.filter(|e| e.as_ref().unwrap().file_type().unwrap().is_dir())
                    {
                        let host_dir = entry.unwrap();
                        if let Ok(mut entries) = fs::read_dir(host_dir.path()) {
                            // Find same name directory, it's a package
                            if let Some(_) = entries.find(|e| {
                                let e = e.as_ref().unwrap();
                                e.file_type().unwrap().is_dir()
                                    && name == e.file_name().to_str().unwrap()
                            }) {
                                let host = host_dir.file_name().to_str().unwrap().to_string();
                                info!("It's a Git package in {}", host);
                                return Ok(PackageDesignator::Git {
                                    host: host,
                                    path: name.clone(),
                                });
                            }
                        }
                    }
                }

                // Finally, we cannot determine type because it's not found in .mimium directory.
                error!("Cannot determine its type for the package {}", name.clone());
                Err(())
            }
            pkg_dsn => Ok(pkg_dsn),
        }
    }

    pub fn exists(&self, root_dir: PathBuf) -> bool {
        if let Ok(path) = self.package_file_path() {
            let mut full_path = root_dir.clone();
            full_path.extend(&[path]);

            info!("Check if the `mmmp.toml` exists at {:?}", full_path);
            if let Ok(_) = fs::File::open(full_path.as_path()) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

pub fn is_mimium_package(pkg_path: &PathBuf) -> Result<bool, io::Error> {
    info!("Validating repository cloned is a mimium package.");
    match fs::read_dir(pkg_path) {
        Ok(mut entries) => {
            if let Some(_) = entries.find(|e| e.as_ref().unwrap().file_name() == MMMPM_PACKAGE_FILE)
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

#[derive(Deserialize)]
pub struct Package {
    pub entrypoint: String,
}

impl Package {
    pub fn from_path(pkg_path: &Path) -> Result<Package, toml::de::Error> {
        info!("Parse package file {:?}", pkg_path);

        let mut pkg_file = String::new();
        let mut f = fs::File::open(pkg_path).unwrap();
        let _ = f.read_to_string(&mut pkg_file);

        match toml::from_str(&pkg_file) {
            Ok(pkg) => Ok(pkg),
            Err(err) => Err(err),
        }
    }
}
