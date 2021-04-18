/// Denotes archive file retrieved from package host.
pub enum Archive {
    Zip(Vec<u8>),
}

/// Errors denotes operation failure to hosts.
pub enum HostOperationError {
    CannotConnectToHost(String),
    PackageNotFound,
    InvalidPackage,
}

/// Allowed operation to package hosts.
pub trait PackageHost {
    fn exists(&self) -> Result<bool, HostOperationError>;
    fn retrieve(&self) -> Result<Archive, HostOperationError>;
}

pub struct NotImplementedHost();

impl PackageHost for NotImplementedHost {
    fn exists(&self) -> Result<bool, HostOperationError> {
        Ok(false)
    }

    fn retrieve(&self) -> Result<Archive, HostOperationError> {
        Err(HostOperationError::PackageNotFound)
    }
}
