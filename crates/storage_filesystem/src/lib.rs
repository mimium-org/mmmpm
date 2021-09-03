use std::path;

use mmmpm_storage::{Object, ObjectKind, ObjectList, Path, StorageError, StorageOperation};

pub struct FilesystemStorage {
    root: Path,
}

impl FilesystemStorage {
    pub fn new(path: &path::Path) -> FilesystemStorage {
        FilesystemStorage { root: path }
    }
}

impl StorageOperation for FilesystemStorage {
    fn connect(self) -> Result<(), StorageError> {
        Ok(())
    }

    fn object_exists(self, path: &Path) -> Result<bool, StorageError> {
        Ok(false)
    }

    fn read_object(self, path: &Path) -> Result<Object, StorageError> {
        Ok(Object {
            kind: ObjectKind::Text,
            bin: None,
            text: None,
            child: None,
        })
    }

    fn write_object(self, path: &Path, obj: &Object) -> Result<(), StorageError> {
        Ok(())
    }

    fn create_dir(self, path: &Path) -> Result<(), StorageError> {
        Ok(())
    }

    fn read_dir(self, path: &Path) -> Result<ObjectList, StorageError> {
        let list = Vec::new();
        Ok(ObjectList::new(list))
    }
}
