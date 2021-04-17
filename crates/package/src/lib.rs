use std::path::Path;

use serde::Deserialize;

/// The abstract representation of name of packages.
/// Each implementers of this trait denots a certain type of package.
pub trait PackageDesignator {
    //
    fn name(&self) -> String;
}

/// [**Not used now**]
/// Denotes mimium packages hosted by mimium package website in the future.
pub struct MimiumPackage(String);

/// Denotes mimium packages hosted as Git repository by GitHub.
pub struct GithubRepository {
    /// User/Organization name on GitHub.
    user: String,
    /// Repository name on GitHub.
    name: String,
}

/// Denotes packages that its type is not determined yet.
/// A package name specified by the user via CLI interface is treated as this type.
/// To use as a package that have concrete type, first, we must determine its type from its internal string.
pub struct UndeterminedPackage(String);

/// The package configuration.
/// This struct stores some information to run the package as a mimium program.
#[derive(Deserialize)]
pub struct Package {
    /// An entrypoint file path.
    /// This path must be a file and the file pointed by the path must contain `dsp` function,
    /// it is an entrypoint of mimium program.
    pub entrypoint: String,
}

/// The meta information of a package.
/// This is not needed to run a package, but is useful information to users.
#[derive(Deserialize)]
pub struct PackageMetadata {
    /// A name of this package.
    pub name: String,
    /// A version of this package.
    /// TODO: specify a format for version string.
    pub version: String,
    /// A description of this package.
    pub description: Option<String>,
    /// Authors of this package.
    pub authors: Vec<String>,
    /// Licenses applied to this package.
    pub licenses: Vec<String>,
}

/// The mimium package.
/// This includes metadata, configuration to run and, in the future, package dependency.
///
/// This object created from `mmm.toml` in the storage.
#[derive(Deserialize)]
pub struct MimiumPackage {
    pub metadata: PackageMetadata,
    pub package: Package,
    // pub deps: PackageDependency,
}

impl PackageDesignator for MimiumPackage {
    fn name(&self) -> String {
        self.0.clone()
    }
}

impl PackageDesignator for GithubRepository {
    fn name(&self) -> String {
        format!("{}/{}", self.user, self.name)
    }
}

impl PackageDesignator for UndeterminedPackage {
    fn name(&self) -> String {
        self.0.clone()
    }
}

impl UndeterminedPackage {
    /// Determine its package type from name string.
    pub fn determine(&self) -> Option<PackageDesignator> {
        let s = self.0;
        if let Some(_) = s.find(':') {
            let parts: Vec<&str> = s.splitn(2, ":").collect();
            if parts.len() == 2 {
                Some(GithubRepository {
                    user: parts.get(0).unwrap().to_string(),
                    name: parts.get(1).unwrap().to_string(),
                })
            } else {
                None
            }
        } else {
            None
        }
    }
}
