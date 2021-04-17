//! Storage abstraction for mimium packages.
//!
//! This module is under designing so this is not *used* from any other code now.

use crate::package::{Package, PackageDesignator};

/// Path is a file designator in `Storage`. It is like an absolute path in filesystem.
pub struct Path {
    path: Vec<String>,
}

pub enum ObjKind {
    File,
    Dir,
}

pub enum PackageKind {
    Mimium,
    GitHub,
    Path,
}

pub struct ReadDir {
    pub package_kind: PackageKind,
    pub kind: ObjKind,
    pub path: Path,
}

/// Storage is where mimium package files are stored, like filesystem.
/// The purpose of it is to provide filesystem abstraction and, in the future, to support
/// another strage like something in browser.
pub trait Storage {
    fn exists(pkg: &Package) -> bool;

    fn read_text_file(pkg: &Package, path: &Path) -> Result<String, ()>;
    fn read_bin_file(pkg: &Package, path: &Path) -> Result<String, ()>;

    fn read_dir(pkg: &Package, path: &Path) -> Iterator<ReadDir>;
}
