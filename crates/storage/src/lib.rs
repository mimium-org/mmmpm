//! Storage abstraction for mimium packages.
//!
//! This module is under designing so this is not *used* from any other code now.

#[derive(Debug)]
pub struct Path(Vec<String>);

impl Path {
    pub fn new(path: Vec<&str>) -> Path {
        let mut new_path = Vec::new();

        for o in path.iter() {
            new_path.push(o.to_string());
        }

        Path(new_path)
    }
}

#[derive(Debug)]
pub enum StorageError {
    StorageAccessError,
    FailedToConnect(String),
    ObjectNotFound,
    ObjectAlreadyExsits,
    NotADirectory,
}

#[derive(Clone)]
pub enum ObjectKind {
    Binary,
    Text,
    Directory,
}

/// Path is a file designator in `Storage`. It is like an absolute path in filesystem.
#[derive(Clone)]
pub struct Object {
    pub kind: ObjectKind,
    pub bin: Option<Vec<u8>>,
    pub text: Option<String>,
    pub child: Option<Vec<Object>>,
}

pub struct ObjectList {
    pub objects: Vec<Object>,
    count: usize,
}

impl ObjectList {
    pub fn new(objects: Vec<Object>) -> ObjectList {
        ObjectList {
            objects: objects,
            count: 0,
        }
    }
}

impl Iterator for ObjectList {
    type Item = Object;

    fn next(&mut self) -> Option<Self::Item> {
        if self.objects.len() < self.count {
            self.count += 1;
            Some(self.objects.get(self.count).unwrap().clone())
        } else {
            None
        }
    }
}

/// Storage is where mimium package files are stored, like filesystem.
/// The purpose of it is to provide filesystem abstraction and, in the future, to support
/// another strage like something in browser.
pub trait StorageOperation {
    // Start a session with a storage. This may occurs initialization like existence check,
    // creating directory etc.
    // This method may not be called, this is determined by implementation.
    fn connect(&self) -> Result<(), StorageError>;

    // Check if an object specified as `path` exists. This method does not read contents of the object.
    fn object_exists(&self, path: &Path) -> Result<bool, StorageError>;

    // Read object specified as `path` from the strorage.
    // If no object is found at `path`, this method returns `ObjectNotFound`.
    fn read_object(&self, path: &Path) -> Result<Object, StorageError>;

    // Write an object specified as `path` to the storage.
    // If an object already exists at `path`, this method returns `ObjectAlreadyExists`.
    fn write_object(&self, path: &Path, obj: &Object) -> Result<(), StorageError>;

    // Create a directory specified `path` and return created directory.
    fn create_dir(&self, path: &Path) -> Result<(), StorageError>;

    // Get all objects in the directory specified `path`.
    fn read_dir(&self, path: &Path) -> Result<ObjectList, StorageError>;
}
