use git2::Repository;
use log::error;
use std::path::Path;

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
