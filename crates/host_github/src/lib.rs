extern crate bytes;
extern crate reqwest;
extern crate zip;

use mmmpm_package::{Archive, HostOperationError, PackageHost};

const GITHUB_BASE_URL: &str = "https://github.com";
const GITHUB_API_BASE_URL: &str = "https://api.github.com";
const USER_AGENT: &str = "mmmpm";
const GIT_DEFAULT_REF: &str = "master";

pub struct GithubRepository {
    user: String,
    name: String,
    git_ref: String,
}

impl GithubRepository {
    pub fn new(user: String, name: String, git_ref: Option<String>) -> GithubRepository {
        GithubRepository {
            user: user.clone(),
            name: name.clone(),
            git_ref: if let Some(git_ref) = git_ref {
                git_ref
            } else {
                GIT_DEFAULT_REF.to_string()
            },
        }
    }
}

impl PackageHost for GithubRepository {
    fn exists(&self) -> Result<bool, HostOperationError> {
        let url = format!("{}/{}/{}", GITHUB_BASE_URL, self.user, self.name);
        let client = reqwest::blocking::Client::new();
        let result = client
            .get(url.clone())
            .header(reqwest::header::USER_AGENT, USER_AGENT)
            .send();

        match result {
            Ok(resp) => {
                let result = resp.error_for_status();
                if let Err(err) = result {
                    match err.status() {
                        Some(reqwest::StatusCode::NOT_FOUND) => {
                            Err(HostOperationError::PackageNotFound)
                        }
                        _ => Err(HostOperationError::UnknownError),
                    }
                } else {
                    Ok(true)
                }
            }
            Err(err) => {
                if err.is_connect() {
                    Err(HostOperationError::CannotConnectToHost(url))
                } else {
                    // TODO: log err
                    Err(HostOperationError::UnknownError)
                }
            }
        }
    }

    fn retrieve(&self) -> Result<Archive, HostOperationError> {
        let url = format!(
            "{}/repos/{}/{}/zipball/{}",
            GITHUB_API_BASE_URL, self.user, self.name, self.git_ref
        );
        let client = reqwest::blocking::Client::new();
        let result = client
            .get(url.clone())
            .header(reqwest::header::USER_AGENT, USER_AGENT)
            .send();

        match result {
            Ok(resp) => {
                let result = resp.error_for_status();
                match result {
                    Err(err) => {
                        match err.status() {
                            Some(reqwest::StatusCode::NOT_FOUND) => {
                                // This may not occur because check if the repos exists by `PackageHost.exists()`
                                Err(HostOperationError::PackageNotFound)
                            }
                            _ => Err(HostOperationError::UnknownError),
                        }
                    }
                    Ok(resp) => {
                        let body = resp.bytes().unwrap().to_vec();
                        Ok(Archive::Zip(body))
                    }
                }
            }
            Err(err) => {
                if err.is_connect() {
                    Err(HostOperationError::CannotConnectToHost(url))
                } else {
                    // TODO: log err
                    Err(HostOperationError::UnknownError)
                }
            }
        }
    }
}
