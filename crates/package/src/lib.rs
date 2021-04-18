//! This crate provides some package related abstraction.
//!
//! First, the struct `Package` represents package itself. It contains program configuration,
//! metadata and dependencies of package. This struct is created by reading `mmm.toml` in the
//! directory in the storage.
//!
//! The struct `PackageDesignator` is a name and type of package. This is used to distinguish
//! user-specified package name strings.
//!
//! Ths struct `PackageHost` represents the hosting service of mimium packages.

extern crate serde;
extern crate toml;

mod designator;
mod host;
mod package;

pub use designator::*;
pub use host::*;
pub use package::*;
