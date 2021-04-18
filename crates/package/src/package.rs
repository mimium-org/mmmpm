use serde::Deserialize;

/// The package configuration.
/// This struct stores some information to run the package as a mimium program.
#[derive(Deserialize)]
pub struct PackageConfig {
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
pub struct Package {
    pub metadata: PackageMetadata,
    pub package: PackageConfig,
    // pub deps: PackageDependency,
}
