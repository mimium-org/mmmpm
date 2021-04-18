use crate::host::{NotImplementedHost, PackageHost};

/// The abstract representation of name of packages.
/// Each implementers of this trait denots a certain type of package.
pub trait PackageDesignator {
    fn name(&self) -> String;
    fn host(&self) -> Box<dyn PackageHost>;
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

impl PackageDesignator for MimiumPackage {
    fn name(&self) -> String {
        self.0.clone()
    }

    fn host(&self) -> Box<dyn PackageHost> {
        Box::new(NotImplementedHost())
    }
}

impl PackageDesignator for GithubRepository {
    fn name(&self) -> String {
        format!("{}/{}", self.user, self.name)
    }

    fn host(&self) -> Box<dyn PackageHost> {
        Box::new(NotImplementedHost())
    }
}

impl UndeterminedPackage {
    /// Determine its package type from name string.
    pub fn determine(&self) -> Option<Box<dyn PackageDesignator>> {
        let s = &self.0;
        if let Some(_) = s.find(':') {
            let parts: Vec<&str> = s.splitn(2, ":").collect();
            if parts.len() == 2 {
                let github = GithubRepository {
                    user: parts.get(0).unwrap().to_string(),
                    name: parts.get(1).unwrap().to_string(),
                };
                Some(Box::new(github))
            } else {
                None
            }
        } else {
            None
        }
    }
}
