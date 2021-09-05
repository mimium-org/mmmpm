use std::path;

use mmmpm_storage::{Object, ObjectKind, ObjectList, Path, StorageError, StorageOperation};

pub struct FilesystemStorage {
    root: path::PathBuf,
}

impl FilesystemStorage {
    pub fn new(path: &path::Path) -> FilesystemStorage {
        FilesystemStorage {
            root: path.to_path_buf(),
        }
    }
}

impl StorageOperation for FilesystemStorage {
    fn connect(&self) -> Result<(), StorageError> {
        if !self.root.exists() {
            let msg = format!("Root path {:?} does not exist", self.root);
            return Err(StorageError::FailedToConnect(msg));
        }

        if !self.root.is_dir() {
            let msg = format!("Root path {:?} is not a directory", self.root);
            return Err(StorageError::FailedToConnect(msg));
        }

        Ok(())
    }

    fn object_exists(&self, path: &Path) -> Result<bool, StorageError> {
        Ok(false)
    }

    fn read_object(&self, path: &Path) -> Result<Object, StorageError> {
        Ok(Object {
            kind: ObjectKind::Text,
            bin: None,
            text: None,
            child: None,
        })
    }

    fn write_object(&self, path: &Path, obj: &Object) -> Result<(), StorageError> {
        Ok(())
    }

    fn create_dir(&self, path: &Path) -> Result<(), StorageError> {
        Ok(())
    }

    fn read_dir(&self, path: &Path) -> Result<ObjectList, StorageError> {
        let list = Vec::new();
        Ok(ObjectList::new(list))
    }
}
