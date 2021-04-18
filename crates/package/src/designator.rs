use mmmpm_host::{NotImplementedHost, PackageHost};

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
        let host =
            mmmpm_host_github::GithubRepository::new(self.user.clone(), self.name.clone(), None);
        Box::new(host)
    }
}

impl UndeterminedPackage {
    pub fn new(name: String) -> UndeterminedPackage {
        UndeterminedPackage(name)
    }

    /// Determine its package type from name string.
    pub fn determine(&self) -> Option<Box<dyn PackageDesignator>> {
        let s = &self.0;
        if let Some(_) = s.find(':') {
            let host_path: Vec<&str> = s.splitn(2, ":").collect();

            if host_path.len() != 2 {
                return None;
            }

            let _host = host_path.get(0).unwrap();
            let path = host_path.get(1).unwrap();
            let user_repo: Vec<&str> = path.splitn(2, "/").collect();

            if user_repo.len() != 2 {
                return None;
            }

            let github = GithubRepository {
                user: user_repo.get(0).unwrap().to_string(),
                name: user_repo.get(1).unwrap().to_string(),
            };
            Some(Box::new(github))
        } else {
            let mimium = MimiumPackage(s.to_string());
            Some(Box::new(mimium))
        }
    }
}
