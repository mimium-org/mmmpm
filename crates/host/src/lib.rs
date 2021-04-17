//! The package hosts abstraction.

use mmmpm_package::PackageDesignator;

/// Denotes archive file retrieved from package host.
pub struct Archive();

/// Errors denotes operation failure to hosts.
pub enum HostOperationError {
    CannotConnectToHost(String),
}

/// Allowed operation to package hosts.
pub trait PackageHost {
    fn exists(pkgdsn: &dyn PackageDesignator) -> Result<bool, HostOperationError>;
    fn retrieve(pkgdsn: &dyn PackageDesignator) -> Result<Archive, HostOperationError>;
}
